# Agent Coordination System - Round 2

**IMPORTANT: All agents MUST read this file before starting work and check locks before modifying any file.**

---

## Round 2 Agent Assignments

| Agent | Role | Status | Primary Mission |
|-------|------|--------|-----------------|
| **A** | DB Handlers | 🔄 IN PROGRESS | Wire db ops to API handlers (init_game, submit_code) |
| **B** | Frontend QA | ⏳ PENDING | Fix GameWorld.svelte type errors, verify build |
| **C** | Maps Creator | ⏳ PENDING | Create L16-L25 map files (all 10 maps) |
| **D** | Level Writer | ⏳ PENDING | Add L16-L25 to levels.json, validate C puzzles |

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
| `src/assets/maps/L16_*.json` | 🟢 FREE | - | struct definition |
| `src/assets/maps/L17_*.json` | 🟢 FREE | - | struct members |
| `src/assets/maps/L18_*.json` | 🟢 FREE | - | struct pointers |
| `src/assets/maps/L19_*.json` | 🟢 FREE | - | nested structs |
| `src/assets/maps/L20_*.json` | 🟢 FREE | - | array of structs |
| **Maps L21-L25 (Memory)** |
| `src/assets/maps/L21_*.json` | 🟢 FREE | - | malloc basics |
| `src/assets/maps/L22_*.json` | 🟢 FREE | - | free memory |
| `src/assets/maps/L23_*.json` | 🟢 FREE | - | memory leaks |
| `src/assets/maps/L24_*.json` | 🟢 FREE | - | dynamic arrays |
| `src/assets/maps/L25_*.json` | 🟢 FREE | - | linked lists |
| **Level Definitions** |
| `src/assets/levels.json` | 🟢 FREE | - | Add L16-L25 entries |

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

### Agent C: Maps Creator
**Goal**: Create 10 new map files for Structs (L16-20) and Memory (L21-25) phases

**Files**: `src/assets/maps/L16_*.json` through `src/assets/maps/L25_*.json`

**Tasks**:
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

**Reference**: See existing maps in `src/assets/maps/L01_*.json` for format

---

### Agent D: Level Writer
**Goal**: Add L16-L25 level definitions with validated C puzzles

**Files**: `src/assets/levels.json`

**Dependencies**: Wait for Agent C to create map files

**Tasks**:
1. [ ] Add L16 entry: struct definition puzzle
2. [ ] Add L17 entry: struct members puzzle
3. [ ] Add L18 entry: struct pointers puzzle
4. [ ] Add L19 entry: nested structs puzzle
5. [ ] Add L20 entry: array of structs puzzle
6. [ ] Add L21 entry: malloc basics puzzle
7. [ ] Add L22 entry: free memory puzzle
8. [ ] Add L23 entry: memory leaks puzzle
9. [ ] Add L24 entry: dynamic arrays puzzle
10. [ ] Add L25 entry: linked lists puzzle
11. [ ] **MANDATORY**: Use `compile_and_run_c` to verify EVERY puzzle solution
12. [ ] Validate JSON: `python -m json.tool < levels.json`

**C Puzzle Validation** (REQUIRED):
```
Use MCP tool: compile_and_run_c(source_code, input_data)
Every puzzle MUST compile and produce expected output
```

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

[Agents: Add your updates here]
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
