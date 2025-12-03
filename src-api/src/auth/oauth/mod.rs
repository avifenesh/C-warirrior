//! OAuth authentication providers (Google, GitHub)

pub mod github;
pub mod google;

pub use github::GitHubOAuth;
pub use google::GoogleOAuth;

use serde::{Deserialize, Serialize};

/// OAuth user info returned by providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    /// Provider-specific user ID
    pub provider_user_id: String,
    /// User's email address
    pub email: String,
    /// User's display name (if available)
    pub name: Option<String>,
    /// User's avatar URL (if available)
    pub avatar_url: Option<String>,
    /// Whether the email is verified by the provider
    pub email_verified: bool,
}

/// OAuth provider configuration
#[derive(Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl OAuthConfig {
    /// Load Google OAuth config from environment
    pub fn google_from_env(frontend_url: &str) -> Option<Self> {
        Some(Self {
            client_id: std::env::var("GOOGLE_CLIENT_ID").ok()?,
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET").ok()?,
            redirect_uri: format!("{}/api/auth/oauth/google/callback", frontend_url),
        })
    }

    /// Load GitHub OAuth config from environment
    pub fn github_from_env(frontend_url: &str) -> Option<Self> {
        Some(Self {
            client_id: std::env::var("GITHUB_CLIENT_ID").ok()?,
            client_secret: std::env::var("GITHUB_CLIENT_SECRET").ok()?,
            redirect_uri: format!("{}/api/auth/oauth/github/callback", frontend_url),
        })
    }
}

/// State parameter for OAuth flow (prevents CSRF)
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthState {
    /// Random nonce
    pub nonce: String,
    /// Where to redirect after auth
    pub redirect_to: Option<String>,
}

impl OAuthState {
    pub fn new(redirect_to: Option<String>) -> Self {
        use rand::Rng;
        let nonce: [u8; 16] = rand::thread_rng().gen();
        Self {
            nonce: hex::encode(nonce),
            redirect_to,
        }
    }

    pub fn encode(&self) -> String {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        let json = serde_json::to_string(self).unwrap_or_default();
        URL_SAFE_NO_PAD.encode(json.as_bytes())
    }

    pub fn decode(encoded: &str) -> Option<Self> {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        let bytes = URL_SAFE_NO_PAD.decode(encoded).ok()?;
        let json = String::from_utf8(bytes).ok()?;
        serde_json::from_str(&json).ok()
    }
}

