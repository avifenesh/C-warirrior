//! JWT authentication middleware
//!
//! Validates JWT tokens and extracts user information for protected routes.

use axum::{
    body::Body,
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,        // user_id
    pub email: String,
    pub exp: usize,       // expiry timestamp
    pub iat: usize,       // issued at
    pub xp: u32,          // for adaptive rate limiting
}

/// Extracted user ID from JWT - injected into request extensions
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
    pub xp: u32,
}

/// Get JWT secret from environment
fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

/// Validate a JWT token and extract claims
pub fn validate_token(token: &str) -> Result<JwtClaims, String> {
    let secret = get_jwt_secret();
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();

    decode::<JwtClaims>(token, &key, &validation)
        .map(|data| data.claims)
        .map_err(|e| format!("Invalid token: {}", e))
}

/// JWT authentication middleware
///
/// Extracts JWT from Authorization header, validates it, and injects AuthUser into request.
/// Returns 401 Unauthorized if token is missing or invalid.
pub async fn jwt_auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Authorization header
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Extract Bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate token and extract claims
    let claims = validate_token(token).map_err(|e| {
        tracing::warn!("JWT validation failed: {}", e);
        StatusCode::UNAUTHORIZED
    })?;

    // Check if token is expired (jsonwebtoken does this, but double-check)
    let now = chrono::Utc::now().timestamp() as usize;
    if claims.exp < now {
        tracing::warn!("JWT token expired for user {}", claims.sub);
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Inject AuthUser into request extensions
    let auth_user = AuthUser {
        user_id: claims.sub,
        email: claims.email,
        xp: claims.xp,
    };
    req.extensions_mut().insert(auth_user);

    // Continue to next middleware/handler
    Ok(next.run(req).await)
}

/// Optional JWT authentication middleware
///
/// Like jwt_auth_middleware, but doesn't fail if token is missing.
/// Useful for routes that work for both authenticated and anonymous users.
pub async fn optional_jwt_auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Response {
    // Try to extract Authorization header
    if let Some(auth_header) = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    {
        // Try to extract Bearer token
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            // Try to validate token
            if let Ok(claims) = validate_token(token) {
                let now = chrono::Utc::now().timestamp() as usize;
                if claims.exp >= now {
                    // Valid token - inject AuthUser
                    let auth_user = AuthUser {
                        user_id: claims.sub,
                        email: claims.email,
                        xp: claims.xp,
                    };
                    req.extensions_mut().insert(auth_user);
                }
            }
        }
    }

    // Continue regardless of auth status
    next.run(req).await
}

/// Extract AuthUser from request extensions
///
/// Use this in handlers to get the authenticated user:
/// ```rust
/// async fn my_handler(
///     Extension(auth_user): Extension<AuthUser>,
/// ) -> impl IntoResponse {
///     // auth_user.user_id, auth_user.email, auth_user.xp
/// }
/// ```
pub fn extract_auth_user(req: &Request<Body>) -> Option<&AuthUser> {
    req.extensions().get::<AuthUser>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_claims_structure() {
        let claims = JwtClaims {
            sub: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            exp: 1234567890,
            iat: 1234567800,
            xp: 100,
        };
        
        assert!(!claims.email.is_empty());
        assert!(claims.exp > claims.iat);
    }
}

