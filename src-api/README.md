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

The server will start on `http://127.0.0.1:3000` by default.

## Endpoints

### Health Check
- `GET /health` - Returns server status and version

### Game Management
- `POST /api/game/init` - Initialize a new game session
- `GET /api/game/state` - Get current game state
- `GET /api/game/render-state` - Get render state for UI
- `POST /api/game/action` - Execute a game action
- `POST /api/game/complete-level` - Mark a level as complete

### Level Management
- `GET /api/levels` - List all available levels
- `POST /api/levels/:id/load` - Load a specific level
- `GET /api/levels/current` - Get current level info

### Code Challenges
- `POST /api/code/submit` - Submit C code for validation
- `GET /api/code/hint` - Get a hint for current challenge

## Architecture

This server follows the **Backend Authority** principle:
- All game logic runs in Rust (this server)
- Frontend (Svelte) only handles rendering and user input
- State is managed server-side

## Current Status

This is a placeholder implementation. All endpoints return mock data.

Next steps:
1. Integrate game engine logic from `src/` directory
2. Connect to level definitions from `src/assets/levels.json`
3. Add C code compilation/validation using MCP tool
4. Add authentication and session management
5. Add database persistence (SQLite via Diesel)

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

- `PORT` - Server port (default: 3000)
- `RUST_LOG` - Logging level (default: `code_warrior_api=debug,tower_http=debug`)
