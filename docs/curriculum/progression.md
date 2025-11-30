# Code Warrior: Educational Curriculum

## For AI agents and developers
- Use this to design or modify levels while preserving the intended progression.
- When adding levels, follow the level schema here and keep concepts in the appropriate phase.
- Cross‑check metaphors with `docs/GAME_DESIGN.md` to avoid mismatches between curriculum and gameplay.

## Table of Contents
1. [Curriculum Philosophy](#curriculum-philosophy)
2. [Learning Objectives](#learning-objectives)
3. [Level Structure](#level-structure)
4. [Phase 1: Foundations](#phase-1-foundations)
5. [Phase 2: Functions and Scope](#phase-2-functions-and-scope)
6. [Phase 3: Pointers and Memory](#phase-3-pointers-and-memory)
7. [Phase 4: Advanced Concepts](#phase-4-advanced-concepts)
8. [Assessment and Progress](#assessment-and-progress)

---

## Curriculum Philosophy

### The 80/20 Learning Model

Code Warrior enforces an active learning approach:
- **80% Time Coding**: Player writes real C code to progress
- **20% Time Reading**: Tutorials, NPC dialogue, documentation

**Implementation Rule**: Every level must require C code execution to complete. No level can be "talked through" or solved with pure navigation.

### Learn → Code → Execute → Reflect

Each concept follows this four-step cycle:

```
1. LEARN (5 minutes)
   ↓ Short tutorial or NPC dialogue introduces concept

2. CODE (15 minutes)
   ↓ Player writes C code to solve puzzle

3. EXECUTE (Instant)
   ↓ Code runs, game world reacts visually

4. REFLECT (5 minutes)
   ↓ Result shown, explanation provided, challenge adjusted

(Repeat with increasing difficulty)
```

### Progressive Disclosure

**Principle**: Introduce complexity gradually, never overwhelming the player.

**Structure**:
1. **Introduce**: New concept in isolation
2. **Practice**: Simple exercises with hints
3. **Combine**: Mix with previous concepts
4. **Master**: Complex challenge requiring creativity

---

## Learning Objectives

### Overall Goals

By completing Code Warrior, players will be able to:

1. **Write syntactically correct C programs** from scratch
2. **Understand memory management** (stack vs heap, allocation, deallocation)
3. **Use pointers effectively** (addressing, dereferencing, arithmetic)
4. **Debug common errors** (segfaults, memory leaks, buffer overflows)
5. **Read and understand C code** written by others
6. **Apply C concepts** to solve real programming challenges

### Skill Levels

| Level Range | Proficiency | Real-World Equivalent |
|-------------|-------------|----------------------|
| 1-5 | Beginner | Can write "Hello World", understand variables |
| 6-10 | Novice | Can write functions, understand control flow |
| 11-15 | Intermediate | Can use pointers, manage stack memory |
| 16-20 | Advanced | Can use malloc/free, understand heap |
| 21-25 | Expert | Can debug complex memory issues, write safe code |

---

## Level Structure

### Multi-Quest Level Schema

The current level structure uses a **multi-quest** system where each level contains multiple terminals, each linked to a specific quest. Players complete all quests to finish a level.

```json
{
    "id": "L01",
    "title": "The First Spell",
    "theme": "L01_village",
    "concept": "return values",
    "description": "Master the art of returning values to unlock the door ahead.",
    "code_template": "#include <stdio.h>\n\n// Write your function here\n\nint main() { return 0; }",
    "hints": [],
    "xp_reward": 0,
    "total_xp_reward": 90,
    "map_file": "maps/L01_first_spell.json",
    "world_config": {
        "width": 20,
        "height": 15,
        "spawn_x": 64,
        "spawn_y": 224,
        "terminals": [
            {"x": 192, "y": 224, "quest_id": "L01_Q1"},
            {"x": 320, "y": 224, "quest_id": "L01_Q2"},
            {"x": 448, "y": 224, "quest_id": "L01_Q3"}
        ],
        "preset": "tutorial"
    },
    "quests": [
        {
            "id": "L01_Q1",
            "order": 1,
            "title": "The Secret Number",
            "description": "Return the secret number 42.",
            "recommended": true,
            "function_signature": {
                "name": "getSecret",
                "return_type": "int",
                "parameters": []
            },
            "user_template": "int getSecret() {\n    // Return the secret number: 42\n    \n}",
            "test_cases": [
                {"input": [], "expected": "42", "sample": true}
            ],
            "hints": [
                "Use the 'return' keyword to send a value back",
                "Example: return 42;"
            ],
            "xp_reward": 25
        }
    ]
}
```

### Quest Structure

Each quest is a function-based challenge:
- `function_signature`: Defines the function name, return type, and parameters
- `user_template`: Starting code shown in the terminal
- `test_cases`: Inputs and expected outputs for validation
- `recommended`: Suggests which quest to try first

### Success Criteria Types

| Type | Description | Example |
|------|-------------|---------|
| `exact_output` | Stdout must match exactly | `"Hello World\n"` |
| `contains` | Stdout must contain string | `contains("42")` |
| `regex` | Stdout matches pattern | `r"^Answer: \d+$"` |
| `ast_check` | Code must contain AST nodes | `has_function_call("malloc")` |
| `no_errors` | Must compile and run without crash | Segfault test |

---

## Phase 1: Foundations

**Goal**: Understand basic C syntax, return values, and functions.

**Estimated Time**: 2-3 hours

### Level 1: The First Spell (Return Values)

**Concept**: Functions that return values

**Structure**: 3 quests teaching return statements

**Quests**:
1. **The Secret Number** - Return the literal value 42
2. **Double Trouble** - Return a computed value (21 * 2)
3. **The Sum Spell** - Return a sum (10 + 20 + 12)

**Learning Outcome**: Understand the `return` keyword, function return types.

---

### Level 2: The Empty Backpack (Variables & Parameters)

**Concept**: Function parameters and arithmetic

**Structure**: 3 quests with increasing complexity

**Quests**:
1. **Adding Weights** - Sum two parameters
2. **Calculate Area** - Multiply parameters
3. **Triple Sum** - Sum three parameters

**Learning Outcome**: Understand parameters, arithmetic operators.

---

### Level 3: The Gatekeeper (if/else)

**Concept**: Conditional logic

**Structure**: 3 quests teaching comparisons and branching

**Code Challenge**:
```c
#include <stdio.h>

int main() {
    int player_level = 6;  // Player provides this

    if (player_level >= 5) {
        printf("Access granted\n");
    } else {
        printf("Access denied\n");
    }

    return 0;
}
```

**Game Effect**: Guard steps aside if output is "Access granted".

**Learning Outcome**: Understand boolean expressions, comparison operators, if/else branching.

---

### Level 4: The Repeating Strike (Loops)

**Concept**: `for` loops

**Puzzle**: Enemy requires 5 hits to defeat

**Code Challenge**:
```c
#include <stdio.h>

int main() {
    for (int i = 0; i < 5; i++) {
        printf("Strike!\n");
    }
    return 0;
}
```

**Game Effect**: Enemy HP decreases with each "Strike!" output. Defeated after 5.

**Learning Outcome**: Understand loop syntax, iteration, counters.

---

### Level 5: The Array Corridor (Arrays)

**Concept**: Array declaration and access

**Puzzle**: Player must unlock 3 doors in sequence using keys stored in array

**Code Challenge**:
```c
#include <stdio.h>

int main() {
    int keys[3] = {101, 203, 305};

    for (int i = 0; i < 3; i++) {
        printf("Unlocking door with key: %d\n", keys[i]);
    }

    return 0;
}
```

**Game Effect**: Each door unlocks as its key is printed.

**Learning Outcome**: Understand array indexing, zero-based indexing, array initialization.

---

## Phase 2: Functions and Scope

**Goal**: Understand functions, stack frames, and scope.

**Estimated Time**: 3-4 hours

### Level 6: The Spell Scroll (Void Functions)

**Concept**: Void functions (no return value)

**Puzzle**: Refactor repeated attack code into reusable function

**Code Challenge**:
```c
#include <stdio.h>

void attack() {
    printf("Swing sword!\n");
}

int main() {
    attack();
    attack();
    attack();
    return 0;
}
```

**Game Effect**: Player learns "Sword Dance" ability, can trigger 3 attacks with one cast.

**Learning Outcome**: Understand function definition, function calls, DRY principle.

---

### Level 7: The Damage Calculator (Return Values)

**Concept**: Functions that return values

**Puzzle**: Calculate damage based on player strength and enemy armor

**Code Challenge**:
```c
#include <stdio.h>

int calculate_damage(int strength, int armor) {
    int damage = strength - armor;
    if (damage < 0) damage = 0;
    return damage;
}

int main() {
    int strength = 20;
    int armor = 5;
    int damage = calculate_damage(strength, armor);
    printf("Damage: %d\n", damage);
    return 0;
}
```

**Game Effect**: Enemy takes calculated damage.

**Learning Outcome**: Understand return types, parameters, return statements.

---

### Level 8: The Stack Spire - Room 1 (Stack Frames)

**Concept**: Stack frames and local scope

**Puzzle**: Player enters Stack Spire. Local variables only exist within current room.

**Visualization**: Player enters room labeled `function_a()`. Item labeled `local_x` appears. When player exits room (return), item disappears.

**Code Challenge**:
```c
#include <stdio.h>

void function_a() {
    int local_x = 10;
    printf("Inside function_a: %d\n", local_x);
}

int main() {
    function_a();
    // local_x doesn't exist here
    printf("Back in main\n");
    return 0;
}
```

**Game Effect**: Item `local_x` appears in function room, disappears when returning to main.

**Learning Outcome**: Understand stack frames, local scope, variable lifetime.

---

### Level 9: The Global Artifact (Global Variables)

**Concept**: Global vs local scope

**Puzzle**: Player needs an item accessible from all rooms

**Code Challenge**:
```c
#include <stdio.h>

int global_health = 100;  // Accessible everywhere

void take_damage() {
    global_health -= 10;
    printf("Health: %d\n", global_health);
}

int main() {
    take_damage();
    take_damage();
    printf("Final health: %d\n", global_health);
    return 0;
}
```

**Game Effect**: Health bar appears at top of screen, persists across all rooms.

**Learning Outcome**: Understand global scope, when to use globals (sparingly).

---

### Level 10: The Recursive Mirror (Recursion)

**Concept**: Recursive function calls

**Puzzle**: Player enters a room with mirrors. Each mirror leads to an identical room. Must find the "Base Case Key" to escape.

**Code Challenge**:
```c
#include <stdio.h>

int countdown(int n) {
    if (n <= 0) {
        printf("Blastoff!\n");
        return 0;
    }
    printf("%d\n", n);
    return countdown(n - 1);
}

int main() {
    countdown(5);
    return 0;
}
```

**Game Effect**: Player enters 5 identical rooms, finds key in last room, returns through all rooms.

**Learning Outcome**: Understand recursion, base cases, stack depth.

---

## Phase 3: Pointers and Memory

**Goal**: Master pointers, addresses, and indirection.

**Estimated Time**: 4-5 hours

### Level 11: The Address Spell (Address-of Operator)

**Concept**: Getting memory addresses with `&`

**Puzzle**: Player needs to know the "location" of an object, not the object itself

**Code Challenge**:
```c
#include <stdio.h>

int main() {
    int treasure = 100;
    printf("Treasure value: %d\n", treasure);
    printf("Treasure location: %p\n", (void*)&treasure);
    return 0;
}
```

**Game Effect**: Casting "Address Spell" on treasure chest shows glowing coordinates above it.

**Learning Outcome**: Understand memory addresses, address-of operator, `%p` format.

---

### Level 12: The Grappling Hook (Pointers)

**Concept**: Pointer declaration and initialization

**Puzzle**: Locked chest is across a chasm. Player cannot walk there.

**Code Challenge**:
```c
#include <stdio.h>

int main() {
    int chest_gold = 50;
    int *hook = &chest_gold;  // Attach hook to chest

    printf("Gold at hook location: %d\n", *hook);
    return 0;
}
```

**Game Effect**: Visual line (grappling hook) connects player to chest. Player can now access chest contents remotely.

**Learning Outcome**: Understand pointer declaration, initialization with address.

---

### Level 13: The Dereference Pull (Dereference Operator)

**Concept**: Accessing value at pointer with `*`

**Puzzle**: Pull a lever remotely using grappling hook

**Code Challenge**:
```c
#include <stdio.h>

int main() {
    int lever_state = 0;  // 0 = off, 1 = on
    int *hook = &lever_state;

    *hook = 1;  // Pull lever remotely

    printf("Lever is now: %d\n", lever_state);
    return 0;
}
```

**Game Effect**: Lever flips, door opens.

**Learning Outcome**: Understand dereference operator, modifying values through pointers.

---

### Level 14: The Array Navigator (Pointer Arithmetic)

**Concept**: Pointer arithmetic for array traversal

**Puzzle**: Navigate array of traps by "jumping" memory addresses

**Code Challenge**:
```c
#include <stdio.h>

int main() {
    int tiles[5] = {0, 1, 0, 0, 1};  // 1 = safe, 0 = trap
    int *ptr = tiles;  // Start at beginning

    ptr += 2;  // Jump to index 2
    printf("Tile value: %d\n", *ptr);

    return 0;
}
```

**Game Effect**: Player teleports to index 2, skipping traps at indices 0 and 1.

**Learning Outcome**: Understand pointer arithmetic, arrays as pointers.

---

### Level 15: The Null Trap (Null Pointers)

**Concept**: Null pointers and segmentation faults

**Puzzle**: Broken grappling hook (null pointer) causes player to fall

**Code Challenge**:
```c
#include <stdio.h>

int main() {
    int *broken_hook = NULL;

    if (broken_hook == NULL) {
        printf("Hook is broken! Cannot use.\n");
    } else {
        printf("Value: %d\n", *broken_hook);  // This would segfault
    }

    return 0;
}
```

**Game Effect**: If player dereferences NULL, they fall and take damage. Must check first.

**Learning Outcome**: Understand null pointers, defensive programming, null checks.

---

## Phase 4: Advanced Concepts

**Goal**: Master dynamic memory, structs, and advanced patterns.

**Estimated Time**: 5-6 hours

### Level 16: The Memory Marsh Arrives (malloc)

**Concept**: Dynamic memory allocation with `malloc`

**Puzzle**: Player enters Memory Marsh (all water). Must create land to cross.

**Code Challenge**:
```c
#include <stdio.h>
#include <stdlib.h>

int main() {
    int *land = malloc(sizeof(int) * 5);

    if (land == NULL) {
        printf("Failed to create land\n");
        return 1;
    }

    printf("Land created: 5 tiles\n");

    free(land);
    return 0;
}
```

**Game Effect**: 5 connected land tiles rise from water, creating bridge. Player can walk across.

**Learning Outcome**: Understand malloc, sizeof, heap allocation, null checks.

---

### Level 17: The Banishment (free)

**Concept**: Deallocating memory with `free`

**Puzzle**: Too much land in Marsh causes flooding. Must remove unused land.

**Code Challenge**:
```c
#include <stdio.h>
#include <stdlib.h>

int main() {
    int *temp_bridge = malloc(sizeof(int) * 3);
    printf("Bridge created\n");

    // Cross bridge...

    free(temp_bridge);
    printf("Bridge removed\n");

    return 0;
}
```

**Game Effect**: Land tiles sink back into water after `free`.

**Learning Outcome**: Understand free, resource cleanup, matching malloc/free.

---

### Level 18: The Slime Spawn (Memory Leaks)

**Concept**: Memory leaks

**Puzzle**: Forgotten malloc creates Slime Monsters

**Code Challenge**:
```c
#include <stdio.h>
#include <stdlib.h>

int main() {
    int *leak = malloc(sizeof(int) * 10);
    printf("Allocated memory\n");

    // Player forgets to call free(leak)
    // Player leaves area

    return 0;  // Memory leaked!
}
```

**Game Effect**: Slime Monster spawns on orphaned memory tiles. Drains player health.

**Solution**:
```c
free(leak);  // Slime disappears
```

**Learning Outcome**: Understand memory leaks, valgrind concept, importance of cleanup.

---

### Level 19: The Inventory Struct (Structs)

**Concept**: Struct definition and usage

**Puzzle**: Player needs organized storage for multiple item properties

**Code Challenge**:
```c
#include <stdio.h>

struct Item {
    char name[20];
    int weight;
    int value;
};

int main() {
    struct Item sword = {"Iron Sword", 10, 50};

    printf("Item: %s\n", sword.name);
    printf("Weight: %d\n", sword.weight);
    printf("Value: %d\n", sword.value);

    return 0;
}
```

**Game Effect**: Inventory item now shows detailed properties (name, weight, value).

**Learning Outcome**: Understand structs, member access, composite data types.

---

### Level 20: The Buffer Overflow Boss (Buffer Overflow)

**Concept**: Buffer overflows and bounds checking

**Puzzle**: Final boss - Buffer Overflow Dragon

**Code Challenge** (Vulnerable):
```c
#include <stdio.h>

int main() {
    char buffer[10];

    for (int i = 0; i < 15; i++) {  // OVERFLOW!
        buffer[i] = 'X';
    }

    return 0;
}
```

**Game Effect**: Dragon breathes fire, player takes massive damage, corruption visual effects.

**Code Challenge** (Correct):
```c
#include <stdio.h>

int main() {
    char buffer[10];

    for (int i = 0; i < 10; i++) {  // Correct bounds
        buffer[i] = 'X';
    }

    return 0;
}
```

**Game Effect**: Dragon defeated, player wins!

**Learning Outcome**: Understand buffer overflows, bounds checking, security implications.

---

## Assessment and Progress

### XP System

| Activity | XP Reward |
|----------|-----------|
| Complete level first try | 100 XP |
| Complete level with hints | 75 XP |
| Complete level after failure | 50 XP |
| Find secret optimization | +25 XP bonus |

### Progress Tracking

**Database Schema**:
```sql
CREATE TABLE player_progress (
    player_id INTEGER PRIMARY KEY,
    current_level TEXT,
    completed_levels TEXT,  -- JSON array
    total_xp INTEGER,
    hints_used INTEGER,
    code_submissions INTEGER,
    concepts_mastered TEXT  -- JSON array
);
```

### Mastery Criteria

Concept is "mastered" when:
1. Player completes all related levels
2. Player solves at least one challenge without hints
3. Player can explain concept to "Mentor NPC" (multiple choice quiz)

### Certification

**Final Challenge**: Build a complete C program that:
- Manages dynamic memory (malloc/free)
- Uses pointers correctly
- Implements proper error handling
- Has no memory leaks (validated by checker)

**Reward**: "C Master" badge, unlock sandbox mode with free coding

---

## Adaptive Difficulty

### Hint System

After 3 failed attempts:
```json
{
    "hint_level": 1,
    "message": "Remember to check if your pointer is NULL before dereferencing"
}
```

After 5 failed attempts:
```json
{
    "hint_level": 2,
    "code_template_highlight": "if (ptr == NULL) { ... }"
}
```

After 7 failed attempts:
```json
{
    "hint_level": 3,
    "show_solution": true,
    "explanation": "Here's the correct approach..."
}
```

### Skip Option

After 10 failed attempts, offer:
- **Skip Level**: Move forward (mark as incomplete)
- **Watch Walkthrough**: Animated solution with explanation
- **Request Mentor**: In-game NPC provides live guidance

---

## Future Expansion Concepts

### Advanced Topics (Phase 5+)

- **File I/O**: Read/write save game files
- **Linked Lists**: Dynamic dungeon navigation
- **Function Pointers**: Spell callback system
- **Multi-file Programs**: Modular spell libraries
- **Debugging Tools**: GDB integration tutorial

### Multiplayer Concepts

- **Code Review Mode**: Players review each other's solutions
- **Co-op Puzzles**: Two players write interdependent code
- **Leaderboards**: Fastest completion, most efficient code

---

**Next**: See [IMPLEMENTATION.md](../IMPLEMENTATION.md) for technical implementation of curriculum levels.
