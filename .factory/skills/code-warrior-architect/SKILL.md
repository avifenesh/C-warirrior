---
name: code-warrior-architect
description: System architecture patterns and design decisions for Code Warrior game development
---

# Code Warrior: System Architecture Guide

Expert in the Rust/Tauri/Svelte architecture for Code Warrior: C Mastery.

## Core Architecture Principles

### 1. Backend-Authoritative Design
**All game logic lives in Rust.** Svelte is purely for rendering and user interaction.

```
User Input → Svelte (UI) → Tauri Command → Rust (Logic) → Event → Svelte (Render)
```

**What goes where:**
- **Rust**: Physics, XP calculation, inventory, collision detection, game state
- **Svelte**: Rendering, animations, user input collection, visual feedback
- **Never in Svelte**: Game logic, calculations, state mutations

### 2. Technology Stack
- **Backend**: Rust 2021, Axum, Diesel (SQLite), Tokio
- **Frontend**: Svelte 5 (Runes), TailwindCSS, TypeScript
- **Bridge**: Tauri 2.0 Commands/Events
- **Assets**: Tiled maps, procedural generation

### 3. Feature Implementation Pattern

When adding a new feature:

1. **Define Rust data structures** with `#[derive(Serialize, Deserialize, TS)]`
2. **Implement pure logic functions** in Rust
3. **Create Tauri command** with `#[tauri::command]`
4. **Emit events** for state changes
5. **Generate TypeScript types** with ts-rs
6. **Create Svelte component** using Runes for reactivity

### 4. State Management

```rust
// In Rust: Authoritative state
pub struct GameState {
    player: Player,
    inventory: Vec<Item>,
    world: WorldState,
}

// Wrapped in Arc<Mutex<>> for thread safety
// Frontend receives immutable snapshots
```

### 5. Error Handling

All Tauri commands return `Result<T, String>`:
```rust
#[tauri::command]
async fn do_action(state: State<'_, AppState>) -> Result<Response, String> {
    // Logic here
}
```

## Common Anti-Patterns to Avoid

❌ **Don't**: Implement game logic in Svelte/JavaScript
✅ **Do**: Call Rust commands, receive events

❌ **Don't**: Store authoritative state in Svelte stores
✅ **Do**: Sync Svelte state from Rust events

❌ **Don't**: Calculate XP/damage/physics in frontend
✅ **Do**: Send action to Rust, receive computed result

## File Organization

```
src/
├── main.rs              # Tauri entry point
├── commands/            # Tauri commands
├── game/                # Game logic modules
│   ├── player.rs
│   ├── inventory.rs
│   └── world.rs
├── types/               # Shared types with TS generation
└── db/                  # Database models and migrations

src-ui/
├── lib/
│   ├── components/      # Svelte components
│   ├── stores/          # Local UI state only
│   └── types/           # Generated from Rust
└── routes/              # SvelteKit routes
```

## Integration Points

- **MCP Servers**: Use `compile_and_run_c` to validate C puzzles
- **Procedural Generation**: Map generation for game levels
- **Documentation**: Reference `docs/architecture/system.md` for details
