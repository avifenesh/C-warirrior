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

### 3. WFC (Wave Function Collapse) Generator (Coming Soon)

**Purpose**: Advanced procedural generation using constraint-based wave function collapse.

**Status**: Planned implementation in Rust

**Features**:
- Define tile adjacency rules
- Enforce memory allocation patterns
- Generate semantically correct memory visualizations

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
- [SKILL.md](../SKILL.md) - AI agent tool usage guidelines
