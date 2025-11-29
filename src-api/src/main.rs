use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{Json, Response},
    routing::{get, post},
    Router,
};
use code_warrior::{
    compiler::CCompiler,
    game::{GamePhase, GameState, PlayerAction, RenderState},
    levels::{generate_harness, LevelData, LevelInfo, LevelRegistry, TestCaseResult, TestSuiteResult},
};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Database persistence module (Agent 1)
mod db;

// Shared application state
struct AppState {
    db: Pool<Postgres>,
    levels: Arc<LevelRegistry>,
    compiler: Arc<CCompiler>,
    /// In-memory session cache keyed by device ID to avoid hitting Neon on every tick/move
    sessions: DashMap<String, GameState>,
}

// Request/Response types
#[derive(Debug, Deserialize)]
struct InitGameRequest {
    // Empty for now, could add initial settings
}

#[derive(Debug, Serialize)]
struct InitGameResponse {
    device_id: String,
    game_state: GameState,
}

#[derive(Debug, Serialize)]
struct LoadLevelResponse {
    level_data: LevelData,
    render_state: RenderState,
}

#[derive(Debug, Deserialize)]
struct SubmitCodeRequest {
    code: String,
    #[serde(default)]
    test_only: bool,
}

#[derive(Debug, Serialize)]
struct SubmitCodeResponse {
    success: bool,
    stdout: String,
    stderr: String,
    compile_error: Option<String>,
    execution_time_ms: u64,
    feedback: String,
    hint: Option<String>,
    xp_earned: Option<u32>,
    doors_unlocked: bool,
    render_state: RenderState,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_results: Option<TestSuiteResult>,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    database: String,
}

#[derive(Debug, Serialize)]
struct ProgressResponse {
    total_xp: u32,
    completed_levels: Vec<String>,
    current_level: Option<String>,
}

#[derive(Debug, Serialize)]
struct SaveSlotResponse {
    id: String,
    name: String,
    timestamp: String,
    progress: String,
    empty: bool,
}

// Extension for device ID
#[derive(Clone)]
struct DeviceId(String);

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "code_warrior_api=info,tower_http=info,sqlx=warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");

    // Create database connection pool with Neon optimization
    tracing::info!("Connecting to database...");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3) // Lower max for Neon free tier
        .min_connections(0) // Allow pool to shrink to 0
        .acquire_timeout(Duration::from_secs(10)) // Timeout for getting connections
        .idle_timeout(Duration::from_secs(300)) // Drop idle connections after 5 mins
        .max_lifetime(Duration::from_secs(1800)) // Recreate connections after 30 mins
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connection established");

    // Initialize database tables (run migrations)
    tracing::info!("Running database migrations...");
    db::init_database(&pool)
        .await
        .expect("Failed to initialize database tables");
    tracing::info!("Database migrations complete");

    // Initialize game systems
    let levels = Arc::new(LevelRegistry::load_from_json());
    let compiler = Arc::new(CCompiler::new());

    tracing::info!("Game systems initialized");

    // Initialize application state
    let state = Arc::new(AppState {
        db: pool,
        levels,
        compiler,
        sessions: DashMap::new(),
    });

    // Configure CORS for development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/game/init", post(init_game))
        .route("/api/game/state", get(get_game_state))
        .route("/api/game/render-state", get(get_render_state))
        .route("/api/game/action", post(process_action))
        .route("/api/levels", get(get_available_levels))
        .route("/api/levels/:id/load", post(load_level))
        .route("/api/code/submit", post(submit_code))
        .route("/api/levels/current", get(get_current_level))
        .route("/api/code/hint/:index", get(get_hint))
        .route("/api/player/progress", get(get_progress))
        .route("/api/saves", get(list_saves))
        .route("/api/saves/:slot", post(save_game))
        .route("/api/saves/:slot", get(load_save))
        .route("/api/saves/:slot", axum::routing::delete(delete_save))
        .layer(middleware::from_fn(device_id_middleware))
        .layer(cors)
        .with_state(state);

    // Get port from environment or use default
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    // Bind to 0.0.0.0 for cloud deployment (Railway, etc.)
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Starting Code Warrior API server on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

// Middleware to extract and inject device ID
async fn device_id_middleware(
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let device_id = req
        .headers()
        .get("X-Device-ID")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("default")
        .to_string();

    req.extensions_mut().insert(DeviceId(device_id));
    Ok(next.run(req).await)
}

// Helper to get or create game state for a session using an in-memory cache with DB fallback
async fn get_or_create_session(app: &Arc<AppState>, device_id: &str) -> Result<GameState, String> {
    // Fast path: in-memory session
    if let Some(entry) = app.sessions.get(device_id) {
        return Ok(entry.value().clone());
    }

    // Fallback: load from database or create a new session
    match db::get_session(&app.db, device_id).await {
        Ok(Some(session)) => {
            let game_state: GameState = serde_json::from_value(session.game_state)
                .map_err(|e| format!("Failed to parse game state: {}", e))?;
            app.sessions
                .insert(device_id.to_string(), game_state.clone());
            Ok(game_state)
        }
        Ok(None) => {
            let new_state = GameState::new();
            let session_json = serde_json::to_value(&new_state)
                .map_err(|e| format!("Failed to serialize game state: {}", e))?;

            db::save_session(
                &app.db,
                &db::NewSession {
                    device_id: device_id.to_string(),
                    game_state: session_json,
                },
            )
            .await
            .map_err(|e| format!("Failed to create session: {}", e))?;

            app.sessions
                .insert(device_id.to_string(), new_state.clone());
            Ok(new_state)
        }
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

// Helper to cache session state in memory only (no DB write)
fn cache_session(app: &Arc<AppState>, device_id: &str, state: &GameState) {
    app.sessions.insert(device_id.to_string(), state.clone());
}

// Helper to persist session state to DB (and update in-memory cache)
async fn persist_session(
    app: &Arc<AppState>,
    device_id: &str,
    state: &GameState,
) -> Result<(), String> {
    cache_session(app, device_id, state);

    let state_json = serde_json::to_value(state)
        .map_err(|e| format!("Failed to serialize game state: {}", e))?;

    db::update_session_state(&app.db, device_id, &state_json)
        .await
        .map_err(|e| format!("Failed to update session: {}", e))?;

    let progress = db::NewProgress {
        device_id: device_id.to_string(),
        completed_levels: state.levels_completed.clone(),
        total_xp: state.total_xp as i32,
        current_level: state.current_level_id.clone(),
        achievements: vec![],
    };

    db::save_progress(&app.db, &progress)
        .await
        .map_err(|e| format!("Failed to save progress: {}", e))?;

    Ok(())
}

// Handler functions

async fn health_check(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    // Check database health
    let db_status = match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => "connected".to_string(),
        Err(e) => {
            tracing::error!("Database health check failed: {}", e);
            "disconnected".to_string()
        }
    };

    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status,
    })
}

async fn init_game(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
    Json(_payload): Json<InitGameRequest>,
) -> Result<Json<InitGameResponse>, (StatusCode, String)> {
    tracing::info!("Initializing new game session for device: {}", device_id.0);

    // Force create new state or reset? For now, just get/create
    let game_state = get_or_create_session(&state, &device_id.0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // If we wanted to force reset, we would do it here

    Ok(Json(InitGameResponse {
        device_id: device_id.0,
        game_state,
    }))
}

async fn get_game_state(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
) -> Result<Json<GameState>, (StatusCode, String)> {
    tracing::debug!("Fetching game state for device: {}", device_id.0);

    let game_state = get_or_create_session(&state, &device_id.0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(game_state))
}

async fn get_render_state(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
) -> Result<Json<RenderState>, (StatusCode, String)> {
    tracing::debug!("Fetching render state for device: {}", device_id.0);

    let game_state = get_or_create_session(&state, &device_id.0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(game_state.to_render_state()))
}

async fn process_action(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
    Json(action): Json<PlayerAction>,
) -> Result<Json<RenderState>, (StatusCode, String)> {
    tracing::info!("Processing action for device: {}", device_id.0);

    let mut game_state = get_or_create_session(&state, &device_id.0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // Process the action
    const TILE_SIZE: f32 = 32.0;
    match action {
        PlayerAction::Move { direction } => {
            game_state.move_player(direction, TILE_SIZE);
        }
        PlayerAction::Interact => {
            game_state.interact_with_nearest();
        }
        PlayerAction::Pause => {
            game_state.game_phase = GamePhase::Paused;
        }
        PlayerAction::Resume => {
            if matches!(game_state.game_phase, GamePhase::Paused | GamePhase::Coding) {
                game_state.game_phase = GamePhase::Playing;
            }
        }
        PlayerAction::SubmitCode { .. } => {
            return Err((
                StatusCode::BAD_REQUEST,
                "Use /api/code/submit for code submission".to_string(),
            ));
        }
        PlayerAction::OpenInventory => {
            // TODO: Implement inventory UI state
        }
        PlayerAction::UseItem { .. } => {
            // TODO: Implement item usage
        }
    }

    // Cache updated session in memory; persistence happens on level load / code submit
    cache_session(&state, &device_id.0, &game_state);

    Ok(Json(game_state.to_render_state()))
}

async fn get_available_levels(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
) -> Result<Json<Vec<LevelInfo>>, (StatusCode, String)> {
    tracing::debug!("Fetching available levels for device: {}", device_id.0);

    let game_state = get_or_create_session(&state, &device_id.0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let mut levels_info = state.levels.get_all_info();

    // Update lock/completed status based on game state
    for level in &mut levels_info {
        level.locked = !game_state.is_level_unlocked(&level.id);
        level.completed = game_state.is_level_completed(&level.id);
    }

    Ok(Json(levels_info))
}

async fn load_level(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
    Path(level_id): Path<String>,
) -> Result<Json<LoadLevelResponse>, (StatusCode, String)> {
    tracing::info!("Loading level '{}' for device: {}", level_id, device_id.0);

    // Get level data
    let level = state.levels.get_level(&level_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            format!("Level '{}' not found", level_id),
        )
    })?;

    // Get or create game state
    let mut game_state = get_or_create_session(&state, &device_id.0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // Check if level is unlocked
    if !game_state.is_level_unlocked(&level_id) {
        return Err((
            StatusCode::FORBIDDEN,
            format!("Level '{}' is locked", level_id),
        ));
    }

    // Load level into game state
    let world = code_warrior::game::world::World::from_config(&level.world_config);
    game_state.start_level(level_id.clone(), world);

    // Update unlocked levels based on prerequisites
    game_state.update_unlocked_levels(state.levels.get_prerequisites());

    // Save updated state (persist to DB and cache)
    persist_session(&state, &device_id.0, &game_state)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(LoadLevelResponse {
        level_data: level.clone(),
        render_state: game_state.to_render_state(),
    }))
}

async fn submit_code(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
    Json(payload): Json<SubmitCodeRequest>,
) -> Result<Json<SubmitCodeResponse>, (StatusCode, String)> {
    tracing::info!("Submitting code for device: {}, test_only: {}", device_id.0, payload.test_only);

    // Get game state
    let mut game_state = get_or_create_session(&state, &device_id.0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // Get current level
    let level_id = game_state
        .current_level_id
        .as_ref()
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "No level currently loaded".to_string(),
            )
        })?
        .clone();

    let level = state.levels.get_level(&level_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            format!("Level '{}' not found", level_id),
        )
    })?.clone();

    // Check if this is a function-based challenge
    if level.is_function_based() {
        return run_function_based_challenge(
            &state,
            &device_id.0,
            &payload.code,
            payload.test_only,
            &level,
            &level_id,
            &mut game_state,
        ).await;
    }

    // Legacy output-based challenge
    let execution_result = state
        .compiler
        .compile_and_run(&payload.code)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // Validate output
    let success = level.validate_output(&execution_result);

    let mut xp_earned = None;
    let feedback = if execution_result.compile_error.is_some() {
        "Code failed to compile. Check for syntax errors.".to_string()
    } else if execution_result.timed_out {
        "Code execution timed out. Check for infinite loops.".to_string()
    } else if success {
        // Complete the level and award XP
        let xp = game_state.complete_level(level.xp_reward);
        xp_earned = Some(xp);

        // Update unlocked levels
        game_state.update_unlocked_levels(state.levels.get_prerequisites());

        format!(
            "Success! Your code produced the correct output. You earned {} XP!",
            xp
        )
    } else {
        "Output doesn't match expected result. Try again!".to_string()
    };

    if success {
        let xp_delta = xp_earned.unwrap_or(0);
        let xp_i32 = (xp_delta.min(i32::MAX as u32)) as i32;
        db::complete_level(&state.db, &device_id.0, &level_id, xp_i32)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to record progress: {}", e),
                )
            })?;
    }

    // Save updated state
    persist_session(&state, &device_id.0, &game_state)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(SubmitCodeResponse {
        success,
        stdout: execution_result.stdout,
        stderr: execution_result.stderr,
        compile_error: execution_result.compile_error,
        execution_time_ms: execution_result.execution_time_ms,
        feedback,
        hint: None,
        xp_earned,
        doors_unlocked: success,
        render_state: game_state.to_render_state(),
        test_results: None,
    }))
}

/// Run a function-based challenge with test cases
async fn run_function_based_challenge(
    state: &Arc<AppState>,
    device_id: &str,
    code: &str,
    test_only: bool,
    level: &LevelData,
    level_id: &str,
    game_state: &mut GameState,
) -> Result<Json<SubmitCodeResponse>, (StatusCode, String)> {
    tracing::info!("Running function-based challenge for level: {}", level_id);

    let signature = level
        .function_signature
        .as_ref()
        .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "Function signature missing".to_string()))?;

    // Filter test cases: sample only for TEST, all for SUBMIT
    let test_cases: Vec<_> = level
        .test_cases
        .iter()
        .filter(|tc| !test_only || tc.sample)
        .collect();

    if test_cases.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No test cases defined for this level".to_string()));
    }

    let mut results: Vec<TestCaseResult> = Vec::new();
    let mut total_time_ms = 0u64;

    // Run each test case
    for test_case in &test_cases {
        let harness = generate_harness(code, signature, test_case)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to generate test harness: {}", e)))?;

        let execution_result = state
            .compiler
            .compile_and_run(&harness)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

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

            return Ok(Json(SubmitCodeResponse {
                success: false,
                stdout: String::new(),
                stderr: execution_result.stderr,
                compile_error: Some(err.clone()),
                execution_time_ms: total_time_ms,
                feedback: "Code failed to compile. Check for syntax errors.".to_string(),
                hint: None,
                xp_earned: None,
                doors_unlocked: false,
                render_state: game_state.to_render_state(),
                test_results: Some(test_suite),
            }));
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

    let mut xp_earned = None;

    // Only complete level on SUBMIT (not TEST) and if all passed
    if all_passed && !test_only {
        let xp = game_state.complete_level(level.xp_reward);
        xp_earned = Some(xp);

        game_state.update_unlocked_levels(state.levels.get_prerequisites());

        let xp_i32 = (xp.min(i32::MAX as u32)) as i32;
        db::complete_level(&state.db, device_id, level_id, xp_i32)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to record progress: {}", e),
                )
            })?;
    }

    let feedback = if all_passed {
        if test_only {
            format!("All {} sample tests passed! Click SUBMIT to complete.", passed_count)
        } else if xp_earned.is_some() && xp_earned.unwrap() > 0 {
            format!("All {} tests passed! +{} XP! Doors have been unlocked!", test_suite.total, xp_earned.unwrap())
        } else {
            format!("All {} tests passed! Doors have been unlocked! (Level already completed)", test_suite.total)
        }
    } else {
        format!("{}/{} tests passed. Check your logic and try again!", passed_count, test_suite.total)
    };

    // Save updated state
    persist_session(state, device_id, game_state)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(SubmitCodeResponse {
        success: all_passed,
        stdout: String::new(),
        stderr: String::new(),
        compile_error: None,
        execution_time_ms: total_time_ms,
        feedback,
        hint: None,
        xp_earned,
        doors_unlocked: all_passed && !test_only,
        render_state: game_state.to_render_state(),
        test_results: Some(test_suite),
    }))
}

async fn get_current_level(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
) -> Result<Json<LevelData>, (StatusCode, String)> {
    let game_state = get_or_create_session(&state, &device_id.0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let level_id = game_state.current_level_id.as_ref().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            "No level currently loaded".to_string(),
        )
    })?;

    let level = state.levels.get_level(level_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            format!("Level '{}' not found", level_id),
        )
    })?;

    Ok(Json(level.clone()))
}

async fn get_hint(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
    Path(index): Path<usize>,
) -> Result<Json<String>, (StatusCode, String)> {
    let game_state = get_or_create_session(&state, &device_id.0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let level_id = game_state.current_level_id.as_ref().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            "No level currently loaded".to_string(),
        )
    })?;

    let level = state.levels.get_level(level_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            format!("Level '{}' not found", level_id),
        )
    })?;

    let hint = level.hints.get(index).ok_or_else(|| {
        (StatusCode::NOT_FOUND, "No more hints available".to_string())
    })?;

    Ok(Json(hint.clone()))
}

async fn get_progress(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
) -> Result<Json<ProgressResponse>, (StatusCode, String)> {
    let game_state = get_or_create_session(&state, &device_id.0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // Use progression struct, not legacy fields (which are #[serde(skip)])
    Ok(Json(ProgressResponse {
        total_xp: game_state.progression.total_xp,
        completed_levels: game_state.progression.completed_levels.iter().cloned().collect(),
        current_level: game_state.current_level_id.clone(),
    }))
}

async fn list_saves(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
) -> Result<Json<Vec<SaveSlotResponse>>, (StatusCode, String)> {
    let saves = db::list_save_slots(&state.db, &device_id.0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

    let response: Vec<SaveSlotResponse> = saves
        .into_iter()
        .map(|s| {
            let progress = format!(
                "Level {} | {} XP | {} levels",
                s.current_level.as_deref().unwrap_or("None"),
                s.total_xp,
                s.levels_completed
            );
            SaveSlotResponse {
                id: s.id.to_string(),
                name: s.slot_name,
                timestamp: s.updated_at.to_rfc3339(),
                progress,
                empty: false,
            }
        })
        .collect();

    Ok(Json(response))
}

async fn save_game(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
    Path(slot): Path<String>,
) -> Result<Json<SaveSlotResponse>, (StatusCode, String)> {
    let game_state = get_or_create_session(&state, &device_id.0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let save_data = serde_json::to_value(&game_state)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Serialize error: {}", e)))?;

    let saved = db::upsert_save_slot(
        &state.db,
        &device_id.0,
        &slot,
        &save_data,
        game_state.progression.total_xp as i32,
        game_state.progression.completed_levels.len() as i32,
        game_state.current_level_id.as_deref(),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

    let progress = format!(
        "Level {} | {} XP | {} levels",
        saved.current_level.as_deref().unwrap_or("None"),
        saved.total_xp,
        saved.levels_completed
    );

    Ok(Json(SaveSlotResponse {
        id: saved.id.to_string(),
        name: saved.slot_name,
        timestamp: saved.updated_at.to_rfc3339(),
        progress,
        empty: false,
    }))
}

#[derive(Debug, Serialize)]
struct LoadSaveResponse {
    render_state: RenderState,
}

async fn load_save(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
    Path(slot): Path<String>,
) -> Result<Json<LoadSaveResponse>, (StatusCode, String)> {
    let save = db::get_save_slot(&state.db, &device_id.0, &slot)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Save slot '{}' not found", slot)))?;

    let game_state: GameState = serde_json::from_value(save.save_data)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Parse error: {}", e)))?;

    // Update session with loaded state
    persist_session(&state, &device_id.0, &game_state)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(LoadSaveResponse {
        render_state: game_state.to_render_state(),
    }))
}

async fn delete_save(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
    Path(slot): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    db::delete_save_slot(&state.db, &device_id.0, &slot)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::db::{self, NewSession};
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;

    #[tokio::test]
    async fn session_persists_across_pool_restart() {
        dotenvy::dotenv().ok();
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => {
                eprintln!("Skipping persistence test because DATABASE_URL is not set");
                return;
            }
        };

        let device_id = format!("test-persist-{}", Uuid::new_v4());
        let initial_state = GameState::new();
        let state_json = serde_json::to_value(&initial_state).expect("serialize game state");

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("connect first pool");

        db::init_database(&pool).await.expect("run migrations");

        db::save_session(
            &pool,
            &NewSession {
                device_id: device_id.clone(),
                game_state: state_json.clone(),
            },
        )
        .await
        .expect("save session");

        drop(pool);

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("connect second pool");

        let stored = db::get_session(&pool, &device_id)
            .await
            .expect("load session");

        assert!(stored.is_some());
        assert_eq!(stored.unwrap().device_id, device_id);

        sqlx::query("DELETE FROM sessions WHERE device_id = $1")
            .bind(&device_id)
            .execute(&pool)
            .await
            .expect("cleanup session");

        sqlx::query("DELETE FROM player_progress WHERE device_id = $1")
            .bind(&device_id)
            .execute(&pool)
            .await
            .expect("cleanup progress");
    }
}
