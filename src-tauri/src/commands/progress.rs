use serde::{Deserialize, Serialize};
use tauri::State;

use crate::GameStateWrapper;
use code_warrior::levels::LevelRegistry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelCompleteResult {
    pub xp_earned: u32,
    pub total_xp: u32,
    pub next_level_id: Option<String>,
    pub levels_completed: usize,
    pub newly_unlocked: Vec<String>,
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

    // Get unlocked levels before completion
    let previously_unlocked: std::collections::HashSet<_> =
        game_state.progression.unlocked_levels.clone();

    // Complete the level (this also unlocks doors)
    let xp_earned = game_state.complete_level(level.xp_reward);

    // Update which levels are now unlocked
    game_state.update_unlocked_levels(levels.get_prerequisites());

    // Find newly unlocked levels
    let newly_unlocked: Vec<String> = game_state
        .progression
        .unlocked_levels
        .iter()
        .filter(|id| !previously_unlocked.contains(*id))
        .cloned()
        .collect();

    let next_level_id = levels.get_next_level(&level_id);

    Ok(LevelCompleteResult {
        xp_earned,
        total_xp: game_state.progression.total_xp,
        next_level_id,
        levels_completed: game_state.progression.completed_levels.len(),
        newly_unlocked,
    })
}
