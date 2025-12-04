# Code Warrior Development Protocol (Claude Code)
Expert Systems Engineer building "Code Warrior: C Mastery" - a Rust/Axum/Svelte+WASM game.
## Overview
The game teaches C programming through puzzles, using a Rust backend and Svelte frontend. The AI agent (Claude) assists in puzzle generation, validation, and user support. The goal of the project is to create an engaging learning experience for C programming, with high quality gamification and educational content. It should be fun, to have high quality of mattirals, and the theme should be consistent.
**Cross-agent guidelines**: See `AGENTS.md`. **Flow documentation**: See `docs/logic-mindmap.md`.
## Core Mandates
See `AGENTS.md` for the four core mandates (Backend Authority, Verify C Code, No Magic, Fixed Stack). Details in [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md).
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
## MCP Tools
**Critical for puzzles**: `validate_puzzle_suite(code, tests)` - Batch validation (MANDATORY)b
**All tools**: C compiler, project health, levels, memory, testing, screenshots, Gemini. See `tools/README.md`.
## Skills (Auto-Activate)
Located in `.claude/skills/`. See [`docs/ai/skills.md`](docs/ai/skills.md) for usage and available skills.
## Quick Commands
- **Testing**: `./tools/test-local-e2e.sh` (local) or `./tools/deploy-and-validate.sh` (deploy + validate)
- **Workflows**: See `docs/core/WORKFLOWS.md` for step-by-step guides
## Production URLs
- Frontend: https://code-warrior-seven.vercel.app
- API: https://code-warrior-api-production.up.railway.app
## Dployment 
Always use `./tools/deploy-and-validate.sh` to deploy and validate the build.
## IMPORTANT RULES
1. ALWAYS try to understand the original intent of the code before making changes, especially removing code.
2. NEVER leave unused code or commented-out sections in production code.
3. ALWAYS validate that your changes works, and that there is not regression to other logics.
4. ALWAYS update docs after making changes to logic or structure.
5. ALWAYS validate that the feature is fully implemented, never leave stubs or TODOs.
