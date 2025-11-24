use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Instant;

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

pub struct CCompiler {
    temp_dir: String,
}

impl CCompiler {
    pub fn new() -> Self {
        Self {
            temp_dir: std::env::temp_dir().to_string_lossy().to_string(),
        }
    }

    /// Compile and run C code synchronously (Tauri handles async wrapping)
    pub async fn compile_and_run(&self, source: &str) -> Result<ExecutionOutput, String> {
        let start = Instant::now();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let source_file = format!("{}/user_code_{}.c", self.temp_dir, timestamp);
        let binary_file = format!("{}/user_prog_{}", self.temp_dir, timestamp);

        std::fs::write(&source_file, source)
            .map_err(|e| format!("Failed to write source: {}", e))?;

        // Compile with GCC
        let compile_output = Command::new("gcc")
            .args([&source_file, "-o", &binary_file, "-Wall"])
            .output()
            .map_err(|e| format!("Failed to run gcc: {}", e))?;

        if !compile_output.status.success() {
            let _ = std::fs::remove_file(&source_file);
            return Ok(ExecutionOutput {
                compile_error: Some(String::from_utf8_lossy(&compile_output.stderr).to_string()),
                execution_time_ms: start.elapsed().as_millis() as u64,
                ..Default::default()
            });
        }

        // Run the compiled binary synchronously
        let run_output = Command::new(&binary_file)
            .output()
            .map_err(|e| format!("Failed to run binary: {}", e))?;

        let output = ExecutionOutput {
            stdout: String::from_utf8_lossy(&run_output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&run_output.stderr).to_string(),
            exit_code: run_output.status.code(),
            execution_time_ms: start.elapsed().as_millis() as u64,
            ..Default::default()
        };

        // Cleanup
        let _ = std::fs::remove_file(&source_file);
        let _ = std::fs::remove_file(&binary_file);

        Ok(output)
    }
}
