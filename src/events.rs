use serde::{Deserialize, Serialize};

use code_warrior::game::state::RenderState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTickPayload {
    pub state: RenderState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeOutput {
    pub stream: String, // "stdout" or "stderr"
    pub content: String,
    pub is_final: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelCompleteEvent {
    pub level_id: String,
    pub xp_earned: u32,
    pub next_level_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameError {
    pub code: String,
    pub message: String,
    pub recoverable: bool,
}

// Event emission helpers
use tauri::{AppHandle, Emitter};

pub fn emit_game_tick(app: &AppHandle, state: RenderState) {
    let _ = app.emit("game_tick", state);
}

pub fn emit_code_output(app: &AppHandle, stream: &str, content: &str, is_final: bool) {
    let _ = app.emit(
        "code_output",
        CodeOutput {
            stream: stream.to_string(),
            content: content.to_string(),
            is_final,
        },
    );
}

pub fn emit_level_complete(
    app: &AppHandle,
    level_id: &str,
    xp_earned: u32,
    next_level_id: Option<String>,
) {
    let _ = app.emit(
        "level_complete",
        LevelCompleteEvent {
            level_id: level_id.to_string(),
            xp_earned,
            next_level_id,
        },
    );
}

pub fn emit_game_error(app: &AppHandle, code: &str, message: &str, recoverable: bool) {
    let _ = app.emit(
        "game_error",
        GameError {
            code: code.to_string(),
            message: message.to_string(),
            recoverable,
        },
    );
}
