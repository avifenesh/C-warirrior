---
name: game-metaphor-mapper
description: Map C programming concepts to game mechanics and visual metaphors for Code Warrior
---

# Game Metaphor Mapper for Code Warrior

Expert in creating meaningful connections between C programming concepts and game world mechanics.

## Core Principle: No Magic

**Every game mechanic must map to a real C concept.** The game world is a physical manifestation of memory and code execution.

## The C Memory World

### Memory as Landscape
```
Stack → Mountains (grows downward, temporary, structured)
Heap  → Ocean/Plains (dynamic, player-controlled, persistent)
BSS   → Fog/Void (uninitialized, mysterious)
Code  → Crystalline structures (immutable, precise)
```

### Fundamental Mappings

| C Concept | Game Mechanic | Visual Metaphor |
|-----------|---------------|-----------------|
| `malloc()` | Create land/platforms | Solid ground appears from void |
| `free()` | Remove obstacles | Objects dissolve/crumble |
| `NULL` | Void/abyss | Empty space, danger zone |
| Pointer | Grappling hook | Points to and reaches things |
| `&` (address-of) | Map marker | Shows location |
| `*` (dereference) | Activate/Use | Access what's pointed to |
| Array | Bridge/Path | Sequential connected tiles |
| `sizeof()` | Measuring tool | Shows true size of objects |
| Struct | Container/Building | Groups related items |
| Function | Spell/Ability | Reusable action |
| Variable | Inventory slot | Stores a value |
| Loop | Repeating pattern | Cyclic animation |
| Recursion | Portal/Mirror | Function calls itself |

## Memory Management Mechanics

### Allocation (`malloc`)
```c
int *ptr = malloc(sizeof(int) * 10);
```
**Game Effect:**
- Solid ground materializes
- Size = sizeof(int) * 10 tiles
- Player can now traverse area
- Costs "memory energy" resource

### Deallocation (`free`)
```c
free(ptr);
```
**Game Effect:**
- Ground crumbles/fades
- Can't traverse anymore
- Returns "memory energy"
- Dangling pointers → Crumbling bridges (dangerous)

### Memory Leak
```c
malloc(100); // Never freed
```
**Game Effect:**
- Unused platforms accumulate
- "Memory fog" grows
- Available memory resource decreases
- Eventually blocks progress

## Pointer Mechanics

### Pointer Declaration
```c
int *ptr;
```
**Game Item:**
- **Grappling Hook** (unattached)
- Can point to any integer location
- Must be aimed before use

### Address-Of Operator
```c
ptr = &variable;
```
**Game Action:**
- **Aim grappling hook** at target
- Creates connection (rope appears)
- Can now access target

### Dereference Operator
```c
*ptr = 42;
```
**Game Action:**
- **Use grappling hook** to modify target
- Reaches across distance
- Changes value at pointed location

### Pointer Arithmetic
```c
ptr++;
```
**Game Action:**
- **Move grappling hook** to next tile
- Traverses array/memory
- Visual: Hook slides along bridge

## Control Flow Mechanics

### If/Else
```c
if (condition) { ... } else { ... }
```
**Game Mechanic:**
- **Fork in the path**
- One path opens, other closes
- Decision point with consequences

### Loops
```c
while (condition) { ... }
```
**Game Mechanic:**
- **Repeating pattern** in world
- Cyclic animation or respawning elements
- Breaks when condition false

### Functions
```c
void spell(int power) { ... }
```
**Game Mechanic:**
- **Learnable ability**
- Can be used multiple times
- Parameters = configuration

## Visual Reward System

### Successful Compilation
- **Green glow** on code crystal
- **Sound effect**: Clear chime
- **Animation**: Code solidifies

### Runtime Error
- **Red cracks** in ground
- **Sound effect**: Warning
- **Animation**: Instability/shaking

### Correct Solution
- **New area unlocks**
- **Ability gained** (mapped to concept)
- **Visual**: Door opens, bridge extends

### Memory Leak Detected
- **Purple fog** accumulates
- **Warning indicator**
- **Visual**: Floating unused platforms

## Level Design Integration

### Tutorial (Levels 1-3)
- Focus: One metaphor at a time
- Example: "Malloc creates platforms to jump on"
- Clear visual cause-and-effect

### Intermediate (Levels 4-8)
- Combine metaphors
- Example: "Use pointer (hook) to free() distant memory"
- Requires planning

### Advanced (Levels 9+)
- Complex interactions
- Example: "Manage malloc/free to navigate dynamic maze"
- Strategy and optimization

## Metaphor Consistency Rules

1. **Always show memory as space**: malloc creates, free destroys
2. **Pointers are connections**: Visual rope/line between pointer and target
3. **Values are cargo**: What's being transported or stored
4. **Types are colors/shapes**: int=blue, char=red, struct=container
5. **Scope is distance**: Local=nearby, global=distant but accessible

## Example Puzzle-to-Mechanic Mapping

**Puzzle**: Swap two values using pointers
```c
void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}
```

**Game Visualization**:
1. Two platforms with values A and B
2. Player gets grappling hook (pointer)
3. Hook attaches to A, pulls value to temp (inventory)
4. Hook attaches to B, pulls value to A's platform
5. Hook attaches to A, places temp value on B's platform
6. Success animation: Platforms swap colors/labels

**Reward**: Unlock "Grappling Hook Mastery" ability (can use pointers freely)

## Avoiding Generic Game Tropes

❌ **Don't**: Use generic "collect coins" mechanics
✅ **Do**: Make coins represent memory bytes or stack frames

❌ **Don't**: Add combat that isn't related to C
✅ **Do**: "Combat" is debugging - finding and fixing errors

❌ **Don't**: Create puzzles that feel arbitrary
✅ **Do**: Every puzzle teaches a real C skill

The game world should feel like exploring a living, breathing computer memory landscape where C code directly shapes reality.
