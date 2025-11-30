//! Database models for sessions and player progress

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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

