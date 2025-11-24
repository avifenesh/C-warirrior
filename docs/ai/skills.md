# Claude Code Skills Guide

**Skills are Claude Code-specific features for specialized domain knowledge.**

## Available Skills

Located in `.claude/skills/`

### 1. code-warrior-architect
**Purpose**: System architecture patterns and design decisions

**When to Use**:
- Adding new game features
- Understanding Rust/Tauri/Svelte architecture
- Backend-authoritative pattern questions
- File organization decisions

**Key Knowledge**:
- Backend authority (logic in Rust, not Svelte)
- Tauri command/event flow
- State management patterns
- Feature implementation steps

---

### 2. c-puzzle-designer
**Purpose**: Design and validate C programming puzzles

**When to Use**:
- Creating new C challenges
- Writing test cases
- Verifying puzzle difficulty
- Mapping puzzles to game mechanics

**Key Knowledge**:
- Puzzle creation workflow
- Test case design (3-5 per puzzle)
- **Mandatory C code verification** with compile_and_run_c
- Difficulty progression (beginner → advanced)
- Hint writing strategies

**Tools Required**: `compile_and_run_c` MCP tool

---

### 3. game-metaphor-mapper
**Purpose**: Map C concepts to game mechanics

**When to Use**:
- Designing new game mechanics
- Ensuring C concept accuracy
- Creating visual metaphors
- Level design integration

**Key Mappings**:
- `malloc()` → Create land/platforms
- `free()` → Remove obstacles
- Pointer → Grappling hook
- Array → Bridge/path
- NULL → Void/danger
- Memory leak → Unused platforms, "memory fog"

**Core Principle**: No generic game tropes - every mechanic maps to real C

---

### 4. rust-tauri-patterns
**Purpose**: Backend implementation patterns

**When to Use**:
- Implementing game features
- State management questions
- Error handling patterns
- Async operations
- Database integration

**Key Patterns**:
- Creating Tauri commands
- Event emission for state updates
- TypeScript type generation
- Testing game logic
- Avoiding anti-patterns

---

## How Skills Work

### Automatic Activation
Claude Code automatically loads relevant skills based on your task:
- Mention "add a feature" → `rust-tauri-patterns` activates
- Mention "create a puzzle" → `c-puzzle-designer` activates
- Mention "game mechanic" → `game-metaphor-mapper` activates

### Manual Activation
You can explicitly request a skill:
```
"Use the c-puzzle-designer skill to create a pointer challenge"
```

### Skill + Tools
Skills often work with MCP tools:
- `c-puzzle-designer` + `compile_and_run_c` tool
- `rust-tauri-patterns` + Rust commands
- `game-metaphor-mapper` + level.json

---

## Creating New Skills

Skills live in `.claude/skills/skill-name/SKILL.md`

### Structure:
```markdown
---
name: skill-name
description: Brief description of what this skill does
allowed-tools:         # Optional
  - tool_name
---

# Skill Title

Expert in [domain].

## Sections with clear guidance
...
```

### Best Practices:
- **Focus**: One expertise area per skill
- **Concise**: Short, actionable guidance
- **Examples**: Show, don't just tell
- **Tools**: Reference MCP tools when relevant

---

## Skills vs MCP Servers

| Feature | Skills | MCP Servers |
|---------|--------|-------------|
| **Type** | Instruction prompts | Executable tools |
| **Format** | Markdown files | Python/code |
| **Discovery** | Automatic | Manual invocation |
| **Purpose** | Domain knowledge | Code execution |
| **Example** | "How to design puzzles" | "Compile C code" |

Use **skills** for guidance, use **MCP servers** for execution.

---

## Skill Maintenance

### When to Update:
- Architecture patterns change
- New game mechanics added
- Better practices discovered
- Common mistakes identified

### How to Update:
1. Edit `.claude/skills/skill-name/SKILL.md`
2. Keep examples current with codebase
3. Test with Claude Code
4. Commit changes

---

## Agent Compatibility

**Skills only work with Claude Code.**

For other AI agents (Gemini, etc.):
- Refer to `docs/core/` documentation instead
- Use MCP servers for tools
- See `AGENTS.md` for cross-agent guidance
