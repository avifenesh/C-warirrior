# Agent C: System Integration

## Goal
Complete backend wiring, frontend polish, and save/load functionality.

## Status: ⏳ PENDING

## Files to Modify
- `src-api/src/main.rs` - Wire handlers to DB operations
- `src-api/src/db/operations.rs` - Ensure all CRUD ops work
- `src-ui/src/lib/components/` - UI improvements
- `src-ui/src/lib/stores/` - State persistence

## Tasks

### 1. Backend DB Wiring
**Goal**: Make game state persist to PostgreSQL (Neon)

- [ ] Wire `init_game` handler to create/load player session
- [ ] Wire `submit_code` handler to save puzzle attempts
- [ ] Wire `save_progress` handler to persist XP and completed levels
- [ ] Wire `load_progress` handler to restore game state
- [ ] Test: Start game, complete level, restart - progress should persist

**Files**:
- `src-api/src/main.rs` - Route handlers
- `src-api/src/db/operations.rs` - DB queries

### 2. Frontend Type Fixes
**Goal**: Resolve any remaining TypeScript errors

- [ ] Run `npm run check` - should show 0 errors
- [ ] Fix any tile type comparison issues
- [ ] Verify `TileType` enum matches Rust backend
- [ ] Test: Build succeeds with `npm run build`

**Files**:
- `src-ui/src/lib/components/GameWorld.svelte`
- `src-ui/src/lib/types.ts`

### 3. Level Select UI
**Goal**: Allow players to replay completed levels

- [ ] Add level select screen accessible from menu
- [ ] Show completed levels with checkmarks
- [ ] Show locked levels (not yet reached)
- [ ] Allow jumping to any completed level
- [ ] Display XP earned per level

**Files**:
- `src-ui/src/lib/components/LevelSelect.svelte` (new)
- `src-ui/src/lib/components/MainMenu.svelte`

### 4. Save/Load Integration
**Goal**: Seamless progress persistence

- [ ] Auto-save after each level completion
- [ ] Load saved progress on game start
- [ ] Show "Saving..." indicator during save
- [ ] Handle save failures gracefully
- [ ] Test: Web version persists to Neon DB

**Files**:
- `src-ui/src/lib/stores/gameStore.ts`
- `src-ui/src/lib/backend/http.ts`

### 5. End-to-End Validation
**Goal**: Verify full flow works on production

- [ ] Deploy to Vercel/Railway
- [ ] Run Playwright E2E test
- [ ] Verify: New game → Complete L01 → Save → Refresh → Progress loads
- [ ] Check no console errors
- [ ] Take screenshots of key states

## Verification Commands

```bash
# Backend check
cd src-api && cargo check

# Frontend check
cd src-ui && npm run check

# Run API locally
cd src-api && cargo run

# Test production
curl https://code-warrior-api-production.up.railway.app/health
```

## Dependencies
- Agent A and B can work in parallel (they only touch levels.json and maps/)
- Agent C should coordinate if modifying shared state files

## Notes
- Don't break Tauri IPC - test desktop after changes
- Don't break HTTP API - test web after changes
- Keep interfaces identical for both backends
