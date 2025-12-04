//! Database models for sessions, player progress, and authentication

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Authenticated user
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub email_verified: bool,
    pub is_suspended: bool,
    pub is_blacklisted: bool,
    pub total_xp: i32,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Data for creating a new user (password or OAuth registration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub username: Option<String>,
    pub password_hash: String, // Empty string for OAuth users
}

/// OAuth connection linking a user to an OAuth provider
/// Note: access_token/refresh_token are stored in DB but not fetched (security)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct OAuthConnection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
    pub provider_email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Data for creating a new OAuth connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOAuthConnection {
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
    pub provider_email: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

/// Email verification or password reset token
/// Note: token_hash is stored/compared in DB but not fetched (security)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EmailToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_type: String, // "verify" or "reset"
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Data for creating a new email token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEmailToken {
    pub user_id: Uuid,
    pub token_type: String,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
}

/// Stored game session with full game state
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub device_id: String,
    pub game_state: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Save slot for Save/Load feature
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SaveSlot {
    pub id: Uuid,
    pub device_id: String,
    pub slot_name: String,
    pub save_data: serde_json::Value,
    pub total_xp: i32,
    pub levels_completed: i32,
    pub current_level: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

