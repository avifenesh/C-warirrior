//! GitHub OAuth 2.0 implementation

use super::{OAuthConfig, OAuthState, OAuthUserInfo};
use serde::Deserialize;

/// GitHub OAuth client
#[derive(Clone)]
pub struct GitHubOAuth {
    config: OAuthConfig,
    client: reqwest::Client,
}

/// GitHub OAuth token response
#[derive(Deserialize)]
struct GitHubTokenResponse {
    access_token: String,
    #[allow(dead_code)]
    token_type: String,
    #[allow(dead_code)]
    scope: String,
}

/// GitHub user info response
#[derive(Deserialize)]
struct GitHubUserInfo {
    id: i64,
    login: String,
    email: Option<String>,
    name: Option<String>,
    avatar_url: Option<String>,
}

/// GitHub email response (for getting primary email)
#[derive(Deserialize)]
struct GitHubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

impl GitHubOAuth {
    /// Create a new GitHub OAuth client
    pub fn new(config: OAuthConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::builder()
                .user_agent("Code-Warrior-App")
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    /// Create from environment variables
    pub fn from_env(api_base_url: &str) -> Option<Self> {
        let config = OAuthConfig {
            client_id: std::env::var("GITHUB_CLIENT_ID").ok()?,
            client_secret: std::env::var("GITHUB_CLIENT_SECRET").ok()?,
            redirect_uri: format!("{}/api/auth/oauth/github/callback", api_base_url),
        };
        Some(Self::new(config))
    }

    /// Generate the authorization URL for GitHub OAuth
    pub fn get_authorization_url(&self, state: &OAuthState) -> String {
        let scopes = "user:email read:user";
        let encoded_state = state.encode();

        format!(
            "https://github.com/login/oauth/authorize?\
            client_id={}&\
            redirect_uri={}&\
            scope={}&\
            state={}",
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
            ("redirect_uri", self.config.redirect_uri.as_str()),
        ];

        let response = self
            .client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Failed to exchange code: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!("GitHub token exchange failed: {}", error_text);
            return Err(format!("Token exchange failed: {}", error_text));
        }

        let token_response: GitHubTokenResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse token response: {}", e))?;

        Ok(token_response.access_token)
    }

    /// Get user info using access token
    pub async fn get_user_info(&self, access_token: &str) -> Result<OAuthUserInfo, String> {
        // Get basic user info
        let user_response = self
            .client
            .get("https://api.github.com/user")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| format!("Failed to get user info: {}", e))?;

        if !user_response.status().is_success() {
            let error_text = user_response.text().await.unwrap_or_default();
            tracing::error!("GitHub user info failed: {}", error_text);
            return Err(format!("Failed to get user info: {}", error_text));
        }

        let user_info: GitHubUserInfo = user_response
            .json()
            .await
            .map_err(|e| format!("Failed to parse user info: {}", e))?;

        // Get primary email (may not be in user info if private)
        let (email, email_verified) = if let Some(email) = user_info.email {
            (email, true) // Public email is considered verified
        } else {
            self.get_primary_email(access_token).await?
        };

        Ok(OAuthUserInfo {
            provider_user_id: user_info.id.to_string(),
            email,
            name: user_info.name.or(Some(user_info.login)),
            avatar_url: user_info.avatar_url,
            email_verified,
        })
    }

    /// Get primary verified email from GitHub
    async fn get_primary_email(&self, access_token: &str) -> Result<(String, bool), String> {
        let response = self
            .client
            .get("https://api.github.com/user/emails")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| format!("Failed to get emails: {}", e))?;

        if !response.status().is_success() {
            return Err("Failed to get user emails".to_string());
        }

        let emails: Vec<GitHubEmail> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse emails: {}", e))?;

        // Find primary verified email
        let primary_email = emails
            .iter()
            .find(|e| e.primary && e.verified)
            .or_else(|| emails.iter().find(|e| e.verified))
            .or_else(|| emails.first())
            .ok_or_else(|| "No email found".to_string())?;

        Ok((primary_email.email.clone(), primary_email.verified))
    }

    /// Complete OAuth flow: exchange code and get user info
    pub async fn authenticate(&self, code: &str) -> Result<OAuthUserInfo, String> {
        let access_token = self.exchange_code(code).await?;
        self.get_user_info(&access_token).await
    }
}

