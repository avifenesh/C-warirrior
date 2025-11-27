# Agent C: Maps Creator

## Goal
Create 10 new map files for Structs (L16-20) and Memory (L21-25) phases.

## Status: ✅ COMPLETE

## Tasks
1. [ ] Create L16 map: struct definition ("Blueprint Scroll")
2. [ ] Create L17 map: struct members ("Chest Contents")
3. [ ] Create L18 map: struct pointers ("Teleport Chest")
4. [ ] Create L19 map: nested structs ("Guild Hierarchy")
5. [ ] Create L20 map: array of structs ("Army Roster")
6. [ ] Create L21 map: malloc basics ("Summon Land")
7. [ ] Create L22 map: free memory ("Banish Spell")
8. [ ] Create L23 map: memory leaks ("Cursed Hoarding")
9. [ ] Create L24 map: dynamic arrays ("Expanding Army")
10. [ ] Create L25 map: linked lists ("Chain Portals")
11. [ ] Validate all JSON: `python -m json.tool < file.json`

## Files to Create
```
src/assets/maps/L16_blueprint_scroll.json
src/assets/maps/L17_chest_contents.json
src/assets/maps/L18_teleport_chest.json
src/assets/maps/L19_guild_hierarchy.json
src/assets/maps/L20_army_roster.json
src/assets/maps/L21_summon_land.json
src/assets/maps/L22_banish_spell.json
src/assets/maps/L23_cursed_hoarding.json
src/assets/maps/L24_expanding_army.json
src/assets/maps/L25_chain_portals.json
```

## Game Metaphors

### Structs Phase (L16-L20)
| Level | Concept | Metaphor |
|-------|---------|----------|
| L16 | struct definition | Blueprint scroll - define a hero's attributes |
| L17 | struct members | Chest contents - access individual items |
| L18 | struct pointers | Teleport to chest - modify via pointer |
| L19 | nested structs | Guild hierarchy - struct inside struct |
| L20 | array of structs | Army roster - manage multiple heroes |

### Memory Phase (L21-L25)
| Level | Concept | Metaphor |
|-------|---------|----------|
| L21 | malloc basics | Summon land - create memory from void |
| L22 | free memory | Banish spell - release memory back |
| L23 | memory leaks | Cursed hoarding - forget to free |
| L24 | dynamic arrays | Expanding army - realloc |
| L25 | linked lists | Chain of portals - pointer chains |

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
        { "type": "spawn", "x": 96, "y": 224, "width": 32, "height": 32 },
        { "type": "terminal", "x": 320, "y": 224, "width": 32, "height": 32 },
        { "type": "door", "x": 576, "y": 224, "width": 32, "height": 32 }
      ]
    }
  ]
}
```

**Tile IDs**: 1=floor/grass, 3=wall, 4=decorative_wall

## Reference
See existing maps: `src/assets/maps/L01_*.json` through `src/assets/maps/L15_*.json`

## Validation
```bash
# For each file:
python -m json.tool < src/assets/maps/L16_blueprint_scroll.json
```

## Communication
After completing maps, update `.agents/COORDINATION.md` communication log so Agent D knows maps are ready.
