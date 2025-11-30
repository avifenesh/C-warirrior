use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};
use tracing::{debug, info};

use crate::GameStateWrapper;
use code_warrior::compiler::CCompiler;
use code_warrior::levels::{generate_harness, LevelRegistry, TestCaseResult, TestSuiteResult};

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
    pub render_state: code_warrior::game::RenderState,
    /// Test results for function-based challenges
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_results: Option<TestSuiteResult>,
}

/// Event emitted when a level is completed
#[derive(Debug, Clone, Serialize)]
pub struct LevelCompleteEvent {
    pub level_id: String,
    pub xp_earned: u32,
    pub next_level_id: Option<String>,
    pub newly_unlocked: Vec<String>,
}

/// Event emitted when a quest is completed
#[derive(Debug, Clone, Serialize)]
pub struct QuestCompleteEvent {
    pub level_id: String,
    pub quest_id: String,
    pub xp_earned: u32,
    pub quests_remaining: usize,
}

#[tauri::command]
pub async fn submit_code(
    code: String,
    #[allow(unused_variables)] test_only: Option<bool>,
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
    compiler: State<'_, CCompiler>,
    app: AppHandle,
) -> Result<CodeResult, String> {
    let test_only = test_only.unwrap_or(false);
    debug!("submit_code command received, test_only={}", test_only);

    // Get level data before await to avoid holding MutexGuard across await
    let (level_data, level_id) = {
        let game_state = state.0.lock().map_err(|e| e.to_string())?;
        let level_id = game_state
            .current_level_id
            .as_ref()
            .ok_or("No level currently loaded")?
            .clone();

        debug!(level_id = %level_id, "Processing submission");

        let level = levels
            .get_level(&level_id)
            .cloned()
            .ok_or_else(|| format!("Level {} not found", level_id))?;

        (level, level_id)
    };

    // Check if this is a function-based challenge
    if level_data.is_function_based() {
        return run_function_based_challenge(
            code,
            test_only,
            level_data,
            level_id,
            state,
            levels,
            compiler,
            app,
        )
        .await;
    }

    // Legacy output-based challenge
    debug!("Running legacy output-based challenge");
    let execution_result = compiler.compile_and_run(&code).await?;
    debug!(success = execution_result.run_success(), "Compiler returned");
    let success = level_data.validate_output(&execution_result);

    let mut xp_earned = 0;
    let mut doors_unlocked = false;
    let mut newly_unlocked = Vec::new();
    let mut next_level_id: Option<String> = None;

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

        info!(xp = xp_earned, unlocked = ?newly_unlocked, "Level completed");

        // Determine next level based on registry order
        next_level_id = levels.get_next_level(&level_id);
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
            next_level_id,
            newly_unlocked: newly_unlocked.clone(),
        };
        let _ = app.emit("level_complete", event);
    }

    // Get current render state for response
    let render_state = {
        let game_state = state.0.lock().map_err(|e| e.to_string())?;
        game_state.to_render_state()
    };

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
        render_state,
        test_results: None,
    })
}

/// Run a function-based challenge with test cases
async fn run_function_based_challenge(
    code: String,
    test_only: bool,
    level_data: code_warrior::levels::LevelData,
    level_id: String,
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
    compiler: State<'_, CCompiler>,
    app: AppHandle,
) -> Result<CodeResult, String> {
    debug!("Running function-based challenge");

    let signature = level_data
        .function_signature
        .as_ref()
        .ok_or("Function signature missing")?;

    // Filter test cases: sample only for TEST, all for SUBMIT
    let test_cases: Vec<_> = level_data
        .test_cases
        .iter()
        .filter(|tc| !test_only || tc.sample)
        .collect();

    if test_cases.is_empty() {
        return Err("No test cases defined for this level".to_string());
    }

    let mut results: Vec<TestCaseResult> = Vec::new();
    let mut total_time_ms = 0u64;

    // Run each test case
    for test_case in &test_cases {
        let harness = generate_harness(&code, signature, test_case)
            .map_err(|e| format!("Failed to generate test harness: {}", e))?;

        let execution_result = compiler.compile_and_run(&harness).await?;
        total_time_ms += execution_result.execution_time_ms;

        // Check for compilation error
        if let Some(ref err) = execution_result.compile_error {
            let test_suite = TestSuiteResult {
                passed: false,
                total: test_cases.len(),
                passed_count: 0,
                results: vec![],
                compilation_error: Some(err.clone()),
            };

            let render_state = {
                let game_state = state.0.lock().map_err(|e| e.to_string())?;
                game_state.to_render_state()
            };

            return Ok(CodeResult {
                success: false,
                stdout: String::new(),
                stderr: execution_result.stderr,
                compile_error: Some(err.clone()),
                execution_time_ms: total_time_ms,
                feedback: "Code failed to compile. Check for syntax errors.".to_string(),
                hint: None,
                xp_earned: 0,
                doors_unlocked: false,
                render_state,
                test_results: Some(test_suite),
            });
        }

        let actual = execution_result.stdout.trim().to_string();
        let expected = test_case.expected.trim().to_string();
        let passed = actual == expected;

        results.push(TestCaseResult {
            input: test_case.input.clone(),
            expected: expected.clone(),
            actual,
            passed,
        });
    }

    let passed_count = results.iter().filter(|r| r.passed).count();
    let all_passed = passed_count == results.len();

    let test_suite = TestSuiteResult {
        passed: all_passed,
        total: results.len(),
        passed_count,
        results,
        compilation_error: None,
    };

    let mut xp_earned = 0;
    let mut doors_unlocked = false;

    // Only complete level on SUBMIT (not TEST) and if all passed
    if all_passed && !test_only {
        let mut game_state = state.0.lock().map_err(|e| e.to_string())?;

        let previously_unlocked: std::collections::HashSet<_> =
            game_state.progression.unlocked_levels.clone();

        xp_earned = game_state.complete_level(level_data.xp_reward);
        doors_unlocked = true;

        game_state.update_unlocked_levels(levels.get_prerequisites());

        let newly_unlocked: Vec<_> = game_state
            .progression
            .unlocked_levels
            .iter()
            .filter(|id| !previously_unlocked.contains(*id))
            .cloned()
            .collect();

        info!(xp = xp_earned, unlocked = ?newly_unlocked, "Level completed");

        let next_level_id = levels.get_next_level(&level_id);

        // Emit level_complete event
        let event = LevelCompleteEvent {
            level_id: level_id.clone(),
            xp_earned,
            next_level_id,
            newly_unlocked,
        };
        let _ = app.emit("level_complete", event);
    }

    let feedback = if all_passed {
        if test_only {
            format!("All {} sample tests passed! Click SUBMIT to complete.", passed_count)
        } else if xp_earned > 0 {
            format!("All {} tests passed! +{} XP! Doors have been unlocked!", test_suite.total, xp_earned)
        } else {
            format!("All {} tests passed! Doors have been unlocked! (Level already completed)", test_suite.total)
        }
    } else {
        format!("{}/{} tests passed. Check your logic and try again!", passed_count, test_suite.total)
    };

    let render_state = {
        let game_state = state.0.lock().map_err(|e| e.to_string())?;
        game_state.to_render_state()
    };

    Ok(CodeResult {
        success: all_passed,
        stdout: String::new(),
        stderr: String::new(),
        compile_error: None,
        execution_time_ms: total_time_ms,
        feedback,
        hint: None,
        xp_earned,
        doors_unlocked,
        render_state,
        test_results: Some(test_suite),
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

/// Submit code for a specific quest in a multi-quest level
#[tauri::command]
pub async fn submit_quest_code(
    code: String,
    quest_id: String,
    test_only: Option<bool>,
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
    compiler: State<'_, CCompiler>,
    app: AppHandle,
) -> Result<CodeResult, String> {
    let test_only = test_only.unwrap_or(false);
    debug!("submit_quest_code: quest_id={}, test_only={}", quest_id, test_only);

    // Get level and quest data
    let (level_id, quest, total_quests) = {
        let game_state = state.0.lock().map_err(|e| e.to_string())?;
        let level_id = game_state
            .current_level_id
            .as_ref()
            .ok_or("No level currently loaded")?
            .clone();

        let level = levels
            .get_level(&level_id)
            .ok_or_else(|| format!("Level {} not found", level_id))?;

        let quests = level.get_quests();
        let total_quests = quests.len();
        let quest = quests
            .iter()
            .find(|q| q.id == quest_id)
            .cloned()
            .ok_or_else(|| format!("Quest {} not found in level {}", quest_id, level_id))?;

        (level_id, quest, total_quests)
    };

    // Filter test cases: sample only for TEST, all for SUBMIT
    let test_cases: Vec<_> = quest
        .test_cases
        .iter()
        .filter(|tc| !test_only || tc.sample)
        .collect();

    if test_cases.is_empty() {
        return Err("No test cases defined for this quest".to_string());
    }

    let mut results: Vec<TestCaseResult> = Vec::new();
    let mut total_time_ms = 0u64;

    // Run each test case
    for test_case in &test_cases {
        let harness = generate_harness(&code, &quest.function_signature, test_case)
            .map_err(|e| format!("Failed to generate test harness: {}", e))?;

        let execution_result = compiler.compile_and_run(&harness).await?;
        total_time_ms += execution_result.execution_time_ms;

        // Check for compilation error
        if let Some(ref err) = execution_result.compile_error {
            let test_suite = TestSuiteResult {
                passed: false,
                total: test_cases.len(),
                passed_count: 0,
                results: vec![],
                compilation_error: Some(err.clone()),
            };

            let render_state = {
                let game_state = state.0.lock().map_err(|e| e.to_string())?;
                game_state.to_render_state()
            };

            return Ok(CodeResult {
                success: false,
                stdout: String::new(),
                stderr: execution_result.stderr,
                compile_error: Some(err.clone()),
                execution_time_ms: total_time_ms,
                feedback: "Code failed to compile. Check for syntax errors.".to_string(),
                hint: quest.hints.first().cloned(),
                xp_earned: 0,
                doors_unlocked: false,
                render_state,
                test_results: Some(test_suite),
            });
        }

        let actual = execution_result.stdout.trim().to_string();
        let expected = test_case.expected.trim().to_string();
        let passed = actual == expected;

        results.push(TestCaseResult {
            input: test_case.input.clone(),
            expected: expected.clone(),
            actual,
            passed,
        });
    }

    let passed_count = results.iter().filter(|r| r.passed).count();
    let all_passed = passed_count == results.len();

    let test_suite = TestSuiteResult {
        passed: all_passed,
        total: results.len(),
        passed_count,
        results,
        compilation_error: None,
    };

    let mut xp_earned = 0;
    let mut doors_unlocked = false;
    let mut quests_remaining = total_quests;

    // Only complete quest on SUBMIT (not TEST) and if all passed
    if all_passed && !test_only {
        let mut game_state = state.0.lock().map_err(|e| e.to_string())?;

        // Complete the quest (awards XP only if not already completed)
        xp_earned = game_state.complete_quest(&level_id, &quest_id, quest.xp_reward);

        let completed_count = game_state.get_completed_quest_count(&level_id);
        quests_remaining = total_quests.saturating_sub(completed_count);

        // Check if all quests completed → level complete
        if quests_remaining == 0 {
            // Mark level as complete and unlock doors
            if let Some(_) = game_state.maybe_complete_level(total_quests) {
                doors_unlocked = true;
                game_state.update_unlocked_levels(levels.get_prerequisites());

                // Emit level_complete event
                let event = LevelCompleteEvent {
                    level_id: level_id.clone(),
                    xp_earned: 0, // XP already awarded per-quest
                    next_level_id: levels.get_next_level(&level_id),
                    newly_unlocked: vec![],
                };
                let _ = app.emit("level_complete", event);
            }
        }

        // Emit quest_complete event
        let quest_event = QuestCompleteEvent {
            level_id: level_id.clone(),
            quest_id: quest_id.clone(),
            xp_earned,
            quests_remaining,
        };
        let _ = app.emit("quest_complete", quest_event);

        info!(
            quest_id = %quest_id,
            xp = xp_earned,
            remaining = quests_remaining,
            "Quest completed"
        );
    }

    let feedback = if all_passed {
        if test_only {
            format!("All {} sample tests passed! Click SUBMIT to complete.", passed_count)
        } else if quests_remaining == 0 {
            format!(
                "Quest complete! +{} XP! All quests done - doors unlocked!",
                xp_earned
            )
        } else if xp_earned > 0 {
            format!(
                "Quest complete! +{} XP! {} quest(s) remaining.",
                xp_earned, quests_remaining
            )
        } else {
            "Quest already completed. Try another quest!".to_string()
        }
    } else {
        format!(
            "{}/{} tests passed. Check your logic and try again!",
            passed_count, test_suite.total
        )
    };

    let render_state = {
        let game_state = state.0.lock().map_err(|e| e.to_string())?;
        game_state.to_render_state()
    };

    Ok(CodeResult {
        success: all_passed,
        stdout: String::new(),
        stderr: String::new(),
        compile_error: None,
        execution_time_ms: total_time_ms,
        feedback,
        hint: if !all_passed { quest.hints.first().cloned() } else { None },
        xp_earned,
        doors_unlocked,
        render_state,
        test_results: Some(test_suite),
    })
}
