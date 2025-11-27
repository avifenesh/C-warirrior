# Agent Coordination System - Round 2

**IMPORTANT: All agents MUST read this file before starting work and check locks before modifying any file.**

---

## Quick Reference

- **Check locks before writing**: See [File Lock Table](#file-lock-table)
- **Claim a file**: Edit this file, set `Status: LOCKED`, add your Agent ID
- **Release a file**: Set `Status: DONE`, clear Agent ID
- **Request a file**: Add your ID to `Waiting` column, check back later

---

## Round 2 Agent Assignments

| Agent | Role | Primary Mission |
|-------|------|-----------------|
| Agent A | DB Persistence | Wire db module to save sessions/progress |
| Agent B | Frontend Fixes | Fix GameWorld.svelte type errors |
| Agent C | Content L16-20 | Struct phase maps |
| Agent D | Content L21-25 | Memory mgmt maps + levels.json |

---

## File Lock Table

### Lock Status Key
- `FREE` - Available for any agent to claim
- `LOCKED` - Currently being modified by an agent
- `DONE` - Completed, read-only unless reopened

### Backend (Agent A Domain)

| File Path | Status | Locked By | Waiting | Notes |
|-----------|--------|-----------|---------|-------|
| `src-api/src/main.rs` | FREE | - | - | Wire db ops to handlers |
| `src-api/src/db/operations.rs` | FREE | - | - | May need adjustments |

### Frontend (Agent B Domain)

| File Path | Status | Locked By | Waiting | Notes |
|-----------|--------|-----------|---------|-------|
| `src-ui/src/lib/components/GameWorld.svelte` | FREE | - | - | Fix 3 type errors |
| `src-ui/src/lib/types.ts` | FREE | - | - | May need type fixes |

### Map Files - Levels 16-20 (Agent C Domain)

| File Path | Status | Locked By | Waiting | Notes |
|-----------|--------|-----------|---------|-------|
| `src/assets/maps/L16_blueprint_scroll.json` | FREE | - | - | NEW FILE |
| `src/assets/maps/L17_chest_contents.json` | FREE | - | - | NEW FILE |
| `src/assets/maps/L18_teleport_chest.json` | FREE | - | - | NEW FILE |
| `src/assets/maps/L19_guild_hierarchy.json` | FREE | - | - | NEW FILE |
| `src/assets/maps/L20_army_roster.json` | FREE | - | - | NEW FILE |

### Map Files - Levels 21-25 (Agent D Domain)

| File Path | Status | Locked By | Waiting | Notes |
|-----------|--------|-----------|---------|-------|
| `src/assets/maps/L21_summon_land.json` | FREE | - | - | NEW FILE |
| `src/assets/maps/L22_banish_spell.json` | FREE | - | - | NEW FILE |
| `src/assets/maps/L23_cursed_hoarding.json` | FREE | - | - | NEW FILE |
| `src/assets/maps/L24_expanding_army.json` | FREE | - | - | NEW FILE |
| `src/assets/maps/L25_chain_portals.json` | FREE | - | - | NEW FILE |

### Level Definitions (Agent D Domain)

| File Path | Status | Locked By | Waiting | Notes |
|-----------|--------|-----------|---------|-------|
| `src/assets/levels.json` | FREE | - | - | Add L16-L25 |

---

## Locking Protocol

### To Claim a File:
1. Read this COORDINATION.md file
2. Find the file in the lock table
3. Check if `Status` is `FREE`
4. If FREE: Edit this file, change `Status` to `LOCKED`, add your Agent ID
5. If LOCKED: Add your Agent ID to `Waiting` column, work on something else

### To Release a File:
1. Finish your work on the file
2. Edit this COORDINATION.md
3. Change `Status` to `DONE` (or `FREE` if others may need it)
4. Clear your Agent ID from `Locked By`
5. If someone is in `Waiting`, notify them (or they'll check on next read)

### Conflict Resolution:
- First agent to update COORDINATION.md wins the lock
- If you see a stale lock (>30 min), you may claim it
- When in doubt, wait and check again

---

## Completion Checklist

### Agent A: DB Persistence
- [ ] Modified `init_game` to persist sessions
- [ ] Modified `get_game_state` to load from DB
- [ ] Modified `submit_code` to persist completions
- [ ] `cargo build` passes
- [ ] Updated lock table: All files marked DONE

### Agent B: Frontend Fixes
- [ ] Fixed type errors in `GameWorld.svelte`
- [ ] `npm run check` passes with 0 errors
- [ ] `npm run build` passes
- [ ] Updated lock table: All files marked DONE

### Agent C: Maps L16-L20
- [ ] Created `L16_blueprint_scroll.json`
- [ ] Created `L17_chest_contents.json`
- [ ] Created `L18_teleport_chest.json`
- [ ] Created `L19_guild_hierarchy.json`
- [ ] Created `L20_army_roster.json`
- [ ] Validated JSON syntax
- [ ] Updated lock table: All files marked DONE

### Agent D: Maps L21-L25 + Level Definitions
- [ ] Created `L21_summon_land.json`
- [ ] Created `L22_banish_spell.json`
- [ ] Created `L23_cursed_hoarding.json`
- [ ] Created `L24_expanding_army.json`
- [ ] Created `L25_chain_portals.json`
- [ ] Added L16-L25 to `levels.json`
- [ ] Verified ALL C puzzles with `compile_and_run_c`
- [ ] Updated lock table: All files marked DONE

---

## Communication Log

```
=== ROUND 1 COMPLETED ===

[2025-11-27] Round 1 Summary:
  - Agent 1: Database module (sqlx persistence layer) - DONE
  - Agent 2: Frontend components (Settings, ErrorBoundary, ProgressTracker, Achievements) - DONE
  - Agent 3: Maps L06-L10 (Functions phase) - DONE
  - Agent 4: Maps L11-L15 (Pointers phase) - DONE
  - Agent 5: Level definitions L01-L15 - DONE
  - Agent 6: Integration (wired everything together) - DONE
  - All builds passing (cargo build + npm run build)
  - 3 pre-existing type errors in GameWorld.svelte identified for Round 2

=== ROUND 2 START ===

[Round 2] 4 agents assigned:
  - Agent A: DB Persistence (wire existing db module to handlers)
  - Agent B: Frontend Fixes (GameWorld.svelte type errors)
  - Agent C: Content L16-20 (struct phase)
  - Agent D: Content L21-25 (memory management) + levels.json updates

Dependency: Agent D must wait for Agent C's maps before adding L16-20 to levels.json
```

---

## Dependencies Graph

```
Agent A (DB Persistence) ─────────────────┐
Agent B (Frontend Fixes) ─────────────────┼──→ Integration (Round 3)
Agent C (Maps L16-20) ────┬───────────────┤
Agent D (Maps L21-25) ────┴── levels.json ┘
```

**Phase 1 (Parallel):** Agents A, B, C, D work simultaneously on their own files
**Phase 2 (Sequential):** Agent D adds L16-L20 to levels.json after Agent C completes
**Phase 3:** Integration testing (future round)

---
