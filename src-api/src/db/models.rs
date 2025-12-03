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

/// Data for creating a new user (password registration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub username: Option<String>,
    pub password_hash: String,
}

/// Data for creating a new user (OAuth registration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOAuthUser {
    pub email: String,
    pub username: Option<String>,
    pub email_verified: bool,
}

/// User response for API (excludes sensitive fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: Option<String>,
    pub email_verified: bool,
    pub total_xp: i32,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            email_verified: user.email_verified,
            total_xp: user.total_xp,
            created_at: user.created_at,
        }
    }
}

/// OAuth connection linking a user to an OAuth provider
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct OAuthConnection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
    pub provider_email: Option<String>,
    #[serde(skip_serializing)]
    pub access_token: Option<String>,
    #[serde(skip_serializing)]
    pub refresh_token: Option<String>,
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
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EmailToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_type: String,  // "verify" or "reset"
    #[serde(skip_serializing)]
    pub token_hash: String,
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

/// Data for creating a new session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSession {
    pub device_id: String,
    pub game_state: serde_json::Value,
}

/// Player progress tracking
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PlayerProgress {
    pub id: Uuid,
    pub device_id: String,
    pub completed_levels: Vec<String>,
    pub total_xp: i32,
    pub current_level: Option<String>,
    pub achievements: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Data for creating new player progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProgress {
    pub device_id: String,
    pub completed_levels: Vec<String>,
    pub total_xp: i32,
    pub current_level: Option<String>,
    pub achievements: Vec<String>,
}

impl Default for NewProgress {
    fn default() -> Self {
        Self {
            device_id: String::new(),
            completed_levels: Vec::new(),
            total_xp: 0,
            current_level: None,
            achievements: Vec::new(),
        }
    }
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

