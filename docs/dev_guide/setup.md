# Code Warrior: Implementation Guide

## For AI agents and the (solo) developer
- Use this as a very small index of where to find implementation details.
- Prefer reading the focused docs it points to rather than treating this file as a full spec.
- Keep logic and patterns consistent with `docs/ARCHITECTURE.md` and `docs/GAME_DESIGN.md`.

---

## Where to Implement Things

- **Architecture & IPC patterns**
  - See `docs/ARCHITECTURE.md` → `docs/architecture/system.md` for:
    - Backend‑authoritative model (Rust owns game state and logic).
    - Game loop threading (Tokio / std threads).
    - Axum route flow and Tauri commands/events (frontend `invoke`, backend `emit_all`).
    - Cross-reference `docs/logic-mindmap.md` for the current HTTP endpoints, session lifecycle, and frontend polling cadence.

- **Game mechanics & metaphors**  
  - See `docs/GAME_DESIGN.md` → `docs/game_design/mechanics.md` for:
    - C↔RPG mappings (pointers, malloc/free, stack frames, etc.).
    - World design for Memory Marsh and Stack Spire.

- **Curriculum & level structure**  
  - See `docs/CURRICULUM.md` → `docs/curriculum/progression.md` for:
    - Level JSON schema (concept, pedagogy, challenge, rewards).
    - Phase/level progression and learning goals.

- **Tools and runtime helpers**  
  - See `docs/IMPLEMENTATION.md` → `tools/README.md` for:
    - `c_compiler_mcp.py` (C execution MCP server).
    - `generate_map.py` (procedural map generator).

---

## Minimal Patterns to Copy

When you need concrete examples, copy from these existing patterns instead of inventing new ones:

- **Tauri command + event pattern**  
  - Rust command and event examples live in `docs/architecture/system.md` and in code under `src-tauri/` (e.g. `move_player`, `submit_c_code`, `game_tick` events).

- **Svelte 5 Runes usage**  
  - Use `$state`, `$derived`, `$effect` as shown in:
    - `SKILL.md` (short example).
    - Svelte snippets referenced in `docs/architecture/system.md`.

- **C executor behavior**  
  - The canonical execution pipeline (compile with `gcc`, enforce timeout, capture stdout/stderr) is described in:
    - `docs/architecture/system.md` (`C Runtime Sandbox` section).
    - `tools/c_compiler_mcp.py` (for MCP‑based execution).

---

## Next Steps

1. For any new feature, start by checking `docs/ARCHITECTURE.md` and `docs/GAME_DESIGN.md`.
2. When adding levels, follow the schemas in `docs/CURRICULUM.md`.
3. Use `SKILL.md` when working with AI agents so they respect the constraints above.

