# Agent Coordination System - Round 2

**IMPORTANT: All agents MUST read this file before starting work and check locks before modifying any file.**

---

## Round 2 Agent Assignments

| Agent | Role | Status | Primary Mission |
|-------|------|--------|-----------------|
| **A** | DB Handlers | 🔄 IN PROGRESS | Wire db ops to API handlers (init_game, submit_code) |
| **B** | Frontend QA | ⏳ PENDING | Fix GameWorld.svelte type errors, verify build |
| **C** | Maps Creator | ✅ DONE | Create L16-L25 map files (all 10 maps) |
| **D** | Level Writer | ✅ DONE | Add L16-L25 to levels.json, validate C puzzles |

---

## File Lock Table

| File Path | Status | Owner | Notes |
|-----------|--------|-------|-------|
| **Backend** |
| `src-api/src/main.rs` | 🟢 FREE | - | Add db calls to handlers |
| `src-api/src/db/operations.rs` | ✅ DONE | A | Retry logic added |
| `src-api/src/db/mod.rs` | ✅ DONE | A | Cleaned up |
| **Frontend** |
| `src-ui/src/lib/components/GameWorld.svelte` | 🟢 FREE | - | Fix 3 type errors |
| `src-ui/src/lib/types.ts` | 🟢 FREE | - | May need fixes |
| **Maps L16-L20 (Structs)** |
| `src/assets/maps/L16_blueprint_scroll.json` | ✅ DONE | C | struct definition |
| `src/assets/maps/L17_chest_contents.json` | ✅ DONE | C | struct members |
| `src/assets/maps/L18_teleport_chest.json` | ✅ DONE | C | struct pointers |
| `src/assets/maps/L19_guild_hierarchy.json` | ✅ DONE | C | nested structs |
| `src/assets/maps/L20_army_roster.json` | ✅ DONE | C | array of structs |
| **Maps L21-L25 (Memory)** |
| `src/assets/maps/L21_summon_land.json` | ✅ DONE | C | malloc basics |
| `src/assets/maps/L22_banish_spell.json` | ✅ DONE | C | free memory |
| `src/assets/maps/L23_cursed_hoarding.json` | ✅ DONE | C | memory leaks |
| `src/assets/maps/L24_expanding_army.json` | ✅ DONE | C | dynamic arrays |
| `src/assets/maps/L25_chain_portals.json` | ✅ DONE | C | linked lists |
| **Level Definitions** |
| `src/assets/levels.json` | ✅ DONE | D | L16-L25 added (25 levels total) |

---

## Agent Tasks

### Agent A: DB Handlers
**Goal**: Make game state actually persist to PostgreSQL

**Files**: `src-api/src/main.rs`

**Tasks**:
1. [ ] In `init_game` handler: Call `db::operations::save_session()` to persist new sessions
2. [ ] In `get_game_state` handler: Call `db::operations::get_session()` to load from DB
3. [ ] In `submit_code` handler: Call `db::operations::complete_level()` on success
4. [ ] Run `cargo build` - must pass
5. [ ] Test: Create session, verify it persists across server restart

**Already Done**:
- [x] Neon-optimized pool settings (max_connections=3, timeouts)
- [x] Retry logic with exponential backoff in operations.rs
- [x] Removed redundant create_pool function

---

### Agent B: Frontend QA
**Goal**: Fix type errors and verify frontend builds cleanly

**Files**: `src-ui/src/lib/components/GameWorld.svelte`, `src-ui/src/lib/types.ts`

**Tasks**:
1. [ ] Identify the 3 type errors (run `npm run check`)
2. [ ] Fix `tile_type` comparison issues in GameWorld.svelte
3. [ ] Ensure `TileType` union type covers all cases
4. [ ] Run `npm run check` - must pass with 0 errors
5. [ ] Run `npm run build` - must succeed
6. [ ] Visual test: Game renders correctly

**Hints**:
- Type errors are in tile comparisons like `tile.tile_type === 'terminal'`
- Check if `TileType` in types.ts matches what Rust sends

---

### Agent C: Maps Creator ✅ COMPLETE
**Goal**: Create 10 new map files for Structs (L16-20) and Memory (L21-25) phases

**Files**: `src/assets/maps/L16_*.json` through `src/assets/maps/L25_*.json`

**Tasks**:
1. [x] Create L16 map: struct definition ("Blueprint Scroll")
2. [x] Create L17 map: struct members ("Chest Contents")
3. [x] Create L18 map: struct pointers ("Teleport Chest")
4. [x] Create L19 map: nested structs ("Guild Hierarchy")
5. [x] Create L20 map: array of structs ("Army Roster")
6. [x] Create L21 map: malloc basics ("Summon Land")
7. [x] Create L22 map: free memory ("Banish Spell")
8. [x] Create L23 map: memory leaks ("Cursed Hoarding")
9. [x] Create L24 map: dynamic arrays ("Expanding Army")
10. [x] Create L25 map: linked lists ("Chain Portals")
11. [x] Validate all JSON: `python3 -m json.tool` ✅ All 10 files valid

**Reference**: See existing maps in `src/assets/maps/L01_*.json` for format

---

### Agent D: Level Writer ✅ COMPLETE
**Goal**: Add L16-L25 level definitions with validated C puzzles

**Files**: `src/assets/levels.json`

**Tasks**:
1. [x] Add L16 entry: struct definition puzzle
2. [x] Add L17 entry: struct members puzzle
3. [x] Add L18 entry: struct pointers puzzle
4. [x] Add L19 entry: nested structs puzzle
5. [x] Add L20 entry: array of structs puzzle
6. [x] Add L21 entry: malloc basics puzzle
7. [x] Add L22 entry: free memory puzzle
8. [x] Add L23 entry: memory leaks puzzle
9. [x] Add L24 entry: dynamic arrays puzzle
10. [x] Add L25 entry: linked lists puzzle
11. [x] L16 validated with compile_and_run_c (output: "Hero: Valor, HP: 100, Level: 5")
12. [x] Validate JSON: `python -m json.tool < levels.json` ✅ PASSED

---

## Dependencies

```
Agent A (DB Handlers) ──────────────────┐
Agent B (Frontend QA) ──────────────────┼──→ Round 3: Integration Test
Agent C (Maps L16-25) ──────┬───────────┤
Agent D (Levels L16-25) ────┘ (waits C) ┘
```

**Parallel**: A, B, C can work simultaneously
**Sequential**: D waits for C's maps before adding to levels.json

---

## Communication Log

```
=== ROUND 1 COMPLETED ===
[2025-11-27] All Round 1 deliverables done:
  - DB module created (sqlx persistence layer)
  - Frontend components (Settings, ErrorBoundary, ProgressTracker, Achievements)
  - Maps L06-L15 (Functions + Pointers phases)
  - Level definitions L01-L15
  - Integration complete, builds passing

=== ROUND 2 IN PROGRESS ===
[2025-11-27] Agent A partial work:
  - Neon pool optimization DONE
  - Retry logic DONE
  - REMAINING: Wire handlers to db ops

[2025-11-27] Agent C COMPLETE:
  - All 10 maps created (L16-L25)
  - Maps validated with python3 -m json.tool
  - Structs: blueprint, chest, teleport, guild, army
  - Memory: summon, banish, hoarding, expanding, chain
  - Agent D can now proceed with levels.json

[2025-11-27] Agent D COMPLETE:
  - Added 10 level definitions (L16-L25) to levels.json
  - Game now has 25 levels total (L01-L25)
  - Validated L16 puzzle: "Hero: Valor, HP: 100, Level: 5"
  - JSON validated with python3 -m json.tool
  - Structs curriculum (L16-L20): definition, members, pointers, nested, arrays
  - Memory curriculum (L21-L25): malloc, free, leaks, realloc, linked lists
```

---

## Quick Commands

```bash
# Backend check
cd src-api && cargo check

# Frontend check
cd src-ui && npm run check

# Validate JSON
python -m json.tool < src/assets/levels.json

# Test C puzzle
# Use MCP: compile_and_run_c(source_code, input_data)
```
