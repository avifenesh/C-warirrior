mod sandbox;

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Instant;

pub use sandbox::{is_nsjail_available, SandboxConfig, SandboxResult};

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
    timeout_secs: u64,
    max_code_size: usize,
    use_sandbox: bool,
    sandbox_config: SandboxConfig,
}

impl Default for CCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl CCompiler {
    pub fn new() -> Self {
        let use_sandbox = is_nsjail_available();
        if !use_sandbox {
            eprintln!("WARNING: nsjail not available, using fallback execution (NOT SECURE)");
        }
        Self {
            temp_dir: std::env::temp_dir().to_string_lossy().to_string(),
            timeout_secs: 5,
            max_code_size: 10240,
            use_sandbox,
            sandbox_config: SandboxConfig::default(),
        }
    }

    /// Check for dangerous C functions in source code
    fn check_dangerous_functions(&self, source: &str) -> Result<(), String> {
        const DANGEROUS_FUNCS: &[&str] = &["system(", "exec(", "popen(", "fork("];

        for func in DANGEROUS_FUNCS {
            if source.contains(func) {
                return Err(format!("Dangerous function '{}' is not allowed", func.trim_end_matches('(')));
            }
        }

        Ok(())
    }

    /// Compile and run C code with security protections.
    /// Uses nsjail sandbox when available, falls back to basic execution otherwise.
    pub async fn compile_and_run(&self, source: &str) -> Result<ExecutionOutput, String> {
        let start = Instant::now();

        // Check code size limit
        if source.len() > self.max_code_size {
            return Ok(ExecutionOutput {
                compile_error: Some(format!(
                    "Code size exceeds maximum limit of {} bytes",
                    self.max_code_size
                )),
                execution_time_ms: start.elapsed().as_millis() as u64,
                ..Default::default()
            });
        }

        // Only check dangerous functions in fallback mode (nsjail blocks them via seccomp)
        if !self.use_sandbox {
            if let Err(e) = self.check_dangerous_functions(source) {
                return Ok(ExecutionOutput {
                    compile_error: Some(e),
                    execution_time_ms: start.elapsed().as_millis() as u64,
                    ..Default::default()
                });
            }
        }

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        // Create sandbox directory
        let sandbox_dir = Path::new(&self.temp_dir).join(format!("sandbox_{}", timestamp));
        std::fs::create_dir_all(&sandbox_dir)
            .map_err(|e| format!("Failed to create sandbox dir: {}", e))?;

        let source_file = sandbox_dir.join("code.c");
        let binary_file = sandbox_dir.join("program");

        std::fs::write(&source_file, source)
            .map_err(|e| format!("Failed to write source: {}", e))?;

        // Compile with GCC (in sandbox if available)
        let source_str = source_file.to_string_lossy().to_string();
        let binary_str = binary_file.to_string_lossy().to_string();

        let compile_result = if self.use_sandbox {
            sandbox::sandbox_execute(
                &self.sandbox_config,
                "gcc",
                &[&source_str, "-o", &binary_str, "-Wall"],
                &sandbox_dir,
            )
            .await?
        } else {
            sandbox::fallback_execute(
                "gcc",
                &[&source_str, "-o", &binary_str, "-Wall"],
                self.timeout_secs,
            )
            .await?
        };

        if compile_result.exit_code != Some(0) {
            let _ = std::fs::remove_dir_all(&sandbox_dir);
            return Ok(ExecutionOutput {
                compile_error: Some(compile_result.stderr),
                execution_time_ms: start.elapsed().as_millis() as u64,
                ..Default::default()
            });
        }

        // Run the compiled binary (in sandbox if available)
        let run_result = if self.use_sandbox {
            sandbox::sandbox_execute(
                &self.sandbox_config,
                &binary_str,
                &[],
                &sandbox_dir,
            )
            .await?
        } else {
            sandbox::fallback_execute(&binary_str, &[], self.timeout_secs).await?
        };

        // Cleanup
        let _ = std::fs::remove_dir_all(&sandbox_dir);

        Ok(ExecutionOutput {
            stdout: run_result.stdout,
            stderr: run_result.stderr,
            exit_code: run_result.exit_code,
            execution_time_ms: start.elapsed().as_millis() as u64,
            timed_out: run_result.timed_out,
            runtime_error: if run_result.timed_out {
                Some("Execution timed out".to_string())
            } else {
                None
            },
            ..Default::default()
        })
    }
}
