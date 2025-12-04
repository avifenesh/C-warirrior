//! Authentication module for Code Warrior API
//!
//! Provides email/password authentication with JWT tokens.
//! OAuth (Google, GitHub) and email services.

pub mod handlers;
pub mod jwt;
pub mod oauth;
pub mod password;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Standard auth response with token and user info
#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

/// User info returned in auth responses
#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: Option<String>,
    pub email_verified: bool,
    pub total_xp: u32,
    pub created_at: DateTime<Utc>,
}

/// Register request payload
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
}

/// Login request payload
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Email verification request
#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

/// Resend verification email request
#[derive(Debug, Deserialize)]
pub struct ResendVerifyRequest {
    pub email: String,
}

/// Request password reset
#[derive(Debug, Deserialize)]
pub struct RequestResetRequest {
    pub email: String,
}

/// Reset password with token
#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

/// Auth error types
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Email already registered")]
    EmailExists,

    #[error("Username already taken")]
    UsernameExists,

    #[error("Invalid or expired token")]
    InvalidToken,

    #[error("User is suspended")]
    UserSuspended,

    #[error("User is blacklisted")]
    UserBlacklisted,

    #[error("Password too weak: {0}")]
    WeakPassword(String),

    #[error("Invalid email format")]
    InvalidEmail,

    #[error("User not found")]
    UserNotFound,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl AuthError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        use axum::http::StatusCode;
        match self {
            AuthError::InvalidCredentials | AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::EmailExists | AuthError::UsernameExists => StatusCode::CONFLICT,
            AuthError::UserSuspended | AuthError::UserBlacklisted => StatusCode::FORBIDDEN,
            AuthError::WeakPassword(_) | AuthError::InvalidEmail => StatusCode::BAD_REQUEST,
            AuthError::UserNotFound => StatusCode::NOT_FOUND,
            AuthError::Database(_) | AuthError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl axum::response::IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let body = serde_json::json!({
            "error": self.to_string(),
            "code": format!("{:?}", self).split('(').next().unwrap_or("Unknown")
        });
        (status, axum::Json(body)).into_response()
    }
}
