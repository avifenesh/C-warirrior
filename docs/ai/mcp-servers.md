# MCP Servers for Code Warrior

**MCP (Model Context Protocol) servers provide executable tools for all AI agents.**

## Available MCP Servers

### 1. C Compiler MCP Server

**File**: `tools/c_compiler_mcp.py`

**Purpose**: Compile and execute C code with test validation

**Tools Provided**:
```
compile_and_run_c(source_code: str, input_data: str) -> str
```

**When to Use**:
- **MANDATORY** when creating C puzzles
- Testing C solutions
- Verifying puzzle correctness
- Checking for compilation errors
- Validating test cases

**Example Usage**:
```python
# From AI agent:
compile_and_run_c(
    source_code="""
    #include <stdio.h>
    int main() {
        int a = 5, b = 10;
        // swap logic
        printf("%d %d", a, b);
        return 0;
    }
    """,
    input_data=""
)

# Returns (conceptual shape for puzzle design):
{
    "success": true,
    "output": "10 5",
    "errors": "",
    "exit_code": 0
}
```

> NOTE (FastMCP / MCP detail): the Python implementation returns a single result string, and FastMCP wraps it as `{ "result": "<string>" }` on the wire. Most clients (Claude, Codex via wrappers, etc.) surface that as a plain string. The structured `{success, output, errors, exit_code}` object above is a conceptual contract for puzzle design; treat the tool output as opaque text unless you explicitly change the server to return a dict.

**Features**:
- Sandboxed execution
- Timeout protection
- Captures stdout/stderr
- Returns exit codes
- Handles compilation errors
- Works via wrapper in Codex CLI (see setup below)

**Constraints**:
- Standard C (C99/C11)
- No platform-specific extensions
- Limited execution time
- No file system access (within sandbox)

---

### 2. Map Generator (Future MCP Server)

**File**: `tools/generate_map_mcp.py` (not yet created)

**Current Alternative**: Use `tools/generate_map.py` via bash

**Purpose**: Generate procedural Tiled-compatible maps

**Planned Tools**:
```
generate_drunkard_map(width, height, fill) -> dict
generate_cellular_map(width, height, iterations) -> dict
generate_room_map(width, height, rooms) -> dict
```

**Algorithms**:
- **Drunkard's Walk**: Organic islands/caves
- **Cellular Automata**: Natural cave systems
- **Room-based**: Dungeons with connected rooms

**When to Use**:
- Creating level maps
- Generating terrain
- Procedural level design

**Current Usage** (via bash):
```bash
cd tools
python generate_map.py --algorithm drunkard --width 40 --height 30
```

---

## Setting Up MCP Servers

### For Claude Code

Create `.claude/settings.local.json`:
```json
{
  "mcpServers": {
    "c_compiler": {
      "command": "python",
      "args": ["tools/c_compiler_mcp.py"]
    }
  }
}
```

### For Other Agents

Check agent-specific MCP configuration:
- Gemini: See GEMINI.md
- Other: Consult agent documentation

### Codex CLI (via wrapper)

For project-local MCPs in Codex CLI, use the `codex-mcp-wrapper` to avoid per-server global config.

**Setup** (one-time in `~/.codex/config.toml`):

```toml
# C Compiler MCP - wraps project-local .mcp.json for C code compilation/validation
# Must run Codex from C-warirrior directory for wrapper to discover local config
[mcp_servers.c-compiler]
command = "npx"
args = ["-y", "tsx", "/Users/avifen/.codex/codex-mcp-wrapper/src/index.ts"]
```

**How it works**:
1. Clone `kazuhideoki/codex-mcp-wrapper` (e.g., to `~/.codex/codex-mcp-wrapper`)
2. Add the wrapper config to your global `~/.codex/config.toml`
3. Run `codex` from the `C-warirrior` directory
4. Wrapper discovers `./.mcp.json` and starts the `c_compiler` server
5. Tools appear as `mcp__c_compiler__compile_and_run_c`

**Benefits**:
- Single wrapper config for all project-local MCPs
- No need to update global config when adding new project servers
- Each project defines its own MCP servers in `.mcp.json`

---

## Using MCP Tools from AI Agents

### Discovery
MCP tools are automatically available when servers are configured.

### Invocation
Call tools by name with appropriate parameters:
```
compile_and_run_c(source_code="...", input_data="...")
```

### Error Handling
Tools return structured error information:
```json
{
  "success": false,
  "output": "",
  "errors": "main.c:5:10: error: expected ';'",
  "exit_code": 1
}
```

---

## MCP Server Development

### Creating a New MCP Server

Use FastMCP library (Python):

```python
from fastmcp import FastMCP

mcp = FastMCP("Server-Name")

@mcp.tool
def my_tool(param: str) -> dict:
    """Tool description for AI agents."""
    # Implementation
    return {"result": "success"}

if __name__ == "__main__":
    mcp.run()
```

### Best Practices:
1. **Clear naming**: Tool names should be descriptive
2. **Type hints**: Always include parameter and return types
3. **Documentation**: Docstrings help AI understand usage
4. **Error handling**: Return structured errors, not exceptions
5. **Validation**: Check inputs before processing
6. **Timeout**: Prevent infinite execution

---

## MCP vs Skills

| Aspect | MCP Servers | Skills |
|--------|-------------|--------|
| **What** | Executable code | Instructions/guidance |
| **How** | Python functions | Markdown prompts |
| **When** | Need to run code | Need domain knowledge |
| **Who** | All AI agents | Claude Code only |
| **Example** | Compile C code | How to design puzzles |

**Use both**: Skills provide guidance, MCP tools provide execution.

---

## Troubleshooting

### Server Not Found
- Check `.claude/settings.local.json` configuration
- Verify server file path is correct
- Ensure Python is in PATH

### Tool Timeout
- C code may have infinite loop
- Increase timeout in server code
- Check for blocking operations

### Compilation Errors
- Verify C code syntax
- Check for missing includes
- Ensure standard C (no extensions)

### Permission Errors
- MCP servers need execute permissions
- Check file permissions: `chmod +x tools/*.py`
- Verify Python environment

---

## Future MCP Servers

Planned additions:

### Level Validator MCP
- Validate level JSON structure
- Check puzzle-to-map consistency
- Verify progression requirements

### Asset Pipeline MCP
- Process sprites/textures
- Optimize assets
- Generate asset manifests

### Game Mechanics Tester MCP
- Test game logic in isolation
- Simulate game scenarios
- Validate metaphor mappings

---

## Agent Compatibility

**MCP servers work with ALL AI agents** that support MCP:
- Claude Code ✓
- Gemini (with MCP support) ✓
- Other MCP-compatible agents ✓

For agent-specific setup:
- Claude: This file + `.claude/settings.local.json`
- Gemini: See `GEMINI.md`
- Others: See `AGENTS.md`
