//! Password hashing and verification using Argon2id

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sha2::{Digest, Sha256};

use super::AuthError;

/// Hash a password using Argon2id
pub fn hash_password(password: &str) -> Result<String, AuthError> {
    // Validate password strength first
    validate_password_strength(password)?;
    
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AuthError::Internal(format!("Failed to hash password: {}", e)))
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AuthError::Internal(format!("Invalid password hash format: {}", e)))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Validate password strength
pub fn validate_password_strength(password: &str) -> Result<(), AuthError> {
    if password.len() < 8 {
        return Err(AuthError::WeakPassword(
            "Password must be at least 8 characters long".to_string(),
        ));
    }

    if password.len() > 128 {
        return Err(AuthError::WeakPassword(
            "Password must be at most 128 characters long".to_string(),
        ));
    }

    // Check for at least one letter and one number
    let has_letter = password.chars().any(|c| c.is_alphabetic());
    let has_number = password.chars().any(|c| c.is_numeric());

    if !has_letter || !has_number {
        return Err(AuthError::WeakPassword(
            "Password must contain at least one letter and one number".to_string(),
        ));
    }

    Ok(())
}

/// Generate a secure random token (for email verification, password reset)
pub fn generate_secure_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    hex::encode(bytes)
}

/// Hash a token for storage (we don't store raw tokens)
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_verify() {
        let password = "TestPassword123";
        let hash = hash_password(password).unwrap();

        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("WrongPassword123", &hash).unwrap());
    }

    #[test]
    fn test_password_validation() {
        // Too short
        assert!(matches!(
            validate_password_strength("short1"),
            Err(AuthError::WeakPassword(_))
        ));

        // No number
        assert!(matches!(
            validate_password_strength("NoNumberHere"),
            Err(AuthError::WeakPassword(_))
        ));

        // No letter
        assert!(matches!(
            validate_password_strength("12345678"),
            Err(AuthError::WeakPassword(_))
        ));

        // Valid
        assert!(validate_password_strength("ValidPass123").is_ok());
    }

    #[test]
    fn test_secure_token() {
        let token1 = generate_secure_token();
        let token2 = generate_secure_token();

        assert_eq!(token1.len(), 64); // 32 bytes = 64 hex chars
        assert_ne!(token1, token2);
    }
    
    #[test]
    fn test_hash_token() {
        let token = "test_token_123";
        let hash1 = hash_token(token);
        let hash2 = hash_token(token);
        
        // Same input should produce same hash
        assert_eq!(hash1, hash2);
        
        // Different input should produce different hash
        let hash3 = hash_token("different_token");
        assert_ne!(hash1, hash3);
    }
}
