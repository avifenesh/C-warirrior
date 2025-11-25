use tauri::State;

use code_warrior::game::world::World;
use code_warrior::levels::{load_map_file, LevelData, LevelInfo, LevelRegistry};
use crate::GameStateWrapper;

#[tauri::command]
pub async fn get_available_levels(
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
) -> Result<Vec<LevelInfo>, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;

    // Get level info with actual locked/completed status from progression
    let all_levels: Vec<LevelInfo> = levels
        .get_level_order()
        .iter()
        .filter_map(|id| levels.get_level(id))
        .map(|level| LevelInfo {
            id: level.id.clone(),
            title: level.title.clone(),
            concept: level.concept.clone(),
            completed: game_state.is_level_completed(&level.id),
            locked: !game_state.is_level_unlocked(&level.id),
        })
        .collect();

    Ok(all_levels)
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

    // Check if level is unlocked
    if !game_state.is_level_unlocked(&level_id) {
        return Err(format!("Level {} is locked", level_id));
    }

    // Try to load from map file, fallback to world_config
    let world = if let Some(ref map_path) = level.map_file {
        match load_map_file(map_path) {
            Ok(map_data) => map_data.to_world(),
            Err(e) => {
                eprintln!("Warning: Failed to load map {}: {}", map_path, e);
                World::from_config(&level.world_config)
            }
        }
    } else {
        World::from_config(&level.world_config)
    };

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
