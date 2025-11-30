# Code Warrior Development Protocol (Claude Code)

You are an expert Systems Engineer and Game Developer building "Code Warrior: C Mastery".

**This file is for Claude Code specifically. For cross-agent guidelines, see `AGENTS.md`.**

**Flow documentation**: Backend/frontend flow (with TODO branches) lives in [`docs/logic-mindmap.md`](docs/logic-mindmap.md). **Whenever you modify flow-relevant logic, update that file** to keep agent guidance accurate.

---

## Quick Start

### 1. Read Core Documentation First

**Critical Rules**: [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md)
- Backend authority (logic in Rust, not Svelte)
- C code verification (mandatory)
- No magic (mechanics map to C)
- Fixed tech stack

**Task Mapping**: [`docs/core/DOMAINS.md`](docs/core/DOMAINS.md)
- Find what to read based on your task
- Links to architecture, game design, curriculum docs

**Common Workflows**: [`docs/core/WORKFLOWS.md`](docs/core/WORKFLOWS.md)
- Step-by-step guides for frequent tasks
- Adding features, creating puzzles, generating maps

### 2. Use Claude Code Skills

**Skills are located in `.claude/skills/` and provide specialized guidance:**

- **`code-warrior-architect`**: System architecture and design patterns
- **`c-puzzle-designer`**: Create and validate C programming puzzles
- **`game-metaphor-mapper`**: Map C concepts to game mechanics
- **`rust-tauri-patterns`**: Backend implementation patterns

**Skills activate automatically** based on your task or can be invoked explicitly:
```
"Use the c-puzzle-designer skill to create a pointer challenge"
```

**Learn more**: [`docs/ai/skills.md`](docs/ai/skills.md)

### 3. Use MCP Tools

**Available MCP servers in `tools/`:**

- **`c_compiler_mcp`**: Compile and run C code (MANDATORY for puzzle validation)
  - `compile_and_run_c(source_code, input_data)` - Human-readable output
  - `compile_and_run_c_structured(source_code, input_data)` - JSON output (preferred for programmatic use)
  - `validate_puzzle_suite(solution_code, test_cases)` - Batch validate multiple test cases (most efficient)

- **`generate_map`** (future): Procedural map generation
  - Current: Use `tools/generate_map.py` via bash

**Tool Selection Guide:**
| Use Case | Tool |
|----------|------|
| Quick single test | `compile_and_run_c` |
| Programmatic validation | `compile_and_run_c_structured` |
| Multiple test cases | `validate_puzzle_suite` (compiles once, runs many) |

**Learn more**: [`docs/ai/mcp-servers.md`](docs/ai/mcp-servers.md)

---

## The Four Core Mandates

**From [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md):**

1. **Backend Authority**: NEVER write game logic in Svelte/JS. Physics, XP, and inventory live in Rust. Svelte is for rendering only.

2. **Verify C Code**: When writing a C puzzle or solution, YOU MUST use the `compile_and_run_c` tool to verify it runs and produces the expected output. Do not guess.

3. **Procedural Assets**: Do not try to generate SVG art. Use the `generate_map.py` tool to configure Tiled maps.

4. **No Magic**: Game mechanics must map to real C concepts. (e.g., "Malloc" creates land, "Free" removes it).

---

## Project Structure

```
C-warrior/
├── .claude/
│   └── skills/              # Claude Code skills (guidance)
├── tools/                   # MCP servers (execution)
├── docs/
│   ├── core/               # Shared AI documentation (START HERE)
│   ├── ai/                 # Skills & MCP guides
│   ├── architecture/       # Technical design
│   ├── game_design/        # Mechanics and metaphors
│   └── curriculum/         # Educational progression
├── src/                    # Rust backend (game logic)
├── src-ui/                 # Svelte frontend (UI only)
└── src/assets/levels.json  # Level definitions (SOURCE OF TRUTH)
```

---

## Common Tasks

| Task | Start Here | Tools/Skills |
|------|------------|--------------|
| **Add game feature** | `rust-tauri-patterns` skill | None |
| **Create C puzzle** | `c-puzzle-designer` skill | `compile_and_run_c` (REQUIRED) |
| **Design game mechanic** | `game-metaphor-mapper` skill | None |
| **Generate map** | `docs/core/WORKFLOWS.md` | `generate_map.py` |
| **Fix bug** | `docs/architecture/system.md` | Depends on issue |
| **Understand architecture** | `code-warrior-architect` skill | None |
| **Test changes** | See Testing Protocol below | Playwright skill |

---

## Testing Protocol (MANDATORY)

**Every code change MUST be tested on both platforms:**

### Production URLs

| Service | URL |
|---------|-----|
| **Frontend (Vercel)** | https://code-warrior-seven.vercel.app |
| **API (Railway)** | https://code-warrior-api-production.up.railway.app |
| **Database (Neon)** | PostgreSQL on Neon (connection string in Railway env) |

### Local Development Setup

**Desktop (Tauri IPC):**
```bash
# Terminal 1: Start frontend dev server
cd src-ui && npm run dev

# Terminal 2: Start Tauri app
cargo tauri dev
```

**Web (HTTP Backend) - Local:**
```bash
# Terminal 1: Start API server
cd src-api && cargo run

# Terminal 2: Start frontend with local API
cd src-ui && API_URL=http://localhost:3000 npm run dev
```

### Dual-Platform Testing

1. **Web (HTTP Backend)** - Primary for automation
   - Production: https://code-warrior-seven.vercel.app
   - Local: http://localhost:1420 (with API_URL=http://localhost:3000)
   - Uses HTTP API calls to backend
   - **Preferred for automated testing** - same game logic, easier to script

2. **Desktop (Tauri IPC)** - Verify native integration
   - Run locally via `cargo tauri dev`
   - Uses direct Rust IPC calls
   - Test after web passes to catch IPC-specific issues

### Testing Checklist

**Before any PR or deployment:**

- [ ] **No Regressions**: Existing features still work
- [ ] **No UI Breaks**: All components render correctly
- [ ] **Full Flow Test**: Start game → Play level → Submit code → Complete level
- [ ] **Movement**: WASD navigation works smoothly
- [ ] **Interactions**: E key triggers terminals/NPCs
- [ ] **Code Submission**: C code compiles and validates correctly
- [ ] **State Persistence**: Game state syncs between actions

### Automated Testing with Playwright

Use the `playwright-skill` for web testing:

```javascript
// Example test flow
await page.goto('https://code-warrior-seven.vercel.app');
await page.click('text=NEW QUEST');
await page.keyboard.press('d'); // Move right
await page.keyboard.press('e'); // Interact
// Verify state changes via console logs or screenshots
```

**Key test scenarios:**
- Boot sequence completes without errors
- Menu navigation works
- Level loading succeeds
- Player movement updates position
- Terminal interaction opens code editor
- Code submission returns results

### Backend Communication

**Critical**: When modifying `src-ui/src/lib/backend/`:

1. **Never break Tauri IPC** - Test desktop after changes
2. **Never break HTTP API** - Test web after changes
3. **Keep interfaces identical** - Both backends implement same `Backend` interface
4. **Test both paths** - A fix for one platform might break the other

### UI/UX Testing

- **Visual inspection**: Take screenshots at key states
- **Responsive check**: Game renders correctly at different sizes
- **Performance**: No lag during movement or state updates
- **Error handling**: Graceful failures with user feedback
- **Console cleanliness**: No unexpected errors (404s, exceptions)

### Automated Deployment & Validation

**Local E2E only (run after every change):**

```bash
./tools/test-local-e2e.sh
```

This script:
1. Assumes local API (`cd src-api && cargo run`) and frontend (`cd src-ui && API_URL=http://localhost:3000 npm run dev`) are running
2. Runs full Playwright E2E tests against `localhost` (same flow as prod)

**Local E2E then deploy (one-shot flow):**

```bash
./tools/test-local-and-deploy.sh
```

This script:
1. Calls `test-local-e2e.sh` (aborts if local tests fail)
2. Then calls `./tools/deploy-and-validate.sh` to deploy and validate production

**Production-only deploy & validation**

```bash
./tools/deploy-and-validate.sh
```

This script:
1. Deploys API to Railway
2. Deploys frontend to Vercel
3. Waits for propagation
4. Runs Playwright validation (API health, levels, frontend, game flow, movement)
5. Reports success/failure with production URLs

**Rule of thumb:** always run `./tools/test-local-e2e.sh` after changes; only trigger `test-local-and-deploy.sh` or `deploy-and-validate.sh` when you intend to deploy.

---

## Technology Stack

**From [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md) - Fixed, no alternatives:**

- **Backend**: Rust 2021, Axum, Diesel (SQLite), Tokio
- **Frontend**: Svelte 5 (Runes), TailwindCSS, TypeScript
- **Bridge**: Tauri 2.0 (Commands/Events)
- **Assets**: Tiled maps, procedural generation

---

## Solo Developer Context

**From [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md):**

- Optimize for **fast, focused iteration**, not team workflows
- Avoid proposing heavy process (RFCs, multi-person reviews, large documentation suites)
- Keep documentation **short and directly useful** for implementation
- Default to **concise answers and minimal boilerplate**

---

## Response Style

- Be **concise** by default; avoid long explanations unless requested
- Provide **direct, actionable answers**
- **Don't re-explain** the overall architecture unless asked
- **Don't create verbose documentation** "for the team"

---

## Large Codebase Analysis with Gemini CLI

**When analyzing large code chunks or understanding the whole codebase**, use Gemini's massive context window:

```bash
gemini -p "<prompt>" --model gemini-3-pro-preview
```

### When to Use Gemini CLI

- Analyzing files that exceed your context window
- Understanding cross-cutting concerns across many files
- Reviewing large PRs or diffs
- Getting a holistic view of system architecture
- Tracing data flow across the entire codebase

### Best Practices for Gemini Prompts

1. **Structure your prompt clearly** - Use sections/headers:
   ```bash
   gemini -p "## Task
   Analyze the data flow in this codebase.

   ## Context
   This is a Rust/Tauri game with Svelte frontend.

   ## Output Format
   Provide a numbered list of the flow steps." --model gemini-3-pro-preview
   ```

2. **Place large context first** - Put code/files at the beginning, then ask your question:
   ```bash
   gemini -p "$(cat src/*.rs)

   Based on the code above, identify all Tauri commands and their purposes." --model gemini-3-pro-preview
   ```

3. **Be specific about output format** - Request structured output:
   - "Return as JSON with keys: function, purpose, dependencies"
   - "Provide a bullet list of findings"
   - "Create a markdown table of X vs Y"

4. **Break complex analysis into steps** - Ask Gemini to plan first:
   ```bash
   gemini -p "First, list all the modules. Then, for each module, describe its responsibility. Finally, draw the dependency graph." --model gemini-3-pro-preview
   ```

5. **Use self-critique for accuracy** - Ask it to verify:
   ```bash
   gemini -p "Analyze this code for bugs. After listing potential issues, review each one and rate your confidence (high/medium/low)." --model gemini-3-pro-preview
   ```

### Example Use Cases

```bash
# Understand entire codebase structure
gemini -p "$(find src -name '*.rs' -exec cat {} \;)

Analyze this Rust codebase and provide:
1. High-level architecture overview
2. Module dependency graph
3. Key data structures and their relationships" --model gemini-3-pro-preview

# Trace a specific feature
gemini -p "$(cat src/**/*.rs src-ui/src/**/*.ts)

Trace how player movement works from frontend input to backend state update. List every file and function involved." --model gemini-3-pro-preview
```

---

## When in Doubt

1. **Check constraints**: [`docs/core/CONSTRAINTS.md`](docs/core/CONSTRAINTS.md)
2. **Find your domain**: [`docs/core/DOMAINS.md`](docs/core/DOMAINS.md)
3. **Activate a skill**: Use Claude Code skills for specialized guidance
4. **Use MCP tools**: For C compilation and code execution
5. **Use Gemini CLI**: For large context analysis beyond your window

---

## Documentation Loading Strategy

**Load documentation on-demand, not upfront.** This reduces context pollution and improves accuracy.

### Always Loaded (from CLAUDE.md)
- Four Core Mandates
- Technology Stack
- Production URLs

### Load When Working In Domain

| Task | Load These Files |
|------|------------------|
| **Creating C puzzle** | `src/assets/levels.json` (structure), invoke `c-puzzle-designer` skill |
| **Adding Rust feature** | Invoke `rust-tauri-patterns` skill, grep existing patterns |
| **Designing game mechanic** | Invoke `game-metaphor-mapper` skill |
| **Understanding architecture** | Invoke `code-warrior-architect` skill |
| **Debugging frontend** | Read specific component in `src-ui/src/lib/` |
| **Debugging backend** | Read specific handler in `src-api/src/` or `src-tauri/src/` |
| **Modifying levels** | `src/assets/levels.json` directly |

### Don't Pre-Read
- ❌ `docs/architecture/system.md` - Use skill instead
- ❌ `docs/game_design/mechanics.md` - Use skill instead
- ❌ `docs/curriculum/progression.md` - Only when designing curriculum
- ❌ Multiple files "just in case" - Load on demand

### Skill-First Approach
Skills contain condensed, actionable guidance. **Prefer invoking a skill over reading documentation files.**

```
Good: "Use the rust-tauri-patterns skill to add a command"
Bad:  "Let me read docs/architecture/system.md first"
```

### Full Documentation Index (Reference Only)

**Core**:
- `docs/core/CONSTRAINTS.md` - Critical rules
- `docs/core/DOMAINS.md` - Task mapping
- `docs/core/WORKFLOWS.md` - Step-by-step guides

**Technical** (load when needed):
- `docs/architecture/system.md` - System design
- `docs/game_design/mechanics.md` - Game mechanics
- `docs/curriculum/progression.md` - Educational theory
- `src/assets/levels.json` - Level definitions (SOURCE OF TRUTH)

**Cross-Agent**:
- `AGENTS.md` - Guidelines for all AI agents
- `GEMINI.md` - Gemini-specific instructions
