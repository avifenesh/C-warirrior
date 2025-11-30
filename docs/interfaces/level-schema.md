# Level Schema & Code Validation

**Owner**: Gemini 3 (Level System)
**Consumers**: All agents

This document defines level data structures, success criteria, and code execution results.

---

## Level Data Structures

### LevelData (Full Level Information)

```rust
// Rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    pub id: String,
    pub title: String,
    pub concept: String,
    pub description: String,
    pub code_template: String,
    pub success_criteria: SuccessCriteria,
    pub hints: Vec<String>,
    pub xp_reward: u32,
    pub world_config: WorldConfig,
}

impl LevelData {
    pub fn from_json(json: &serde_json::Value) -> Result<Self, String> {
        serde_json::from_value(json.clone())
            .map_err(|e| format!("Failed to parse level: {}", e))
    }
}
```

```typescript
// TypeScript
export interface LevelData {
    id: string;
    title: string;
    concept: string;
    description: string;
    code_template: string;
    success_criteria: SuccessCriteria;
    hints: string[];
    xp_reward: number;
    world_config: WorldConfig;
}
```

---

### WorldConfig (Level-specific World Settings)

```rust
// Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub width: usize,
    pub height: usize,
    pub spawn_x: f32,
    pub spawn_y: f32,
    pub terminal_x: f32,
    pub terminal_y: f32,
    pub preset: WorldPreset,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorldPreset {
    Tutorial,        // Simple room with terminal
    Corridor,        // Linear path
    Maze,            // More complex navigation
    Custom(String),  // Reference to custom map file
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            width: 20,
            height: 15,
            spawn_x: 2.0 * 32.0,
            spawn_y: 2.0 * 32.0,
            terminal_x: 10.0 * 32.0,
            terminal_y: 7.0 * 32.0,
            preset: WorldPreset::Tutorial,
        }
    }
}
```

```typescript
// TypeScript
export type WorldPreset = 'tutorial' | 'corridor' | 'maze' | { custom: string };

export interface WorldConfig {
    width: number;
    height: number;
    spawn_x: number;
    spawn_y: number;
    terminal_x: number;
    terminal_y: number;
    preset: WorldPreset;
}
```

---

## Success Criteria

### SuccessCriteria (How to Validate Code)

```rust
// Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SuccessCriteria {
    /// Output must match exactly
    ExactMatch {
        expected_stdout: String,
    },

    /// Output must match regex pattern
    RegexMatch {
        regex: String,
    },

    /// Count occurrences of a token in output
    OutputCount {
        token: String,
        count: usize,
    },

    /// Code must compile without errors (no output check)
    CompileOnly,

    /// Multiple criteria must all pass
    All {
        criteria: Vec<SuccessCriteria>,
    },

    /// At least one criterion must pass
    Any {
        criteria: Vec<SuccessCriteria>,
    },
}

impl SuccessCriteria {
    /// Validate execution output against criteria
    pub fn validate(&self, output: &ExecutionOutput) -> bool {
        match self {
            SuccessCriteria::ExactMatch { expected_stdout } => {
                output.stdout.trim() == expected_stdout.trim()
            }

            SuccessCriteria::RegexMatch { regex } => {
                regex::Regex::new(regex)
                    .map(|re| re.is_match(&output.stdout))
                    .unwrap_or(false)
            }

            SuccessCriteria::OutputCount { token, count } => {
                output.stdout.matches(token).count() == *count
            }

            SuccessCriteria::CompileOnly => {
                output.compile_error.is_none()
            }

            SuccessCriteria::All { criteria } => {
                criteria.iter().all(|c| c.validate(output))
            }

            SuccessCriteria::Any { criteria } => {
                criteria.iter().any(|c| c.validate(output))
            }
        }
    }
}
```

```typescript
// TypeScript
export type SuccessCriteria =
    | { type: 'exact_match'; expected_stdout: string }
    | { type: 'regex_match'; regex: string }
    | { type: 'output_count'; token: string; count: number }
    | { type: 'compile_only' }
    | { type: 'all'; criteria: SuccessCriteria[] }
    | { type: 'any'; criteria: SuccessCriteria[] };
```

---

## Code Execution Results

### ExecutionOutput (Raw Compiler/Runtime Output)

```rust
// Rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionOutput {
    pub stdout: String,
    pub stderr: String,
    pub compile_error: Option<String>,
    pub runtime_error: Option<String>,
    pub exit_code: Option<i32>,
    pub execution_time_ms: u64,
    pub timed_out: bool,
}

impl ExecutionOutput {
    pub fn compile_success(&self) -> bool {
        self.compile_error.is_none()
    }

    pub fn run_success(&self) -> bool {
        self.compile_success()
            && self.runtime_error.is_none()
            && !self.timed_out
            && self.exit_code == Some(0)
    }
}
```

```typescript
// TypeScript
export interface ExecutionOutput {
    stdout: string;
    stderr: string;
    compile_error: string | null;
    runtime_error: string | null;
    exit_code: number | null;
    execution_time_ms: number;
    timed_out: boolean;
}
```

---

### CodeResult (Final Result Sent to Frontend)

```rust
// Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeResult {
    /// Whether the code passed validation
    pub success: bool,

    /// Standard output from execution
    pub stdout: String,

    /// Standard error from execution
    pub stderr: String,

    /// Compilation error if any
    pub compile_error: Option<String>,

    /// Execution time in milliseconds
    pub execution_time_ms: u64,

    /// Human-readable feedback message
    pub feedback: String,

    /// Hint for what went wrong (if failed)
    pub hint: Option<String>,
}

impl CodeResult {
    pub fn from_execution(
        output: &ExecutionOutput,
        criteria: &SuccessCriteria,
    ) -> Self {
        let success = output.compile_success() && criteria.validate(output);

        let feedback = if !output.compile_success() {
            "Code failed to compile. Check for syntax errors.".to_string()
        } else if output.timed_out {
            "Code execution timed out. Check for infinite loops.".to_string()
        } else if success {
            "Success! Your code produced the correct output.".to_string()
        } else {
            "Output doesn't match expected result. Try again!".to_string()
        };

        Self {
            success,
            stdout: output.stdout.clone(),
            stderr: output.stderr.clone(),
            compile_error: output.compile_error.clone(),
            execution_time_ms: output.execution_time_ms,
            feedback,
            hint: None, // Set separately if needed
        }
    }
}
```

```typescript
// TypeScript
export interface CodeResult {
    success: boolean;
    stdout: string;
    stderr: string;
    compile_error: string | null;
    execution_time_ms: number;
    feedback: string;
    hint: string | null;
}
```

---

## Level Registry

### LevelRegistry (Manages All Levels)

```rust
// Rust
use std::collections::HashMap;

pub struct LevelRegistry {
    levels: HashMap<String, LevelData>,
    order: Vec<String>,
}

impl LevelRegistry {
    /// Load levels from src/assets/levels.json
    pub fn load_from_json() -> Self {
        let json_str = include_str!("../assets/levels.json");
        let levels_json: Vec<serde_json::Value> =
            serde_json::from_str(json_str).expect("Invalid levels.json");

        let mut levels = HashMap::new();
        let mut order = Vec::new();

        for level_json in levels_json {
            let level = LevelData::from_json_raw(&level_json)
                .expect("Failed to parse level");
            order.push(level.id.clone());
            levels.insert(level.id.clone(), level);
        }

        Self { levels, order }
    }

    pub fn get_level(&self, id: &str) -> Option<&LevelData> {
        self.levels.get(id)
    }

    pub fn get_all_info(&self) -> Vec<LevelInfo> {
        self.order.iter()
            .filter_map(|id| self.levels.get(id))
            .map(|l| LevelInfo {
                id: l.id.clone(),
                title: l.title.clone(),
                concept: l.concept.clone(),
                completed: false, // Set from GameState
                locked: false,    // Set based on progression
            })
            .collect()
    }

    pub fn get_next_level(&self, current_id: &str) -> Option<String> {
        let current_idx = self.order.iter().position(|id| id == current_id)?;
        self.order.get(current_idx + 1).cloned()
    }
}
```

---

## JSON Schema (levels.json)

The source of truth for level definitions is `src/assets/levels.json`.

### Quest-Based Level Schema (Current)

The current level system uses a **multi-quest** structure where each level contains multiple quests (terminals), each teaching a specific aspect of the concept.

```json
[
  {
    "id": "L01",
    "title": "The First Spell",
    "theme": "L01_village",
    "concept": "return values",
    "description": "Master the art of returning values to unlock the door ahead.",
    "code_template": "#include <stdio.h>\n\n// Write your function here\n\nint main() {\n    return 0;\n}",
    "hints": [],
    "xp_reward": 0,
    "total_xp_reward": 90,
    "map_file": "maps/L01_first_spell.json",
    "challenges": [],
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
]
```

### Quest Structure

Each quest defines a function-based challenge:

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique quest identifier (e.g., "L01_Q1") |
| `order` | number | Display order in the level |
| `title` | string | Quest title shown to player |
| `description` | string | What the player needs to accomplish |
| `recommended` | boolean | Whether this is the suggested starting quest |
| `function_signature` | object | Function name, return type, and parameters |
| `user_template` | string | Code template shown in the editor |
| `test_cases` | array | Test inputs and expected outputs |
| `hints` | array | Progressive hints for struggling players |
| `xp_reward` | number | XP awarded on completion |

### Test Case Structure

```json
{
  "input": [3, 4],
  "expected": "7",
  "sample": true
}
```

- `input`: Array of arguments to pass to the function
- `expected`: Expected return value as a string
- `sample`: Whether this test case is visible to the player

### World Config with Terminals

The `world_config` now includes an array of terminals linked to quests:

```json
{
  "terminals": [
    {"x": 192, "y": 224, "quest_id": "L01_Q1"},
    {"x": 320, "y": 224, "quest_id": "L01_Q2"}
  ]
}
```

### Legacy Challenge Schema (Deprecated)

Some older levels may still use the single-challenge format with `success_criteria`:

```json
{
  "id": "L22",
  "success_criteria": {
    "type": "exact_match",
    "expected_stdout": "Value before banish: 999\nMemory banished!\n"
  },
  "challenges": [
    {
      "id": "free_memory",
      "prompt": "Free the allocated memory...",
      "expected_output": "...",
      "starter_code": "..."
    }
  ]
}
```

---

## Parsing levels.json

### From Raw JSON to LevelData

```rust
impl LevelData {
    /// Parse from raw JSON with defaults for optional fields
    pub fn from_json_raw(json: &serde_json::Value) -> Result<Self, String> {
        let id = json["id"].as_str()
            .ok_or("Level missing 'id'")?
            .to_string();

        let title = json["title"].as_str()
            .ok_or("Level missing 'title'")?
            .to_string();

        let concept = json["concept"].as_str()
            .ok_or("Level missing 'concept'")?
            .to_string();

        let description = json["description"].as_str()
            .unwrap_or("")
            .to_string();

        let code_template = json["code_template"].as_str()
            .ok_or("Level missing 'code_template'")?
            .to_string();

        let success_criteria: SuccessCriteria =
            serde_json::from_value(json["success_criteria"].clone())
                .map_err(|e| format!("Invalid success_criteria: {}", e))?;

        // Optional fields with defaults
        let hints = json["hints"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let xp_reward = json["xp_reward"]
            .as_u64()
            .unwrap_or(50) as u32;

        let world_config = json.get("world_config")
            .map(|wc| serde_json::from_value(wc.clone()).unwrap_or_default())
            .unwrap_or_default();

        Ok(Self {
            id,
            title,
            concept,
            description,
            code_template,
            success_criteria,
            hints,
            xp_reward,
            world_config,
        })
    }
}
```

---

## C Compiler Interface

### CCompiler (Compiles and Runs C Code)

```rust
// Rust - src/compiler/mod.rs
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::time::timeout;

pub struct CCompiler {
    timeout_secs: u64,
    temp_dir: String,
}

impl CCompiler {
    pub fn new() -> Self {
        Self {
            timeout_secs: 5,
            temp_dir: std::env::temp_dir().to_string_lossy().to_string(),
        }
    }

    /// Compile and run C source code
    pub async fn compile_and_run(&self, source: &str) -> Result<ExecutionOutput, String> {
        let start = Instant::now();
        let source_file = format!("{}/user_code.c", self.temp_dir);
        let binary_file = format!("{}/user_prog", self.temp_dir);

        // Write source to temp file
        std::fs::write(&source_file, source)
            .map_err(|e| format!("Failed to write source: {}", e))?;

        // Compile with GCC
        let compile_output = Command::new("gcc")
            .args([&source_file, "-o", &binary_file, "-Wall"])
            .output()
            .map_err(|e| format!("Failed to run gcc: {}", e))?;

        if !compile_output.status.success() {
            return Ok(ExecutionOutput {
                compile_error: Some(
                    String::from_utf8_lossy(&compile_output.stderr).to_string()
                ),
                execution_time_ms: start.elapsed().as_millis() as u64,
                ..Default::default()
            });
        }

        // Run with timeout
        let run_result = timeout(
            Duration::from_secs(self.timeout_secs),
            tokio::task::spawn_blocking(move || {
                Command::new(&binary_file).output()
            })
        ).await;

        let output = match run_result {
            Ok(Ok(Ok(out))) => ExecutionOutput {
                stdout: String::from_utf8_lossy(&out.stdout).to_string(),
                stderr: String::from_utf8_lossy(&out.stderr).to_string(),
                exit_code: out.status.code(),
                execution_time_ms: start.elapsed().as_millis() as u64,
                ..Default::default()
            },
            Ok(Ok(Err(e))) => ExecutionOutput {
                runtime_error: Some(format!("Execution failed: {}", e)),
                execution_time_ms: start.elapsed().as_millis() as u64,
                ..Default::default()
            },
            Ok(Err(e)) => ExecutionOutput {
                runtime_error: Some(format!("Task failed: {}", e)),
                execution_time_ms: start.elapsed().as_millis() as u64,
                ..Default::default()
            },
            Err(_) => ExecutionOutput {
                timed_out: true,
                execution_time_ms: self.timeout_secs * 1000,
                ..Default::default()
            },
        };

        // Cleanup temp files
        let _ = std::fs::remove_file(&source_file);
        let _ = std::fs::remove_file(&binary_file);

        Ok(output)
    }
}
```

---

## File Mapping

| Interface | Rust File | TypeScript File |
|-----------|-----------|-----------------|
| LevelData | `src/levels/loader.rs` | `src-ui/src/lib/types.ts` |
| WorldConfig | `src/levels/loader.rs` | `src-ui/src/lib/types.ts` |
| SuccessCriteria | `src/levels/validator.rs` | `src-ui/src/lib/types.ts` |
| ExecutionOutput | `src/compiler/mod.rs` | `src-ui/src/lib/types.ts` |
| CodeResult | `src/levels/validator.rs` | `src-ui/src/lib/types.ts` |
| Quest | `src/levels/loader.rs` | `src-ui/src/lib/types.ts` |
| FunctionSignature | `src/levels/loader.rs` | `src-ui/src/lib/types.ts` |
| TestCase | `src/levels/loader.rs` | `src-ui/src/lib/types.ts` |
| TestSuiteResult | `src/levels/harness.rs` | `src-ui/src/lib/types.ts` |
| LevelRegistry | `src/levels/mod.rs` | N/A (backend only) |
| CCompiler | `src/compiler/mod.rs` | N/A (backend only) |

---

## Usage Notes

### For Development
- Source of truth: `src/assets/levels.json`
- LevelRegistry loads levels at startup
- Use quest-based validation for function challenges
- Use stdout matching for legacy challenges

### For Adding Levels
- Add level definition to `src/assets/levels.json`
- Include all quests with function signatures and test cases
- Create map file in `src/assets/maps/` if needed
- Test with the C compiler MCP tool
