//! Verification check middleware
//!
//! Checks if the user's email is verified.
//! Must run after JWT auth middleware (needs AuthUser in extensions).

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use super::auth::AuthUser;

/// Response for unverified users
#[derive(Debug, Serialize)]
pub struct VerificationResponse {
    pub error: String,
    pub code: String,
    pub message: String,
}

/// Verification check middleware
///
/// Checks if the authenticated user has a verified email.
/// Returns 403 Forbidden if email is not verified.
///
/// Must be used after jwt_auth_middleware.
pub async fn verification_check_middleware(
    Extension(pool): Extension<Arc<Pool<Postgres>>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    // Get authenticated user from extensions
    let auth_user = match req.extensions().get::<AuthUser>() {
        Some(user) => user.clone(),
        None => {
            // No auth user - this middleware should only be used on authenticated routes
            // Let the request through (auth middleware will handle it)
            return Ok(next.run(req).await);
        }
    };

    // Check verification status in database
    // We check DB because verification status might change after token issuance
    let email_verified: Option<bool> = sqlx::query_scalar(
        r#"
        SELECT email_verified
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check verification status: {}", e);
        // On database error, default to block for security? Or fail open?
        // Let's return 500
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()).into_response()
    })?;

    match email_verified {
        Some(true) => {
            // Verified - continue
            Ok(next.run(req).await)
        }
        Some(false) => {
            // Not verified
            tracing::warn!("Unverified user {} attempted access", auth_user.user_id);
            Err((
                StatusCode::FORBIDDEN,
                Json(VerificationResponse {
                    error: "forbidden".to_string(),
                    code: "EMAIL_NOT_VERIFIED".to_string(),
                    message: "Please verify your email address to access this resource.".to_string(),
                }),
            )
                .into_response())
        }
        None => {
            // User not found (should happen rarely if token is valid)
            Err((StatusCode::UNAUTHORIZED, "User not found".to_string()).into_response())
        }
    }
}
