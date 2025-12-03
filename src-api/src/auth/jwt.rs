//! JWT token generation and validation
//!
//! Uses HS256 symmetric signing with configurable expiry.

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::AuthError;

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// User ID (subject)
    pub sub: Uuid,
    /// User email
    pub email: String,
    /// Expiry timestamp (Unix epoch)
    pub exp: usize,
    /// Issued at timestamp (Unix epoch)
    pub iat: usize,
    /// User's XP (for adaptive rate limiting)
    pub xp: u32,
}

impl JwtClaims {
    /// Create new claims for a user
    pub fn new(user_id: Uuid, email: String, xp: u32, expires_in: Duration) -> Self {
        let now = Utc::now();
        let exp = (now + expires_in).timestamp() as usize;
        let iat = now.timestamp() as usize;
        
        Self {
            sub: user_id,
            email,
            exp,
            iat,
            xp,
        }
    }
}

/// Get the JWT secret from environment
fn get_jwt_secret() -> Result<String, AuthError> {
    std::env::var("JWT_SECRET").map_err(|_| {
        AuthError::Internal("JWT_SECRET not configured".to_string())
    })
}

/// Create a JWT token for a user
pub fn create_token(user_id: Uuid, email: &str, xp: u32) -> Result<String, AuthError> {
    let secret = get_jwt_secret()?;
    
    // Token expires in 7 days
    let claims = JwtClaims::new(user_id, email.to_string(), xp, Duration::days(7));
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AuthError::Internal(format!("Failed to create token: {}", e)))
}

/// Create a short-lived token for email verification or password reset
pub fn create_short_token(user_id: Uuid, email: &str, expires_in_hours: i64) -> Result<String, AuthError> {
    let secret = get_jwt_secret()?;
    
    let claims = JwtClaims::new(user_id, email.to_string(), 0, Duration::hours(expires_in_hours));
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AuthError::Internal(format!("Failed to create token: {}", e)))
}

/// Verify and decode a JWT token
pub fn verify_token(token: &str) -> Result<JwtClaims, AuthError> {
    let secret = get_jwt_secret()?;
    
    let token_data: TokenData<JwtClaims> = decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        tracing::debug!("Token verification failed: {}", e);
        AuthError::InvalidToken
    })?;
    
    Ok(token_data.claims)
}

/// Extract bearer token from Authorization header
pub fn extract_bearer_token(auth_header: &str) -> Option<&str> {
    auth_header.strip_prefix("Bearer ").or_else(|| auth_header.strip_prefix("bearer "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_token() {
        // Set test secret
        std::env::set_var("JWT_SECRET", "test-secret-key-for-testing-only-32bytes!");
        
        let user_id = Uuid::new_v4();
        let email = "test@example.com";
        let xp = 100;
        
        let token = create_token(user_id, email, xp).expect("should create token");
        let claims = verify_token(&token).expect("should verify token");
        
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.xp, xp);
    }

    #[test]
    fn test_extract_bearer_token() {
        assert_eq!(extract_bearer_token("Bearer abc123"), Some("abc123"));
        assert_eq!(extract_bearer_token("bearer abc123"), Some("abc123"));
        assert_eq!(extract_bearer_token("Basic abc123"), None);
    }
    
    #[test]
    fn test_short_token() {
        std::env::set_var("JWT_SECRET", "test-secret-key-for-testing-only-32bytes!");
        
        let user_id = Uuid::new_v4();
        let email = "test@example.com";
        
        let token = create_short_token(user_id, email, 1).expect("should create short token");
        let claims = verify_token(&token).expect("should verify token");
        
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.xp, 0); // Short tokens have 0 XP
    }
}
