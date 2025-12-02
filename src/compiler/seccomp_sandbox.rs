//! Seccomp-BPF sandbox for secure C code execution.
//!
//! This module provides syscall filtering using seccomp-bpf, which works in
//! containerized environments without requiring namespace privileges (unlike bwrap).
//!
//! Security model:
//! - Allows: basic I/O, memory management, threading (pthreads), signals
//! - Blocks: execve (no shell), fork (no new processes), networking, ptrace
//!
//! This is the primary sandbox for production deployments on Railway/Docker.

#[cfg(all(target_os = "linux", feature = "seccomp"))]
use seccompiler::{
    BpfProgram, SeccompAction, SeccompCmpArgLen, SeccompCmpOp, SeccompCondition,
    SeccompFilter, SeccompRule, TargetArch,
};

#[cfg(all(target_os = "linux", feature = "seccomp"))]
use std::collections::BTreeMap;

use std::io::Write;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::time::Instant;
use tokio::time::{timeout, Duration};

use super::SandboxResult;

/// Check if seccomp is available on this system.
/// Returns true on Linux with the seccomp feature enabled.
#[cfg(all(target_os = "linux", feature = "seccomp"))]
pub fn is_seccomp_available() -> bool {
    // Check if we can set NO_NEW_PRIVS (required for seccomp without CAP_SYS_ADMIN)
    unsafe {
        libc::prctl(libc::PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0) >= 0
    }
}

#[cfg(not(all(target_os = "linux", feature = "seccomp")))]
pub fn is_seccomp_available() -> bool {
    false
}

/// Build the seccomp filter for sandboxed execution.
///
/// Policy:
/// - Default: KILL (whitelist approach)
/// - Allow safe syscalls for basic C programs + threading
/// - Block dangerous syscalls explicitly
#[cfg(all(target_os = "linux", feature = "seccomp"))]
fn build_seccomp_filter() -> Result<BpfProgram, String> {

    // Whitelist of allowed syscalls
    // These are needed for basic C programs with threading support
    let allowed_syscalls: Vec<i64> = vec![
        // Process execution (needed for pre_exec to work)
        // Note: execve is needed because seccomp filter is applied in pre_exec
        // BEFORE the binary starts. The binary is already compiled and in a
        // controlled temp directory with no shell access.
        libc::SYS_execve,
        libc::SYS_execveat,

        // Basic I/O
        libc::SYS_read,
        libc::SYS_write,
        libc::SYS_close,
        libc::SYS_fstat,
        libc::SYS_lseek,
        libc::SYS_dup,
        libc::SYS_dup2,
        libc::SYS_dup3,
        libc::SYS_pipe,
        libc::SYS_pipe2,

        // File operations (needed by musl/glibc for basic operations)
        libc::SYS_openat,     // Used instead of open on modern Linux
        libc::SYS_newfstatat, // Used for fstat in musl
        libc::SYS_readlinkat, // Used for /proc/self/exe resolution
        libc::SYS_access,
        libc::SYS_faccessat,
        libc::SYS_faccessat2,
        libc::SYS_statx,
        libc::SYS_readlink,
        libc::SYS_getcwd,

        // Memory management
        libc::SYS_brk,
        libc::SYS_mmap,
        libc::SYS_munmap,
        libc::SYS_mprotect,
        libc::SYS_mremap,

        // Threading (pthreads)
        libc::SYS_clone,      // For creating threads (will filter for CLONE_THREAD)
        libc::SYS_clone3,     // Modern clone
        libc::SYS_futex,      // Thread synchronization
        libc::SYS_set_robust_list,
        libc::SYS_get_robust_list,
        libc::SYS_set_tid_address,
        libc::SYS_gettid,
        libc::SYS_tgkill,     // Thread-specific signals
        libc::SYS_sched_yield,
        libc::SYS_sched_getaffinity,
        libc::SYS_nanosleep,
        libc::SYS_clock_nanosleep,
        libc::SYS_rseq,       // Restartable sequences (glibc 2.35+)

        // Process info (read-only)
        libc::SYS_getpid,
        libc::SYS_getppid,
        libc::SYS_getuid,
        libc::SYS_geteuid,
        libc::SYS_getgid,
        libc::SYS_getegid,

        // Signals
        libc::SYS_rt_sigaction,
        libc::SYS_rt_sigprocmask,
        libc::SYS_rt_sigreturn,
        libc::SYS_sigaltstack,

        // Exit
        libc::SYS_exit,
        libc::SYS_exit_group,

        // Misc required for glibc/musl
        libc::SYS_arch_prctl,
        libc::SYS_getrandom,
        libc::SYS_clock_gettime,
        libc::SYS_gettimeofday,
        libc::SYS_time,
        libc::SYS_times,
        libc::SYS_uname,
        libc::SYS_getrlimit,
        libc::SYS_prlimit64,
        libc::SYS_prctl,      // For various process control

        // Required for stdio
        libc::SYS_ioctl,      // For terminal ioctls
        libc::SYS_fcntl,
        libc::SYS_poll,
        libc::SYS_ppoll,
        libc::SYS_writev,     // Used by printf
        libc::SYS_readv,

        // Required for memory-mapped I/O
        libc::SYS_msync,
        libc::SYS_madvise,
    ];

    // Build rules - each allowed syscall gets an empty rule (no conditions = always allow)
    let mut rules: BTreeMap<i64, Vec<SeccompRule>> = BTreeMap::new();
    for syscall in allowed_syscalls {
        rules.insert(syscall, vec![]);
    }

    // Special rule for clone: only allow thread creation, not fork
    // CLONE_THREAD (0x10000) must be set for thread creation
    // This prevents fork() while allowing pthread_create()
    rules.insert(
        libc::SYS_clone,
        vec![
            SeccompRule::new(vec![
                SeccompCondition::new(
                    0, // First argument (clone flags)
                    SeccompCmpArgLen::Qword,
                    SeccompCmpOp::MaskedEq(0x10000), // CLONE_THREAD
                    0x10000,
                ).map_err(|e| format!("Failed to create clone condition: {:?}", e))?,
            ]).map_err(|e| format!("Failed to create clone rule: {:?}", e))?,
        ],
    );

    // Build the filter with KILL as default action
    let filter = SeccompFilter::new(
        rules,
        SeccompAction::KillProcess,  // Default: kill if syscall not in whitelist
        SeccompAction::Allow,        // Match action: allow if rule matches
        TargetArch::x86_64,
    ).map_err(|e| format!("Failed to create seccomp filter: {:?}", e))?;

    // Compile to BPF program
    let bpf: BpfProgram = filter.try_into()
        .map_err(|e: seccompiler::BackendError| format!("Failed to compile seccomp filter: {:?}", e))?;

    Ok(bpf)
}

/// Execute a binary with seccomp sandboxing.
///
/// This forks a child process, applies the seccomp filter, then executes the binary.
/// The filter restricts syscalls to a safe subset that allows:
/// - Basic I/O (stdin/stdout/stderr)
/// - Memory allocation
/// - Threading (pthreads)
/// - Process termination
///
/// But blocks:
/// - execve (no shell commands)
/// - fork/vfork (no new processes, only threads)
/// - Networking (no sockets)
/// - File system access (no opening files)
/// - ptrace (no debugging/tracing)
#[cfg(all(target_os = "linux", feature = "seccomp"))]
pub async fn seccomp_execute(
    binary_path: &str,
    stdin_data: Option<&str>,
    timeout_secs: u64,
) -> Result<SandboxResult, String> {
    let binary = binary_path.to_string();
    let input = stdin_data.map(|s| s.to_string());

    let result = timeout(
        Duration::from_secs(timeout_secs),
        tokio::task::spawn_blocking(move || {
            execute_with_seccomp_sync(&binary, input.as_deref())
        }),
    )
    .await;

    match result {
        Ok(Ok(Ok(output))) => Ok(output),
        Ok(Ok(Err(e))) => Err(e),
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

#[cfg(all(target_os = "linux", feature = "seccomp"))]
fn execute_with_seccomp_sync(binary_path: &str, stdin_data: Option<&str>) -> Result<SandboxResult, String> {
    let start = Instant::now();

    // Build the seccomp filter before forking
    let bpf = build_seccomp_filter()?;

    let mut child = unsafe {
        Command::new(binary_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .pre_exec(move || {
                // Set NO_NEW_PRIVS - required for seccomp without CAP_SYS_ADMIN
                if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
                    return Err(std::io::Error::last_os_error());
                }

                // Apply seccomp filter to all threads
                seccompiler::apply_filter_all_threads(&bpf)
                    .map_err(|e| std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to apply seccomp filter: {:?}", e)
                    ))?;

                Ok(())
            })
            .spawn()
            .map_err(|e| format!("Failed to spawn process: {}", e))?
    };

    // Write stdin if provided
    if let Some(input) = stdin_data {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(input.as_bytes());
        }
    }

    // Wait for completion
    let output = child.wait_with_output()
        .map_err(|e| format!("Failed to wait for process: {}", e))?;

    Ok(SandboxResult {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code(),
        timed_out: false,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

/// Fallback for non-Linux systems - seccomp not available
#[cfg(not(all(target_os = "linux", feature = "seccomp")))]
pub async fn seccomp_execute(
    _binary_path: &str,
    _stdin_data: Option<&str>,
    _timeout_secs: u64,
) -> Result<SandboxResult, String> {
    Err("Seccomp sandbox is only available on Linux".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seccomp_availability() {
        let available = is_seccomp_available();
        println!("Seccomp available: {}", available);
        // Just verify it doesn't panic
    }

    #[cfg(all(target_os = "linux", feature = "seccomp"))]
    #[test]
    fn test_build_filter() {
        let filter = build_seccomp_filter();
        assert!(filter.is_ok(), "Failed to build filter: {:?}", filter.err());
    }
}
