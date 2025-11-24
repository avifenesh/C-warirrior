# Code Warrior Glossary

## Overview

This glossary defines terms, concepts, and metaphors used throughout Code Warrior documentation and development.

---

## Technical Terms

### Architecture

**Backend-Authoritative**
- Pattern where Rust backend is the single source of truth
- Svelte frontend only visualizes state, never creates or modifies it
- Prevents security issues and state desynchronization

**IPC (Inter-Process Communication)**
- Communication bridge between Rust backend and JavaScript frontend
- Uses Tauri's `invoke` (commands) and `emit` (events) patterns

**Game Loop**
- Continuous thread running game logic at fixed tick rate (20 TPS)
- Updates physics, AI, collisions independent of rendering

**Render Loop**
- Frontend animation loop running at 60 FPS
- Interpolates between game states for smooth visuals

**ECS (Entity Component System)**
- Design pattern separating data (components) from logic (systems)
- Entities are IDs, components are data structs, systems are functions

**WFC (Wave Function Collapse)**
- Procedural generation algorithm using constraint satisfaction
- Generates maps by enforcing adjacency rules between tiles

---

### Programming Concepts

**C Concept**
- Low-level programming concept taught in the game
- Examples: pointers, malloc, stack frames, segfaults

**Metaphor Mapping**
- Translation of C concept to RPG game mechanic
- Example: Pointer → Grappling Hook

**Stack Frame**
- Memory region created when function is called
- Contains local variables and return address
- **Game Metaphor**: Room in Stack Spire

**Heap**
- Dynamically allocated memory managed by malloc/free
- **Game Metaphor**: Memory Marsh

**Pointer**
- Variable storing memory address of another variable
- **Game Metaphor**: Grappling Hook connecting player to object

**Dereference**
- Accessing value at pointer's address using `*` operator
- **Game Metaphor**: Pulling grappling hook to interact with object

**Memory Leak**
- Allocated memory that is never freed
- **Game Metaphor**: Slime Monster spawning on orphaned memory

**Segmentation Fault (Segfault)**
- Accessing invalid memory address
- **Game Metaphor**: The Void, causes screen glitch and damage

**Buffer Overflow**
- Writing beyond allocated memory bounds
- **Game Metaphor**: Water flooding from container, damaging adjacent tiles

---

### Game Systems

**TPS (Ticks Per Second)**
- Number of game logic updates per second
- Code Warrior uses 20 TPS

**FPS (Frames Per Second)**
- Number of visual frames rendered per second
- Target: 60 FPS

**Interpolation**
- Smoothing technique to display movement between game ticks
- Creates 60 FPS visuals from 20 TPS logic

**Chunk**
- Subsection of game world loaded and rendered at once
- Optimizes performance by culling off-screen areas

**Viewport**
- Visible portion of game world on screen
- Code Warrior uses 20x15 tile viewport

---

## Game World

### Locations

**Memory Marsh**
- Main game world representing the heap
- Dynamic, fragmented terrain of land and water
- Land = allocated memory, Water = unallocated/null

**Stack Spire**
- Vertical tower representing the call stack
- Each room is a stack frame
- LIFO structure: enter from bottom, exit from top

**The Void**
- Dangerous area representing invalid memory
- Accessed by dereferencing null or out-of-bounds pointers
- Causes damage and glitch effects

**Tutorial Chamber**
- Safe starting area for Level 1-3
- Introduces basic mechanics without danger

---

### Entities

**Player Character**
- Player-controlled avatar representing the programmer
- Navigates memory space and writes code

**NPC (Non-Player Character)**
- Quest givers, tutors, merchants in game world
- Provide hints, lore, and challenges

**Slime Monster**
- Enemy spawned by memory leaks
- Slowly drains player health
- Despawns when memory is freed

**Memory Fragment**
- Collectible item representing allocated memory block
- Used in crafting or puzzles

---

### Items

**Grimoire**
- Player's code editor (spellbook)
- Where C code is written
- Implemented using CodeMirror

**Grappling Hook**
- Pointer tool used to interact with distant objects
- Visual line connecting player to target

**Debugger Scroll**
- Item enabling Ghost Mode (step-through debugging)
- Allows player to see instruction pointer

**Memory Potion**
- Restores Memory Points (MP / malloc budget)
- Allows more heap allocations

**Key**
- Unlocks doors when correct C code is executed
- Represents conditional logic success

---

## Game Mechanics

### Core Actions

**Cast (Spell)**
- Execute C code written in Grimoire
- Triggers game world changes based on output

**Allocate (malloc)**
- Create land in Memory Marsh
- Requires Memory Points

**Deallocate (free)**
- Remove land in Memory Marsh
- Restores Memory Points

**Traverse**
- Move through game world (WASD keys)
- Limited by collision/terrain

**Aim (Pointer)**
- Target object with grappling hook
- Sets pointer to object's address

**Dereference**
- Interact with object via pointer
- Pull grappling hook to bring object close

---

### Stats

**HP (Health Points)**
- Player vitality
- Reduced by errors, segfaults, enemy attacks
- Game over at 0

**MP (Memory Points)**
- Available heap memory budget
- Consumed by malloc, restored by free
- Limited to prevent excessive allocation

**XP (Experience Points)**
- Programming skill progress
- Earned by completing levels and challenges
- Unlocks new abilities and concepts

**Level**
- Player proficiency tier
- Unlocks access to advanced game areas

---

### Curriculum Terms

**Phase**
- Major section of learning progression
- Phase 1: Foundations, Phase 2: Functions, Phase 3: Pointers, etc.

**Level (Educational)**
- Individual lesson/puzzle teaching specific concept
- Contains tutorial, code challenge, and rewards

**Concept**
- Specific C programming topic being taught
- Examples: "int variables", "pointer arithmetic", "malloc/free"

**Challenge**
- Code problem player must solve to progress
- Defined by success criteria (stdout, ast_check, etc.)

**Success Criteria**
- Validation rules for code solution
- Types: exact_output, contains, regex, ast_check, no_errors

**Hint**
- Optional clue provided after multiple failures
- Progressive disclosure: increasingly specific

**80/20 Rule**
- Design principle: 80% coding time, 20% reading time
- Ensures active learning dominates

---

## Development Terms

### Tools

**MCP (Model Context Protocol)**
- Standard for AI agents to access external tools
- Code Warrior uses C compiler and filesystem MCPs

**ts-rs**
- Rust library that generates TypeScript types from Rust structs
- Ensures type safety across IPC boundary

**Diesel**
- Rust ORM (Object-Relational Mapper)
- Provides type-safe database operations

**Tokio**
- Rust async runtime
- Powers game loop thread and async operations

**Svelte Runes**
- Svelte 5 reactive primitives
- $state, $derived, $effect

**Tiled**
- Map editor for 2D tile-based games
- Exports JSON/TMX formats

---

### Patterns

**Command-Event Pattern**
- IPC protocol: Commands (FE→BE), Events (BE→FE)
- Prevents polling, enables reactive updates

**Progressive Disclosure**
- Design principle: introduce complexity gradually
- Start simple, add layers over time

**Test-First Design**
- Write C test code before implementing game mechanics
- Ensures educational accuracy

**Fail-Forward Design**
- Errors are learning opportunities, not punishments
- Allow experimentation without harsh penalties

---

## Educational Concepts

**Active Learning**
- Learning by doing (writing code) vs passive (reading)
- More effective for skill acquisition

**Spatial Metaphor**
- Representing abstract concepts as physical spaces
- Makes memory management tangible

**Visual Debugging**
- Seeing code execution as game world changes
- Links abstract code to concrete results

**Muscle Memory**
- Physical memory from repeated coding practice
- Goal: make C syntax feel natural

**Flow State**
- Mental state of engaged focus
- Achieved by balancing challenge and skill

---

## Abbreviations

- **BE**: Backend (Rust)
- **FE**: Frontend (Svelte)
- **IPC**: Inter-Process Communication
- **ECS**: Entity Component System
- **WFC**: Wave Function Collapse
- **TPS**: Ticks Per Second
- **FPS**: Frames Per Second
- **HP**: Health Points
- **MP**: Memory Points
- **XP**: Experience Points
- **NPC**: Non-Player Character
- **AST**: Abstract Syntax Tree
- **MCP**: Model Context Protocol
- **ORM**: Object-Relational Mapper
- **LIFO**: Last In, First Out (stack behavior)
- **GCC**: GNU Compiler Collection
- **DB**: Database

---

## Metaphor Quick Reference

| C Concept | Game Metaphor | Key Visual |
|-----------|---------------|------------|
| Pointer | Grappling Hook | Cyan line connecting player to object |
| malloc() | Summon Land | Land rising from water |
| free() | Banish Land | Land sinking into water |
| Stack Frame | Tower Room | Room in vertical spire |
| Local Variable | Room Item | Object visible only in current room |
| Segfault | The Void | Screen glitch, damage |
| Memory Leak | Slime Monster | Green enemy on orphaned memory |
| Buffer Overflow | Flood | Water overflowing container |
| Null Pointer | Broken Hook | Hook snaps when used |
| Array | Corridor | Linear sequence of connected tiles |
| Struct | Backpack | Organized container |
| Function Call | Climb Tower | Enter new room (push frame) |
| Return | Descend Tower | Exit room (pop frame) |
| Recursion | Mirror Room | Identical nested rooms |
| Stack Overflow | Tower Collapse | Too many nested rooms |

---

## Usage Notes

### For Developers

Use this glossary when:
- Writing documentation
- Discussing features with team
- Explaining architecture to AI agents
- Onboarding new contributors

### For AI Agents

Reference this glossary to:
- Understand project-specific terminology
- Maintain consistency in code and comments
- Use correct metaphor mappings
- Follow established conventions

### For Players

Players encounter these terms through:
- In-game tutorial dialogue
- Quest descriptions
- Item/ability names
- NPC conversations

---

## Contributing

When adding new terms:
1. Place in appropriate category
2. Provide clear, concise definition
3. Include game metaphor if applicable
4. Add examples where helpful
5. Link to relevant documentation

---

**Last Updated**: 2024-01-01
**Maintained By**: Core Development Team
