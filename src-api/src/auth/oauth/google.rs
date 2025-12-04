//! Google OAuth 2.0 implementation

use super::{OAuthConfig, OAuthState, OAuthUserInfo};
use serde::Deserialize;

/// Google OAuth client
#[derive(Clone)]
pub struct GoogleOAuth {
    config: OAuthConfig,
    client: reqwest::Client,
}

/// Google OAuth token response (only access_token is used, serde ignores other fields)
#[derive(Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

/// Google user info response
#[derive(Deserialize)]
struct GoogleUserInfo {
    id: String,
    email: String,
    verified_email: bool,
    name: Option<String>,
    picture: Option<String>,
}

impl GoogleOAuth {
    /// Create a new Google OAuth client
    pub fn new(config: OAuthConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// Create from environment variables
    pub fn from_env(api_base_url: &str) -> Option<Self> {
        let config = OAuthConfig {
            client_id: std::env::var("GOOGLE_CLIENT_ID").ok()?,
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET").ok()?,
            redirect_uri: format!("{}/api/auth/oauth/google/callback", api_base_url),
        };
        Some(Self::new(config))
    }

    /// Generate the authorization URL for Google OAuth
    pub fn get_authorization_url(&self, state: &OAuthState) -> String {
        let scopes = "openid email profile";
        let encoded_state = state.encode();

        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?\
            client_id={}&\
            redirect_uri={}&\
            response_type=code&\
            scope={}&\
            state={}&\
            access_type=offline&\
            prompt=consent",
            urlencoding::encode(&self.config.client_id),
            urlencoding::encode(&self.config.redirect_uri),
            urlencoding::encode(scopes),
            urlencoding::encode(&encoded_state)
        )
    }

    /// Exchange authorization code for access token
    pub async fn exchange_code(&self, code: &str) -> Result<String, String> {
        let params = [
            ("client_id", self.config.client_id.as_str()),
            ("client_secret", self.config.client_secret.as_str()),
            ("code", code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", self.config.redirect_uri.as_str()),
        ];

        let response = self
            .client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Failed to exchange code: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!("Google token exchange failed: {}", error_text);
            return Err(format!("Token exchange failed: {}", error_text));
        }

        let token_response: GoogleTokenResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse token response: {}", e))?;

        Ok(token_response.access_token)
    }

    /// Get user info using access token
    pub async fn get_user_info(&self, access_token: &str) -> Result<OAuthUserInfo, String> {
        let response = self
            .client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| format!("Failed to get user info: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!("Google user info failed: {}", error_text);
            return Err(format!("Failed to get user info: {}", error_text));
        }

        let user_info: GoogleUserInfo = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse user info: {}", e))?;

        Ok(OAuthUserInfo {
            provider_user_id: user_info.id,
            email: user_info.email,
            name: user_info.name,
            avatar_url: user_info.picture,
            email_verified: user_info.verified_email,
        })
    }

    /// Complete OAuth flow: exchange code and get user info
    pub async fn authenticate(&self, code: &str) -> Result<OAuthUserInfo, String> {
        let access_token = self.exchange_code(code).await?;
        self.get_user_info(&access_token).await
    }
}
