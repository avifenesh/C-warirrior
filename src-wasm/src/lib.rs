use code_warrior::{
    GamePhase, GameState, PlayerAction,
    levels::{LevelInfo, LevelRegistry},
    game::world::World,
};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmGame {
    state: GameState,
    levels: LevelRegistry,
}

#[wasm_bindgen]
impl WasmGame {
    /// Create a new game instance with embedded level data
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGame {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

        WasmGame {
            state: GameState::new(),
            levels: LevelRegistry::load_from_json(),
        }
    }

    /// Initialize or restore game state from server
    #[wasm_bindgen]
    pub fn init_from_state(&mut self, state_json: JsValue) -> Result<(), JsValue> {
        let state: GameState = from_value(state_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse state: {}", e)))?;
        self.state = state;
        Ok(())
    }

    /// Process a player action locally (movement, interact, pause)
    /// Code submission is NOT handled here - must go through HTTP
    #[wasm_bindgen]
    pub fn process_action(&mut self, action_json: JsValue) -> Result<JsValue, JsValue> {
        let action: PlayerAction = from_value(action_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid action: {}", e)))?;

        match &action {
            PlayerAction::Move { direction } => {
                self.state.move_player(*direction, code_warrior::TILE_SIZE);
            }
            PlayerAction::Interact => {
                self.state.interact_with_nearest();
            }
            PlayerAction::Pause => {
                self.state.game_phase = GamePhase::Paused;
            }
            PlayerAction::Resume => {
                if matches!(self.state.game_phase, GamePhase::Paused | GamePhase::Coding) {
                    self.state.game_phase = GamePhase::Playing;
                }
            }
            PlayerAction::SubmitCode { .. } => {
                return Err(JsValue::from_str(
                    "Code submission must go through HTTP backend",
                ));
            }
            PlayerAction::OpenInventory => {
                // Toggle inventory view
            }
            PlayerAction::UseItem { .. } => {
                // Handle item usage
            }
        }

        to_value(&self.state.to_render_state())
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get current render state for UI
    #[wasm_bindgen]
    pub fn get_render_state(&self) -> Result<JsValue, JsValue> {
        to_value(&self.state.to_render_state())
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get full game state (for syncing to server)
    #[wasm_bindgen]
    pub fn get_game_state(&self) -> Result<JsValue, JsValue> {
        to_value(&self.state)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Load a level locally from embedded levels.json
    #[wasm_bindgen]
    pub fn load_level(&mut self, level_id: &str) -> Result<JsValue, JsValue> {
        let level = self
            .levels
            .get_level(level_id)
            .ok_or_else(|| JsValue::from_str(&format!("Level '{}' not found", level_id)))?;

        let world = World::from_config(&level.world_config);
        self.state.start_level(level_id.to_string(), world);
        self.state
            .update_unlocked_levels(self.levels.get_prerequisites());

        to_value(level).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get list of all levels with current unlock/completion status
    #[wasm_bindgen]
    pub fn get_available_levels(&self) -> Result<JsValue, JsValue> {
        let mut levels_info: Vec<LevelInfo> = self.levels.get_all_info();
        for level in &mut levels_info {
            level.locked = !self.state.is_level_unlocked(&level.id);
            level.completed = self.state.is_level_completed(&level.id);
        }
        to_value(&levels_info)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get data for currently loaded level
    #[wasm_bindgen]
    pub fn get_level_data(&self) -> Result<JsValue, JsValue> {
        let level_id = self
            .state
            .current_level_id
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No level loaded"))?;

        let level = self
            .levels
            .get_level(level_id)
            .ok_or_else(|| JsValue::from_str("Level not found in registry"))?;

        to_value(level).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Mark level complete (call after HTTP backend confirms code success)
    #[wasm_bindgen]
    pub fn complete_level(&mut self, xp_reward: u32) -> Result<JsValue, JsValue> {
        let _xp_earned = self.state.complete_level(xp_reward);
        self.state
            .update_unlocked_levels(self.levels.get_prerequisites());

        to_value(&self.state.to_render_state())
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Sync progression state from server (after load save, login, etc.)
    #[wasm_bindgen]
    pub fn sync_progression(&mut self, total_xp: u32, completed_levels: Vec<String>) {
        for level_id in completed_levels {
            self.state.progression.completed_levels.insert(level_id);
        }
        self.state.progression.total_xp = total_xp;
        self.state
            .update_unlocked_levels(self.levels.get_prerequisites());
    }

    /// Get hint for current level by index
    #[wasm_bindgen]
    pub fn get_hint(&self, hint_index: usize) -> Result<String, JsValue> {
        let level_id = self
            .state
            .current_level_id
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No level loaded"))?;

        let level = self
            .levels
            .get_level(level_id)
            .ok_or_else(|| JsValue::from_str("Level not found"))?;

        level
            .hints
            .get(hint_index)
            .cloned()
            .ok_or_else(|| JsValue::from_str("No more hints available"))
    }
}

impl Default for WasmGame {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_game_creation() {
        let game = WasmGame::new();
        assert!(matches!(game.state.game_phase, GamePhase::MainMenu));
    }
}
