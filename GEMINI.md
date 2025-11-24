# Code Warrior Development Protocol (Gemini)

You are an expert Systems Engineer and Game Developer building "Code Warrior: C Mastery".

**This file is for Gemini specifically. For cross-agent guidelines, see `AGENTS.md`.**

---

## Quick Start

### 1. Read Core Documentation First

**Critical Rules**: [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md)
- Backend authority (logic in Rust, not Svelte)
- C code verification (mandatory with MCP tool)
- No magic (mechanics map to C concepts)
- Fixed tech stack

**Task Mapping**: [`docs/core/DOMAINS.md`](docs/core/DOMAINS.md)
- Find what to read based on your task
- Links to architecture, game design, curriculum docs

**Common Workflows**: [`docs/core/WORKFLOWS.md`](docs/core/WORKFLOWS.md)
- Step-by-step guides for frequent tasks
- Adding features, creating puzzles, generating maps

### 2. Use MCP Tools

**Available MCP servers in `tools/`:**

- **`c_compiler_mcp`**: Compile and run C code (MANDATORY for puzzle validation)
  - Tool: `compile_and_run_c(source_code, input_data)`
  - Configure MCP for Gemini (see below)
- **`generate_map`** (future): Procedural map generation
  - Current: Use `tools/generate_map.py` via bash

**Learn more**: [`docs/ai/mcp-servers.md`](docs/ai/mcp-servers.md)

### 3. Gemini-Specific Setup

**MCP Configuration**: (Add configuration instructions if needed)
```
# TODO: Add Gemini-specific MCP configuration once available
# For now, use bash to call tools directly
```

---

## The Four Core Mandates

**From [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md):**

1. **Backend Authority**: NEVER write game logic in Svelte/JS. Physics, XP, and inventory live in Rust. Svelte is for rendering only.

2. **Verify C Code**: When writing a C puzzle or solution, YOU MUST use the `compile_and_run_c` tool to verify it runs and produces the expected output. Do not guess.

3. **Procedural Assets**: Do not try to generate SVG art. Use the `generate_map.py` tool to configure Tiled maps.

4. **No Magic**: Game mechanics must map to real C concepts. (e.g., "Malloc" creates land, "Free" removes it).

---

## Project Structure

```
C-warrior/
├── tools/                   # MCP servers and scripts
├── docs/
│   ├── core/               # Shared AI documentation (START HERE)
│   │   ├── CONSTRAINTS.md  # Critical rules
│   │   ├── DOMAINS.md      # Task mapping
│   │   └── WORKFLOWS.md    # Step-by-step guides
│   ├── ai/                 # AI assistant guides
│   │   ├── README.md       # Entry point
│   │   ├── skills.md       # Claude Code skills (not applicable)
│   │   └── mcp-servers.md  # MCP tools (for all agents)
│   ├── architecture/       # Technical design
│   ├── game_design/        # Mechanics and metaphors
│   └── curriculum/         # Educational progression
├── src/                    # Rust backend (game logic)
├── src-ui/                 # Svelte frontend (UI only)
└── src/assets/levels.json  # Level definitions (SOURCE OF TRUTH)
```

---

## Common Tasks

| Task | Start Here | Tools |
|------|------------|-------|
| **Add game feature** | `docs/core/WORKFLOWS.md` #1 | None |
| **Create C puzzle** | `docs/core/WORKFLOWS.md` #2 | `compile_and_run_c` (REQUIRED) |
| **Design game mechanic** | `docs/game_design/mechanics.md` | None |
| **Generate map** | `docs/core/WORKFLOWS.md` #3 | `generate_map.py` (bash) |
| **Fix bug** | `docs/architecture/system.md` | Depends on issue |
| **Understand architecture** | `docs/architecture/system.md` | None |

---

## Technology Stack

**From [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md) - Fixed, no alternatives:**

- **Backend**: Rust 2021, Axum, Diesel (SQLite), Tokio
- **Frontend**: Svelte 5 (Runes), TailwindCSS, TypeScript
- **Bridge**: Tauri 2.0 (Commands/Events)
- **Assets**: Tiled maps, procedural generation

---

## Solo Developer Context

**From [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md):**

- Optimize for **fast, focused iteration**, not team workflows
- Avoid proposing heavy process (RFCs, multi-person reviews, large documentation suites)
- Keep documentation **short and directly useful** for implementation
- Default to **concise answers and minimal boilerplate**

---

## Response Style

- Be **concise** by default; avoid long explanations unless requested
- Provide **direct, actionable answers**
- **Don't re-explain** the overall architecture unless asked
- **Don't create verbose documentation** "for the team"

---

## When in Doubt

1. **Check constraints**: [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md)
2. **Find your domain**: [`docs/core/DOMAINS.md`](docs/core/DOMAINS.md)
3. **Follow workflows**: [`docs/core/WORKFLOWS.md`](docs/core/WORKFLOWS.md)
4. **Use MCP tools**: For C compilation (if configured)

---

## Differences from Claude Code

**Gemini does NOT have access to:**
- Claude Code skills (`.claude/skills/`)
- Automatic skill activation

**Gemini CAN use:**
- All shared core documentation (`docs/core/`)
- MCP tools (if MCP is configured for Gemini)
- Bash commands to call tools directly
- All technical documentation

**To compensate:**
- Read detailed guides in `docs/core/WORKFLOWS.md`
- Reference `docs/game_design/mechanics.md` for game metaphors
- Use `docs/architecture/system.md` for implementation patterns

---

## Full Documentation Index

**Entry Point**: [`docs/ai/README.md`](docs/ai/README.md)

**Core** (Read these first):
- [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md) - Critical rules
- [`docs/core/DOMAINS.md`](docs/core/DOMAINS.md) - Task mapping
- [`docs/core/WORKFLOWS.md`](docs/core/WORKFLOWS.md) - Step-by-step guides

**AI Assistant Guides**:
- [`docs/ai/README.md`](docs/ai/README.md) - Entry point for all agents
- [`docs/ai/mcp-servers.md`](docs/ai/mcp-servers.md) - MCP tools
- [`docs/ai/skills.md`](docs/ai/skills.md) - Claude Code only (not applicable)

**Technical**:
- `docs/architecture/system.md` - System design (READ for implementation patterns)
- `docs/game_design/mechanics.md` - Game mechanics (READ for metaphors)
- `docs/curriculum/progression.md` - Educational theory
- `src/assets/levels.json` - Level definitions (SOURCE OF TRUTH)

**Cross-Agent**:
- `AGENTS.md` - Guidelines for all AI agents
- `CLAUDE.md` - Claude Code specific instructions
- `GEMINI.md` - This file

---

## Using Tools Without MCP

If MCP is not configured for Gemini, use bash commands directly:

### Compile and Run C Code
```bash
cd tools
python c_compiler_mcp.py
# Then interact with the MCP server
```

### Generate Map
```bash
cd tools
python generate_map.py --algorithm drunkard --width 40 --height 30 --output ../src/assets/maps/level_X.json
```

---

## Key Documentation to Reference Often

Since Gemini doesn't have Claude Code skills, reference these frequently:

### For Implementation Patterns
**Read**: `docs/architecture/system.md`
- Rust/Tauri/Svelte architecture
- Backend-authoritative pattern
- Command/event flow
- State management
- Error handling

### For Game Metaphors
**Read**: `docs/game_design/mechanics.md`
- C concept → game mechanic mappings
- Visual metaphors
- Memory as landscape
- Pointer as grappling hook
- malloc/free mechanics

### For Workflows
**Read**: `docs/core/WORKFLOWS.md`
- Adding features (step-by-step)
- Creating C puzzles (with validation)
- Generating maps
- Debugging
- Testing

---

## Verification Checklist

Before committing work, verify:
- [ ] Game logic is in Rust (not Svelte)
- [ ] C code tested with compile_and_run_c or manual testing
- [ ] Game mechanics map to C concepts
- [ ] No manual asset generation
- [ ] Follows backend-authoritative pattern
- [ ] Error handling uses Result<T, String>
- [ ] TypeScript types generated from Rust

See [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md) for full details.
