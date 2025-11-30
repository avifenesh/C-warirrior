# Code Warrior: Domain Mapping

**Quick reference for finding the right documentation.**

## When Asked About... → Read This

### Architecture & System Design
**Keywords**: structure, Tauri, Rust backend, Svelte frontend, how it works

**Primary Source**: `docs/architecture/system.md`
**Flow Map**: `docs/logic-mindmap.md` (backend routes, frontend entry points)

**Key Topics**:
- Rust/Tauri/Svelte stack details
- Backend-authoritative pattern
- Command/Event flow
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

**Primary Source**: `tools/` directory

**Available Tools**:
- `c_compiler_mcp.py` → Compile and run C code
- `generate_map.py` → Create procedural Tiled maps
- (Future) `map_generator_mcp.py` → MCP version

**Usage**: See `docs/ai/mcp-servers.md`

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
**Keywords**: how to add feature, Rust patterns, Tauri commands

**Use Claude Skill**: `rust-tauri-patterns` (Claude Code only)

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
| Add new game feature | `rust-tauri-patterns` skill + `docs/architecture/` |
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
src/                 # Rust backend
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
```

### Assets
```
src/assets/levels.json    # Level definitions (SOURCE OF TRUTH)
src/assets/maps/          # Tiled map files
src/assets/sprites/       # Game sprites
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
- Gemini: See `GEMINI.md`
