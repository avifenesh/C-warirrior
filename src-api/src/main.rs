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
    levels::{LevelData, LevelInfo, LevelRegistry},
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Database persistence module (Agent 1)
mod db;

// Shared application state
#[derive(Clone)]
struct AppState {
    db: Pool<Postgres>,
    levels: Arc<LevelRegistry>,
    compiler: Arc<CCompiler>,
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
}

#[derive(Debug, Serialize)]
struct SubmitCodeResponse {
    success: bool,
    stdout: String,
    stderr: String,
    compile_error: Option<String>,
    execution_time_ms: u64,
    feedback: String,
    xp_earned: Option<u32>,
    render_state: RenderState,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    database: String,
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
                .unwrap_or_else(|_| "code_warrior_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");

    // Create database connection pool
    tracing::info!("Connecting to database...");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
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

// Helper to get or create game state for a session (using DB)
async fn get_or_create_session(
    pool: &Pool<Postgres>,
    device_id: &str,
) -> Result<GameState, String> {
    // Try to fetch session from DB
    match db::get_session(pool, device_id).await {
        Ok(Some(session)) => {
            // Parse game state from JSON
            let game_state: GameState = serde_json::from_value(session.game_state)
                .map_err(|e| format!("Failed to parse game state: {}", e))?;
            Ok(game_state)
        },
        Ok(None) => {
            // Create new session
            let new_state = GameState::new();
            let session_json = serde_json::to_value(&new_state)
                .map_err(|e| format!("Failed to serialize game state: {}", e))?;
            
            db::save_session(pool, &db::NewSession {
                device_id: device_id.to_string(),
                game_state: session_json,
            })
            .await
            .map_err(|e| format!("Failed to create session: {}", e))?;
            
            Ok(new_state)
        },
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

// Helper to update session state (using DB)
async fn update_session(
    pool: &Pool<Postgres>,
    device_id: &str,
    state: GameState,
) -> Result<(), String> {
    let state_json = serde_json::to_value(&state)
        .map_err(|e| format!("Failed to serialize game state: {}", e))?;
        
    db::update_session_state(pool, device_id, &state_json)
        .await
        .map_err(|e| format!("Failed to update session: {}", e))?;

    // Also update player progress table for analytics/leaderboards
    let progress = db::NewProgress {
        device_id: device_id.to_string(),
        completed_levels: state.levels_completed.clone(),
        total_xp: state.total_xp as i32,
        current_level: state.current_level_id.clone(),
        achievements: vec![], // TODO: Extract achievements from state if we add them
    };
    
    db::save_progress(pool, &progress)
        .await
        .map_err(|e| format!("Failed to save progress: {}", e))?;
        
    Ok(())
}

// Handler functions

async fn health_check(
    State(state): State<Arc<AppState>>,
) -> Json<HealthResponse> {
    // Check database health
    let db_status = match sqlx::query("SELECT 1")
        .fetch_one(&state.db)
        .await
    {
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
    let game_state = get_or_create_session(&state.db, &device_id.0).await
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

    let game_state = get_or_create_session(&state.db, &device_id.0).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(game_state))
}

async fn get_render_state(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
) -> Result<Json<RenderState>, (StatusCode, String)> {
    tracing::debug!("Fetching render state for device: {}", device_id.0);

    let game_state = get_or_create_session(&state.db, &device_id.0).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(game_state.to_render_state()))
}

async fn process_action(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
    Json(action): Json<PlayerAction>,
) -> Result<Json<RenderState>, (StatusCode, String)> {
    tracing::info!("Processing action for device: {}", device_id.0);

    let mut game_state = get_or_create_session(&state.db, &device_id.0).await
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
            game_state.game_phase = GamePhase::Playing;
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

    // Update session
    update_session(&state.db, &device_id.0, game_state.clone()).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(game_state.to_render_state()))
}

async fn get_available_levels(
    State(state): State<Arc<AppState>>,
    axum::Extension(device_id): axum::Extension<DeviceId>,
) -> Result<Json<Vec<LevelInfo>>, (StatusCode, String)> {
    tracing::debug!("Fetching available levels for device: {}", device_id.0);

    let game_state = get_or_create_session(&state.db, &device_id.0).await
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
    let level = state
        .levels
        .get_level(&level_id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Level '{}' not found", level_id)))?;

    // Get or create game state
    let mut game_state = get_or_create_session(&state.db, &device_id.0).await
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

    // Save updated state
    update_session(&state.db, &device_id.0, game_state.clone()).await
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
    tracing::info!("Submitting code for device: {}", device_id.0);

    // Get game state
    let mut game_state = get_or_create_session(&state.db, &device_id.0).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // Get current level
    let level_id = game_state
        .current_level_id
        .as_ref()
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "No level currently loaded".to_string()))?
        .clone();

    let level = state
        .levels
        .get_level(&level_id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Level '{}' not found", level_id)))?;

    // Compile and run code
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

        format!("Success! Your code produced the correct output. You earned {} XP!", xp)
    } else {
        "Output doesn't match expected result. Try again!".to_string()
    };

    // Save updated state
    update_session(&state.db, &device_id.0, game_state.clone()).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(SubmitCodeResponse {
        success,
        stdout: execution_result.stdout,
        stderr: execution_result.stderr,
        compile_error: execution_result.compile_error,
        execution_time_ms: execution_result.execution_time_ms,
        feedback,
        xp_earned,
        render_state: game_state.to_render_state(),
    }))
}
