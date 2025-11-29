// Code Warrior Tauri Application Entry Point
// This wraps the code-warrior library

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::Manager;

use code_warrior::compiler::CCompiler;
use code_warrior::game::GameState;
use code_warrior::levels::LevelRegistry;
use code_warrior::persistence::SaveManager;

mod commands;

use commands::code::{get_hint, submit_code};
use commands::game::{get_game_state, get_progress, get_render_state, init_game, process_action};
use commands::levels::{get_available_levels, get_level_data, load_level};
use commands::save::{autosave, delete_save, list_saves, load_game, save_game};

pub struct GameStateWrapper(pub Mutex<GameState>);

fn main() {
    // Initialize save manager
    let save_manager = SaveManager::new().expect("Failed to initialize save manager");

    tauri::Builder::default()
        .manage(GameStateWrapper(Mutex::new(GameState::default())))
        .manage(LevelRegistry::load_from_json())
        .manage(CCompiler::new())
        .manage(save_manager)
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Game commands
            init_game,
            get_game_state,
            get_render_state,
            process_action,
            get_progress,
            // Level commands
            get_available_levels,
            load_level,
            get_level_data,
            // Code commands
            submit_code,
            get_hint,
            // Save/Load commands
            save_game,
            load_game,
            list_saves,
            delete_save,
            autosave,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
