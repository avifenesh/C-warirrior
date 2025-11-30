//! Sandbox execution using nsjail for secure C code compilation and execution.
//!
//! Provides OS-level isolation via Linux namespaces (PID, NET, IPC, mount)
//! and seccomp-bpf syscall filtering.

use std::path::Path;
use std::process::Command;
use std::time::Instant;
use tokio::time::{timeout, Duration};

/// Configuration for the nsjail sandbox.
pub struct SandboxConfig {
    /// Path to nsjail binary
    pub nsjail_path: String,
    /// Path to nsjail configuration file
    pub config_path: String,
    /// Execution timeout in seconds
    pub timeout_secs: u64,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            nsjail_path: "nsjail".to_string(),
            config_path: "/app/nsjail.cfg".to_string(),
            timeout_secs: 5,
        }
    }
}

/// Result of sandbox execution.
pub struct SandboxResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub timed_out: bool,
    pub execution_time_ms: u64,
}

/// Check if nsjail is available on the system.
pub fn is_nsjail_available() -> bool {
    Command::new("which")
        .arg("nsjail")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Execute a command inside nsjail sandbox.
///
/// # Arguments
/// * `config` - Sandbox configuration
/// * `command` - Command to execute (e.g., "gcc" or path to binary)
/// * `args` - Arguments to pass to the command
/// * `working_dir` - Working directory inside the sandbox
///
/// # Returns
/// * `Ok(SandboxResult)` - Execution completed (may have failed)
/// * `Err(String)` - Failed to start sandbox
pub async fn sandbox_execute(
    config: &SandboxConfig,
    command: &str,
    args: &[&str],
    working_dir: &Path,
) -> Result<SandboxResult, String> {
    let start = Instant::now();

    // Build nsjail command
    let nsjail_path = config.nsjail_path.clone();
    let config_path = config.config_path.clone();
    let cwd = working_dir.to_string_lossy().to_string();
    let cmd_str = command.to_string();
    let args_vec: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let timeout_secs = config.timeout_secs;

    // Spawn in blocking thread with timeout
    let result = timeout(
        Duration::from_secs(timeout_secs),
        tokio::task::spawn_blocking(move || {
            let mut cmd = Command::new(&nsjail_path);
            cmd.arg("--config")
                .arg(&config_path)
                .arg("--cwd")
                .arg(&cwd)
                .arg("--")
                .arg(&cmd_str);
            for arg in &args_vec {
                cmd.arg(arg);
            }
            cmd.output()
        }),
    )
    .await;

    match result {
        Ok(Ok(Ok(output))) => Ok(SandboxResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            timed_out: false,
            execution_time_ms: start.elapsed().as_millis() as u64,
        }),
        Ok(Ok(Err(e))) => Err(format!("Failed to spawn sandbox: {}", e)),
        Ok(Err(e)) => Err(format!("Task panicked: {}", e)),
        Err(_) => Ok(SandboxResult {
            stdout: String::new(),
            stderr: "Execution timed out".to_string(),
            exit_code: None,
            timed_out: true,
            execution_time_ms: timeout_secs * 1000,
        }),
    }
}

/// Fallback execution without sandbox (for development/Tauri).
/// Uses basic string filtering and timeout but NO OS-level isolation.
///
/// WARNING: This is NOT secure for untrusted code. Only use in development
/// or when nsjail is unavailable.
pub async fn fallback_execute(
    command: &str,
    args: &[&str],
    timeout_secs: u64,
) -> Result<SandboxResult, String> {
    let start = Instant::now();

    let cmd_str = command.to_string();
    let args_vec: Vec<String> = args.iter().map(|s| s.to_string()).collect();

    let result = timeout(
        Duration::from_secs(timeout_secs),
        tokio::task::spawn_blocking(move || {
            let mut cmd = Command::new(&cmd_str);
            for arg in &args_vec {
                cmd.arg(arg);
            }
            cmd.output()
        }),
    )
    .await;

    match result {
        Ok(Ok(Ok(output))) => Ok(SandboxResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            timed_out: false,
            execution_time_ms: start.elapsed().as_millis() as u64,
        }),
        Ok(Ok(Err(e))) => Err(format!("Failed to execute: {}", e)),
        Ok(Err(e)) => Err(format!("Task panicked: {}", e)),
        Err(_) => Ok(SandboxResult {
            stdout: String::new(),
            stderr: "Execution timed out".to_string(),
            exit_code: None,
            timed_out: true,
            execution_time_ms: timeout_secs * 1000,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nsjail_availability_check() {
        // This test just verifies the function runs without panic
        let _ = is_nsjail_available();
    }
}
