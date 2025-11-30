# Code Warrior: Game Design Document

## For AI agents and developers
- Use this as the source of truth for C↔RPG metaphors and allowed mechanics.
- When proposing new gameplay, ensure every mechanic maps to a concrete C concept defined here.
- Do not invent mechanics that contradict the metaphor rules in this document.

---

## Implementation Status (Audit: 2025-11-30)

> **IMPORTANT**: Many features in this document are design goals, not current implementation.
> Features marked **[WORKING]** are implemented and tested. Features marked **[PLANNED]** are design specs for future development.

| Feature | Status | Notes |
|---------|--------|-------|
| Core gameplay loop | **[WORKING]** | Explore → Code → Execute → Progress |
| Player movement | **[WORKING]** | WASD/Arrows with collision |
| Code submission | **[WORKING]** | C compilation via MCP tool |
| Quest system | **[WORKING]** | Multi-quest levels with test harness |
| XP & progression | **[WORKING]** | Level completion awards XP |
| Level unlocking | **[WORKING]** | Complete L01 to unlock L02, etc. |
| Save/Load | **[WORKING]** | PostgreSQL persistence |
| Terminal interaction | **[WORKING]** | Approach terminal → code editor |
| HP/MP Stats | **[PLANNED]** | Not implemented |
| Inventory UI | **[PLANNED]** | Data structures exist, no UI |
| Combat system | **[PLANNED]** | Not implemented |
| Memory Marsh visuals | **[PLANNED]** | Metaphor only, no special visuals |
| Stack Spire tower | **[PLANNED]** | Metaphor only, no special visuals |
| Grappling Hook pointers | **[PLANNED]** | Metaphor only, not interactive |
| malloc visual effects | **[PLANNED]** | No land-rising animation |
| Memory Leak Slimes | **[PLANNED]** | No enemy spawning |
| Debug/Ghost Mode | **[PLANNED]** | Not implemented |
| WFC procedural generation | **[PLANNED]** | Levels are handcrafted JSON |

---

## Table of Contents
1. [Design Philosophy](#design-philosophy)
2. [Core Metaphor System](#core-metaphor-system)
3. [World Design](#world-design)
4. [RPG Systems](#rpg-systems)
5. [Gameplay Mechanics](#gameplay-mechanics)
6. [Visual Design](#visual-design)
7. [Asset Pipeline](#asset-pipeline)

---

## Design Philosophy

### Educational First, Game Second

Code Warrior is not a game with programming elements; it is **C programming visualized as a game**. Every mechanic must:

1. **Map directly to a C concept** - No arbitrary game rules
2. **Maintain technical accuracy** - The metaphor cannot break
3. **Provide immediate feedback** - Code results are instantly visible
4. **Encourage experimentation** - Safe environment to learn from errors

### The 80/20 Rule

- **80% Active Coding**: Players write real C code to progress
- **20% Passive Learning**: Tutorials, NPC dialogue, documentation

**Anti-pattern**: Long cutscenes, text-heavy tutorials, or puzzles solvable without coding.

### Progressive Complexity

Start with concrete concepts (`printf`, variables) before abstract ones (pointers, memory management). Each level builds on previous knowledge.

---

## Core Metaphor System

### Unified C-to-RPG Mapping

This is the **single source of truth** for all metaphor mappings in Code Warrior.

| C Concept | RPG Metaphor | Visual Representation | Gameplay Mechanic | Educational Goal |
|-----------|--------------|----------------------|-------------------|------------------|
| **Variable (`int x`)** | Inventory Slot | Empty box labeled with name | Player must declare slots before storing items | Understand typed storage |
| **Pointer (`int *p`)** | Grappling Hook | Glowing line from player to target | Player can interact with distant objects via pointer | Understand indirection |
| **Dereference (`*p`)** | Pull Hook | Hook retracts, pulling object to player | Access the value at the address | Understand dereference operation |
| **Address-of (`&x`)** | Location Spell | Shows glowing coordinates above object | Get memory address of variable | Understand addresses vs values |
| **malloc()** | Summon Land | Land rises from water in Memory Marsh | Creates traversable terrain | Understand dynamic allocation |
| **free()** | Banish Land | Land sinks back into water | Removes allocated memory | Understand deallocation |
| **Memory Leak** | Slime Monster | Green enemy spawns on orphaned memory | Allocated memory not freed | Understand resource cleanup |
| **Stack Frame** | Tower Room | Vertical room in Stack Spire | Enter on function call, exit on return | Understand call stack |
| **Local Variable** | Room Item | Item appears when entering room | Lost when leaving room (scope) | Understand local scope |
| **Global Variable** | World Item | Item always visible on world map | Accessible from anywhere | Understand global scope |
| **Array** | Corridor | Linear sequence of connected rooms | Indexed access to sequential memory | Understand contiguous storage |
| **Struct** | Backpack/Container | Organized container with labeled pockets | Grouped related data | Understand composite types |
| **Function Call** | Climb Tower | Enter new floor in Stack Spire | Push new frame onto stack | Understand function invocation |
| **Return** | Descend Tower | Exit room, lose local items | Pop frame from stack | Understand function return |
| **Segfault** | The Void | Glitch effect, screen distortion | Player takes damage, level resets | Understand invalid memory access |
| **Buffer Overflow** | Flood | Water overflows from container | Damages adjacent memory tiles | Understand boundary violations |
| **Null Pointer** | Broken Hook | Hook snaps, player falls | Dereferencing causes crash | Understand null checks |
| **Recursion** | Mirror Room | Identical rooms nested inside each other | Multiple calls to same function | Understand recursive calls |
| **Stack Overflow** | Tower Collapse | Spire shakes and crumbles | Too many recursive calls | Understand stack limits |

### Metaphor Consistency Rules

**Rule 1: Physical = Memory**
Every physical location in the game world corresponds to a memory location. Walking through the world IS navigating memory.

**Rule 2: Actions = Code**
Player actions must correspond to C code operations. No "magic spells" that don't map to actual C.

**Rule 3: Consequences are Real**
Memory leaks spawn enemies, segfaults cause damage. The game world reacts to code quality.

---

## World Design

### The Memory Marsh (Heap) [PLANNED - Visual metaphor only]

**Concept**: The heap is dynamic, fragmented, and unmanaged. The Marsh reflects this chaos.

#### Visual Design
- **Aesthetic**: Foggy swamp with floating islands
- **Water**: Represents unallocated memory (null)
- **Islands**: Represent allocated memory blocks
- **Slime**: Memory leaks (allocated but unreferenced)
- **Fog**: Uninitialized memory (unknown state)

#### Mechanics

**Allocation (`malloc`)**:
```c
// Player writes this code
int *ptr = malloc(sizeof(int) * 5);
```
**Game Result**: 5 connected land tiles rise from the water, creating a bridge. Player can now walk across.

**Deallocation (`free`)**:
```c
// Player writes this code
free(ptr);
```
**Game Result**: The 5 land tiles sink back into water. Player loses the bridge.

**Memory Leak**:
```c
// Player writes this code
int *ptr = malloc(100);
// ... player leaves area without calling free()
```
**Game Result**: A Slime Monster spawns on the orphaned memory. It slowly drains player health until addressed.

**Fragmentation**:
- If player randomly allocates and frees, the Marsh becomes a chaotic archipelago
- Navigating becomes difficult (teaches cost of fragmentation)

#### Procedural Generation

Use **Wave Function Collapse (WFC)** algorithm:

```rust
enum TileType {
    Water,      // Unallocated
    Land,       // Allocated
    Slime,      // Leaked memory
    Bridge,     // Pointer connection
}

struct WFCGenerator {
    adjacency_rules: HashMap<TileType, Vec<TileType>>,
    entropy_map: Grid<f32>,
}

impl WFCGenerator {
    fn generate(&mut self, width: usize, height: usize) -> Grid<TileType> {
        // WFC algorithm implementation
        // Rules: Water can be adjacent to Water or Land
        //        Land can be adjacent to Land or Bridge
        //        Slime only spawns on isolated Land
    }
}
```

---

### The Stack Spire (Stack) [PLANNED - Visual metaphor only]

**Concept**: The stack is ordered, strict, and follows LIFO. The Spire is a vertical tower representing this structure.

#### Visual Design
- **Aesthetic**: Gothic tower with stacked rooms
- **Rooms**: Each room is a stack frame
- **Doors**: Lock behind player (LIFO enforcement)
- **Items**: Local variables visible only in current room

#### Mechanics

**Function Call (Push Frame)**:
```c
// Player writes this code
void attack_enemy() {
    int damage = 10;  // Local variable
    // ...
}
attack_enemy();
```
**Game Result**:
- Player enters a new room labeled `attack_enemy()`
- Door locks behind them
- An item labeled `damage` appears in the room

**Return (Pop Frame)**:
```c
// Function ends
return;
```
**Game Result**:
- Player exits room
- All items in that room disappear (locals out of scope)
- Previous room's items become visible again

**Recursion**:
```c
void factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n-1);
}
```
**Game Result**:
- Player enters a series of identical `factorial()` rooms
- Each room has a different `n` value
- Player must find the "Base Case Key" to start returning

**Stack Overflow**:
```c
void infinite() {
    infinite();  // No base case
}
```
**Game Result**:
- Spire begins shaking after ~100 frames
- Warning message: "Tower Structural Integrity Critical"
- Tower collapses (game over) after stack limit reached

---

### The Code Editor (Grimoire)

**Concept**: The player's spellbook where they write C code.

#### Visual Design
- **Aesthetic**: Ancient book with glowing runes
- **Implementation**: CodeMirror editor with C syntax highlighting
- **Keybind**: Press `G` to open Grimoire

#### Mechanics

**Writing Code**:
1. Player opens Grimoire
2. Game world pauses (grey overlay)
3. Player writes C code
4. Clicks "Cast Spell" button

**Code Execution Flow**:
```
Player Submits Code
    ↓
Syntax Check (frontend linting)
    ↓ (valid)
Send to Rust Backend via IPC
    ↓
Compile C Code (gcc)
    ↓ (success)
Execute with Timeout
    ↓
Capture Output
    ↓
Parse Output
    ↓
Trigger Game Events
    ↓
Visual Feedback in Game World
```

**Spell Effects by Output**:

| Code Output | Game Effect | Visual |
|-------------|-------------|--------|
| `stdout == "42"` | Door unlocks | Door glows, opens |
| Segfault | Player takes damage | Screen glitch, red flash |
| Timeout | Spell fizzles | Grey smoke, no effect |
| Compile error | Mental strain | Player health -5 |
| Correct malloc | Land appears | Rising earth animation |

---

## RPG Systems

### Player Stats

| Stat | Meaning | C Concept Link | Status |
|------|---------|----------------|--------|
| **HP (Health)** | Player vitality | Represents code quality (errors reduce HP) | [PLANNED] |
| **MP (Memory Points)** | Available heap memory | Represents malloc budget | [PLANNED] |
| **XP (Experience)** | Programming skill | Earned by solving challenges | [WORKING] |
| **Level** | Proficiency tier | Unlocks advanced concepts | [WORKING] |

### Inventory System [PLANNED - Data structures only]

**Implementation**: Backend-authoritative

```rust
pub struct Inventory {
    slots: Vec<Option<Item>>,
    max_slots: usize,
}

pub struct Item {
    id: ItemId,
    name: String,
    item_type: ItemType,
}

pub enum ItemType {
    Key,           // Unlocks doors (if condition)
    Scroll,        // Contains code snippet
    Debugger,      // Allows step-through mode
    MemoryPotion,  // Restores MP (malloc budget)
}
```

**Metaphor**: Inventory slots = declared variables

```c
// Player declares inventory slots
int slot1;
char slot2;
Item *slot3;

// Now they can store items
slot1 = pick_up_key();
```

### Quest System

**Structure**: JSON-based quest definitions

```json
{
    "quest_id": "Q001_malloc_basics",
    "title": "Bridge the Void",
    "description": "The Water Sage needs a bridge across the marsh.",
    "objectives": [
        {
            "type": "execute_code",
            "success_condition": "stdout_contains('Bridge created')",
            "hint": "Use malloc to allocate memory for the bridge."
        }
    ],
    "rewards": {
        "xp": 100,
        "items": ["scroll_of_free"],
        "unlocks": "memory_marsh_east"
    }
}
```

### Combat System (Optional) [PLANNED]

**Concept**: Debugging as combat

- **Enemy HP** = Bug severity
- **Player Attack** = Correct code
- **Enemy Attack** = Code breaks

**Example**: Buffer Overflow Slime
- **HP**: 50
- **Weakness**: Correct bounds checking
- **Attack**: Corrupts adjacent memory tiles

```c
// Player writes code to defeat slime
char buffer[10];
for (int i = 0; i < 10; i++) {  // Correct bound
    buffer[i] = 'X';
}
// Slime defeated (no overflow)
```

---

## Gameplay Mechanics

### Core Loop

```
1. Explore World
    ↓
2. Encounter Obstacle (locked door, enemy, puzzle)
    ↓
3. Open Grimoire (Code Editor)
    ↓
4. Write C Code
    ↓
5. Execute Code
    ↓
6. Observe Visual Result
    ↓
7. Iterate if Failed / Progress if Succeeded
    ↓
(repeat)
```

### Movement System

**Implementation**:
- **WASD** or Arrow Keys: Send `move_player(direction)` command to Rust
- **Rust validates**: Collision detection, tile passability
- **Rust emits**: Updated position to Svelte
- **Svelte animates**: Smooth interpolation between positions

### Pointer Mechanics [PLANNED - Visual metaphor only]

**The Grappling Hook**:

1. **Equip Pointer**: Player gains `int *p` in inventory
2. **Target Object**: Click on distant object
3. **Attach Hook**: Visual line appears from player to object
4. **Interact Options**:
   - **Dereference (`*p`)**: Pull object to player
   - **Pointer Arithmetic (`p + n`)**: Move hook to adjacent object
   - **Assign (`*p = value`)**: Change object properties remotely

**Example Puzzle**:
```
[Player] ---- 10 tiles away ---- [Lever]
```
Player cannot walk there (lava blocking).

**Solution**:
```c
int *lever_ptr = &lever;  // Aim grappling hook
*lever_ptr = 1;           // Pull lever remotely
```

### Debugging Mode (Ghost Mode) [PLANNED]

**Activation**: Use "Debugger Scroll" item

**Effect**:
- Time freezes
- Player becomes translucent
- Instruction Pointer (yellow ghost) appears on current code line
- Player can:
  - **Step Forward**: Execute next line, ghost moves
  - **Step Backward**: Reverse execution (visual only)
  - **Inspect Variables**: Hover over objects to see values

**Educational Goal**: Visualize code execution flow

---

## Visual Design

### Art Style

**Target Aesthetic**: 16-bit pixel art with modern lighting

**Reference**: Hyper Light Drifter, Eastward, Celeste

### Color Palette

#### Memory Marsh
- **Water**: Dark teal (#0D3B52)
- **Land**: Mossy green (#4A7C59)
- **Slime**: Toxic green (#7FFF00)
- **Pointers**: Cyan glow (#00FFFF)

#### Stack Spire
- **Stone**: Dark grey (#2C2C2C)
- **Active Frame**: Orange highlight (#FF8C00)
- **Locked Doors**: Red (#DC143C)
- **Unlocked Doors**: Green (#32CD32)

### Tile Size
- **Standard**: 32x32 pixels
- **Sprites**: 32x32 or 32x64 for player/NPCs
- **UI**: Tailwind CSS components

### Animation Targets
- **Player walking**: 4-frame cycle
- **Water tiles**: Gentle wave animation
- **Land rising**: 8-frame rising from water
- **Pointer beam**: Animated shimmer

---

## Asset Pipeline

### Handcrafted Assets (Tutorial Levels)

**Tool**: Tiled Map Editor

**Workflow**:
1. Design map in Tiled (.tmx format)
2. Export as JSON
3. Rust loader parses JSON with `tiled` crate
4. Extract layers:
   - **Tile Layer**: Visual tiles
   - **Collision Layer**: Passability grid
   - **Object Layer**: Entity spawn points

```rust
use tiled::Loader;

fn load_level(path: &str) -> Result<Level> {
    let map = Loader::new().load_tmx_map(path)?;

    let tile_layer = map.get_layer(0).unwrap().as_tile_layer()?;
    let collision_layer = map.get_layer(1).unwrap().as_tile_layer()?;
    let object_layer = map.get_layer(2).unwrap().as_object_layer()?;

    Ok(Level {
        tiles: parse_tiles(tile_layer),
        collision: parse_collision(collision_layer),
        entities: spawn_entities(object_layer),
    })
}
```

### Procedural Assets (Dynamic Content)

**Tool**: Wave Function Collapse (WFC) in Rust

**Workflow**:
1. Define tile adjacency rules
2. Run WFC algorithm in Rust
3. Generate `Grid<TileType>`
4. Emit to Svelte as tile ID array
5. Svelte maps IDs to sprites and renders

**See**: [IMPLEMENTATION.md](../IMPLEMENTATION.md) for WFC code examples

### Texture Sources

**Options**:
1. **Kenney.nl** - Free public domain assets
2. **itch.io** - Commercial pixel art packs
3. **Procedural** - Generate textures via MCP tool (AI)
4. **Custom** - Commission or create original art

---

## Level Design Principles

### 1. Concept Introduction

Every level teaches ONE primary concept:
- Level 1: `printf` (output)
- Level 2: `int` variables
- Level 3: `if` statements
- Level 8: Pointers

### 2. Puzzle Structure

```
Tutorial Zone (Safe)
    ↓
Practice Puzzle (Guided)
    ↓
Challenge Puzzle (Unguided)
    ↓
Boss Puzzle (Combines Concepts)
```

### 3. Fail-Forward Design

**No Punishment for Errors**:
- Wrong code? Try again immediately
- Segfault? Lose 10 HP, respawn at checkpoint
- Infinite loop? Timeout after 2s, no penalty

**Goal**: Encourage experimentation without fear

---

## Accessibility Considerations

### Visual
- **Colorblind Mode**: Alternative palette
- **High Contrast**: Toggle for visibility
- **Font Size**: Adjustable in code editor

### Input
- **Keyboard Only**: Full navigation without mouse
- **Remappable Keys**: Customizable controls

### Difficulty
- **Hint System**: NPC provides code templates
- **Skip Puzzle**: Option to bypass after 3 failures (educational mode)

---

**Next**: See [CURRICULUM.md](../CURRICULUM.md) for the complete learning progression and level-by-level breakdown.
