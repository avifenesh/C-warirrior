# Agent B: Frontend QA

## Goal
Fix type errors in GameWorld.svelte and verify frontend builds cleanly.

## Status: ⏳ PENDING

## Tasks
1. [ ] Identify the 3 type errors (run `npm run check`)
2. [ ] Fix `tile_type` comparison issues in GameWorld.svelte
3. [ ] Ensure `TileType` union type covers all cases
4. [ ] Run `npm run check` - must pass with 0 errors
5. [ ] Run `npm run build` - must succeed
6. [ ] Visual test: Game renders correctly

## Files
- **Modify**: `src-ui/src/lib/components/GameWorld.svelte`
- **Modify**: `src-ui/src/lib/types.ts` (if needed)

## Known Issues
The type errors are in tile comparisons like:
```typescript
if (tile.tile_type === 'terminal') { ... }
```

The `TileType` might not be properly typed, causing TypeScript to complain.

## Debugging Steps

### 1. Run check to see exact errors:
```bash
cd src-ui && npm run check
```

### 2. Check TileType definition in types.ts:
Look for something like:
```typescript
type TileType = 'floor' | 'wall' | 'water' | 'void' | 'terminal' | 'door' | 'npc';
```

### 3. Common fixes:
- If `tile_type` is typed as `string`, change to `TileType`
- If missing tile types, add them to the union
- If switch statement missing cases, add them

## Verification
```bash
cd src-ui
npm run check   # Should show 0 errors
npm run build   # Should succeed
```

## DO NOT
- Change game logic
- Modify backend files
- Break existing functionality
- Add unnecessary type casts (fix properly)
