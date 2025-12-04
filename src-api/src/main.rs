use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::Json,
    routing::{get, post},
    Router,
};
use code_warrior::{
    compiler::CCompiler,
    game::{GamePhase, GameState, PlayerAction, RenderState},
    levels::{
        generate_harness, LevelData, LevelInfo, LevelRegistry, TestCaseResult, TestSuiteResult,
    },
};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

mod auth;
mod auth_middleware;
mod db;
mod email;

struct AppState {
    db: Pool<Postgres>,
    levels: Arc<LevelRegistry>,
    compiler: Arc<CCompiler>,
    /// In-memory session cache keyed by device ID to avoid hitting Neon on every tick/move
    sessions: DashMap<String, GameState>,
    rate_limiter: auth_middleware::SharedRateLimiter,
}

#[derive(Debug, Deserialize)]
struct InitGameRequest {
    // Empty for now, could add initial settings
}

#[derive(Debug, Deserialize)]
struct SyncGameRequest {
    game_state: GameState,
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

#[derive(Debug, Deserialize)]
struct SubmitQuestCodeRequest {
    code: String,
    quest_id: String,
    #[serde(default)]
    test_only: bool,
}

#[derive(Debug, Serialize)]
struct QuestInfoResponse {
    id: String,
    order: u32,
    title: String,
    description: String,
    recommended: bool,
    completed: bool,
    xp_reward: u32,
    user_template: String,
    function_signature: code_warrior::levels::FunctionSignature,
    hints: Vec<String>,
    test_cases: Vec<code_warrior::levels::TestCase>,
    teaching: Option<code_warrior::levels::QuestTeaching>,
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

    let rate_limiter = auth_middleware::create_rate_limiter();

    let state = Arc::new(AppState {
        db: pool,
        levels,
        compiler,
        sessions: DashMap::new(),
        rate_limiter,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:1420".to_string());

    let auth_state = Arc::new(auth::handlers::AuthState {
        db: state.db.clone(),
        email: email::OptionalEmailService::new(),
        google_oauth: auth::oauth::GoogleOAuth::from_env(&api_url),
        github_oauth: auth::oauth::GitHubOAuth::from_env(&api_url),
        frontend_url,
    });

    let auth_routes = Router::new()
        .route("/register", post(auth::handlers::register))
        .route("/login", post(auth::handlers::login))
        .route("/logout", post(auth::handlers::logout))
        .route("/me", get(auth::handlers::me))
        .route("/verify-email", post(auth::handlers::verify_email))
        .route("/resend-verify", post(auth::handlers::resend_verify))
        .route("/request-reset", post(auth::handlers::request_reset))
        .route("/reset-password", post(auth::handlers::reset_password))
        .route(
            "/oauth/google/start",
            get(auth::handlers::google_oauth_start),
        )
        .route(
            "/oauth/google/callback",
            get(auth::handlers::google_oauth_callback),
        )
        .route(
            "/oauth/github/start",
            get(auth::handlers::github_oauth_start),
        )
        .route(
            "/oauth/github/callback",
            get(auth::handlers::github_oauth_callback),
        )
        .layer(axum::Extension(state.rate_limiter.clone()))
        .layer(middleware::from_fn(
            auth_middleware::auth_rate_limit_middleware,
        ))
        .with_state(auth_state);

    let protected_routes = Router::new()
        .route("/game/init", post(init_game))
        .route("/game/sync", post(sync_game))
        .route("/game/state", get(get_game_state))
        .route("/game/render-state", get(get_render_state))
        .route("/game/action", post(process_action))
        .route("/levels", get(get_available_levels))
        .route("/levels/:id/load", post(load_level))
        .route("/code/submit", post(submit_code))
        .route("/code/submit-quest", post(submit_quest_code))
        .route("/levels/current", get(get_current_level))
        .route("/levels/current/quests", get(get_level_quests))
        .route("/levels/current/quests/:quest_id", get(get_quest))
        .route("/code/hint/:index", get(get_hint))
        .route("/player/progress", get(get_progress))
        .route("/saves", get(list_saves))
        .route("/saves/:slot", post(save_game))
        .route("/saves/:slot", get(load_save))
        .route("/saves/:slot", axum::routing::delete(delete_save))
        .layer(axum::Extension(state.rate_limiter.clone()))
        .layer(axum::Extension(state.db.clone()))
        .layer(middleware::from_fn(auth_middleware::rate_limit_middleware))
        .layer(middleware::from_fn(
            auth_middleware::verification_check_middleware,
        ))
        .layer(middleware::from_fn(auth_middleware::ban_check_middleware))
        .layer(middleware::from_fn(auth_middleware::jwt_auth_middleware));

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/auth", auth_routes)
        .nest("/api", protected_routes)
        .layer(cors)
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Starting Code Warrior API server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

// Helper to get or create game state for a session using an in-memory cache with DB fallback
// All progress is user-based (account-only) - no anonymous/device tracking
async fn get_or_create_session(app: &Arc<AppState>, user_id: Uuid) -> Result<GameState, String> {
    let cache_key = format!("user-{}", user_id);

    // Fast path: in-memory session
    if let Some(entry) = app.sessions.get(&cache_key) {
        let gs = entry.value();
        tracing::info!(
            "DEBUG get_or_create_session: cache hit for {}, completed_quests: {:?}, total_xp: {}",
            cache_key,
            gs.progression.completed_quests,
            gs.progression.total_xp
        );
        return Ok(gs.clone());
    }

    // Fallback: load from database or create a new session
    let db_session = db::get_session_by_user_id(&app.db, user_id).await;

    match db_session {
        Ok(Some(session)) => {
            let mut game_state: GameState = serde_json::from_value(session.game_state)
                .map_err(|e| format!("Failed to parse game state: {}", e))?;

            // Backfill quest completions for old sessions that have completed levels but no quest tracking
            game_state.backfill_quest_completions(&app.levels.get_quest_counts());

            // Recalculate unlocked levels from scratch based on current prerequisites
            // This removes levels that shouldn't be unlocked and adds those that should be
            game_state.recalculate_unlocked_levels(app.levels.get_prerequisites());

            app.sessions.insert(cache_key, game_state.clone());
            Ok(game_state)
        }
        Ok(None) => {
            let mut new_state = GameState::new();

            // Initialize unlocked levels based on prerequisites
            new_state.recalculate_unlocked_levels(app.levels.get_prerequisites());

            let session_json = serde_json::to_value(&new_state)
                .map_err(|e| format!("Failed to serialize game state: {}", e))?;

            // Save new session for user
            db::upsert_session_by_user_id(&app.db, user_id, &session_json)
                .await
                .map_err(|e| format!("Failed to create user session: {}", e))?;

            app.sessions.insert(cache_key, new_state.clone());
            Ok(new_state)
        }
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

// Helper to cache session state in memory only (no DB write)
fn cache_session(app: &Arc<AppState>, user_id: Uuid, state: &GameState) {
    let cache_key = format!("user-{}", user_id);
    app.sessions.insert(cache_key, state.clone());
}

// Helper to persist session state to DB (and update in-memory cache)
// All progress is user-based (account-only) - no anonymous/device tracking
async fn persist_session(
    app: &Arc<AppState>,
    user_id: Uuid,
    state: &GameState,
) -> Result<(), String> {
    cache_session(app, user_id, state);

    let state_json = serde_json::to_value(state)
        .map_err(|e| format!("Failed to serialize game state: {}", e))?;

    db::update_user_session_state(&app.db, user_id, &state_json)
        .await
        .map_err(|e| format!("Failed to update user session: {}", e))?;

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
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
    Json(_payload): Json<InitGameRequest>,
) -> Result<Json<InitGameResponse>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    tracing::info!("Initializing new game session for user: {}", user_id);

    // Force create new state or reset? For now, just get/create
    let game_state = get_or_create_session(&state, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // If we wanted to force reset, we would do it here

    Ok(Json(InitGameResponse {
        device_id: user_id.to_string(),
        game_state,
    }))
}

async fn sync_game(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
    Json(request): Json<SyncGameRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user_id = auth_user.user_id;
    tracing::debug!("Syncing game state for user: {}", user_id);

    persist_session(&state, user_id, &request.game_state)
        .await
        .map_err(|e| {
            tracing::error!("Sync failed for {}: {}", user_id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(serde_json::json!({ "success": true })))
}

async fn get_game_state(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
) -> Result<Json<GameState>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    tracing::debug!("Fetching game state for user: {}", user_id);

    let game_state = get_or_create_session(&state, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(game_state))
}

async fn get_render_state(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
) -> Result<Json<RenderState>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    tracing::debug!("Fetching render state for user: {}", user_id);

    let game_state = get_or_create_session(&state, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(game_state.to_render_state()))
}

async fn process_action(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
    Json(action): Json<PlayerAction>,
) -> Result<Json<RenderState>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    tracing::info!("Processing action for user: {}", user_id);

    let mut game_state = get_or_create_session(&state, user_id)
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
            // Allow resuming from Paused, Coding, or LevelComplete (to continue exploring)
            if matches!(
                game_state.game_phase,
                GamePhase::Paused | GamePhase::Coding | GamePhase::LevelComplete
            ) {
                game_state.game_phase = GamePhase::Playing;
            }
        }
        PlayerAction::SubmitCode { .. } => {
            return Err((
                StatusCode::BAD_REQUEST,
                "Use /api/code/submit for code submission".to_string(),
            ));
        }
    }

    // Cache updated session in memory; persistence happens on level load / code submit
    cache_session(&state, user_id, &game_state);

    Ok(Json(game_state.to_render_state()))
}

async fn get_available_levels(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
) -> Result<Json<Vec<LevelInfo>>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    tracing::info!("Fetching available levels for user: {}", user_id);

    let game_state = get_or_create_session(&state, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // Debug: log progression state
    tracing::info!(
        "User {} progression: total_xp={}, completed_levels={:?}, unlocked_levels={:?}",
        user_id,
        game_state.progression.total_xp,
        game_state.progression.completed_levels,
        game_state.progression.unlocked_levels
    );

    let mut levels_info = state.levels.get_all_info();

    // Update lock/completed status and quest progress based on game state
    for level in &mut levels_info {
        level.locked = !game_state.is_level_unlocked(&level.id);
        level.completed = game_state.is_level_completed(&level.id);

        // Populate quest completion progress
        let completed_count = game_state.get_completed_quest_count(&level.id);
        level.completed_quests = completed_count;
        if level.total_quests > 0 {
            level.completion_percentage =
                (completed_count as f32 / level.total_quests as f32) * 100.0;
        }

        // Debug logging for first few levels
        if level.id == "L01" || level.id == "L02" {
            tracing::info!(
                "DEBUG get_available_levels: {} - completed={}, locked={}, quests={}/{}, xp={}",
                level.id,
                level.completed,
                level.locked,
                level.completed_quests,
                level.total_quests,
                game_state.progression.total_xp
            );
        }
    }

    Ok(Json(levels_info))
}

async fn load_level(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
    Path(level_id): Path<String>,
) -> Result<Json<LoadLevelResponse>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    tracing::info!("Loading level '{}' for user: {}", level_id, user_id);

    // Get level data
    let level = state.levels.get_level(&level_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            format!("Level '{}' not found", level_id),
        )
    })?;

    // Get or create game state
    let mut game_state = get_or_create_session(&state, user_id)
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
    persist_session(&state, user_id, &game_state)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(LoadLevelResponse {
        level_data: level.clone(),
        render_state: game_state.to_render_state(),
    }))
}

async fn submit_code(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
    Json(payload): Json<SubmitCodeRequest>,
) -> Result<Json<SubmitCodeResponse>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    tracing::info!(
        "Submitting code for user: {}, test_only: {}",
        user_id,
        payload.test_only
    );

    // Get game state
    let mut game_state = get_or_create_session(&state, user_id)
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

    let level = state
        .levels
        .get_level(&level_id)
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("Level '{}' not found", level_id),
            )
        })?
        .clone();

    // Check if this is a function-based challenge
    if level.is_function_based() {
        return run_function_based_challenge(
            &state,
            user_id,
            &payload.code,
            payload.test_only,
            &level,
            &level_id,
            &mut game_state,
        )
        .await;
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

    // Save updated state
    persist_session(&state, user_id, &game_state)
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
    user_id: Uuid,
    code: &str,
    test_only: bool,
    level: &LevelData,
    level_id: &str,
    game_state: &mut GameState,
) -> Result<Json<SubmitCodeResponse>, (StatusCode, String)> {
    tracing::info!("Running function-based challenge for level: {}", level_id);

    let signature = level.function_signature.as_ref().ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Function signature missing".to_string(),
        )
    })?;

    // Filter test cases: sample only for TEST, all for SUBMIT
    let test_cases: Vec<_> = level
        .test_cases
        .iter()
        .filter(|tc| !test_only || tc.sample)
        .collect();

    if test_cases.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "No test cases defined for this level".to_string(),
        ));
    }

    let mut results: Vec<TestCaseResult> = Vec::new();
    let mut total_time_ms = 0u64;

    // Run each test case
    for test_case in &test_cases {
        let harness = generate_harness(code, signature, test_case).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to generate test harness: {}", e),
            )
        })?;

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
    }

    let feedback = if all_passed {
        if test_only {
            format!(
                "All {} sample tests passed! Click SUBMIT to complete.",
                passed_count
            )
        } else if let Some(xp) = xp_earned.filter(|&x| x > 0) {
            format!(
                "All {} tests passed! +{} XP! Doors have been unlocked!",
                test_suite.total, xp
            )
        } else {
            format!(
                "All {} tests passed! Doors have been unlocked! (Level already completed)",
                test_suite.total
            )
        }
    } else {
        format!(
            "{}/{} tests passed. Check your logic and try again!",
            passed_count, test_suite.total
        )
    };

    // Save updated state
    persist_session(state, user_id, game_state)
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
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
) -> Result<Json<LevelData>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    let game_state = get_or_create_session(&state, user_id)
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

async fn get_level_quests(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
) -> Result<Json<Vec<QuestInfoResponse>>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    let game_state = get_or_create_session(&state, user_id)
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

    let quests = level.get_quests();
    let response: Vec<QuestInfoResponse> = quests
        .iter()
        .map(|q| {
            let completed = game_state.is_quest_completed(level_id, &q.id);
            QuestInfoResponse {
                id: q.id.clone(),
                order: q.order,
                title: q.title.clone(),
                description: q.description.clone(),
                recommended: q.recommended,
                completed,
                xp_reward: q.xp_reward,
                user_template: q.user_template.clone(),
                function_signature: q.function_signature.clone(),
                hints: q.hints.clone(),
                test_cases: q.test_cases.clone(),
                teaching: q.teaching.clone(),
            }
        })
        .collect();

    Ok(Json(response))
}

async fn get_quest(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
    Path(quest_id): Path<String>,
) -> Result<Json<QuestInfoResponse>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    let game_state = get_or_create_session(&state, user_id)
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

    let quests = level.get_quests();
    let quest = quests.iter().find(|q| q.id == quest_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            format!("Quest '{}' not found in level '{}'", quest_id, level_id),
        )
    })?;

    let completed = game_state.is_quest_completed(level_id, &quest_id);

    Ok(Json(QuestInfoResponse {
        id: quest.id.clone(),
        order: quest.order,
        title: quest.title.clone(),
        description: quest.description.clone(),
        recommended: quest.recommended,
        completed,
        xp_reward: quest.xp_reward,
        user_template: quest.user_template.clone(),
        function_signature: quest.function_signature.clone(),
        hints: quest.hints.clone(),
        test_cases: quest.test_cases.clone(),
        teaching: quest.teaching.clone(),
    }))
}

async fn submit_quest_code(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
    Json(payload): Json<SubmitQuestCodeRequest>,
) -> Result<Json<SubmitCodeResponse>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    tracing::info!(
        "Submitting quest code for user: {}, quest: {}, test_only: {}",
        user_id,
        payload.quest_id,
        payload.test_only
    );
    tracing::info!(
        "DEBUG: Quest submission started for quest_id={}",
        payload.quest_id
    );

    let mut game_state = get_or_create_session(&state, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

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
    })?;

    let quests = level.get_quests();
    let total_quests = quests.len();
    let quest = quests
        .iter()
        .find(|q| q.id == payload.quest_id)
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("Quest '{}' not found", payload.quest_id),
            )
        })?
        .clone();

    // Filter test cases: sample only for TEST, all for SUBMIT
    let test_cases: Vec<_> = quest
        .test_cases
        .iter()
        .filter(|tc| !payload.test_only || tc.sample)
        .collect();

    if test_cases.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "No test cases defined for this quest".to_string(),
        ));
    }

    let mut results: Vec<TestCaseResult> = Vec::new();
    let mut total_time_ms = 0u64;

    for test_case in &test_cases {
        let harness = generate_harness(&payload.code, &quest.function_signature, test_case)
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to generate test harness: {}", e),
                )
            })?;

        let execution_result = state
            .compiler
            .compile_and_run(&harness)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        total_time_ms += execution_result.execution_time_ms;

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
                hint: quest.hints.first().cloned(),
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
    let mut doors_unlocked = false;
    let mut quests_remaining = total_quests;

    // Only complete quest on SUBMIT (not TEST) and if all passed
    if all_passed && !payload.test_only {
        tracing::info!(
            "DEBUG: All tests passed, completing quest {} for level {}",
            payload.quest_id,
            level_id
        );
        let xp = game_state.complete_quest(&level_id, &payload.quest_id, quest.xp_reward);
        xp_earned = Some(xp);
        tracing::info!("DEBUG: Quest completed, XP earned: {}", xp);

        let completed_count = game_state.get_completed_quest_count(&level_id);
        tracing::info!(
            "DEBUG: Completed quest count for {}: {}/{}",
            level_id,
            completed_count,
            total_quests
        );
        quests_remaining = total_quests.saturating_sub(completed_count);

        // Check if all quests completed → level complete
        if quests_remaining == 0 {
            if let Some(_) = game_state.maybe_complete_level(total_quests) {
                doors_unlocked = true;
            }
        }
        // Refresh unlocked levels after any quest completion
        game_state.update_unlocked_levels(state.levels.get_prerequisites());

        // Clear active_quest_id after successful completion to prevent auto-reload
        // Player must interact with a terminal again to start another quest
        game_state.active_quest_id = None;

        // Persist state
        persist_session(&state, user_id, &game_state)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
        tracing::info!(
            "DEBUG: Session persisted for user {}, completed_quests for {}: {:?}",
            user_id,
            level_id,
            game_state.progression.completed_quests.get(&level_id)
        );
    }

    let feedback = if all_passed {
        if payload.test_only {
            format!(
                "All {} sample tests passed! Click SUBMIT to complete.",
                passed_count
            )
        } else if quests_remaining == 0 {
            format!(
                "Quest complete! +{} XP! All quests done - doors unlocked!",
                xp_earned.unwrap_or(0)
            )
        } else if let Some(xp) = xp_earned.filter(|&x| x > 0) {
            format!(
                "Quest complete! +{} XP! {} quest(s) remaining.",
                xp, quests_remaining
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

    Ok(Json(SubmitCodeResponse {
        success: all_passed,
        stdout: String::new(),
        stderr: String::new(),
        compile_error: None,
        execution_time_ms: total_time_ms,
        feedback,
        hint: if !all_passed {
            quest.hints.first().cloned()
        } else {
            None
        },
        xp_earned,
        doors_unlocked,
        render_state: game_state.to_render_state(),
        test_results: Some(test_suite),
    }))
}

async fn get_hint(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
    Path(index): Path<usize>,
) -> Result<Json<String>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    let game_state = get_or_create_session(&state, user_id)
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

    let hint = level
        .hints
        .get(index)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "No more hints available".to_string()))?;

    Ok(Json(hint.clone()))
}

async fn get_progress(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
) -> Result<Json<ProgressResponse>, (StatusCode, String)> {
    let user_id = auth_user.user_id;
    let game_state = get_or_create_session(&state, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // Use progression struct, not legacy fields (which are #[serde(skip)])
    Ok(Json(ProgressResponse {
        total_xp: game_state.progression.total_xp,
        completed_levels: game_state
            .progression
            .completed_levels
            .iter()
            .cloned()
            .collect(),
        current_level: game_state.current_level_id.clone(),
    }))
}

async fn list_saves(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
) -> Result<Json<Vec<SaveSlotResponse>>, (StatusCode, String)> {
    let user_id = auth_user.user_id;

    let saves = db::list_save_slots_by_user_id(&state.db, user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", e),
            )
        })?;

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
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
    Path(slot): Path<String>,
) -> Result<Json<SaveSlotResponse>, (StatusCode, String)> {
    let user_id = auth_user.user_id;

    let game_state = get_or_create_session(&state, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let save_data = serde_json::to_value(&game_state).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Serialize error: {}", e),
        )
    })?;

    let saved = db::upsert_save_slot_for_user(
        &state.db,
        user_id,
        &slot,
        &save_data,
        game_state.progression.total_xp as i32,
        game_state.progression.completed_levels.len() as i32,
        game_state.current_level_id.as_deref(),
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB error: {}", e),
        )
    })?;

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
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
    Path(slot): Path<String>,
) -> Result<Json<LoadSaveResponse>, (StatusCode, String)> {
    let user_id = auth_user.user_id;

    let save = db::get_save_slot_by_user_id(&state.db, user_id, &slot)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", e),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("Save slot '{}' not found", slot),
            )
        })?;

    let game_state: GameState = serde_json::from_value(save.save_data).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Parse error: {}", e),
        )
    })?;

    // Update session with loaded state
    persist_session(&state, user_id, &game_state)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(LoadSaveResponse {
        render_state: game_state.to_render_state(),
    }))
}

async fn delete_save(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<auth_middleware::AuthUser>,
    Path(slot): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = auth_user.user_id;

    db::delete_save_slot_for_user(&state.db, user_id, &slot)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", e),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::db;
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

        // Create a test user ID (simulating authenticated user)
        let user_id = Uuid::new_v4();
        let initial_state = GameState::new();
        let state_json = serde_json::to_value(&initial_state).expect("serialize game state");

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("connect first pool");

        db::init_database(&pool).await.expect("run migrations");

        // Create a test user first
        let test_email = format!("test-{}@example.com", user_id);
        sqlx::query(
            "INSERT INTO users (id, email, username, password_hash) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
        )
        .bind(user_id)
        .bind(&test_email)
        .bind(format!("testuser-{}", user_id))
        .bind("test_hash")
        .execute(&pool)
        .await
        .expect("create test user");

        // Save session for authenticated user
        db::upsert_session_by_user_id(&pool, user_id, &state_json)
            .await
            .expect("save session");

        drop(pool);

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("connect second pool");

        let stored = db::get_session_by_user_id(&pool, user_id)
            .await
            .expect("load session");

        assert!(stored.is_some());

        // Cleanup
        sqlx::query("DELETE FROM sessions WHERE user_id = $1")
            .bind(user_id)
            .execute(&pool)
            .await
            .expect("cleanup session");

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&pool)
            .await
            .expect("cleanup user");
    }
}
