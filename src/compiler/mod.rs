//! C code compilation and secure execution.
//!
//! Sandbox priority:
//! 1. seccomp-bpf (Linux, works in containers without namespace privileges)
//! 2. bubblewrap (Linux, requires namespace support)
//! 3. Fallback (development only, NOT SECURE)

mod sandbox;

#[cfg(all(target_os = "linux", feature = "seccomp"))]
mod seccomp_sandbox;

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Instant;

pub use sandbox::{SandboxConfig, SandboxResult};

#[cfg(all(target_os = "linux", feature = "seccomp"))]
use seccomp_sandbox::{is_seccomp_available, seccomp_execute};

/// Available sandbox modes, in order of preference
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SandboxMode {
    /// seccomp-bpf syscall filtering (works in containers)
    Seccomp,
    /// bubblewrap namespace isolation (requires namespace privileges)
    Bubblewrap,
    /// No sandbox - DEVELOPMENT ONLY
    Fallback,
}

impl std::fmt::Display for SandboxMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SandboxMode::Seccomp => write!(f, "seccomp"),
            SandboxMode::Bubblewrap => write!(f, "bubblewrap"),
            SandboxMode::Fallback => write!(f, "fallback (INSECURE)"),
        }
    }
}

/// Detect the best available sandbox mode.
///
/// SECURITY: Will panic if no secure sandbox is available unless
/// `ALLOW_INSECURE_SANDBOX=1` is explicitly set (development only).
fn detect_sandbox_mode() -> SandboxMode {
    // Try seccomp first (preferred for containers)
    #[cfg(all(target_os = "linux", feature = "seccomp"))]
    {
        if is_seccomp_available() {
            return SandboxMode::Seccomp;
        }
    }

    // Try bubblewrap
    if sandbox::is_bwrap_available() {
        // Test if bwrap actually works (namespace creation)
        if test_bwrap_namespaces() {
            return SandboxMode::Bubblewrap;
        }
    }

    // macOS: Always allow fallback (development only, servers are Linux)
    #[cfg(target_os = "macos")]
    {
        return SandboxMode::Fallback;
    }

    // Linux: Check if insecure mode is explicitly allowed
    #[cfg(not(target_os = "macos"))]
    {
        if std::env::var("ALLOW_INSECURE_SANDBOX").map(|v| v == "1").unwrap_or(false) {
            return SandboxMode::Fallback;
        }

        // FAIL HARD in production - do not silently run without sandbox
        panic!(
            "FATAL: No secure sandbox available!\n\
             \n\
             Neither seccomp nor bubblewrap is working on this system.\n\
             Running untrusted C code without a sandbox is a critical security risk.\n\
             \n\
             Options:\n\
             1. Deploy on Linux with seccomp support (recommended)\n\
             2. Enable user namespaces for bubblewrap\n\
             3. Set ALLOW_INSECURE_SANDBOX=1 for development ONLY\n\
             \n\
             DO NOT set ALLOW_INSECURE_SANDBOX=1 in production!"
        );
    }
}

/// Test if bubblewrap can create namespaces (may fail in containers)
fn test_bwrap_namespaces() -> bool {
    use std::process::Command;

    // Try to run a simple command with namespace isolation
    Command::new("bwrap")
        .args(["--unshare-pid", "--", "/bin/true"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Legacy alias for compatibility
pub fn is_nsjail_available() -> bool {
    matches!(detect_sandbox_mode(), SandboxMode::Seccomp | SandboxMode::Bubblewrap)
}

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
    sandbox_mode: SandboxMode,
    sandbox_config: SandboxConfig,
}

impl Default for CCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl CCompiler {
    pub fn new() -> Self {
        let sandbox_mode = detect_sandbox_mode();

        match sandbox_mode {
            SandboxMode::Seccomp => {
                eprintln!("INFO: Using seccomp-bpf sandbox (container-compatible)");
            }
            SandboxMode::Bubblewrap => {
                eprintln!("INFO: Using bubblewrap sandbox");
            }
            SandboxMode::Fallback => {
                #[cfg(target_os = "macos")]
                eprintln!("INFO: Using fallback sandbox (macOS development mode)");

                #[cfg(not(target_os = "macos"))]
                {
                    eprintln!("╔══════════════════════════════════════════════════════════════╗");
                    eprintln!("║  ⚠️  WARNING: INSECURE SANDBOX MODE ENABLED                   ║");
                    eprintln!("║                                                              ║");
                    eprintln!("║  ALLOW_INSECURE_SANDBOX=1 is set. Code execution is NOT      ║");
                    eprintln!("║  sandboxed. This is acceptable for local development only.   ║");
                    eprintln!("║                                                              ║");
                    eprintln!("║  DO NOT USE IN PRODUCTION - you WILL be compromised.         ║");
                    eprintln!("╚══════════════════════════════════════════════════════════════╝");
                }
            }
        }

        Self {
            temp_dir: std::env::temp_dir().to_string_lossy().to_string(),
            timeout_secs: 5,
            max_code_size: 10240,
            sandbox_mode,
            sandbox_config: SandboxConfig::default(),
        }
    }

    /// Get the current sandbox mode
    pub fn sandbox_mode(&self) -> SandboxMode {
        self.sandbox_mode
    }

    /// Check for dangerous C functions in source code (used in fallback mode)
    fn check_dangerous_functions(&self, source: &str) -> Result<(), String> {
        const DANGEROUS_FUNCS: &[&str] = &["system(", "exec(", "popen(", "fork("];

        for func in DANGEROUS_FUNCS {
            if source.contains(func) {
                return Err(format!(
                    "Dangerous function '{}' is not allowed",
                    func.trim_end_matches('(')
                ));
            }
        }

        Ok(())
    }

    /// Compile and run C code with security protections.
    ///
    /// Sandbox priority:
    /// 1. seccomp (syscall filtering, works in containers)
    /// 2. bubblewrap (namespace isolation)
    /// 3. fallback (string filtering only, NOT SECURE)
    pub async fn compile_and_run(&self, source: &str) -> Result<ExecutionOutput, String> {
        self.compile_and_run_with_input(source, None).await
    }

    /// Compile and run C code with optional stdin input
    #[allow(unused_variables)] // stdin_input only used with seccomp on Linux
    pub async fn compile_and_run_with_input(
        &self,
        source: &str,
        stdin_input: Option<&str>,
    ) -> Result<ExecutionOutput, String> {
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

        // Only check dangerous functions in fallback mode
        // (seccomp and bwrap block them at OS level)
        if self.sandbox_mode == SandboxMode::Fallback {
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

        // Create working directory
        let sandbox_dir = Path::new(&self.temp_dir).join(format!("sandbox_{}", timestamp));
        std::fs::create_dir_all(&sandbox_dir)
            .map_err(|e| format!("Failed to create sandbox dir: {}", e))?;

        let source_file = sandbox_dir.join("code.c");
        let binary_file = sandbox_dir.join("program");

        std::fs::write(&source_file, source)
            .map_err(|e| format!("Failed to write source: {}", e))?;

        let source_str = source_file.to_string_lossy().to_string();
        let binary_str = binary_file.to_string_lossy().to_string();

        // COMPILE PHASE
        // Compilation is safe - just run gcc directly (no untrusted code execution)
        let compile_result = sandbox::fallback_execute(
            "gcc",
            &[&source_str, "-o", &binary_str, "-Wall", "-lpthread"],
            self.timeout_secs,
        )
        .await?;

        if compile_result.exit_code != Some(0) {
            let _ = std::fs::remove_dir_all(&sandbox_dir);
            return Ok(ExecutionOutput {
                compile_error: Some(compile_result.stderr),
                execution_time_ms: start.elapsed().as_millis() as u64,
                ..Default::default()
            });
        }

        // EXECUTION PHASE - This is where sandboxing matters
        let run_result = match self.sandbox_mode {
            #[cfg(all(target_os = "linux", feature = "seccomp"))]
            SandboxMode::Seccomp => {
                seccomp_execute(&binary_str, stdin_input, self.timeout_secs).await?
            }

            #[cfg(not(all(target_os = "linux", feature = "seccomp")))]
            SandboxMode::Seccomp => {
                // Should never happen - detect_sandbox_mode wouldn't return Seccomp
                return Err("Seccomp not available on this platform".to_string());
            }

            SandboxMode::Bubblewrap => {
                sandbox::sandbox_execute(
                    &self.sandbox_config,
                    &binary_str,
                    &[],
                    &sandbox_dir,
                )
                .await?
            }

            SandboxMode::Fallback => {
                sandbox::fallback_execute(&binary_str, &[], self.timeout_secs).await?
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_mode_detection() {
        let mode = detect_sandbox_mode();
        println!("Detected sandbox mode: {}", mode);
        // Just ensure it doesn't panic
    }

    #[test]
    fn test_dangerous_function_check() {
        let compiler = CCompiler {
            temp_dir: "/tmp".to_string(),
            timeout_secs: 5,
            max_code_size: 10240,
            sandbox_mode: SandboxMode::Fallback,
            sandbox_config: SandboxConfig::default(),
        };

        assert!(compiler.check_dangerous_functions("int main() { return 0; }").is_ok());
        assert!(compiler.check_dangerous_functions("system(\"ls\");").is_err());
        assert!(compiler.check_dangerous_functions("popen(\"cmd\", \"r\");").is_err());
    }
}
