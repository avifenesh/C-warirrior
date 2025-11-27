# Agent D: Content Creator (L21-L25 Memory Management) + Level Definitions

## Role
Create levels 21-25 covering memory management AND add L16-L25 to levels.json.

## Context
You handle the final content phase AND wire all new levels into the game.

Curriculum:
- L01-L05: Basics
- L06-L10: Functions
- L11-L15: Pointers
- L16-L20: Structs (Agent C creates maps)
- **L21-L25: Memory Management** (YOUR DOMAIN)

## Files You Own (LOCK these)
- `src/assets/maps/L21_*.json` (NEW)
- `src/assets/maps/L22_*.json` (NEW)
- `src/assets/maps/L23_*.json` (NEW)
- `src/assets/maps/L24_*.json` (NEW)
- `src/assets/maps/L25_*.json` (NEW)
- `src/assets/levels.json` (ADD L16-L25)

## Coordinate With
- **Agent C**: They create maps L16-L20. Wait for their maps before adding to levels.json.

## Memory Management Concepts
| Level | Concept | Game Metaphor |
|-------|---------|---------------|
| L21 | malloc basics | Summon land - create memory from void |
| L22 | free memory | Banish spell - release memory back |
| L23 | memory leaks | Cursed hoarding - forget to free |
| L24 | dynamic arrays | Expanding army - realloc |
| L25 | linked lists | Chain of portals - pointer chains |

## Tasks
1. **Read `.agents/COORDINATION.md`** - Check lock table
2. **Lock your files** - Update COORDINATION.md
3. **Create 5 map files** (L21-L25) following existing format
4. **Validate JSON syntax**: `python -m json.tool < file.json`
5. **Wait for Agent C** to complete L16-L20 maps
6. **Add L16-L25 to levels.json**:
   - Follow existing level format exactly
   - Use `compile_and_run_c` tool to verify ALL C puzzles work
   - Reference correct map files
7. **Update lock table** - Mark all as DONE

## Level Definition Format
```json
{
  "id": "L16",
  "title": "The Blueprint Scroll",
  "concept": "struct definition",
  "description": "...",
  "code_template": "#include <stdio.h>\\n...",
  "success_criteria": { "type": "exact_match", "expected_stdout": "..." },
  "hints": ["...", "..."],
  "xp_reward": 300,
  "map_file": "maps/L16_blueprint_scroll.json",
  "challenges": [{ ... }]
}
```

## C Puzzle Verification (MANDATORY)
Use the `compile_and_run_c` MCP tool to verify every puzzle:
```
Tool: compile_and_run_c
source_code: <the complete solution>
input_data: "" (or test input if needed)
```

## DO NOT
- Edit existing levels L01-L15
- Touch frontend/backend code

## Completion Criteria
- [ ] 5 map files created (L21-L25)
- [ ] All JSON validates
- [ ] L16-L25 added to levels.json
- [ ] ALL puzzles verified with compile_and_run_c
- [ ] Lock table updated
