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

## When in Doubt

1. **Check constraints first**: [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md)
2. **Find your domain**: [`docs/core/DOMAINS.md`](docs/core/DOMAINS.md)
3. **Follow workflows**: [`docs/core/WORKFLOWS.md`](docs/core/WORKFLOWS.md)
4. **Use MCP tools**: For C compilation and execution

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
