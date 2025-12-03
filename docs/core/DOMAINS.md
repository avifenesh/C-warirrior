# Code Warrior: Domain Mapping

**Quick reference for finding the right documentation.**

## When Asked About... → Read This

### Architecture & System Design
**Keywords**: structure, Rust backend, Axum, WASM, Svelte frontend, how it works

**Primary Source**: `docs/architecture/system.md`
**Flow Map**: `docs/logic-mindmap.md` (backend routes, frontend entry points)

**Key Topics**:
- Rust/Axum/Svelte+WASM stack details
- Backend-authoritative pattern
- HTTP contract and frontend flow
- State management
- Database schema

---

### Game Mechanics & Features
**Keywords**: gameplay, mechanics, how X works in-game, physics, metaphors

**Primary Source**: `docs/game_design/mechanics.md`

**Key Topics**:
- C concept → game mechanic mappings
- malloc/free as land creation/destruction
- Pointer as grappling hook
- Memory visualization
- Player abilities

---

### Levels & Progression
**Keywords**: level design, difficulty, curriculum, what concepts are taught

**Primary Source**: `src/assets/levels.json` (SOURCE OF TRUTH)

**Secondary**: `docs/curriculum/progression.md` (theory)

**Key Topics**:
- Exact level definitions
- Concept introduction order
- Difficulty progression
- Prerequisites per level

---

### Tools & Automation
**Keywords**: compile C, generate maps, test puzzles, MCP servers

**Primary Source**: `tools/README.md`

**Available Tools**:
- `c_compiler_mcp.py` → Compile and run C code
- `level_tools_mcp.py` → Level management and validation
- `memory_mcp.py` → Cross-session memory
- `project_health_mcp.py` → Build status checks
- `test_runner_mcp.py` → Test suite runner

**For Testing**: See `docs/core/TESTING.md` for full testing protocol
**For Large Analysis**: See `docs/ai/gemini-usage.md` for Gemini CLI usage

---

### C Puzzle Design
**Keywords**: create puzzle, test C code, write challenge

**Use Claude Skill**: `c-puzzle-designer` (Claude Code only)

**Key Steps**:
1. Define educational goal
2. Design minimal challenge
3. Create test cases
4. **VERIFY with compile_and_run_c tool**
5. Map to game mechanics

---

### Implementation Patterns
**Keywords**: how to add feature, Rust patterns, backend routes

**Common Patterns**:
- Adding new game feature
- State management
- Error handling
- Async operations
- Database integration

---

### System Constraints
**Keywords**: rules, constraints, what not to do

**Primary Source**: `docs/core/CONSTRAINTS.md`

**Critical Rules**:
- Backend authority (logic in Rust)
- C code verification (mandatory testing)
- No magic (mechanics map to C)
- Tech stack is fixed

---

## Task → Documentation Quick Reference

| Task Type | Primary Resource |
|-----------|------------------|
| Add new game feature | `rust-backend-patterns` skill + `docs/architecture/` |
| Create C puzzle | `c-puzzle-designer` skill + `compile_and_run_c` tool |
| Design level | `src/assets/levels.json` + `game-metaphor-mapper` skill |
| Fix bug | Check `docs/architecture/system.md` first |
| Generate map | Use `generate_map.py` or future MCP server |
| Understand metaphors | `game-metaphor-mapper` skill |
| Architecture decision | `code-warrior-architect` skill |

---

## Finding Files

### Source Code
```
src/                 # Rust shared library (game + levels + compiler)
src-api/             # Axum HTTP API server
src-ui/              # Svelte frontend
tools/               # Scripts and MCP servers
```

### Documentation
```
docs/core/           # Shared AI docs (this file)
docs/architecture/   # Technical details
docs/game_design/    # Mechanics and metaphors
docs/curriculum/     # Educational theory
docs/ai/             # AI agent guides
docs/interfaces/     # Type schemas (game-state, level-schema)
```

### Assets
```
src/assets/levels.json    # Level definitions (SOURCE OF TRUTH)
src/assets/maps/          # Tiled map files
```

---

## When in Doubt

1. **Check constraints first**: `docs/core/CONSTRAINTS.md`
2. **Find the right domain**: Use this file (DOMAINS.md)
3. **Use tools**: MCP servers for execution tasks
4. **Activate skills**: Claude Code skills for specialized guidance (Claude only)

---

## Cross-Agent Compatibility

This documentation works for:
- **Claude Code** (with skills and MCP)
- **Gemini** (with MCP tools)
- **Other agents** (with MCP tools)

Agent-specific instructions:
- Claude: See `CLAUDE.md`
- All agents: See `AGENTS.md`
