use serde::{Deserialize, Serialize};
use tauri::State;

use crate::GameStateWrapper;
use code_warrior::persistence::{SaveData, SaveManager, SaveSlotInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveResult {
    pub success: bool,
    pub slot_name: String,
    pub message: String,
}

/// Save the current game state to a slot
#[tauri::command]
pub async fn save_game(
    slot_name: String,
    state: State<'_, GameStateWrapper>,
    save_manager: State<'_, SaveManager>,
) -> Result<SaveResult, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;

    let mut save_data = SaveData::new(slot_name.clone());
    save_data.progression = game_state.progression.clone();
    save_data.current_level_id = game_state.current_level_id.clone();
    save_data.player_position = game_state.player.position;
    save_data.timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    save_manager.save(&save_data)?;

    Ok(SaveResult {
        success: true,
        slot_name,
        message: "Game saved successfully!".to_string(),
    })
}

/// Load a game from a save slot
#[tauri::command]
pub async fn load_game(
    slot_name: String,
    state: State<'_, GameStateWrapper>,
    save_manager: State<'_, SaveManager>,
) -> Result<SaveResult, String> {
    let save_data = save_manager.load(&slot_name)?;

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

    Ok(SaveResult {
        success: true,
        slot_name,
        message: "Game loaded successfully!".to_string(),
    })
}

/// List all available save slots
#[tauri::command]
pub async fn list_saves(save_manager: State<'_, SaveManager>) -> Result<Vec<SaveSlotInfo>, String> {
    save_manager.list_saves()
}

/// Delete a save slot
#[tauri::command]
pub async fn delete_save(
    slot_name: String,
    save_manager: State<'_, SaveManager>,
) -> Result<SaveResult, String> {
    save_manager.delete(&slot_name)?;

    Ok(SaveResult {
        success: true,
        slot_name,
        message: "Save deleted successfully!".to_string(),
    })
}

/// Auto-save the current game (called after level completion)
#[tauri::command]
pub async fn autosave(
    state: State<'_, GameStateWrapper>,
    save_manager: State<'_, SaveManager>,
) -> Result<SaveResult, String> {
    save_game("autosave".to_string(), state, save_manager).await
}
