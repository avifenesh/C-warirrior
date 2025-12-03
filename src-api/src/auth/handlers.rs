//! Auth endpoint handlers
//!
//! Implements register, login, logout, me, email verification, password reset, and OAuth.

use axum::{
    extract::{Query, State},
    http::{header, HeaderMap},
    response::{Json, Redirect},
};
use chrono::{Duration, Utc};
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use crate::db::models::{NewEmailToken, NewOAuthConnection, NewUser};
use crate::db::operations;
use crate::email::OptionalEmailService;

use super::jwt::{create_short_token, create_token, extract_bearer_token, verify_token};
use super::oauth::{GitHubOAuth, GoogleOAuth, OAuthState, OAuthUserInfo};
use super::password::{hash_password, hash_token, verify_password};
use super::{
    AuthError, AuthResponse, LoginRequest, RegisterRequest, RequestResetRequest,
    ResendVerifyRequest, ResetPasswordRequest, UserResponse, VerifyEmailRequest,
};

/// Application state for auth handlers
pub struct AuthState {
    pub db: Pool<Postgres>,
    /// Email service for verification/reset emails (optional - works without in dev)
    pub email: OptionalEmailService,
    /// Google OAuth client (optional - requires env vars)
    pub google_oauth: Option<GoogleOAuth>,
    /// GitHub OAuth client (optional - requires env vars)
    pub github_oauth: Option<GitHubOAuth>,
    /// Frontend URL for email links and OAuth redirects
    pub frontend_url: String,
}

/// POST /api/auth/register
pub async fn register(
    State(state): State<Arc<AuthState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    // Validate email format
    if !is_valid_email(&payload.email) {
        return Err(AuthError::InvalidEmail);
    }
    
    // Normalize email to lowercase
    let email = payload.email.to_lowercase();
    
    // Check if email already exists (using db::operations)
    if operations::get_user_by_email(&state.db, &email)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?
        .is_some()
    {
        return Err(AuthError::EmailExists);
    }
    
    // Check if username is taken (if provided)
    if let Some(ref username) = payload.username {
        if operations::get_user_by_username(&state.db, username)
            .await
            .map_err(|e| AuthError::Database(e.to_string()))?
            .is_some()
        {
            return Err(AuthError::UsernameExists);
        }
    }
    
    // Hash password (also validates strength)
    let password_hash = hash_password(&payload.password)?;
    
    // Create user using db::operations
    let new_user = NewUser {
        email,
        username: payload.username.clone(),
        password_hash,
    };
    
    let user = operations::create_user(&state.db, &new_user)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;
    
    // Create verification token (valid for 24 hours)
    let verify_token_str = create_short_token(user.id, &user.email, 24)?;
    let token_hash = hash_token(&verify_token_str);
    
    // Store verification token using db::operations
    let new_token = NewEmailToken {
        user_id: user.id,
        token_type: "verify".to_string(),
        token_hash,
        expires_at: Utc::now() + Duration::hours(24),
    };
    
    operations::create_email_token(&state.db, &new_token)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;
    
    // Send verification email
    if let Err(e) = state
        .email
        .send_verification(
            &user.email,
            user.username.clone(),
            &verify_token_str,
            &state.frontend_url,
        )
        .await
    {
        // Log but don't fail registration if email fails
        tracing::warn!("Failed to send verification email to {}: {}", user.email, e);
    }
    
    // Create session token
    let token = create_token(user.id, &user.email, user.total_xp as u32)?;
    
    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
            email_verified: user.email_verified,
            total_xp: user.total_xp as u32,
            created_at: user.created_at,
        },
    }))
}

/// POST /api/auth/login
pub async fn login(
    State(state): State<Arc<AuthState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    let email = payload.email.to_lowercase();
    
    // Fetch user by email using db::operations
    let user = operations::get_user_by_email(&state.db, &email)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?
        .ok_or(AuthError::InvalidCredentials)?;
    
    // Check if user has a password (might be OAuth-only)
    let password_hash = user.password_hash.as_ref().ok_or(AuthError::InvalidCredentials)?;
    
    // Verify password
    if !verify_password(&payload.password, password_hash)? {
        return Err(AuthError::InvalidCredentials);
    }
    
    // Check account status
    if user.is_blacklisted {
        return Err(AuthError::UserBlacklisted);
    }
    if user.is_suspended {
        return Err(AuthError::UserSuspended);
    }
    
    // Update last login using db::operations
    operations::update_last_login(&state.db, user.id)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;
    
    // Create token
    let token = create_token(user.id, &user.email, user.total_xp as u32)?;
    
    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
            email_verified: user.email_verified,
            total_xp: user.total_xp as u32,
            created_at: user.created_at,
        },
    }))
}

/// POST /api/auth/logout
pub async fn logout() -> Json<serde_json::Value> {
    // JWT tokens are stateless - client should discard the token
    // For enhanced security, we could maintain a token blocklist in Redis
    Json(json!({ "success": true, "message": "Logged out successfully" }))
}

/// GET /api/auth/me
pub async fn me(
    State(state): State<Arc<AuthState>>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, AuthError> {
    let claims = extract_and_verify_token(&headers)?;
    
    // Get user using db::operations
    let user = operations::get_user_by_id(&state.db, claims.sub)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?
        .ok_or(AuthError::UserNotFound)?;
    
    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        email_verified: user.email_verified,
        total_xp: user.total_xp as u32,
        created_at: user.created_at,
    }))
}

/// POST /api/auth/verify-email
pub async fn verify_email(
    State(state): State<Arc<AuthState>>,
    Json(payload): Json<VerifyEmailRequest>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // Verify the JWT token structure
    let claims = verify_token(&payload.token)?;
    let token_hash = hash_token(&payload.token);
    
    // Check if token exists and is valid using db::operations
    let token_record = operations::get_email_token_by_hash(&state.db, &token_hash, "verify")
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?
        .ok_or(AuthError::InvalidToken)?;
    
    // Verify user ID matches
    if token_record.user_id != claims.sub {
        return Err(AuthError::InvalidToken);
    }
    
    // Mark email as verified using db::operations
    operations::verify_user_email(&state.db, claims.sub)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;
    
    // Mark token as used using db::operations
    operations::mark_email_token_used(&state.db, token_record.id)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;
    
    Ok(Json(json!({ "success": true, "message": "Email verified successfully" })))
}

/// POST /api/auth/resend-verify
pub async fn resend_verify(
    State(state): State<Arc<AuthState>>,
    Json(payload): Json<ResendVerifyRequest>,
) -> Result<Json<serde_json::Value>, AuthError> {
    let email = payload.email.to_lowercase();
    
    // Find user using db::operations
    let user = operations::get_user_by_email(&state.db, &email)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;
    
    // Always return success to prevent email enumeration
    let Some(user) = user else {
        return Ok(Json(json!({ "success": true, "message": "If the email exists, a verification email has been sent" })));
    };
    
    if user.email_verified {
        return Ok(Json(json!({ "success": true, "message": "Email is already verified" })));
    }
    
    // Create new verification token (db::operations::create_email_token deletes old ones)
    let verify_token_str = create_short_token(user.id, &user.email, 24)?;
    let token_hash = hash_token(&verify_token_str);
    
    let new_token = NewEmailToken {
        user_id: user.id,
        token_type: "verify".to_string(),
        token_hash,
        expires_at: Utc::now() + Duration::hours(24),
    };
    
    operations::create_email_token(&state.db, &new_token)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;
    
    // Send verification email
    if let Err(e) = state
        .email
        .send_verification(&email, user.username, &verify_token_str, &state.frontend_url)
        .await
    {
        tracing::warn!("Failed to send verification email to {}: {}", email, e);
    }
    
    Ok(Json(json!({ "success": true, "message": "If the email exists, a verification email has been sent" })))
}

/// POST /api/auth/request-reset
pub async fn request_reset(
    State(state): State<Arc<AuthState>>,
    Json(payload): Json<RequestResetRequest>,
) -> Result<Json<serde_json::Value>, AuthError> {
    let email = payload.email.to_lowercase();
    
    // Find user using db::operations
    let user = operations::get_user_by_email(&state.db, &email)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;
    
    // Always return success to prevent email enumeration
    let Some(user) = user else {
        return Ok(Json(json!({ "success": true, "message": "If the email exists, a reset link has been sent" })));
    };
    
    // Create reset token (db::operations::create_email_token deletes old ones)
    let reset_token = create_short_token(user.id, &user.email, 1)?;
    let token_hash = hash_token(&reset_token);
    
    let new_token = NewEmailToken {
        user_id: user.id,
        token_type: "reset".to_string(),
        token_hash,
        expires_at: Utc::now() + Duration::hours(1),
    };
    
    operations::create_email_token(&state.db, &new_token)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;
    
    // Send password reset email
    if let Err(e) = state
        .email
        .send_password_reset(&email, user.username, &reset_token, &state.frontend_url)
        .await
    {
        tracing::warn!("Failed to send password reset email to {}: {}", email, e);
    }
    
    Ok(Json(json!({ "success": true, "message": "If the email exists, a reset link has been sent" })))
}

/// POST /api/auth/reset-password
pub async fn reset_password(
    State(state): State<Arc<AuthState>>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // Verify the JWT token structure
    let claims = verify_token(&payload.token)?;
    let token_hash = hash_token(&payload.token);
    
    // Check if token exists and is valid using db::operations
    let token_record = operations::get_email_token_by_hash(&state.db, &token_hash, "reset")
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?
        .ok_or(AuthError::InvalidToken)?;
    
    // Verify user ID matches
    if token_record.user_id != claims.sub {
        return Err(AuthError::InvalidToken);
    }
    
    // Hash new password (also validates strength)
    let password_hash = hash_password(&payload.new_password)?;
    
    // Update password using db::operations
    operations::update_user_password(&state.db, claims.sub, &password_hash)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;
    
    // Mark token as used using db::operations
    operations::mark_email_token_used(&state.db, token_record.id)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;
    
    // Delete all reset tokens for this user (invalidate any other reset links)
    // Note: This is done by creating a new token which deletes old ones, but we already
    // marked the current one as used. We could add a cleanup function, but for now
    // expired tokens will be cleaned up by the periodic cleanup task.
    
    Ok(Json(json!({ "success": true, "message": "Password reset successfully" })))
}

fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let local = parts[0];
    let domain = parts[1];
    
    !local.is_empty() 
        && !domain.is_empty() 
        && domain.contains('.') 
        && !domain.starts_with('.') 
        && !domain.ends_with('.')
        && email.len() <= 254
}

fn extract_and_verify_token(headers: &HeaderMap) -> Result<super::jwt::JwtClaims, AuthError> {
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::InvalidToken)?;
    
    let token = extract_bearer_token(auth_header).ok_or(AuthError::InvalidToken)?;
    
    verify_token(token)
}

// OAuth

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
}

/// GET /api/auth/oauth/google/start
pub async fn google_oauth_start(
    State(state): State<Arc<AuthState>>,
) -> Result<Redirect, AuthError> {
    let google = state
        .google_oauth
        .as_ref()
        .ok_or_else(|| AuthError::Internal("Google OAuth not configured".to_string()))?;

    let oauth_state = OAuthState::new(Some("/".to_string()));
    let auth_url = google.get_authorization_url(&oauth_state);

    Ok(Redirect::temporary(&auth_url))
}

/// GET /api/auth/oauth/google/callback
pub async fn google_oauth_callback(
    State(state): State<Arc<AuthState>>,
    Query(query): Query<OAuthCallbackQuery>,
) -> Result<Redirect, AuthError> {
    let google = state
        .google_oauth
        .as_ref()
        .ok_or_else(|| AuthError::Internal("Google OAuth not configured".to_string()))?;

    // Validate state parameter (CSRF protection)
    let oauth_state = OAuthState::decode(&query.state)
        .ok_or_else(|| AuthError::Internal("Invalid OAuth state".to_string()))?;

    // Exchange code for user info
    let user_info = google
        .authenticate(&query.code)
        .await
        .map_err(|e| AuthError::Internal(format!("Google auth failed: {}", e)))?;

    // Find or create user and generate token
    let (user, token) = find_or_create_oauth_user(&state, "google", &user_info).await?;

    // Redirect to frontend with token
    let redirect_to = oauth_state.redirect_to.unwrap_or_else(|| "/".to_string());
    let redirect_url = format!(
        "{}{}?token={}&user_id={}",
        state.frontend_url,
        redirect_to,
        urlencoding::encode(&token),
        user.id
    );

    Ok(Redirect::temporary(&redirect_url))
}

/// GET /api/auth/oauth/github/start
pub async fn github_oauth_start(
    State(state): State<Arc<AuthState>>,
) -> Result<Redirect, AuthError> {
    let github = state
        .github_oauth
        .as_ref()
        .ok_or_else(|| AuthError::Internal("GitHub OAuth not configured".to_string()))?;

    let oauth_state = OAuthState::new(Some("/".to_string()));
    let auth_url = github.get_authorization_url(&oauth_state);

    Ok(Redirect::temporary(&auth_url))
}

/// GET /api/auth/oauth/github/callback
pub async fn github_oauth_callback(
    State(state): State<Arc<AuthState>>,
    Query(query): Query<OAuthCallbackQuery>,
) -> Result<Redirect, AuthError> {
    let github = state
        .github_oauth
        .as_ref()
        .ok_or_else(|| AuthError::Internal("GitHub OAuth not configured".to_string()))?;

    // Validate state parameter (CSRF protection)
    let oauth_state = OAuthState::decode(&query.state)
        .ok_or_else(|| AuthError::Internal("Invalid OAuth state".to_string()))?;

    // Exchange code for user info
    let user_info = github
        .authenticate(&query.code)
        .await
        .map_err(|e| AuthError::Internal(format!("GitHub auth failed: {}", e)))?;

    // Find or create user and generate token
    let (user, token) = find_or_create_oauth_user(&state, "github", &user_info).await?;

    // Redirect to frontend with token
    let redirect_to = oauth_state.redirect_to.unwrap_or_else(|| "/".to_string());
    let redirect_url = format!(
        "{}{}?token={}&user_id={}",
        state.frontend_url,
        redirect_to,
        urlencoding::encode(&token),
        user.id
    );

    Ok(Redirect::temporary(&redirect_url))
}

/// Helper: Find existing user by OAuth connection or create new user
async fn find_or_create_oauth_user(
    state: &AuthState,
    provider: &str,
    user_info: &OAuthUserInfo,
) -> Result<(crate::db::models::User, String), AuthError> {
    // Check if OAuth connection already exists
    let existing_connection =
        operations::get_oauth_connection(&state.db, provider, &user_info.provider_user_id)
            .await
            .map_err(|e| AuthError::Database(e.to_string()))?;

    let user = if let Some(connection) = existing_connection {
        // User already linked - fetch their account
        operations::get_user_by_id(&state.db, connection.user_id)
            .await
            .map_err(|e| AuthError::Database(e.to_string()))?
            .ok_or(AuthError::UserNotFound)?
    } else {
        // Check if email already exists (link to existing account)
        let existing_user = operations::get_user_by_email(&state.db, &user_info.email)
            .await
            .map_err(|e| AuthError::Database(e.to_string()))?;

        let user = if let Some(user) = existing_user {
            // Link OAuth to existing account
            user
        } else {
            // Create new user from OAuth
            let new_user = NewUser {
                email: user_info.email.clone(),
                username: user_info.name.clone(),
                password_hash: String::new(), // No password for OAuth users
            };

            let mut user = operations::create_user(&state.db, &new_user)
                .await
                .map_err(|e| AuthError::Database(e.to_string()))?;

            // Mark email as verified if provider verified it
            if user_info.email_verified {
                operations::verify_user_email(&state.db, user.id)
                    .await
                    .map_err(|e| AuthError::Database(e.to_string()))?;
                user.email_verified = true;
            }

            user
        };

        // Create OAuth connection
        let new_connection = NewOAuthConnection {
            access_token: None,
            provider_email: None,
            refresh_token: None,
            user_id: user.id,
            provider: provider.to_string(),
            provider_user_id: user_info.provider_user_id.clone(),
        };

        operations::create_oauth_connection(&state.db, &new_connection)
            .await
            .map_err(|e| AuthError::Database(e.to_string()))?;

        user
    };

    // Check account status
    if user.is_blacklisted {
        return Err(AuthError::UserBlacklisted);
    }
    if user.is_suspended {
        return Err(AuthError::UserSuspended);
    }

    // Update last login
    operations::update_last_login(&state.db, user.id)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;

    // Generate JWT token
    let token = create_token(user.id, &user.email, user.total_xp as u32)?;

    Ok((user, token))
}
