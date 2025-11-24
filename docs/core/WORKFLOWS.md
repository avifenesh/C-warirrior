# Code Warrior: Common Workflows

**Step-by-step guides for frequent tasks.**

## Workflow 1: Adding a New Game Feature

### Step 1: Check Architecture
Read `docs/architecture/system.md` to understand where the feature fits.

### Step 2: Define Data Structures (Rust)
```rust
use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NewFeature {
    pub data: String,
}
```

### Step 3: Implement Logic (Rust)
```rust
impl NewFeature {
    pub fn do_something(&mut self) -> Result<(), String> {
        // Pure logic here
        Ok(())
    }
}
```

### Step 4: Create Tauri Command
```rust
#[tauri::command]
async fn feature_action(
    state: State<'_, AppState>,
) -> Result<Response, String> {
    let mut game = state.game.lock().await;
    game.feature.do_something()
}
```

### Step 5: Register Command
Add to `main.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    feature_action,
    // ... other commands
])
```

### Step 6: Generate TypeScript Types
```bash
cargo test  # Generates bindings/*.ts
```

### Step 7: Create Svelte Component
```svelte
<script lang="ts">
import { invoke } from '@tauri-apps/api/core';

async function doAction() {
    await invoke('feature_action');
}
</script>
```

### Step 8: Test
- Rust: Unit tests for logic
- Integration: Test Tauri command
- Manual: Test in UI

---

## Workflow 2: Creating a C Programming Puzzle

### Step 1: Identify Concept
Choose from curriculum: pointers, malloc, arrays, etc.

### Step 2: Design Challenge
Write minimal code that teaches the concept:
```c
// Example: Pointer basics
void swap(int *a, int *b) {
    // Student fills this in
}
```

### Step 3: Write Solution
```c
void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}
```

### Step 4: Create Test Cases
```json
{
  "tests": [
    {"input": "5 10", "expected": "10 5"},
    {"input": "0 0", "expected": "0 0"},
    {"input": "-1 1", "expected": "1 -1"}
  ]
}
```

### Step 5: **VERIFY WITH TOOL** (MANDATORY)
```
Use compile_and_run_c MCP tool:
- Pass solution code
- Run all test cases
- Verify output matches expected
- Check for memory issues
```

### Step 6: Map to Game Mechanics
Determine reward: "Unlock grappling hook" (for pointers)

### Step 7: Add to Level
Update `src/assets/levels.json` with puzzle definition.

---

## Workflow 3: Generating a Map

### Using Python Script
```bash
cd tools
python generate_map.py --algorithm drunkard --width 40 --height 30 --output ../src/assets/maps/level_3.json
```

### Using MCP Server (Future)
```
Call generate_drunkard_map(width=40, height=30, fill=0.4)
```

### Map Algorithms:
- **Drunkard's Walk**: Organic islands/caves
- **Cellular Automata**: Natural cave systems
- **Room-based**: Dungeons with connected rooms

---

## Workflow 4: Debugging a Feature

### Step 1: Identify Layer
- **Backend issue**: Check Rust logs
- **Frontend issue**: Check browser console
- **Communication issue**: Check Tauri command/event flow

### Step 2: Read Relevant Docs
- Architecture: `docs/architecture/system.md`
- Game mechanics: `docs/game_design/mechanics.md`

### Step 3: Add Logging
```rust
// Rust
println!("DEBUG: {:?}", variable);
log::debug!("State: {:?}", state);

// Svelte
console.log('DEBUG:', variable);
```

### Step 4: Test in Isolation
- Rust: Write unit test
- Svelte: Test component independently
- Integration: Test Tauri command

### Step 5: Verify Fix
- Run tests
- Manual testing in UI
- Check for regressions

---

## Workflow 5: Implementing a Game Metaphor

### Step 1: Identify C Concept
Example: `malloc()` and `free()`

### Step 2: Choose Visual Metaphor
Example: Creating/destroying land platforms

### Step 3: Design Mechanic
```
malloc(size) in code → Solid ground appears (size tiles)
free(ptr) in code → Ground crumbles/fades
Memory leak → Unused platforms accumulate, cause "memory fog"
```

### Step 4: Implement in Rust
```rust
pub fn handle_malloc(&mut self, size: usize) -> Result<(), String> {
    // Create platforms in game world
    self.world.create_platforms(size);
    Ok(())
}
```

### Step 5: Add Visual Feedback (Svelte)
```svelte
<!-- Show platform appearing animation -->
{#if platform.newly_created}
    <div class="platform-spawn-animation">
        {platform.render()}
    </div>
{/if}
```

### Step 6: Document Mapping
Update `docs/game_design/mechanics.md` with the new mapping.

---

## Workflow 6: Adding to Curriculum

### Step 1: Check Current Progression
Read `src/assets/levels.json` to see what's already covered.

### Step 2: Identify Gap or Next Concept
Example: "Need to introduce function pointers"

### Step 3: Design Puzzle
Follow "Workflow 2: Creating a C Puzzle"

### Step 4: Determine Difficulty
- Beginner (1-5): Single concept, < 10 lines
- Intermediate (6-10): 2-3 concepts, 10-20 lines
- Advanced (11+): Multiple concepts, 20+ lines

### Step 5: Create Level Entry
```json
{
  "id": 7,
  "title": "Function Pointers",
  "description": "Learn to use function pointers...",
  "concept": "function_pointers",
  "puzzle": { ... },
  "map": "level_7.json",
  "reward": "unlock_teleport"
}
```

### Step 6: Generate Map
Use `generate_map.py` for level's map file.

### Step 7: Test Progression
Ensure level flows naturally from previous level.

---

## Workflow 7: Running Tests

### Rust Tests
```bash
# All tests
cargo test

# Specific module
cargo test game::player

# With output
cargo test -- --nocapture
```

### C Code Tests (MCP)
```
Use compile_and_run_c tool with test cases
```

### Manual UI Tests
```bash
# Development mode
cargo tauri dev

# Build and test
cargo tauri build
```

---

## Workflow 8: Committing Changes

### Step 1: Verify Constraints
Check against `docs/core/CONSTRAINTS.md`:
- [ ] Game logic in Rust
- [ ] C code verified with tool
- [ ] Mechanics map to C concepts

### Step 2: Run Tests
```bash
cargo test
```

### Step 3: Commit
```bash
git add .
git commit -m "feat: add [description]"
```

### Commit Types:
- `feat:` New feature
- `fix:` Bug fix
- `refactor:` Code restructuring
- `docs:` Documentation
- `test:` Adding tests

---

## Quick Command Reference

### Rust/Cargo
```bash
cargo build              # Build project
cargo test               # Run tests
cargo run                # Run backend only
```

### Tauri
```bash
cargo tauri dev          # Development mode with hot reload
cargo tauri build        # Production build
```

### Tools
```bash
cd tools
python c_compiler_mcp.py                    # Start C compiler MCP server
python generate_map.py --help               # Map generation help
```

### MCP (from AI agents)
```
compile_and_run_c(source_code, input_data)  # Test C code
generate_map(algorithm, width, height)       # Create map (future)
```

---

## When Things Go Wrong

### Compilation Errors
1. Read error message carefully
2. Check Rust syntax
3. Verify imports and types
4. Check `cargo.toml` dependencies

### Runtime Errors
1. Add logging/debugging
2. Check state mutations
3. Verify command flow
4. Review error handling

### C Code Fails
1. Re-run with compile_and_run_c tool
2. Check test case inputs/outputs
3. Verify C syntax (C99/C11)
4. Check for memory issues (valgrind if available)

### UI Not Updating
1. Verify event emission in Rust
2. Check event listener in Svelte
3. Confirm data serialization
4. Check for reactive statement issues ($effect)
