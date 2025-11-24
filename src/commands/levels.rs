use tauri::State;

use code_warrior::game::world::World;
use code_warrior::levels::{LevelData, LevelInfo, LevelRegistry};
use crate::GameStateWrapper;

#[tauri::command]
pub async fn get_available_levels(
    levels: State<'_, LevelRegistry>,
) -> Result<Vec<LevelInfo>, String> {
    Ok(levels.get_all_info())
}

#[tauri::command]
pub async fn load_level(
    level_id: String,
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
) -> Result<LevelData, String> {
    let level = levels
        .get_level(&level_id)
        .ok_or_else(|| format!("Level {} not found", level_id))?;

    let mut game_state = state.0.lock().map_err(|e| e.to_string())?;

    let world = World::from_config(&level.world_config);
    game_state.start_level(level_id, world);

    Ok(level.clone())
}

#[tauri::command]
pub async fn get_level_data(
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
) -> Result<LevelData, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;
    let level_id = game_state
        .current_level_id
        .as_ref()
        .ok_or("No level currently loaded")?;

    let level = levels
        .get_level(level_id)
        .ok_or_else(|| format!("Level {} not found", level_id))?;

    Ok(level.clone())
}
