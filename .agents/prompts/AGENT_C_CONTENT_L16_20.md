# Agent C: Content Creator (L16-L20 Structs Phase)

## Role
Create levels 16-20 covering C structs.

## Context
The curriculum progression:
- L01-L05: Basics (printf, variables, if/else, loops, arrays)
- L06-L10: Functions (void, return, stack, scope, recursion)
- L11-L15: Pointers (address-of, declaration, dereference, arithmetic, NULL)
- **L16-L20: Structs** (YOUR DOMAIN)
- L21-L25: Memory Management (Agent D)

## Files You Own (LOCK these)
- `src/assets/maps/L16_*.json` (NEW)
- `src/assets/maps/L17_*.json` (NEW)
- `src/assets/maps/L18_*.json` (NEW)
- `src/assets/maps/L19_*.json` (NEW)
- `src/assets/maps/L20_*.json` (NEW)

## Files to READ (coordinate with Agent D)
- `src/assets/levels.json` - Agent D will add your levels here

## Struct Concepts to Cover
| Level | Concept | Game Metaphor |
|-------|---------|---------------|
| L16 | struct definition | Blueprint scroll - define a hero's attributes |
| L17 | struct members | Chest contents - access individual items |
| L18 | struct pointers | Teleport to chest - modify via pointer |
| L19 | nested structs | Guild hierarchy - struct inside struct |
| L20 | array of structs | Army roster - manage multiple heroes |

## Tasks
1. **Read `.agents/COORDINATION.md`** - Check lock table
2. **Lock your map files** - Update COORDINATION.md
3. **Create 5 map files** following existing format (see L01-L15 maps)
4. **Validate JSON syntax**: `python -m json.tool < file.json`
5. **Document map files** in communication log for Agent D
6. **Update lock table** - Mark maps as DONE

## Map Format Reference
```json
{
  "width": 20,
  "height": 15,
  "tilewidth": 32,
  "tileheight": 32,
  "layers": [
    {
      "type": "tilelayer",
      "name": "floor",
      "data": [/* 20x15 = 300 tile IDs */]
    },
    {
      "type": "objectgroup",
      "name": "objects",
      "objects": [
        { "type": "spawn", "x": 96, "y": 224, ... },
        { "type": "terminal", "x": 320, "y": 224, ... },
        { "type": "door", "x": 576, "y": 224, ... }
      ]
    }
  ]
}
```

Tile IDs: 1=floor, 3=wall, 4=decorative_wall

## DO NOT
- Edit `levels.json` (Agent D does this)
- Touch existing map files L01-L15

## Completion Criteria
- [ ] 5 map files created (L16-L20)
- [ ] All JSON validates
- [ ] Maps documented in communication log
- [ ] Lock table updated
