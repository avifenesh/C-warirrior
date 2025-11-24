use serde::{Deserialize, Serialize};
use tauri::State;

use code_warrior::levels::LevelRegistry;
use crate::GameStateWrapper;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelCompleteResult {
    pub xp_earned: u32,
    pub total_xp: u32,
    pub next_level_id: Option<String>,
    pub levels_completed: usize,
}

#[tauri::command]
pub async fn complete_level(
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
) -> Result<LevelCompleteResult, String> {
    let mut game_state = state.0.lock().map_err(|e| e.to_string())?;
    let level_id = game_state
        .current_level_id
        .clone()
        .ok_or("No level currently loaded")?;

    let level = levels
        .get_level(&level_id)
        .ok_or_else(|| format!("Level {} not found", level_id))?;

    let xp_reward = level.xp_reward;
    game_state.complete_level(xp_reward);

    let next_level_id = levels.get_next_level(&level_id);

    Ok(LevelCompleteResult {
        xp_earned: xp_reward,
        total_xp: game_state.total_xp,
        next_level_id,
        levels_completed: game_state.levels_completed.len(),
    })
}
