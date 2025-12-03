# Code Warrior – Agent Guidelines

**All AI agents**: Start here. For Claude Code specifics, see `CLAUDE.md`.

**Flow documentation**: The current backend/frontend flow lives in [`docs/logic-mindmap.md`](docs/logic-mindmap.md). Update it when modifying flow-relevant logic.

---

## Quick Start

1. **Read core docs first**: [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md) (critical rules)
2. **Find your domain**: [`docs/core/DOMAINS.md`](docs/core/DOMAINS.md) (task → docs mapping)
3. **Follow workflows**: [`docs/core/WORKFLOWS.md`](docs/core/WORKFLOWS.md) (step-by-step guides)

---

## The Four Core Mandates

**From [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md) - ALL agents must follow:**

1. **Backend Authority**: Game logic in Rust ONLY. Svelte renders.
2. **Verify C Code**: Use `compile_and_run_c` MCP tool (MANDATORY).
3. **No Magic**: Every mechanic maps to real C concepts.
4. **Fixed Stack**: Rust 2021 + Axum + Svelte 5 + WASM. No alternatives.

See `docs/core/CONSTRAINTS.md` for details.

---

## Project Structure

```
src/          → Rust game logic (shared)
src-api/      → Axum HTTP backend
src-ui/       → Svelte frontend (UI only)
docs/core/    → Shared AI docs (START HERE)
tools/        → MCP servers (see tools/README.md)
```

---

## When You Need Task-Specific Info

- **Testing**: [`docs/core/TESTING.md`](docs/core/TESTING.md)
- **C Puzzles**: [`docs/core/WORKFLOWS.md`](docs/core/WORKFLOWS.md) #2 + `compile_and_run_c` tool
- **Large Context Analysis**: [`docs/ai/gemini-usage.md`](docs/ai/gemini-usage.md)
- **Architecture**: [`docs/architecture/system.md`](docs/architecture/system.md)
- **Game Mechanics**: [`docs/game_design/mechanics.md`](docs/game_design/mechanics.md)
- **Levels**: `src/assets/levels.json` (SOURCE OF TRUTH)

---

## Agent-Specific

- **Claude Code**: See [`CLAUDE.md`](CLAUDE.md)
- **Gemini**: Use CLI for large context (see [`docs/ai/gemini-usage.md`](docs/ai/gemini-usage.md))
- **Other agents**: Use MCP tools if supported, follow `docs/core/`

---

## Production URLs

- **Frontend**: https://code-warrior-seven.vercel.app
- **API**: https://code-warrior-api-production.up.railway.app
