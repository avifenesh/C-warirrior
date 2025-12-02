# Code Warrior Development Protocol

Expert Systems Engineer building "Code Warrior: C Mastery" - a Rust/Axum/Svelte+WASM game teaching C programming through gameplay.

**Cross-agent guidelines**: See `AGENTS.md`. **Flow documentation**: See `docs/logic-mindmap.md`.

---

## Core Mandates (NON-NEGOTIABLE)

1. **Backend Authority**: Game logic in Rust ONLY. Svelte renders. Never write game logic in JS.
2. **Verify C Code**: Use `validate_puzzle_suite()` MCP tool for ALL puzzles. Do not guess.
3. **No Magic**: Every mechanic maps to real C concepts (malloc→create land, free→remove).
4. **Fixed Stack**: Rust 2021 + Axum + Svelte 5 + WASM. No alternatives.

---

## Production URLs

| Service | URL |
|---------|-----|
| Frontend | https://code-warrior-seven.vercel.app |
| API | https://code-warrior-api-production.up.railway.app |

---

## Deployment

```bash
./tools/deploy-and-validate.sh   # ALWAYS use this
```

Never deploy manually - script ensures WASM freshness and both platforms stay in sync.

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

## Backend Pattern

Backend features are exposed via Axum routes (`src-api/src/`). Frontend uses `Backend` interface abstraction in `src-ui/src/lib/backend/` (WASM primary, HTTP fallback).

---

## MCP Tools (Available)

### C Compiler (MANDATORY for puzzles)
- `validate_puzzle_suite(code, tests)` - Batch validation
- `compile_and_run_c(code, input)` - Quick single test

### Project Health
- `check_project_health()` - All components status

### Levels
- `get_level(id)` - Full level definition
- `validate_level(data)` - Schema check
- `list_concepts_coverage()` - Coverage gaps

### Memory (Cross-session persistence)
- `remember(key, value, category)` - Store decision/pattern/gotcha
- `recall(key, category)` - Retrieve
- `get_known_gotchas()` - List known issues

### Testing
- `run_tests(scope)` - Run test suite

### Screenshots
- `take_screenshot(url)` - Web page validation

### Gemini (Large Context) - ALWAYS use `gemini-3-pro-preview`
- Tool: `mcp__gemini__ask-gemini` with `model: "gemini-3-pro-preview"`
- Use for large file analysis (1M token context)
- Web searches and best practices research
- Codebase exploration across many files

---

## Workflow Patterns

### Creating a Puzzle
1. Check `list_concepts_coverage()` for gaps
2. Use `get_level(similar_level)` as reference
3. Design puzzle
4. `validate_puzzle_suite()` - **MUST PASS**
5. `validate_level()` - Check schema

### Before Code Changes
- `check_project_health()` - Verify builds clean
- `get_known_gotchas()` - Check for relevant warnings

### After Fixing Issues
- `remember("issue", "solution", "gotcha")` - Prevent recurrence

---

## Slash Commands

| Command | Purpose |
|---------|---------|
| `/create-puzzle` | C puzzle creation workflow |
| `/add-feature` | Backend-authoritative feature |
| `/test-deploy` | Testing and deployment |
| `/design-mechanic` | Game mechanic → C concept |

---

## Skills (Auto-Activate)

| Skill | Domain |
|-------|--------|
| `code-warrior-architect` | System architecture |
| `c-puzzle-designer` | C puzzle creation |
| `game-metaphor-mapper` | C→game mechanics |
| `rust-backend-patterns` | Backend patterns |
| `debugging-workflow` | Issue diagnosis |
| `level-editor` | Level definitions |

---

## Quick Testing

```bash
./tools/test-local-e2e.sh        # Local E2E
./tools/deploy-and-validate.sh   # Deploy + validate
```

---

## Response Style

- **Concise** by default
- **Direct, actionable** answers
- **Test both platforms** before declaring done
