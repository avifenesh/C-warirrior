use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

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
    pub xp_earned: u32,
    pub doors_unlocked: bool,
}

/// Event emitted when a level is completed
#[derive(Debug, Clone, Serialize)]
pub struct LevelCompleteEvent {
    pub level_id: String,
    pub xp_earned: u32,
    pub newly_unlocked: Vec<String>,
}

#[tauri::command]
pub async fn submit_code(
    code: String,
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
    compiler: State<'_, CCompiler>,
    app: AppHandle,
) -> Result<CodeResult, String> {
    println!("[submit_code] Command received");

    // Get level data before await to avoid holding MutexGuard across await
    let (level_data, level_id) = {
        let game_state = state.0.lock().map_err(|e| e.to_string())?;
        let level_id = game_state
            .current_level_id
            .as_ref()
            .ok_or("No level currently loaded")?
            .clone();

        println!("[submit_code] Level ID: {}", level_id);

        let level = levels
            .get_level(&level_id)
            .cloned()
            .ok_or_else(|| format!("Level {} not found", level_id))?;

        (level, level_id)
    };

    println!("[submit_code] Calling compiler...");
    let execution_result = compiler.compile_and_run(&code).await?;
    println!("[submit_code] Compiler returned: success={}", execution_result.run_success());
    let success = level_data.validate_output(&execution_result);

    let mut xp_earned = 0;
    let mut doors_unlocked = false;
    let mut newly_unlocked = Vec::new();

    // If code is successful, complete the level
    if success {
        let mut game_state = state.0.lock().map_err(|e| e.to_string())?;

        // Get previously unlocked levels
        let previously_unlocked: std::collections::HashSet<_> =
            game_state.progression.unlocked_levels.clone();

        // Complete the level (this unlocks doors and awards XP)
        xp_earned = game_state.complete_level(level_data.xp_reward);
        doors_unlocked = true;

        // Update which levels are now unlocked
        game_state.update_unlocked_levels(levels.get_prerequisites());

        // Find newly unlocked levels
        newly_unlocked = game_state
            .progression
            .unlocked_levels
            .iter()
            .filter(|id| !previously_unlocked.contains(*id))
            .cloned()
            .collect();

        println!("[submit_code] Level completed! XP earned: {}, newly unlocked: {:?}",
                 xp_earned, newly_unlocked);
    }

    let feedback = if execution_result.compile_error.is_some() {
        "Code failed to compile. Check for syntax errors.".to_string()
    } else if execution_result.timed_out {
        "Code execution timed out. Check for infinite loops.".to_string()
    } else if success {
        if xp_earned > 0 {
            format!("Success! +{} XP! Doors have been unlocked!", xp_earned)
        } else {
            "Success! Doors have been unlocked! (Level already completed)".to_string()
        }
    } else {
        "Output doesn't match expected result. Try again!".to_string()
    };

    // Emit level_complete event for frontend to react
    if success {
        let event = LevelCompleteEvent {
            level_id: level_id.clone(),
            xp_earned,
            newly_unlocked: newly_unlocked.clone(),
        };
        let _ = app.emit("level_complete", event);
    }

    Ok(CodeResult {
        success,
        stdout: execution_result.stdout,
        stderr: execution_result.stderr,
        compile_error: execution_result.compile_error,
        execution_time_ms: execution_result.execution_time_ms,
        feedback,
        hint: None,
        xp_earned,
        doors_unlocked,
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
