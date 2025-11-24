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

### Minimal Schema

```json
[
  {
    "id": "L01",
    "title": "The First Spell",
    "concept": "printf",
    "description": "The door is voice-activated...",
    "code_template": "#include <stdio.h>\n\nint main() {\n    // Print 'Hello World'\n    return 0;\n}",
    "success_criteria": {
      "type": "exact_match",
      "expected_stdout": "Hello World\n"
    }
  }
]
```

### Extended Schema (with all fields)

```json
[
  {
    "id": "L01",
    "title": "The First Spell",
    "concept": "printf",
    "description": "The door is voice-activated. You must speak the ancient password to enter.",
    "code_template": "#include <stdio.h>\n\nint main() {\n    // Print 'Hello World'\n    return 0;\n}",
    "success_criteria": {
      "type": "exact_match",
      "expected_stdout": "Hello World\n"
    },
    "hints": [
      "Use printf() to print text to the screen",
      "Don't forget the \\n at the end for a newline",
      "The syntax is: printf(\"text here\\n\");"
    ],
    "xp_reward": 50,
    "world_config": {
      "width": 20,
      "height": 15,
      "spawn_x": 64,
      "spawn_y": 64,
      "terminal_x": 320,
      "terminal_y": 224,
      "preset": "tutorial"
    }
  }
]
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
| LevelRegistry | `src/levels/mod.rs` | N/A (backend only) |
| CCompiler | `src/compiler/mod.rs` | N/A (backend only) |

---

## Usage Notes

### For Gemini 3 (Level System)
- Implement `LevelRegistry` to load from `src/assets/levels.json`
- Implement `SuccessCriteria::validate()` for all criteria types
- Implement `CCompiler` for safe code execution
- Add proper timeout and sandboxing

### For Sonnet 4.5 1M (Rust Backend)
- Use `LevelData.world_config` to create level worlds
- Call `LevelRegistry` methods from game state, don't implement directly

### For GPT 5.1 Codex Max (Svelte Frontend)
- Display `LevelData` in level select and game UI
- Show `CodeResult` feedback in code editor
- Use `code_template` as initial editor content

### For Opus 4.5 Standard (Integration)
- Wire up `submit_code` command to call compiler
- Return `CodeResult` from commands, not raw `ExecutionOutput`
- Include friendly feedback messages for the player
