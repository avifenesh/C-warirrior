use serde::{Deserialize, Serialize};
use tauri::State;

use crate::GameStateWrapper;
use code_warrior::game::RenderState;
use code_warrior::persistence::{SaveData, SaveManager};

/// Save slot info matching frontend SaveSlot type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSlot {
    pub id: String,
    pub name: String,
    pub timestamp: String,
    pub progress: String,
    pub empty: bool,
}

/// Save the current game state to a slot
#[tauri::command]
pub async fn save_game(
    slot_id: String,
    state: State<'_, GameStateWrapper>,
    save_manager: State<'_, SaveManager>,
) -> Result<(), String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;

    let mut save_data = SaveData::new(slot_id.clone());
    save_data.progression = game_state.progression.clone();
    save_data.current_level_id = game_state.current_level_id.clone();
    save_data.player_position = game_state.player.position;
    save_data.timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    save_manager.save(&save_data)?;
    Ok(())
}

/// Load a game from a save slot
#[tauri::command]
pub async fn load_game(
    slot_id: String,
    state: State<'_, GameStateWrapper>,
    save_manager: State<'_, SaveManager>,
) -> Result<RenderState, String> {
    let save_data = save_manager.load(&slot_id)?;

    let mut game_state = state.0.lock().map_err(|e| e.to_string())?;

    // Restore progression state
    game_state.progression = save_data.progression;
    game_state.current_level_id = save_data.current_level_id;
    game_state.player.position = save_data.player_position;

    // Sync legacy fields
    game_state.total_xp = game_state.progression.total_xp;
    game_state.levels_completed = game_state
        .progression
        .completed_levels
        .iter()
        .cloned()
        .collect();

    Ok(game_state.to_render_state())
}

/// List all available save slots
#[tauri::command]
pub async fn list_saves(save_manager: State<'_, SaveManager>) -> Result<Vec<SaveSlot>, String> {
    let slots = save_manager.list_saves()?;
    Ok(slots
        .into_iter()
        .map(|s| {
            let progress = format!(
                "Level {} | {} XP | {} levels",
                s.current_level.as_deref().unwrap_or("None"),
                s.total_xp,
                s.levels_completed
            );
            // Convert timestamp to ISO string
            let timestamp = chrono::DateTime::from_timestamp(s.timestamp as i64, 0)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "Unknown".to_string());

            SaveSlot {
                id: s.slot_name.clone(),
                name: s.slot_name,
                timestamp,
                progress,
                empty: false,
            }
        })
        .collect())
}

/// Delete a save slot
#[tauri::command]
pub async fn delete_save(
    slot_id: String,
    save_manager: State<'_, SaveManager>,
) -> Result<(), String> {
    save_manager.delete(&slot_id)?;
    Ok(())
}

/// Auto-save the current game (called after level completion)
#[tauri::command]
pub async fn autosave(
    state: State<'_, GameStateWrapper>,
    save_manager: State<'_, SaveManager>,
) -> Result<(), String> {
    save_game("autosave".to_string(), state, save_manager).await
}
