# Quick Launch Guide

Copy-paste these prompts to launch each agent.

---

## Phase 1: Launch These 4 in Parallel

### Agent 1: Database Layer
```
You are Agent 1 working on the Code Warrior project.

FIRST: Read .agents/COORDINATION.md and claim locks on your files.

YOUR MISSION: Create a Diesel PostgreSQL persistence layer in src-api/src/db/

FILES TO CREATE:
- src-api/src/db/mod.rs
- src-api/src/db/schema.rs
- src-api/src/db/models.rs
- src-api/src/db/operations.rs

ALSO MODIFY: src-api/Cargo.toml (add diesel dependencies)

DO NOT TOUCH: src-api/src/main.rs, any frontend files

REQUIREMENTS:
- Use diesel-async with Neon PostgreSQL
- Create Session and PlayerProgress models
- Implement CRUD operations
- Test with: cargo check

WHEN DONE: Update COORDINATION.md, mark files as DONE.

Full instructions: .agents/prompts/AGENT_1_DATABASE.md
```

---

### Agent 2: Frontend Components
```
You are Agent 2 working on the Code Warrior project.

FIRST: Read .agents/COORDINATION.md and claim locks on your files.

YOUR MISSION: Create new Svelte 5 UI components

FILES TO CREATE:
- src-ui/src/lib/components/Settings.svelte
- src-ui/src/lib/components/ErrorBoundary.svelte
- src-ui/src/lib/components/ProgressTracker.svelte
- src-ui/src/lib/components/Achievements.svelte

DO NOT TOUCH: +page.svelte, existing components, backend files

REQUIREMENTS:
- Use Svelte 5 Runes ($state, $derived, $props)
- Match existing pixel-art aesthetic
- Make components standalone (no external deps)
- Test with: npm run check

WHEN DONE: Update COORDINATION.md, mark files as DONE.

Full instructions: .agents/prompts/AGENT_2_FRONTEND.md
```

---

### Agent 3: Maps L06-L10
```
You are Agent 3 working on the Code Warrior project.

FIRST: Read .agents/COORDINATION.md and claim locks on your files.

YOUR MISSION: Create Tiled JSON map files for Levels 6-10 (Functions phase)

FILES TO CREATE:
- src/assets/maps/L06_spell_scroll.json (void functions)
- src/assets/maps/L07_damage_calc.json (return values)
- src/assets/maps/L08_stack_spire.json (stack frames - TALL map)
- src/assets/maps/L09_global_artifact.json (scope - WIDE map)
- src/assets/maps/L10_recursive_mirror.json (recursion - symmetric)

DO NOT TOUCH: levels.json, existing L01-L05 maps, any code files

REQUIREMENTS:
- Follow structure of existing maps (read L01_first_spell.json first)
- Include: player_spawn, terminal, door, npc objects
- Use tile IDs 0-5 only
- Validate JSON syntax

WHEN DONE: Update COORDINATION.md, mark files as DONE.

Full instructions: .agents/prompts/AGENT_3_MAPS_6_10.md
```

---

### Agent 4: Maps L11-L15
```
You are Agent 4 working on the Code Warrior project.

FIRST: Read .agents/COORDINATION.md and claim locks on your files.

YOUR MISSION: Create Tiled JSON map files for Levels 11-15 (Pointers phase)

FILES TO CREATE:
- src/assets/maps/L11_address_spell.json (& operator - grid layout)
- src/assets/maps/L12_grappling_hook.json (pointers - TALL vertical)
- src/assets/maps/L13_dereference_pull.json (* operator - barriers)
- src/assets/maps/L14_array_navigator.json (ptr arithmetic - LONG corridor)
- src/assets/maps/L15_null_trap.json (NULL - trap maze)

DO NOT TOUCH: levels.json, L01-L10 maps, any code files

REQUIREMENTS:
- More complex layouts than earlier levels
- Visual metaphors for pointer concepts
- Include danger zones for L15
- Validate JSON syntax

WHEN DONE: Update COORDINATION.md, mark files as DONE.

Full instructions: .agents/prompts/AGENT_4_MAPS_11_15.md
```

---

## Phase 2: Launch After Maps Complete

### Agent 5: Level Content
```
You are Agent 5 working on the Code Warrior project.

FIRST: Read .agents/COORDINATION.md
CHECK: Agents 3 & 4 must have marked their maps as DONE
CLAIM: Lock on src/assets/levels.json

YOUR MISSION: Add L06-L15 level definitions with C puzzles

FILE TO MODIFY: src/assets/levels.json (append to existing L01-L05)

DO NOT TOUCH: Map files, frontend, backend code

CRITICAL REQUIREMENTS:
- Use compile_and_run_c MCP tool to VERIFY every puzzle
- DO NOT submit any puzzle without testing it first
- Match exact expected output including newlines
- Reference correct map files from Agents 3 & 4

PUZZLE CONCEPTS:
L06: void functions, L07: return values, L08: stack frames
L09: scope, L10: recursion, L11: & operator, L12: pointers
L13: * operator, L14: pointer arithmetic, L15: NULL

WHEN DONE: Update COORDINATION.md, mark levels.json as DONE.

Full instructions: .agents/prompts/AGENT_5_LEVELS.md
```

---

## Phase 3: Launch After ALL Complete

### Agent 6: Integration
```
You are Agent 6 working on the Code Warrior project.

FIRST: Read .agents/COORDINATION.md
VERIFY: ALL agents (1-5) must have marked their work as DONE
DO NOT START until everything is ready

YOUR MISSION: Wire all components together and test

FILES TO MODIFY:
- src-api/src/main.rs (import db module)
- src-ui/src/routes/+page.svelte (integrate new components)
- src-ui/src/lib/components/MainMenu.svelte (add Settings)
- src-ui/src/lib/components/GameHUD.svelte (add ProgressTracker)

TASKS:
1. Import db module into API
2. Wire Settings, ErrorBoundary, ProgressTracker into UI
3. Run cargo build && npm run build
4. Use playwright-skill to test full game flow
5. Verify all 15 levels work

WHEN DONE: Update COORDINATION.md, leave detailed test report.

Full instructions: .agents/prompts/AGENT_6_INTEGRATION.md
```

---

## Parallel Execution Commands

### Terminal Method (4 terminals)
```bash
# Terminal 1
claude "$(cat .agents/prompts/AGENT_1_DATABASE.md)"

# Terminal 2
claude "$(cat .agents/prompts/AGENT_2_FRONTEND.md)"

# Terminal 3
claude "$(cat .agents/prompts/AGENT_3_MAPS_6_10.md)"

# Terminal 4
claude "$(cat .agents/prompts/AGENT_4_MAPS_11_15.md)"
```

### Factory Method
Launch 4 Factory instances, paste each prompt above.

### Monitor Progress
```bash
# Watch coordination file
watch -n 5 cat .agents/COORDINATION.md
```
