// Code Warrior Tauri Application Entry Point
// This wraps the code-warrior library

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use code_warrior::compiler::CCompiler;
use code_warrior::game::GameState;
use code_warrior::levels::LevelRegistry;

mod commands;

use commands::code::{get_hint, submit_code};
use commands::game::{get_game_state, init_game, process_action};
use commands::levels::{get_available_levels, get_level_data, load_level};
use commands::progress::complete_level;

pub struct GameStateWrapper(pub Mutex<GameState>);

fn main() {
    tauri::Builder::default()
        .manage(GameStateWrapper(Mutex::new(GameState::default())))
        .manage(LevelRegistry::load_from_json())
        .manage(CCompiler::new())
        .invoke_handler(tauri::generate_handler![
            // Game commands
            init_game,
            get_game_state,
            process_action,
            // Level commands
            get_available_levels,
            load_level,
            get_level_data,
            // Code commands
            submit_code,
            get_hint,
            // Progress commands
            complete_level,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
