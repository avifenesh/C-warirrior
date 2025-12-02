# Code Warrior: Implementation Guide

## For AI agents and the (solo) developer
- Use this as a very small index of where to find implementation details.
- Prefer reading the focused docs it points to rather than treating this file as a full spec.
- Keep logic and patterns consistent with `docs/ARCHITECTURE.md` and `docs/GAME_DESIGN.md`.

---

## Project Structure

```
src/                  # Rust shared library (game logic + types)
├── game/            # Pure game logic (state, physics, player, world)
├── levels/          # Level loading, validation, puzzle harness
├── compiler/        # C code compilation and execution
├── models/          # Database models
└── persistence/     # Save/load functionality

src-api/             # Axum HTTP API server (for web frontend)
└── src/             # API routes and handlers

src-ui/              # Svelte frontend (UI only)
├── src/lib/         # Shared components, backend abstraction, types
└── src/routes/      # Pages

tools/               # MCP servers and scripts
```

---

## Where to Implement Things

- **Architecture & communication patterns**
  - See `docs/ARCHITECTURE.md` → `docs/architecture/system.md` for:
    - Backend‑authoritative model (Rust owns game state and logic).
    - Game loop threading (Tokio / std threads).
    - Axum route flow; frontend communicates via HTTP/WASM.
    - Cross-reference `docs/logic-mindmap.md` for the current HTTP endpoints, session lifecycle, and frontend polling cadence.

- **Game mechanics & metaphors**  
  - See `docs/GAME_DESIGN.md` → `docs/game_design/mechanics.md` for:
    - C↔RPG mappings (pointers, malloc/free, stack frames, etc.).
    - World design for Memory Marsh and Stack Spire.

- **Curriculum & level structure**  
  - See `docs/CURRICULUM.md` → `docs/curriculum/progression.md` for:
    - Multi-quest level schema with function-based challenges.
    - Phase/level progression and learning goals.
  - Source of truth: `src/assets/levels.json`

- **Tools and runtime helpers**  
  - See `tools/README.md` for:
    - `c_compiler_mcp.py` (C execution MCP server).
    - `generate_map.py` (procedural map generator).

---

## Minimal Patterns to Copy

When you need concrete examples, copy from these existing patterns instead of inventing new ones:

- **HTTP API route (Axum)**  
  - Routes are defined in `src-api/src/main.rs`
  - Use `State<Arc<AppState>>` for shared state access

- **Svelte 5 Runes usage**  
  - Use `$state`, `$derived`, `$effect` as shown in `src-ui/src/routes/+page.svelte`
  - Use backend abstraction: `import { getBackend } from '$lib/backend'`

- **C executor behavior**  
  - The canonical execution pipeline (compile with `gcc`, enforce timeout, capture stdout/stderr) is described in:
    - `docs/architecture/system.md` (`C Runtime Sandbox` section).
    - `tools/c_compiler_mcp.py` (for MCP‑based execution).

---

## Local Development Setup

### Prerequisites

- **Rust** (2021 edition)
- **Node.js** (for Svelte frontend)
- **GCC** (for C code compilation)
- **PostgreSQL** (or use Neon cloud)

### Environment Variables

Create `.env` in the project root:

```bash
# Database (required)
DATABASE_URL=postgres://user:pass@host/dbname

# Sandbox (required for macOS development)
ALLOW_INSECURE_SANDBOX=1
```

### Sandbox Security Note

The C code sandbox uses **seccomp-BPF** on Linux (production). On macOS:

| Environment | Sandbox | Notes |
|-------------|---------|-------|
| **Railway/Docker** | seccomp-bpf | Automatic, secure |
| **Linux (native)** | seccomp-bpf | Automatic, secure |
| **macOS (dev)** | None | Requires `ALLOW_INSECURE_SANDBOX=1` |

**Warning**: Never set `ALLOW_INSECURE_SANDBOX=1` in production. The API will panic on startup if no secure sandbox is available and this variable is not set.

### Running Locally

```bash
# Terminal 1: API server
export ALLOW_INSECURE_SANDBOX=1  # macOS only
cargo run -p code-warrior-api

# Terminal 2: Frontend
cd src-ui && npm run dev
```

---

## Next Steps

1. For any new feature, start by checking `docs/ARCHITECTURE.md` and `docs/GAME_DESIGN.md`.
2. When adding levels, follow the schemas in `docs/CURRICULUM.md` and update `src/assets/levels.json`.
3. Use the AI documentation in `docs/ai/` and `docs/core/` for agent-specific guidance.
