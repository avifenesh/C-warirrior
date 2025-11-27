# Agent D: Level Writer

## Goal
Add L16-L25 level definitions to levels.json with validated C puzzles.

## Status: ⏳ PENDING

## Dependencies
**WAIT for Agent C** to complete map files before starting.

## Tasks
1. [ ] Add L16 entry: struct definition puzzle
2. [ ] Add L17 entry: struct members puzzle
3. [ ] Add L18 entry: struct pointers puzzle
4. [ ] Add L19 entry: nested structs puzzle
5. [ ] Add L20 entry: array of structs puzzle
6. [ ] Add L21 entry: malloc basics puzzle
7. [ ] Add L22 entry: free memory puzzle
8. [ ] Add L23 entry: memory leaks puzzle
9. [ ] Add L24 entry: dynamic arrays puzzle
10. [ ] Add L25 entry: linked lists puzzle
11. [ ] **MANDATORY**: Use `compile_and_run_c` to verify EVERY puzzle
12. [ ] Validate JSON: `python -m json.tool < levels.json`

## Files
- **Modify**: `src/assets/levels.json`
- **Reference**: Agent C's map files

## Level Definition Format
```json
{
  "id": "L16",
  "title": "The Blueprint Scroll",
  "concept": "struct definition",
  "description": "Learn to define custom data types...",
  "code_template": "#include <stdio.h>\n\nstruct Hero {\n    // TODO: Define hero attributes\n};\n\nint main() {\n    // Your code here\n    return 0;\n}",
  "success_criteria": {
    "type": "exact_match",
    "expected_stdout": "Hero: Valor, HP: 100, Level: 5\n"
  },
  "hints": [
    "Structs group related data together",
    "Use dot notation to access members"
  ],
  "xp_reward": 300,
  "map_file": "maps/L16_blueprint_scroll.json",
  "challenges": []
}
```

## C Puzzle Validation (MANDATORY)

**For EVERY puzzle, use the MCP tool:**
```
Tool: compile_and_run_c
source_code: <complete solution code>
input_data: "" (or test input if needed)
```

Example validation:
```c
#include <stdio.h>

struct Hero {
    char name[50];
    int hp;
    int level;
};

int main() {
    struct Hero h = {"Valor", 100, 5};
    printf("Hero: %s, HP: %d, Level: %d\n", h.name, h.hp, h.level);
    return 0;
}
```
Expected output: `Hero: Valor, HP: 100, Level: 5\n`

## Curriculum Concepts

### Structs (L16-L20)
- L16: Basic struct definition and initialization
- L17: Accessing struct members with dot notation
- L18: Passing structs by pointer, arrow operator
- L19: Structs containing other structs
- L20: Arrays of structs, iteration

### Memory Management (L21-L25)
- L21: malloc() to allocate memory
- L22: free() to release memory
- L23: Detecting memory leaks (valgrind concepts)
- L24: realloc() for dynamic sizing
- L25: Linked list basics with malloc/free

## Validation
```bash
python -m json.tool < src/assets/levels.json
```

## DO NOT
- Create map files (Agent C does this)
- Skip C puzzle validation
- Modify existing L01-L15 levels
