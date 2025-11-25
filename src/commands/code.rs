use serde::{Deserialize, Serialize};
use tauri::State;

use code_warrior::compiler::CCompiler;
use code_warrior::levels::LevelRegistry;
use crate::GameStateWrapper;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub compile_error: Option<String>,
    pub execution_time_ms: u64,
    pub feedback: String,
    pub hint: Option<String>,
}

#[tauri::command]
pub async fn submit_code(
    code: String,
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
    compiler: State<'_, CCompiler>,
) -> Result<CodeResult, String> {
    // Get level data before await to avoid holding MutexGuard across await
    let level_data = {
        let game_state = state.0.lock().map_err(|e| e.to_string())?;
        let level_id = game_state
            .current_level_id
            .as_ref()
            .ok_or("No level currently loaded")?
            .clone();

        levels
            .get_level(&level_id)
            .cloned()
            .ok_or_else(|| format!("Level {} not found", level_id))?
    };

    let execution_result = compiler.compile_and_run(&code).await?;
    let success = level_data.validate_output(&execution_result);

    // If successful, complete the level (this unlocks doors and awards XP)
    if success {
        let mut game_state = state.0.lock().map_err(|e| e.to_string())?;

        println!("[submit_code] Success! Completing level...");

        // Complete the level - this awards XP, unlocks doors, and sets phase to LevelComplete
        let xp_earned = game_state.complete_level(level_data.xp_reward);
        println!("[submit_code] Level completed! XP earned: {}", xp_earned);
    }

    let feedback = if execution_result.compile_error.is_some() {
        "Code failed to compile. Check for syntax errors.".to_string()
    } else if execution_result.timed_out {
        "Code execution timed out. Check for infinite loops.".to_string()
    } else if success {
        "Success! Your code produced the correct output. Doors have been unlocked!".to_string()
    } else {
        "Output doesn't match expected result. Try again!".to_string()
    };

    Ok(CodeResult {
        success,
        stdout: execution_result.stdout,
        stderr: execution_result.stderr,
        compile_error: execution_result.compile_error,
        execution_time_ms: execution_result.execution_time_ms,
        feedback,
        hint: None,
    })
}

#[tauri::command]
pub async fn get_hint(
    hint_index: usize,
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
) -> Result<String, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;
    let level_id = game_state
        .current_level_id
        .as_ref()
        .ok_or("No level currently loaded")?;

    let level = levels
        .get_level(level_id)
        .ok_or_else(|| format!("Level {} not found", level_id))?;

    level
        .hints
        .get(hint_index)
        .cloned()
        .ok_or_else(|| "No more hints available".to_string())
}
