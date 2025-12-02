# Claude Code Skills Guide

Skills in `.claude/skills/` provide specialized domain knowledge.

## Available Skills

### code-warrior-architect
System architecture patterns and design decisions.

### c-puzzle-designer
Design and validate C programming puzzles.
**Requires**: `validate_puzzle_suite` MCP tool

### game-metaphor-mapper
Map C concepts to game mechanics.
- `malloc()` → Create land/platforms
- `free()` → Remove obstacles
- Pointer → Grappling hook
- Array → Bridge/path

### rust-backend-patterns
Backend implementation patterns for Axum routes and shared game logic.

### debugging-workflow
Systematic debugging across frontend/backend.

### level-editor
Create and edit levels with proper structure and validation.

---

## Usage

Skills auto-activate based on task context. Manual activation:
```
"Use the c-puzzle-designer skill to create a pointer challenge"
```

---

## Creating Skills

Skills live in `.claude/skills/skill-name/SKILL.md`

```markdown
---
name: skill-name
description: Brief description
allowed-tools:
  - tool_name
---

# Skill Title

Content...
```
