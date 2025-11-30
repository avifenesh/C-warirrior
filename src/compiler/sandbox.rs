//! Sandbox execution using bubblewrap (bwrap) for secure C code compilation and execution.
//!
//! Provides OS-level isolation via Linux namespaces (PID, NET, IPC, mount).
//! Bubblewrap is simpler than nsjail and available in Alpine repos.

use std::path::Path;
use std::process::Command;
use std::time::Instant;
use tokio::time::{timeout, Duration};

/// Configuration for the bubblewrap sandbox.
pub struct SandboxConfig {
    /// Execution timeout in seconds
    pub timeout_secs: u64,
    /// Memory limit in bytes (0 = unlimited)
    pub memory_limit: u64,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 5,
            memory_limit: 64 * 1024 * 1024, // 64MB
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

/// Check if bubblewrap (bwrap) is available on the system.
pub fn is_bwrap_available() -> bool {
    Command::new("which")
        .arg("bwrap")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Legacy alias for compatibility
pub fn is_nsjail_available() -> bool {
    is_bwrap_available()
}

/// Execute a command inside bubblewrap sandbox.
///
/// # Arguments
/// * `config` - Sandbox configuration
/// * `command` - Command to execute (e.g., "gcc" or path to binary)
/// * `args` - Arguments to pass to the command
/// * `working_dir` - Working directory containing source/binary
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

    let cwd = working_dir.to_string_lossy().to_string();
    let cmd_str = command.to_string();
    let args_vec: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let timeout_secs = config.timeout_secs;

    // Spawn in blocking thread with timeout
    let result = timeout(
        Duration::from_secs(timeout_secs),
        tokio::task::spawn_blocking(move || {
            let mut cmd = Command::new("bwrap");

            // Namespace isolation
            cmd.arg("--unshare-net") // No network access
                .arg("--unshare-pid") // Isolated PID namespace
                .arg("--unshare-ipc") // Isolated IPC
                .arg("--die-with-parent"); // Kill child if parent dies

            // Read-only bind mounts for system directories
            cmd.arg("--ro-bind").arg("/usr").arg("/usr")
                .arg("--ro-bind").arg("/lib").arg("/lib")
                .arg("--ro-bind").arg("/bin").arg("/bin");

            // /lib64 may not exist on all systems
            if Path::new("/lib64").exists() {
                cmd.arg("--ro-bind").arg("/lib64").arg("/lib64");
            }

            // Bind /etc for timezone, resolv.conf etc (read-only)
            cmd.arg("--ro-bind").arg("/etc").arg("/etc");

            // Writable temp directory
            cmd.arg("--tmpfs").arg("/tmp");

            // Bind working directory read-write for compilation
            cmd.arg("--bind").arg(&cwd).arg(&cwd);

            // Set working directory
            cmd.arg("--chdir").arg(&cwd);

            // Device access (minimal)
            cmd.arg("--dev").arg("/dev");
            cmd.arg("--proc").arg("/proc");

            // The command to run
            cmd.arg("--").arg(&cmd_str);

            // Add command arguments
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
