# AI Assistant Documentation

Documentation for AI agents working on Code Warrior.

## Quick Start

1. **Read constraints**: [`../core/CONSTRAINTS.md`](../core/CONSTRAINTS.md)
2. **Find your domain**: [`../core/DOMAINS.md`](../core/DOMAINS.md)
3. **Check flow map**: [`../logic-mindmap.md`](../logic-mindmap.md)

### For Claude Code
- Use skills in `.claude/skills/` for specialized guidance
- MCP tools available: c_compiler, project_health, level_tools, memory, test_runner

---

## Documentation Structure

```
docs/
├── ai/                    # AI agent documentation
│   ├── README.md         # This file
│   ├── skills.md         # Claude Code skills guide
│   └── gemini-usage.md   # Gemini CLI usage guide
│
├── core/                 # Shared documentation
│   ├── CONSTRAINTS.md    # Critical rules
│   ├── DOMAINS.md        # Task → documentation mapping
│   ├── WORKFLOWS.md      # Development patterns
│   └── TESTING.md        # Testing protocol
│
├── architecture/         # Technical system design
├── game_design/          # Game mechanics
└── curriculum/           # Educational progression
```

---

## Critical Rules (TL;DR)

1. **Backend Authority**: All game logic in Rust, not Svelte
2. **C Code Verification**: ALWAYS use `validate_puzzle_suite` tool
3. **No Magic**: Every game mechanic maps to real C concept
4. **Fixed Tech Stack**: Rust (Axum + WASM), Svelte - no alternatives

---

## Common Tasks

| Task | Documentation | Tools |
|------|---------------|-------|
| **Create C puzzle** | [`../core/WORKFLOWS.md`](../core/WORKFLOWS.md) #2 | `validate_puzzle_suite` (REQUIRED) |
| **Add game feature** | [`../core/WORKFLOWS.md`](../core/WORKFLOWS.md) #1 | Check `docs/logic-mindmap.md` |
| **Design mechanic** | [`../game_design/mechanics.md`](../game_design/mechanics.md) | None |
| **Add level** | [`../curriculum/progression.md`](../curriculum/progression.md) | `level_tools` MCP + `levels.json` |
| **Test changes** | [`../core/TESTING.md`](../core/TESTING.md) | Playwright (web) |
| **Large codebase analysis** | [`gemini-usage.md`](gemini-usage.md) | Gemini CLI |
