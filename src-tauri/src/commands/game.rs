use tauri::State;

use code_warrior::game::constants::TILE_SIZE;
use code_warrior::game::state::{GamePhase, GameState, PlayerAction, RenderState};
use crate::GameStateWrapper;

#[tauri::command]
pub async fn init_game(state: State<'_, GameStateWrapper>) -> Result<RenderState, String> {
    let mut game_state = state.0.lock().map_err(|e| e.to_string())?;
    *game_state = GameState::default();
    // Start in playing mode so player can move
    game_state.game_phase = GamePhase::Playing;
    Ok(game_state.to_render_state())
}

#[tauri::command]
pub async fn get_game_state(state: State<'_, GameStateWrapper>) -> Result<GameState, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;
    Ok(game_state.clone())
}

#[tauri::command]
pub async fn get_render_state(state: State<'_, GameStateWrapper>) -> Result<RenderState, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;
    Ok(game_state.to_render_state())
}

#[tauri::command]
pub async fn process_action(
    action: PlayerAction,
    state: State<'_, GameStateWrapper>,
) -> Result<RenderState, String> {
    let mut game_state = state.0.lock().map_err(|e| e.to_string())?;

    match action {
        PlayerAction::Move { direction } => {
            game_state.move_player(direction, TILE_SIZE);
        }
        PlayerAction::Interact => {
            game_state.interact_with_nearest();
        }
        PlayerAction::SubmitCode { .. } => {
            return Err("Use submit_code command for code submission".to_string());
        }
        PlayerAction::Pause => {
            game_state.game_phase = GamePhase::Paused;
        }
        PlayerAction::Resume => {
            // Only resume to Playing from Paused or Coding - don't override LevelComplete
            if matches!(game_state.game_phase, GamePhase::Paused | GamePhase::Coding) {
                game_state.game_phase = GamePhase::Playing;
            }
        }
        PlayerAction::OpenInventory => {}
        PlayerAction::UseItem { .. } => {}
    }

    Ok(game_state.to_render_state())
}
