//! Ban and suspension check middleware
//!
//! Checks if a user is banned or suspended before allowing access.
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
use uuid::Uuid;

use super::auth::AuthUser;

/// Response for banned/suspended users
#[derive(Debug, Serialize)]
pub struct BanResponse {
    pub error: String,
    pub code: String,
    pub message: String,
}

/// Check user status (suspended/blacklisted) from database
async fn check_user_status(
    pool: &Pool<Postgres>,
    user_id: Uuid,
) -> Result<(bool, bool), sqlx::Error> {
    let result: Option<(bool, bool)> = sqlx::query_as(
        r#"
        SELECT is_suspended, is_blacklisted
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(result.unwrap_or((false, false)))
}

/// Ban check middleware
///
/// Checks if the authenticated user is banned or suspended.
/// Returns 403 Forbidden if user is banned, 423 Locked if suspended.
///
/// Must be used after jwt_auth_middleware.
pub async fn ban_check_middleware(
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

    // Check user status in database
    let (is_suspended, is_blacklisted) = match check_user_status(&pool, auth_user.user_id).await {
        Ok(status) => status,
        Err(e) => {
            tracing::error!("Failed to check user status: {}", e);
            // On database error, let request through rather than blocking
            return Ok(next.run(req).await);
        }
    };

    // Check if blacklisted (permanent ban)
    if is_blacklisted {
        tracing::warn!("Blacklisted user {} attempted access", auth_user.user_id);
        return Err((
            StatusCode::FORBIDDEN,
            Json(BanResponse {
                error: "forbidden".to_string(),
                code: "USER_BLACKLISTED".to_string(),
                message: "Your account has been permanently banned.".to_string(),
            }),
        )
            .into_response());
    }

    // Check if suspended (temporary)
    if is_suspended {
        tracing::warn!("Suspended user {} attempted access", auth_user.user_id);
        return Err((
            StatusCode::LOCKED,
            Json(BanResponse {
                error: "locked".to_string(),
                code: "USER_SUSPENDED".to_string(),
                message: "Your account has been temporarily suspended.".to_string(),
            }),
        )
            .into_response());
    }

    // User is not banned - continue
    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ban_response_serialization() {
        let response = BanResponse {
            error: "forbidden".to_string(),
            code: "USER_BLACKLISTED".to_string(),
            message: "Your account has been permanently banned.".to_string(),
        };
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("USER_BLACKLISTED"));
    }
}

