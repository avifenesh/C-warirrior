# AI Assistant Documentation

**Welcome! This directory contains documentation for AI agents working on Code Warrior.**

## Quick Start

### For All AI Agents
1. **Read constraints first**: [`../core/CONSTRAINTS.md`](../core/CONSTRAINTS.md) - Critical rules
2. **Find your domain**: [`../core/DOMAINS.md`](../core/DOMAINS.md) - What to read when
3. **Follow workflows**: [`../core/WORKFLOWS.md`](../core/WORKFLOWS.md) - Step-by-step guides
4. **Check flow map**: [`../logic-mindmap.md`](../logic-mindmap.md) - Current backend/frontend routes and sources

### For Claude Code
4. **Use skills**: [`skills.md`](skills.md) - Specialized domain knowledge
5. **Use MCP tools**: [`mcp-servers.md`](mcp-servers.md) - Executable tools

### For Other Agents (Gemini, etc.)
4. **Use MCP tools**: [`mcp-servers.md`](mcp-servers.md) - Executable tools
5. **Check agent docs**: See `AGENTS.md` or `GEMINI.md` in root

---

## Documentation Structure

```
docs/
├── ai/                    # AI agent documentation (you are here)
│   ├── README.md         # This file - entry point
│   ├── skills.md         # Claude Code skills guide
│   └── mcp-servers.md    # MCP tool documentation
│
├── core/                 # Shared documentation for ALL agents
│   ├── CONSTRAINTS.md    # Critical rules (READ FIRST)
│   ├── DOMAINS.md        # Task → documentation mapping
│   └── WORKFLOWS.md      # Common development patterns
│
├── architecture/         # Technical system design
│   └── system.md
│
├── game_design/          # Game mechanics and metaphors
│   └── mechanics.md
│
└── curriculum/           # Educational progression
    └── progression.md
```

---

## Agent-Specific Entry Points

### Claude Code
**Start here**: `CLAUDE.md` (root) → References this documentation

**Features**:
- Skills in `.claude/skills/` for specialized guidance
- MCP tools for C compilation and map generation
- Full access to all core documentation

---

### Gemini
**Start here**: `GEMINI.md` (root) → References this documentation

**Features**:
- MCP tools for C compilation and map generation
- Full access to all core documentation
- Cross-agent compatibility via `AGENTS.md`

---

### Other Agents
**Start here**: `AGENTS.md` (root) → References this documentation

**Features**:
- MCP tools (if agent supports MCP)
- Full access to all core documentation
- Cross-agent guidelines

---

## The Critical Rules (TL;DR)

From `docs/core/CONSTRAINTS.md`:

1. **Backend Authority**: All game logic in Rust, not Svelte
2. **C Code Verification**: ALWAYS use `compile_and_run_c` tool
3. **No Magic**: Every game mechanic maps to real C concept
4. **Fixed Tech Stack**: Rust, Tauri, Svelte - no alternatives
5. **Solo Developer**: Optimize for fast iteration, not team process

---

## Common Tasks → Quick Reference

| Task | Documentation | Tools |
|------|---------------|-------|
| **Add game feature** | `rust-tauri-patterns` skill (Claude) or `core/WORKFLOWS.md`; confirm endpoints in `logic-mindmap.md` | None |
| **Create C puzzle** | `c-puzzle-designer` skill (Claude) or `core/WORKFLOWS.md` | `compile_and_run_c` (REQUIRED) |
| **Generate map** | `core/WORKFLOWS.md` | `generate_map.py` or future MCP |
| **Design game mechanic** | `game-metaphor-mapper` skill (Claude) or `game_design/mechanics.md` | None |
| **Fix bug** | `architecture/system.md` | Depends on issue |
| **Add level** | `curriculum/progression.md` + `assets/levels.json`; check unlock flow in `logic-mindmap.md` | `compile_and_run_c`, `generate_map.py` |

---

## File Organization

### Where Things Live

**Code**:
```
src/          # Rust backend (game logic)
src-ui/       # Svelte frontend (UI only)
tools/        # MCP servers and scripts
```

**Data**:
```
src/assets/levels.json    # Level definitions (SOURCE OF TRUTH)
src/assets/maps/          # Tiled map files
```

**Documentation**:
```
docs/core/          # Shared AI documentation
docs/architecture/  # Technical design
docs/game_design/   # Mechanics
docs/curriculum/    # Educational theory
docs/ai/           # This directory
```

**Agent Instructions**:
```
CLAUDE.md          # Claude Code specific
AGENTS.md          # Cross-agent guidelines
GEMINI.md          # Gemini specific
.claude/skills/    # Claude Code skills
```

---

## Getting Unstuck

### "I don't know where to start"
→ Read [`../core/DOMAINS.md`](../core/DOMAINS.md) to find the right documentation

### "I don't understand the architecture"
→ Read `../architecture/system.md`

### "I need to verify C code"
→ Use `compile_and_run_c` MCP tool (see [`mcp-servers.md`](mcp-servers.md))

### "I'm not sure about a game mechanic"
→ Check `../game_design/mechanics.md` or use `game-metaphor-mapper` skill (Claude)

### "What can I change?"
→ Read [`../core/CONSTRAINTS.md`](../core/CONSTRAINTS.md) to see what's fixed

---

## Contributing to AI Documentation

### When to Update:
- New patterns emerge
- Common mistakes identified
- Tools added/changed
- Architecture evolves

### How to Update:
1. Edit relevant markdown file
2. Keep examples current
3. Update cross-references
4. Test with AI agents
5. Commit changes

### Keep It:
- **Concise**: Solo developer doesn't need verbose docs
- **Actionable**: Focus on "how to do X"
- **Current**: Update when codebase changes
- **Cross-agent**: Works for Claude, Gemini, others

---

## Philosophy

This documentation follows MCP best practices:
- **Progressive discovery**: Start simple, drill down as needed
- **Tool-focused**: MCP servers for execution, skills for guidance
- **Single source of truth**: `levels.json` for levels, `CONSTRAINTS.md` for rules
- **Agent-agnostic core**: Works across different AI assistants

The goal is **fast, focused iteration** for a solo developer with AI assistance.
