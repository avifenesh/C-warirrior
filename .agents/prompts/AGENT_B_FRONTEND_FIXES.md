# Agent B: Frontend Fixes & Polish

## Role
Fix type errors in GameWorld.svelte and polish the frontend.

## Context
There are 3 pre-existing type errors in `GameWorld.svelte` related to `tile_type` comparisons. The `TileType` is a union type but comparisons treat it as string.

## Files You Own (LOCK these)
- `src-ui/src/lib/components/GameWorld.svelte`
- `src-ui/src/lib/types.ts` (if type changes needed)

## Files to READ ONLY
- `src-ui/src/lib/components/*.svelte` (reference existing patterns)
- `src-ui/src/routes/+page.svelte`

## Tasks
1. **Read `.agents/COORDINATION.md`** - Check lock table before starting
2. **Lock your files** - Update COORDINATION.md
3. **Fix type errors in GameWorld.svelte**:
   - Check if `TileType` is properly typed in `types.ts`
   - Fix comparisons like `tile.tile_type === 'terminal'` to be type-safe
   - Ensure switch statements cover all tile types
4. **Polish opportunities** (if time permits):
   - Add loading states/animations
   - Improve error messages
   - Add keyboard shortcut hints
5. **Test**: `npm run check` (should pass with 0 errors)
6. **Test**: `npm run build`
7. **Update lock table** - Mark files as DONE

## Type Fix Hints
Look at how `TileType` is defined. It might be:
```typescript
type TileType = 'floor' | 'wall' | 'water' | 'void' | 'terminal' | 'door' | 'npc';
```

If comparisons fail, check:
1. Is `tile_type` properly typed on the Tile interface?
2. Are there missing types in the union?
3. Is there a mismatch between Rust enum serialization and TS types?

## DO NOT
- Change game logic
- Modify backend files
- Break existing functionality

## Completion Criteria
- [ ] `npm run check` passes with 0 errors
- [ ] `npm run build` succeeds
- [ ] GameWorld still renders correctly
- [ ] Lock table updated
