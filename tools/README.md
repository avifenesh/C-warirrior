# Code Warrior Development Tools

This directory contains scripts and servers that enhance development workflow for Code Warrior.

## Tools Overview

### 1. C Compiler MCP Server (`c_compiler_mcp.py`)

**Purpose**: Allows AI agents to compile and execute C code safely for testing and validation.

**Installation**:
```bash
pip install fastmcp
```

**Configuration** (add to your MCP settings):

**Cursor** (`.cursor/config.json`):
```json
{
    "mcpServers": {
        "c-runtime": {
            "command": "python",
            "args": ["/absolute/path/to/C-warrior/tools/c_compiler_mcp.py"]
        }
    }
}
```

**Cline** (`cline_mcp_settings.json`):
```json
{
    "c-runtime": {
        "command": "python",
        "args": ["/absolute/path/to/C-warrior/tools/c_compiler_mcp.py"]
    }
}
```

**Usage Example**:
```python
# AI agent can now use this tool
result = compile_and_run_c("""
#include <stdio.h>
int main() {
    int x = 42;
    printf("Answer: %d\\n", x);
    return 0;
}
""")
# Result: "Exit Code: 0\n--- STDOUT ---\nAnswer: 42\n"
```

**Features**:
- Compiles C code with gcc
- Enforces 2-second timeout (prevents infinite loops)
- Captures stdout and stderr
- Safe temporary file handling
- Clear error messages

---

### 2. Procedural Map Generator (`generate_map.py`)

**Purpose**: Generates Tiled-compatible JSON maps using procedural algorithms.

**Installation**: No dependencies required (uses standard library)

**Usage**:

**Drunkard's Walk** (organic islands/caves):
```bash
python tools/generate_map.py --width 40 --height 30 --algorithm drunkard --fill 0.4 > maps/marsh_01.json
```

**Cellular Automata** (natural caves):
```bash
python tools/generate_map.py --algorithm cellular --iterations 5 > maps/cave_01.json
```

**Room-Based Dungeon**:
```bash
python tools/generate_map.py --algorithm rooms --rooms 8 > maps/dungeon_01.json
```

**Parameters**:
- `--width`: Map width in tiles (default: 40)
- `--height`: Map height in tiles (default: 30)
- `--algorithm`: Generation algorithm (drunkard, cellular, rooms)
- `--fill`: Land fill percentage for drunkard walk (0.0-1.0)
- `--iterations`: Smoothing iterations for cellular automata
- `--rooms`: Number of rooms for room algorithm

**Output**: Tiled JSON format compatible with Tiled Map Editor

---

### 3. Level Tools MCP Server (`level_tools_mcp.py`)

**Purpose**: Level management, validation, and analysis.

**Tools**:
- `get_level(id)` - Get full level definition
- `list_all_levels()` - List all levels with summary
- `validate_level(data)` - Check level schema
- `list_concepts_coverage()` - Show C concept coverage gaps

---

### 4. Memory MCP Server (`memory_mcp.py`)

**Purpose**: Cross-session memory persistence for decisions, patterns, and gotchas.

**Tools**:
- `remember(key, value, category)` - Store information
- `recall(key, category)` - Retrieve stored info
- `get_known_gotchas()` - List known issues
- `get_project_decisions()` - List architectural decisions

---

### 5. Project Health MCP Server (`project_health_mcp.py`)

**Purpose**: One-call build status for all components.

**Tools**:
- `check_project_health()` - Full status of rust, frontend, api
- `quick_build_check(component)` - Fast single component check
- `get_git_status()` - Current git branch and changes

---

### 6. Test Runner MCP Server (`test_runner_mcp.py`)

**Purpose**: Integrated testing with smart test selection.

**Tools**:
- `run_tests(scope)` - Run test suite (all, rust, frontend, api)
- `check_for_regressions(changed_files)` - Smart test selection
- `validate_c_solution(code, tests)` - Validate C code against test cases

---

### 7. Local E2E Only (`test-local-e2e.sh`)

**Purpose**: Run full end-to-end tests against a local backend/frontend on `localhost` (no deploy).

**Usage**:
```bash
./tools/test-local-e2e.sh
```

**Requirements**:
- Local API running (default: `http://127.0.0.1:3000`):
  ```bash
  cd src-api && cargo run
  ```
- Local frontend dev server running (default: `http://localhost:1420`):
  ```bash
  cd src-ui && API_URL=http://127.0.0.1:3000 npm run dev
  ```

You can override the defaults:
```bash
LOCAL_API_URL=http://localhost:3000 LOCAL_FRONTEND_URL=http://localhost:1420 ./tools/test-local-e2e.sh
```

### 9. Gemini MCP (`gemini-mcp-tool`)

**Purpose**: Leverage Gemini's 1M token context for large file analysis and web searches.

**Installation**: Auto-installed via npx (configured in `.mcp.json`)

**Model**: **ALWAYS use `gemini-3-pro-preview`**

**MCP Tool Usage**:
```
mcp__gemini__ask-gemini with model: "gemini-3-pro-preview"
```

**CLI Usage**:
```bash
gemini -m gemini-3-pro-preview "<prompt>"
```

**Features**:
- Large file analysis using `@` syntax
- Web searches and best practices research
- Codebase understanding across many files
- Sandbox mode for safe code testing

**When to use**:
- Analyzing files that exceed Claude's context window
- Researching best practices for technologies
- Understanding cross-cutting concerns across many files

---

### 10. Local E2E + Deploy (`test-local-and-deploy.sh`)

**Purpose**: Convenience wrapper that runs local E2E (`test-local-e2e.sh`) and, if it passes, deploys to Railway/Vercel and re-runs validation against production.

**Usage**:
```bash
./tools/test-local-and-deploy.sh
```

---

## Utility Scripts

### `test-all-levels.py`
Validates all levels in `src/assets/levels.json` by testing C solutions against test cases.

### `generate_assets.py`
Generates placeholder game assets (sprites, tiles) for development.

### `generate-theme-sprites.py`
Creates themed sprite variations for different level environments.

### `convert-ascii-to-worldconfig.py`
Converts ASCII art map layouts to JSON world configurations.

### `apply-worldconfigs.py`
Applies world config templates to multiple levels in bulk.

---

## Workflow Integration

### Creating a New Level

1. **Generate the map**:
   ```bash
   python tools/generate_map.py --algorithm drunkard > src/assets/maps/level_05.json
   ```

2. **Test the level layout** in Tiled editor (optional)

3. **Load in Rust backend**:
   ```rust
   use tiled::Loader;
   let map = Loader::new().load_tmx_map("src/assets/maps/level_05.json")?;
   ```

4. **Render in Svelte frontend**

### Testing C Puzzles

1. **Write the challenge code** in your level definition

2. **Test with MCP tool**:
   ```
   Ask AI: "Test this C code with the C compiler tool"
   ```

3. **Verify output** matches success criteria

4. **Add to curriculum** in database

---

## Future Tools

### Asset Pipeline Tool
- Automated sprite sheet generation
- Texture optimization
- Asset versioning

### Level Validator
- Checks level JSON structure
- Validates C code success criteria
- Ensures metaphor consistency

### Curriculum Analyzer
- Analyzes difficulty progression
- Identifies knowledge gaps
- Suggests level ordering

---

## Contributing New Tools

When adding new tools:

1. **Create the script** in this directory
2. **Add documentation** to this README
3. **Include usage examples**
4. **Add installation instructions**
5. **Update IMPLEMENTATION.md** if needed

### Tool Template

```python
#!/usr/bin/env python3
"""
Tool Name

Brief description of what this tool does.

Usage:
    python tool_name.py --arg value
"""

import argparse

def main():
    parser = argparse.ArgumentParser(description="Tool description")
    parser.add_argument("--arg", help="Argument description")
    args = parser.parse_args()

    # Tool implementation
    pass

if __name__ == "__main__":
    main()
```

---

## Troubleshooting

### C Compiler MCP Server

**Issue**: "gcc not found"
**Solution**: Install GCC:
- macOS: `xcode-select --install`
- Ubuntu: `sudo apt install build-essential`
- Windows: Install MinGW or WSL

**Issue**: "ModuleNotFoundError: No module named 'fastmcp'"
**Solution**: `pip install fastmcp`

**Issue**: MCP server not showing in IDE
**Solution**:
1. Check config file path is correct
2. Restart IDE/editor
3. Verify Python path in config

### Map Generator

**Issue**: "Invalid JSON output"
**Solution**: Redirect output to file instead of printing to console

**Issue**: "Maps don't load in game"
**Solution**: Verify tile IDs match your tileset configuration

---

## Performance Notes

### C Compiler Tool
- Compilation: ~50-200ms per code snippet
- Execution: Limited to 2s timeout
- Overhead: Minimal (temporary file I/O)

### Map Generator
- Small maps (40x30): < 10ms
- Large maps (100x100): < 100ms
- Algorithm complexity: O(width × height)

---

For more information, see:
- [IMPLEMENTATION.md](../docs/IMPLEMENTATION.md) - Technical implementation details
- [CLAUDE.md](../CLAUDE.md) - Claude Code AI guidelines
