# Code Warrior – Agent Guidelines (All AI Agents)

**These instructions apply to ANY AI agent (Claude, Gemini, GPT, etc.) working on Code Warrior.**

---

## Quick Start (All Agents)

### 1. Read Core Documentation First

**Start here** - Critical rules that apply to ALL agents:

- **[`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md)** - Critical rules (READ FIRST)
  - Backend authority (logic in Rust, not Svelte)
  - C code verification (mandatory with MCP tool)
  - No magic (mechanics map to C concepts)
  - Fixed tech stack (Rust/Tauri/Svelte)

- **[`docs/core/DOMAINS.md`](docs/core/DOMAINS.md)** - Task → documentation mapping
  - Find what to read based on your task
  - Links to architecture, game design, curriculum

- **[`docs/core/WORKFLOWS.md`](docs/core/WORKFLOWS.md)** - Common workflows
  - Step-by-step guides for frequent tasks
  - Adding features, creating puzzles, generating maps

### 2. Use MCP Tools

**All agents can use MCP servers** (if MCP is supported):

- **`c_compiler_mcp`**: Compile and run C code
  - Tool: `compile_and_run_c(source_code, input_data)`
  - **MANDATORY** for C puzzle validation
  - See [`docs/ai/mcp-servers.md`](docs/ai/mcp-servers.md)

- **`generate_map`** (future): Procedural map generation
  - Current: Use `tools/generate_map.py` via bash

### 3. Agent-Specific Features

**Claude Code users**: See [`CLAUDE.md`](CLAUDE.md) for Claude-specific skills
**Gemini users**: See [`GEMINI.md`](GEMINI.md) for Gemini-specific setup
**Other agents**: Continue reading this file

---

## Project Context

### Solo Developer Focus
- This is a **single-developer** project with multiple AI assistants
- Optimize for **fast, practical iteration** over process or ceremony
- No team coordination, RFCs, or multi-person reviews needed

### Response Style (All Agents)
- Be **concise** by default; avoid long explanations unless requested
- Provide **direct, actionable answers** with minimal boilerplate
- **Don't re-explain** architecture or metaphors every time unless asked
- **Don't propose** large documentation "for the team"

---

## The Four Core Mandates

**From [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md) - ALL agents must follow:**

1. **Backend Authority**
   - NEVER write game logic in Svelte/JS
   - Physics, XP, inventory, state → Rust only
   - Svelte is for rendering and UI only

2. **Verify C Code**
   - When writing C puzzles/solutions, YOU MUST use `compile_and_run_c` MCP tool
   - Do not guess if C code works - test it
   - Validate with all test cases

3. **Procedural Assets**
   - Do NOT generate SVG art manually
   - Use `generate_map.py` tool for Tiled maps
   - Leverage procedural generation

4. **No Magic**
   - Every game mechanic MUST map to real C concept
   - Example: `malloc()` creates land, `free()` removes it
   - See [`docs/game_design/mechanics.md`](docs/game_design/mechanics.md)

---

## Technology Stack

**From [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md) - Fixed, no alternatives:**

- **Backend**: Rust 2021, Axum, Diesel (SQLite), Tokio
- **Frontend**: Svelte 5 (Runes), TailwindCSS, TypeScript
- **Bridge**: Tauri 2.0 (Commands/Events)
- **Assets**: Tiled maps, procedural generation

---

## Project Structure

```
C-warrior/
├── docs/
│   ├── core/               # Shared AI docs (START HERE)
│   │   ├── CONSTRAINTS.md  # Critical rules
│   │   ├── DOMAINS.md      # Task mapping
│   │   └── WORKFLOWS.md    # Step-by-step guides
│   ├── ai/                 # AI assistant guides
│   │   ├── README.md       # Entry point
│   │   ├── skills.md       # Claude Code skills (Claude only)
│   │   └── mcp-servers.md  # MCP tools (all agents)
│   ├── architecture/       # Technical design
│   ├── game_design/        # Mechanics and metaphors
│   └── curriculum/         # Educational progression
├── src/                    # Rust backend (game logic)
├── src-ui/                 # Svelte frontend (UI only)
├── src/assets/levels.json  # Level definitions (SOURCE OF TRUTH)
├── tools/                  # MCP servers and scripts
├── CLAUDE.md               # Claude Code specific
├── AGENTS.md               # This file (all agents)
└── GEMINI.md               # Gemini specific
```

---

## Common Tasks

| Task | Documentation | MCP Tools |
|------|---------------|-----------|
| **Add game feature** | `docs/core/WORKFLOWS.md` #1 | None |
| **Create C puzzle** | `docs/core/WORKFLOWS.md` #2 | `compile_and_run_c` (REQUIRED) |
| **Generate map** | `docs/core/WORKFLOWS.md` #3 | `generate_map.py` (bash) |
| **Design game mechanic** | `docs/game_design/mechanics.md` | None |
| **Fix bug** | `docs/architecture/system.md` | Depends on issue |
| **Add level** | `docs/curriculum/progression.md` | `compile_and_run_c`, `generate_map.py` |
| **Test changes** | See Testing Protocol below | Playwright (web) |

---

## Testing Protocol (MANDATORY)

**Every code change MUST be tested on both platforms:**

### Production URLs

| Service | URL |
|---------|-----|
| **Frontend (Vercel)** | https://code-warrior-seven.vercel.app |
| **API (Railway)** | https://code-warrior-api-production.up.railway.app |

### Local Development Setup

**Desktop (Tauri IPC):**
```bash
cd src-ui && npm run dev  # Terminal 1: Frontend dev server
cargo tauri dev           # Terminal 2: Tauri app
```

**Web (HTTP Backend) - Local:**
```bash
cd src-api && cargo run                              # Terminal 1: API server
cd src-ui && API_URL=http://localhost:3000 npm run dev  # Terminal 2: Frontend
```

### Dual-Platform Testing

1. **Web (HTTP Backend)** - Primary for automation
   - Production: https://code-warrior-seven.vercel.app
   - Local: http://localhost:1420
   - Uses HTTP API to backend
   - **Preferred for automated testing** - same game logic, scriptable

2. **Desktop (Tauri IPC)** - Verify native integration
   - Run via `cargo tauri dev`
   - Uses direct Rust IPC
   - Test after web passes to catch IPC-specific issues

### Testing Checklist

**Before any PR or deployment:**

- [ ] **No Regressions**: Existing features still work
- [ ] **No UI Breaks**: All components render correctly
- [ ] **Full Flow**: Start → Play → Submit code → Complete level
- [ ] **Movement**: WASD navigation works
- [ ] **Interactions**: E key triggers terminals/NPCs
- [ ] **Code Submission**: C code compiles and validates
- [ ] **State Sync**: Game state persists between actions

### Backend Communication

**When modifying `src-ui/src/lib/backend/`:**

1. **Never break Tauri IPC** - Test desktop after changes
2. **Never break HTTP API** - Test web after changes
3. **Keep interfaces identical** - Both implement `Backend` interface
4. **Test both paths** - Fix for one might break the other

### UI/UX Testing

- **Visual inspection**: Screenshots at key states
- **Console cleanliness**: No unexpected errors (404s, exceptions)
- **Performance**: No lag during movement/state updates
- **Error handling**: Graceful failures with user feedback

### Automated Deployment & Validation

**One-command deploy to all platforms:**

```bash
./tools/deploy-and-validate.sh
```

This script:
1. Deploys API to Railway
2. Deploys frontend to Vercel
3. Waits for propagation
4. Runs Playwright validation (API health, levels, frontend, game flow, movement)
5. Reports success/failure with production URLs

**Use this script after any code changes to ensure both platforms work.**

---

## Documentation Philosophy

### What to Document
- **Short, tightly scoped** to the problem
- **Immediately useful** for implementation or future AI context
- **Fill clear gaps** in existing documentation

### What NOT to Document
- Large, general-purpose documents "for the team" (there is no team)
- Verbose explanations of well-understood concepts
- Redundant content that exists elsewhere

### Existing Core Documentation
Treat these as sufficient unless there's a clear gap:
- [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md)
- [`docs/core/DOMAINS.md`](docs/core/DOMAINS.md)
- [`docs/core/WORKFLOWS.md`](docs/core/WORKFLOWS.md)
- `docs/architecture/system.md`
- `docs/game_design/mechanics.md`
- `docs/curriculum/progression.md`

---

## Domain Mapping (Quick Reference)

**From [`docs/core/DOMAINS.md`](docs/core/DOMAINS.md):**

- **Architecture questions** → `docs/architecture/system.md`
- **Game mechanics** → `docs/game_design/mechanics.md`
- **Levels & curriculum** → `src/assets/levels.json` (SOURCE OF TRUTH)
- **Tools** → `tools/` directory + `docs/ai/mcp-servers.md`
- **System constraints** → `docs/core/CONSTRAINTS.md`

---

## Large Codebase Analysis with Gemini CLI

**When analyzing large code chunks or understanding the whole codebase**, use Gemini's massive context window:

```bash
gemini -p "<prompt>" --model gemini-3-pro-preview
```

### When to Use Gemini CLI

- Analyzing files that exceed your context window
- Understanding cross-cutting concerns across many files
- Reviewing large PRs or diffs
- Getting a holistic view of system architecture
- Tracing data flow across the entire codebase

### Best Practices for Gemini Prompts

1. **Structure your prompt clearly** - Use sections/headers:
   ```bash
   gemini -p "## Task
   Analyze the data flow in this codebase.

   ## Context
   This is a Rust/Tauri game with Svelte frontend.

   ## Output Format
   Provide a numbered list of the flow steps." --model gemini-3-pro-preview
   ```

2. **Place large context first** - Put code/files at the beginning, then ask your question:
   ```bash
   gemini -p "$(cat src/*.rs)

   Based on the code above, identify all Tauri commands and their purposes." --model gemini-3-pro-preview
   ```

3. **Be specific about output format** - Request structured output:
   - "Return as JSON with keys: function, purpose, dependencies"
   - "Provide a bullet list of findings"
   - "Create a markdown table of X vs Y"

4. **Break complex analysis into steps** - Ask Gemini to plan first:
   ```bash
   gemini -p "First, list all the modules. Then, for each module, describe its responsibility. Finally, draw the dependency graph." --model gemini-3-pro-preview
   ```

5. **Use self-critique for accuracy** - Ask it to verify:
   ```bash
   gemini -p "Analyze this code for bugs. After listing potential issues, review each one and rate your confidence (high/medium/low)." --model gemini-3-pro-preview
   ```

### Example Use Cases

```bash
# Understand entire codebase structure
gemini -p "$(find src -name '*.rs' -exec cat {} \;)

Analyze this Rust codebase and provide:
1. High-level architecture overview
2. Module dependency graph
3. Key data structures and their relationships" --model gemini-3-pro-preview

# Trace a specific feature
gemini -p "$(cat src/**/*.rs src-ui/src/**/*.ts)

Trace how player movement works from frontend input to backend state update. List every file and function involved." --model gemini-3-pro-preview
```

---

## When in Doubt

1. **Check constraints first**: [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md)
2. **Find your domain**: [`docs/core/DOMAINS.md`](docs/core/DOMAINS.md)
3. **Follow workflows**: [`docs/core/WORKFLOWS.md`](docs/core/WORKFLOWS.md)
4. **Use MCP tools**: For C compilation and execution
5. **Use Gemini CLI**: For large context analysis beyond your window

---

## Agent-Specific Instructions

### For Claude Code
See [`CLAUDE.md`](CLAUDE.md) for:
- Claude Code skills (`.claude/skills/`)
- Skill activation and usage
- Claude-specific features

### For Gemini
See [`GEMINI.md`](GEMINI.md) for:
- Gemini-specific setup
- MCP configuration
- Best practices

### For Other Agents
- Use MCP tools if your agent supports MCP
- Follow core documentation in `docs/core/`
- Respect all constraints in `docs/core/CONSTRAINTS.md`

---

## Full Documentation Index

**Entry Point**: [`docs/ai/README.md`](docs/ai/README.md)

**Core** (Read first):
- [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md)
- [`docs/core/DOMAINS.md`](docs/core/DOMAINS.md)
- [`docs/core/WORKFLOWS.md`](docs/core/WORKFLOWS.md)

**AI Assistant Guides**:
- [`docs/ai/README.md`](docs/ai/README.md) - Entry point
- [`docs/ai/skills.md`](docs/ai/skills.md) - Claude Code skills (Claude only)
- [`docs/ai/mcp-servers.md`](docs/ai/mcp-servers.md) - MCP tools (all agents)

**Technical**:
- `docs/architecture/system.md` - System design
- `docs/game_design/mechanics.md` - Game mechanics
- `docs/curriculum/progression.md` - Educational theory
- `src/assets/levels.json` - Level definitions (SOURCE OF TRUTH)

**Agent-Specific**:
- `CLAUDE.md` - Claude Code
- `AGENTS.md` - This file (all agents)
- `GEMINI.md` - Gemini

## Anti-Mock Policy (Added per user request)
- **NEVER** rely on mock backends for final implementation or testing.
- **ALWAYS** test against the real backend (`src-api` or Tauri IPC).
- Mocks are strictly for transient UI prototyping only and must be removed before completion.
- If the real backend is broken, **FIX THE BACKEND** instead of bypassing it.
