# Code Warrior HTTP API Server

Axum-based HTTP API server for Code Warrior.

## Quick Start

```bash
# From the src-api directory
cargo run

# With custom port
PORT=8080 cargo run

# With debug logging
RUST_LOG=debug cargo run
```

The server will start on `http://127.0.0.1:3000` by default. Set `DATABASE_URL` to a Postgres connection string before running (Neon/localhost both work).

## Endpoints

### Health Check
- `GET /health` - Returns server status and version

### Game Management
- `POST /api/game/init` - Initialize or resume a session (creates DB row if missing)
- `POST /api/game/sync` - Persist the provided `GameState`
- `GET /api/game/state` - Get current game state
- `GET /api/game/render-state` - Get render-ready `RenderState`
- `POST /api/game/action` - Apply `PlayerAction` (move, interact, pause/resume)

### Level Management
- `GET /api/levels` - List levels with locked/completed flags
- `POST /api/levels/:id/load` - Load a level, update unlocks, cache session
- `GET /api/levels/current/quests` - List quests for the active level
- `GET /api/levels/current/quests/:quest_id` - Get a single quest

### Code Challenges
- `POST /api/code/submit` - Submit single-challenge code (completes level)
- `POST /api/code/submit-quest` - Submit quest code (per-test harness)
- `GET /api/code/hint/:index` - Get hint by index for current challenge/quest

### Progress & Saves
- `GET /api/player/progress` - Aggregate XP + completed levels
- `GET /api/saves` - List save slots
- `POST /api/saves/:slot` - Upsert a save slot
- `GET /api/saves/:slot` - Load a save slot
- `DELETE /api/saves/:slot` - Delete a save slot

## Architecture

This server follows the **Backend Authority** principle:
- All game logic runs in Rust (this server)
- Frontend (Svelte) only handles rendering and user input
- State is managed server-side

## Current Status

Fully wired to the shared `code-warrior` game library, SQLx/Postgres persistence (sessions + saves), and the C compiler sandbox. Authentication is not implemented; device ID is passed via `X-Device-ID` header.

## Development

```bash
# Check compilation
cargo check

# Run with hot reload (requires cargo-watch)
cargo watch -x run

# Run tests
cargo test

# Build release
cargo build --release
```

## Environment Variables

- `DATABASE_URL` - Postgres connection string (required)
- `PORT` - Server port (default: 3000)
- `RUST_LOG` - Logging level (default: `code_warrior_api=debug,tower_http=debug`)
