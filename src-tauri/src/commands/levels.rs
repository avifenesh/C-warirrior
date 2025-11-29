use tauri::State;
use tracing::warn;

use crate::GameStateWrapper;
use code_warrior::game::world::World;
use code_warrior::levels::{load_map_file, LevelData, LevelInfo, LevelRegistry, Quest, QuestInfo};

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
        .map(|level| {
            let total_quests = level.quest_count();
            let completed_quests = game_state.get_completed_quest_count(&level.id);
            let completion_percentage = if total_quests > 0 {
                (completed_quests as f32 / total_quests as f32) * 100.0
            } else {
                0.0
            };

            LevelInfo {
                id: level.id.clone(),
                title: level.title.clone(),
                concept: level.concept.clone(),
                completed: game_state.is_level_completed(&level.id),
                locked: !game_state.is_level_unlocked(&level.id),
                xp_reward: level.get_total_xp(),
                total_quests,
                completed_quests,
                completion_percentage,
            }
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
                warn!(map = %map_path, error = %e, "Failed to load map, using world_config");
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

/// Get all quests for a level with completion status
#[tauri::command]
pub async fn get_level_quests(
    level_id: String,
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
) -> Result<Vec<QuestInfo>, String> {
    let level = levels
        .get_level(&level_id)
        .ok_or_else(|| format!("Level {} not found", level_id))?;

    let game_state = state.0.lock().map_err(|e| e.to_string())?;

    let quests = level.get_quests();
    let quest_infos: Vec<QuestInfo> = quests
        .iter()
        .map(|q| QuestInfo {
            id: q.id.clone(),
            order: q.order,
            title: q.title.clone(),
            description: q.description.clone(),
            recommended: q.recommended,
            completed: game_state.is_quest_completed(&level_id, &q.id),
            xp_reward: q.xp_reward,
        })
        .collect();

    Ok(quest_infos)
}

/// Load a specific quest's data
#[tauri::command]
pub async fn load_quest(
    level_id: String,
    quest_id: String,
    levels: State<'_, LevelRegistry>,
) -> Result<Quest, String> {
    let level = levels
        .get_level(&level_id)
        .ok_or_else(|| format!("Level {} not found", level_id))?;

    let quests = level.get_quests();
    let quest = quests
        .iter()
        .find(|q| q.id == quest_id)
        .cloned()
        .ok_or_else(|| format!("Quest {} not found in level {}", quest_id, level_id))?;

    Ok(quest)
}
