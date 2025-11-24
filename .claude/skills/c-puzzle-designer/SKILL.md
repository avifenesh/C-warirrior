---
name: c-puzzle-designer
description: Design and validate C programming puzzles with appropriate difficulty progression and comprehensive testing
allowed-tools:
  - mcp__c_compiler__compile_and_run_c
---

# C Puzzle Designer for Code Warrior

Expert in creating educational C programming puzzles that integrate with game mechanics.

## Puzzle Creation Workflow

### 1. Define Educational Goal
Identify the specific C concept to teach:
- **Basics**: printf, variables, scanf
- **Control Flow**: if/else, loops, switch
- **Pointers**: address-of, dereference, pointer arithmetic
- **Memory**: malloc, free, memory leaks
- **Arrays/Strings**: indexing, string manipulation
- **Functions**: parameters, return values, recursion
- **Structs**: definition, member access, typedef

### 2. Design Minimal Challenge
Create a focused problem that requires the concept:
```c
// Example: Pointer Swap
// Challenge: Swap two integers using pointers
void swap(int *a, int *b) {
    // Student fills this in
}
```

### 3. Create Comprehensive Test Cases

**Minimum 3-5 test cases:**
- Normal case (typical input)
- Edge case (boundary values: 0, -1, INT_MAX)
- Edge case (empty input, NULL, etc.)
- Complex case (stress test)

```json
{
  "tests": [
    {"input": "5 10", "expected": "10 5"},
    {"input": "0 0", "expected": "0 0"},
    {"input": "-1 1", "expected": "1 -1"}
  ]
}
```

### 4. MANDATORY: Validate with MCP Tool

**YOU MUST use the `compile_and_run_c` tool** to verify:
- Solution compiles without errors
- All test cases pass
- No memory leaks (if using malloc)
- Output matches expected format

```
Use: compile_and_run_c(source_code, input_data)
```

### 5. Map to Game Mechanics
Connect puzzle success to game world changes:
- `malloc` → Creates new land/platforms
- `free` → Removes obstacles
- `pointer dereference` → Unlocks grappling hook
- `arrays` → Activates bridge builder

### 6. Write Progressive Hints
Guide without revealing solution:
1. **Hint 1**: Concept reminder ("Remember: & gets address, * dereferences")
2. **Hint 2**: Strategy hint ("Use a temporary variable")
3. **Hint 3**: Nearly complete example (with one blank)

## Puzzle Difficulty Progression

**Beginner (Levels 1-5)**:
- Single concept per puzzle
- Minimal code (< 10 lines)
- Direct feedback
- Example: "Print your name"

**Intermediate (Levels 6-10)**:
- Combine 2-3 concepts
- 10-20 lines of code
- Requires planning
- Example: "Reverse array using pointers"

**Advanced (Levels 11+)**:
- Multiple concepts
- 20+ lines
- Optimization challenges
- Example: "Implement malloc/free simulator"

## Puzzle Template

```json
{
  "id": "ptr_swap",
  "level": 3,
  "concept": "pointers",
  "title": "Swap Two Values",
  "description": "Use pointers to swap two integers in place.",
  "starter_code": "void swap(int *a, int *b) {\n    // Your code here\n}",
  "solution": "void swap(int *a, int *b) {\n    int temp = *a;\n    *a = *b;\n    *b = temp;\n}",
  "test_cases": [
    {"input": "5 10", "expected": "10 5"},
    {"input": "0 0", "expected": "0 0"},
    {"input": "-1 1", "expected": "1 -1"}
  ],
  "hints": [
    "Use the dereference operator (*) to access values",
    "You'll need a temporary variable",
    "temp = *a; *a = *b; *b = ___;"
  ],
  "game_reward": {
    "action": "unlock_ability",
    "ability": "grappling_hook"
  }
}
```

## Validation Checklist

Before finalizing a puzzle:
- [ ] Used `compile_and_run_c` tool to verify solution
- [ ] All test cases pass
- [ ] Solution is minimal (no unnecessary complexity)
- [ ] Starter code compiles
- [ ] Hints don't give away answer
- [ ] Game reward maps to C concept
- [ ] Difficulty matches level progression

## Common Pitfalls to Avoid

❌ **Don't**: Create puzzles without testing them
✅ **Do**: Always use `compile_and_run_c` tool

❌ **Don't**: Mix multiple unrelated concepts in beginner levels
✅ **Do**: Focus on one concept at a time early on

❌ **Don't**: Use platform-specific code
✅ **Do**: Stick to standard C (C99/C11)

❌ **Don't**: Make puzzles artificially tricky
✅ **Do**: Focus on educational value
