# Code Warrior: Core Constraints

**These rules apply to ALL AI agents working on this project.**

## 1. Backend Authority (CRITICAL)

**All game logic MUST be in Rust.** Frontend (Svelte/JS) is for rendering ONLY.

### What Goes Where:

**Rust (Backend):**
- Physics calculations
- XP and progression logic
- Inventory management
- Collision detection
- Game state mutations
- Score calculation

**Svelte (Frontend):**
- Rendering visuals
- Animations
- User input collection
- Visual feedback
- UI state (menus, dialogs)

### ❌ NEVER in Frontend:
- Game logic
- State calculations
- Authoritative game state

## 2. C Code Verification (MANDATORY)

**When writing C puzzles or solutions, you MUST verify them.**

### How to Verify:
- Use the `compile_and_run_c` MCP tool
- Test with ALL test cases
- Verify output matches expected results
- Check for memory leaks (if using malloc)

### Never:
- Guess that C code works without testing
- Skip verification "to save time"
- Assume syntax without compiling

## 3. Procedural Assets

**Do NOT attempt to generate SVG art or create assets manually.**

### For Maps:
- Use `generate_map.py` tool (or MCP equivalent)
- Configure Tiled-compatible JSON
- Let algorithms create organic terrain

### For Sprites:
- Use existing asset pipeline
- Reference existing sprites
- Don't create inline SVG in code

## 4. No Magic Rule

**Every game mechanic MUST map to a real C concept.**

### Valid Mappings:
- `malloc()` → Creates land/platforms
- `free()` → Removes obstacles
- Pointer → Grappling hook (points to and reaches things)
- Array → Bridge/path (sequential tiles)
- NULL → Void/danger zone

### Invalid:
- Generic "collect coins" mechanics
- Combat unrelated to C concepts
- Arbitrary puzzles without C connection

## 5. Technology Stack (Fixed)

**Do NOT suggest alternative technologies.**

- **Backend**: Rust 2021, Axum (HTTP API), SQLx (Postgres), Tokio
- **Frontend**: Svelte 5 (Runes), TailwindCSS, TypeScript
- **Bridge**: HTTP (Axum) for all web clients; WASM embeds core logic for low-latency local play
- **Assets**: Tiled maps, procedural generation
- **Database**: PostgreSQL (Neon for production)

## 6. Solo Developer Context

**Optimize for single developer, fast iteration.**

### Do:
- Provide concise, actionable guidance
- Focus on implementation, not process
- Keep documentation short and useful

### Don't:
- Propose team workflows (RFCs, multi-person reviews)
- Create verbose documentation "for the team"
- Add heavy process for small changes

## 7. Security (Important)

**Validate all inputs from frontend in Rust.**

- Never trust frontend calculations
- Sanitize file paths before operations
- Use Result<T, String> for error handling
- Rate limit expensive operations

## 8. File Organization

**Follow established patterns:**

```
src/                  # Rust shared library (game logic + types)
├── game/            # Pure game logic (state, physics, player, world)
├── levels/          # Level loading, validation, puzzle harness
├── compiler/        # C code compilation and execution
├── models/          # Database models
└── persistence/     # Save/load functionality

src-api/             # Axum HTTP API server (for web frontend)
└── src/             # API routes and handlers

src-wasm/            # WASM wrapper for web

src-ui/              # Svelte frontend (UI only)
├── src/lib/         # Shared components, backend abstraction, types
└── src/routes/      # Pages

docs/
├── core/            # Shared AI docs (this file)
├── architecture/    # Technical specs
├── game_design/     # Mechanics
└── curriculum/      # Level progression

tools/               # MCP servers and scripts
src/assets/          # levels.json (source of truth)
```

## Verification Checklist

Before committing work, verify:
- [ ] Game logic is in Rust (not Svelte)
- [ ] C code tested with compile_and_run_c
- [ ] Game mechanics map to C concepts
- [ ] No manual asset generation
- [ ] Follows backend-authoritative pattern
- [ ] Error handling uses Result<T, String>
- [ ] TypeScript types generated from Rust
