# Code Warrior Development Protocol (Claude Code)

Expert Systems Engineer building "Code Warrior: C Mastery" - a Rust/Axum/Svelte+WASM game.

**Cross-agent guidelines**: See `AGENTS.md`. **Flow documentation**: See `docs/logic-mindmap.md`.

---

## Core Mandates

See `AGENTS.md` for the four core mandates (Backend Authority, Verify C Code, No Magic, Fixed Stack). Details in [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md).

---

## Project Structure

```
src/           → Rust game logic (shared)
src-api/       → Axum HTTP backend (web)
src-wasm/      → WASM wrapper for web
src-ui/        → Svelte 5 frontend
src/assets/levels.json → Level definitions (SOURCE OF TRUTH)
tools/         → MCP servers
.claude/       → Skills, commands, agents
```

---

## MCP Tools

**Available**: See `tools/README.md` for full list.

**Critical for puzzles**: `validate_puzzle_suite(code, tests)` - Batch validation (MANDATORY)

**All tools**: C compiler, project health, levels, memory, testing, screenshots, Gemini. See `tools/README.md`.

---

## Skills (Auto-Activate)

Located in `.claude/skills/`. See [`docs/ai/skills.md`](docs/ai/skills.md) for usage and available skills.

---

## Quick Commands

- **Testing**: `./tools/test-local-e2e.sh` (local) or `./tools/deploy-and-validate.sh` (deploy + validate)
- **Workflows**: See `docs/core/WORKFLOWS.md` for step-by-step guides

---

## Production URLs

- Frontend: https://code-warrior-seven.vercel.app
- API: https://code-warrior-api-production.up.railway.app
