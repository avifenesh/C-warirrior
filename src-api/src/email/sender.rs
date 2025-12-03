//! Email sending service using Resend API

use super::templates::EmailTemplate;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Email service configuration
#[derive(Clone)]
pub struct EmailService {
    api_key: String,
    from_email: String,
    from_name: String,
    client: reqwest::Client,
}

/// Resend API request payload
#[derive(Serialize)]
struct ResendEmailRequest {
    from: String,
    to: Vec<String>,
    subject: String,
    html: String,
    text: String,
}

/// Resend API response
#[derive(Deserialize)]
struct ResendEmailResponse {
    id: String,
}

/// Resend API error response
#[derive(Deserialize)]
struct ResendErrorResponse {
    message: String,
}

impl EmailService {
    /// Create a new email service instance
    pub fn new() -> Option<Self> {
        let api_key = std::env::var("RESEND_API_KEY").ok()?;
        let from_email =
            std::env::var("EMAIL_FROM").unwrap_or_else(|_| "noreply@codewarrior.dev".to_string());
        let from_name =
            std::env::var("EMAIL_FROM_NAME").unwrap_or_else(|_| "Code Warrior".to_string());

        Some(Self {
            api_key,
            from_email,
            from_name,
            client: reqwest::Client::new(),
        })
    }

    /// Create a new email service from explicit config (for testing)
    pub fn with_config(api_key: String, from_email: String, from_name: String) -> Self {
        Self {
            api_key,
            from_email,
            from_name,
            client: reqwest::Client::new(),
        }
    }

    /// Send an email using a template
    pub async fn send(&self, to: &str, template: EmailTemplate) -> Result<String, String> {
        let from = format!("{} <{}>", self.from_name, self.from_email);

        let request = ResendEmailRequest {
            from,
            to: vec![to.to_string()],
            subject: template.subject().to_string(),
            html: template.render_html(),
            text: template.render_text(),
        };

        let response = self
            .client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send email: {}", e))?;

        if response.status().is_success() {
            let result: ResendEmailResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            tracing::info!("Email sent successfully to {}, id: {}", to, result.id);
            Ok(result.id)
        } else {
            let error: ResendErrorResponse = response
                .json()
                .await
                .unwrap_or(ResendErrorResponse {
                    message: "Unknown error".to_string(),
                });
            tracing::error!("Failed to send email to {}: {}", to, error.message);
            Err(format!("Email send failed: {}", error.message))
        }
    }

    /// Send verification email
    pub async fn send_verification(
        &self,
        to: &str,
        username: Option<String>,
        token: &str,
        frontend_url: &str,
    ) -> Result<String, String> {
        let verification_link = format!("{}/verify-email?token={}", frontend_url, token);
        let template = EmailTemplate::Verification {
            username,
            verification_link,
        };
        self.send(to, template).await
    }

    /// Send password reset email
    pub async fn send_password_reset(
        &self,
        to: &str,
        username: Option<String>,
        token: &str,
        frontend_url: &str,
    ) -> Result<String, String> {
        let reset_link = format!("{}/reset-password?token={}", frontend_url, token);
        let template = EmailTemplate::PasswordReset {
            username,
            reset_link,
        };
        self.send(to, template).await
    }

    /// Send welcome email after verification
    pub async fn send_welcome(&self, to: &str, username: Option<String>) -> Result<String, String> {
        let template = EmailTemplate::Welcome { username };
        self.send(to, template).await
    }
}

/// Wrapper for optional email service (allows running without email in dev)
pub struct OptionalEmailService(pub Option<Arc<EmailService>>);

impl OptionalEmailService {
    pub fn new() -> Self {
        Self(EmailService::new().map(Arc::new))
    }

    pub fn is_available(&self) -> bool {
        self.0.is_some()
    }

    pub async fn send_verification(
        &self,
        to: &str,
        username: Option<String>,
        token: &str,
        frontend_url: &str,
    ) -> Result<String, String> {
        match &self.0 {
            Some(service) => {
                service
                    .send_verification(to, username, token, frontend_url)
                    .await
            }
            None => {
                tracing::warn!(
                    "Email service not configured. Verification link: {}/verify-email?token={}",
                    frontend_url,
                    token
                );
                Ok("email-disabled".to_string())
            }
        }
    }

    pub async fn send_password_reset(
        &self,
        to: &str,
        username: Option<String>,
        token: &str,
        frontend_url: &str,
    ) -> Result<String, String> {
        match &self.0 {
            Some(service) => {
                service
                    .send_password_reset(to, username, token, frontend_url)
                    .await
            }
            None => {
                tracing::warn!(
                    "Email service not configured. Reset link: {}/reset-password?token={}",
                    frontend_url,
                    token
                );
                Ok("email-disabled".to_string())
            }
        }
    }

    pub async fn send_welcome(&self, to: &str, username: Option<String>) -> Result<String, String> {
        match &self.0 {
            Some(service) => service.send_welcome(to, username).await,
            None => {
                tracing::warn!("Email service not configured. Skipping welcome email to {}", to);
                Ok("email-disabled".to_string())
            }
        }
    }
}

