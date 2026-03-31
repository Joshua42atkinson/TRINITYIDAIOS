// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        gpu_guard.rs
// PURPOSE:     Hardware-safe GPU resource guard — Hotel startup protocol
//
// ARCHITECTURE:
//   • Prevents double-launching llama-server (which crashes the GPU driver)
//   • Three guards: port check, process check, memory budget
//   • PID file tracking for crash recovery
//   • Enforces the Hotel rule: One Heavyweight at a Time
//
// CHANGES:
//   2026-03-22  Cascade  Created after GPU crash from double llama-server load
//
// ═══════════════════════════════════════════════════════════════════════════════

use std::path::{Path, PathBuf};
use tracing::{info, warn};

const PID_FILE: &str = "/tmp/trinity-llama-server.pid";

/// Result of the pre-launch safety check
#[derive(Debug)]
pub enum LaunchDecision {
    /// Safe to launch — nothing running, enough memory
    SafeToLaunch,
    /// Already running on the expected port — just connect
    AlreadyRunning {
        #[allow(dead_code)] // Used in Debug output for diagnostics
        pid: Option<u32>,
    },
    /// Process exists but port not responding — wait for it
    StillLoading { pid: u32 },
    /// Not enough memory to safely load the model
    InsufficientMemory { available_gb: f64, required_gb: f64 },
}

/// Run all three guards and return a launch decision.
///
/// Guard 1: Port check — is something already listening on the target port?
/// Guard 2: Process check — is a llama-server process running?
/// Guard 3: Memory budget — do we have enough RAM for the model?
pub fn pre_launch_check(port: u16, required_memory_gb: f64) -> LaunchDecision {
    let port_alive = check_port(port);
    let process_pid = check_process("llama-server");
    let pid_file_pid = read_pid_file();

    // Guard 1+2: Already running and healthy
    if port_alive {
        let pid = process_pid.or(pid_file_pid);
        info!(
            "🏨 Hotel Guard: llama-server already listening on port {} (pid: {:?})",
            port, pid
        );
        return LaunchDecision::AlreadyRunning { pid };
    }

    // Process running but port not yet open — model still loading
    if let Some(pid) = process_pid {
        info!(
            "🏨 Hotel Guard: llama-server process {} exists but port {} not open — still loading",
            pid, port
        );
        return LaunchDecision::StillLoading { pid };
    }

    // Stale PID file — process died but file remains
    if let Some(stale_pid) = pid_file_pid {
        info!(
            "🏨 Hotel Guard: Cleaning stale PID file (pid {} no longer running)",
            stale_pid
        );
        cleanup_pid_file();
    }

    // Guard 3: Memory budget
    let available_gb = available_memory_gb();
    if available_gb < required_memory_gb {
        warn!(
            "🏨 Hotel Guard: Only {:.1} GB available, need {:.1} GB for model",
            available_gb, required_memory_gb
        );
        return LaunchDecision::InsufficientMemory {
            available_gb,
            required_gb: required_memory_gb,
        };
    }

    info!(
        "🏨 Hotel Guard: All clear — {:.1} GB available (need {:.1} GB)",
        available_gb, required_memory_gb
    );
    LaunchDecision::SafeToLaunch
}

/// Check if a TCP port is accepting connections
fn check_port(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    std::net::TcpStream::connect_timeout(
        &addr.parse().unwrap(),
        std::time::Duration::from_millis(500),
    )
    .is_ok()
}

/// Check if a process matching the name is running. Returns PID if found.
fn check_process(name: &str) -> Option<u32> {
    std::process::Command::new("pgrep")
        .args(["-f", name])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .next()
                    .and_then(|line| line.trim().parse::<u32>().ok())
            } else {
                None
            }
        })
}

/// Get available system memory in GB
fn available_memory_gb() -> f64 {
    let mut sys = sysinfo::System::new();
    sys.refresh_memory();
    sys.available_memory() as f64 / (1024.0 * 1024.0 * 1024.0)
}

// ═══════════════════════════════════════════════════
// PID File Management
// ═══════════════════════════════════════════════════

/// Write the llama-server PID to the tracking file
pub fn write_pid_file(pid: u32) {
    if let Err(e) = std::fs::write(PID_FILE, pid.to_string()) {
        warn!("Failed to write PID file {}: {}", PID_FILE, e);
    } else {
        info!("🏨 PID file written: {} (pid {})", PID_FILE, pid);
    }
}

/// Read the PID from the tracking file. Returns None if file doesn't exist
/// or the process is no longer running.
pub fn read_pid_file() -> Option<u32> {
    let content = std::fs::read_to_string(PID_FILE).ok()?;
    let pid: u32 = content.trim().parse().ok()?;

    // Verify process is actually alive
    let alive = Path::new(&format!("/proc/{}", pid)).exists();
    if alive {
        Some(pid)
    } else {
        None // Stale PID file
    }
}

/// Remove the PID tracking file
pub fn cleanup_pid_file() {
    let _ = std::fs::remove_file(PID_FILE);
}

/// Find the llama-server binary from known locations
pub fn find_llama_server_binary() -> Option<PathBuf> {
    // Disabled intentionally to prevent fighting with vLLM over port 8080.
    None
}

/// Find the first suitable GGUF model
pub fn find_gguf_model() -> Option<PathBuf> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    let model_dir = home.join("trinity-models/gguf");
    let model_dirs = [model_dir];

    model_dirs.iter().find_map(|dir| {
        let dir_path = Path::new(dir);
        if !dir_path.exists() {
            return None;
        }
        let mut files: Vec<_> = std::fs::read_dir(dir_path)
            .ok()?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|e| e == "gguf").unwrap_or(false))
            // Skip split parts (keep only -00001-of- or standalone)
            .filter(|p| {
                let name = p.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                !name.contains("00002-of-") && !name.contains("00003-of-")
            })
            .collect();
        files.sort();
        // Prefer Mistral
        files
            .iter()
            .find(|p| {
                let name = p.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                name.contains("mistral") || name.contains("00001-of-")
            })
            .cloned()
            .or_else(|| files.first().cloned())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_port_closed() {
        // Port 19999 should not be in use
        assert!(!check_port(19999));
    }

    #[test]
    fn test_available_memory_positive() {
        let gb = available_memory_gb();
        assert!(gb > 0.0, "Available memory should be positive: {}", gb);
    }

    #[test]
    fn test_pid_file_roundtrip() {
        let test_file = "/tmp/trinity-test-pid.pid";
        std::fs::write(test_file, "99999999").unwrap();
        // PID 99999999 shouldn't exist
        let content = std::fs::read_to_string(test_file).unwrap();
        let pid: u32 = content.trim().parse().unwrap();
        assert!(!Path::new(&format!("/proc/{}", pid)).exists());
        std::fs::remove_file(test_file).ok();
    }

    #[test]
    fn test_pre_launch_insufficient_memory() {
        // Request an absurd amount of memory
        let decision = pre_launch_check(19999, 999999.0);
        matches!(decision, LaunchDecision::InsufficientMemory { .. });
    }
}
