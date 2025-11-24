use super::{CCompiler, ExecutionOutput};
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use std::fs;
use std::path::Path;

impl CCompiler {
    /// Compile and run C source code
    pub async fn compile_and_run(&self, source: &str) -> Result<ExecutionOutput, String> {
        let start = Instant::now();
        
        // Create a unique-ish file name to avoid collisions in shared temp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
            
        let source_filename = format!("cw_code_{}.c", timestamp);
        let binary_filename = format!("cw_prog_{}", timestamp);
        
        let source_path = Path::new(&self.temp_dir).join(&source_filename);
        let binary_path = Path::new(&self.temp_dir).join(&binary_filename);

        // Write source to temp file
        fs::write(&source_path, source)
            .map_err(|e| format!("Failed to write source: {}", e))?;

        // Compile with GCC
        // Note: This blocks the thread, but it's short. For heavy load, use spawn_blocking.
        let compile_output = Command::new("gcc")
            .args([source_path.to_str().unwrap(), "-o", binary_path.to_str().unwrap(), "-Wall"])
            .output()
            .map_err(|e| format!("Failed to run gcc: {}", e))?;

        if !compile_output.status.success() {
            // Cleanup source
            let _ = fs::remove_file(&source_path);
            
            return Ok(ExecutionOutput {
                stdout: String::new(),
                stderr: String::from_utf8_lossy(&compile_output.stderr).to_string(),
                compile_error: Some(
                    String::from_utf8_lossy(&compile_output.stderr).to_string()
                ),
                runtime_error: None,
                exit_code: compile_output.status.code(),
                execution_time_ms: start.elapsed().as_millis() as u64,
                timed_out: false,
            });
        }

        // Run with timeout
        // We use spawn_blocking so we can wrap it in a tokio timeout
        let binary_path_clone = binary_path.clone();
        let run_result = timeout(
            Duration::from_secs(self.timeout_secs),
            tokio::task::spawn_blocking(move || {
                Command::new(&binary_path_clone).output()
            })
        ).await;

        let output = match run_result {
            Ok(Ok(Ok(out))) => ExecutionOutput {
                stdout: String::from_utf8_lossy(&out.stdout).to_string(),
                stderr: String::from_utf8_lossy(&out.stderr).to_string(),
                compile_error: None,
                runtime_error: None,
                exit_code: out.status.code(),
                execution_time_ms: start.elapsed().as_millis() as u64,
                timed_out: false,
            },
            Ok(Ok(Err(e))) => ExecutionOutput {
                stdout: String::new(),
                stderr: String::new(),
                compile_error: None,
                runtime_error: Some(format!("Execution failed: {}", e)),
                exit_code: None,
                execution_time_ms: start.elapsed().as_millis() as u64,
                timed_out: false,
            },
            Ok(Err(e)) => ExecutionOutput {
                stdout: String::new(),
                stderr: String::new(),
                compile_error: None,
                runtime_error: Some(format!("Task failed: {}", e)),
                exit_code: None,
                execution_time_ms: start.elapsed().as_millis() as u64,
                timed_out: false,
            },
            Err(_) => ExecutionOutput {
                stdout: String::new(),
                stderr: String::new(),
                compile_error: None,
                runtime_error: Some("Execution timed out".to_string()),
                exit_code: None,
                execution_time_ms: self.timeout_secs * 1000,
                timed_out: true,
            },
        };

        // Cleanup temp files
        let _ = fs::remove_file(&source_path);
        let _ = fs::remove_file(&binary_path);

        Ok(output)
    }
}

