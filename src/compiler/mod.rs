use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::time::timeout;

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

    pub async fn compile_and_run(&self, source: &str) -> Result<ExecutionOutput, String> {
        let start = Instant::now();
        let source_file = format!("{}/user_code.c", self.temp_dir);
        let binary_file = format!("{}/user_prog", self.temp_dir);

        std::fs::write(&source_file, source)
            .map_err(|e| format!("Failed to write source: {}", e))?;

        let compile_output = Command::new("gcc")
            .args([&source_file, "-o", &binary_file, "-Wall"])
            .output()
            .map_err(|e| format!("Failed to run gcc: {}", e))?;

        if !compile_output.status.success() {
            return Ok(ExecutionOutput {
                compile_error: Some(String::from_utf8_lossy(&compile_output.stderr).to_string()),
                execution_time_ms: start.elapsed().as_millis() as u64,
                ..Default::default()
            });
        }

        let timeout_duration = Duration::from_secs(self.timeout_secs);
        let binary = binary_file.clone();

        let run_result = timeout(
            timeout_duration,
            tokio::task::spawn_blocking(move || Command::new(&binary).output()),
        )
        .await;

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

        let _ = std::fs::remove_file(&source_file);
        let _ = std::fs::remove_file(&binary_file);

        Ok(output)
    }
}
