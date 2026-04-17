// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Hotel Manager
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        hotel_manager.rs
// PURPOSE:     Manage P-ART-Y model hot-swapping via the Hotel Swap Protocol
//
// 🪟 THE LIVING CODE TEXTBOOK (P-ART-Y Infrastructure):
// This file orchestrates the physical loading and unloading of AI models.
// The "Hotel" metaphor: only ONE heavyweight model occupies the swap zone
// at any given time. When a new phase requires a different model, the
// current occupant checks out (process killed) and the new guest checks
// in (process launched + health verified).
//
// 📖 THE HOOK BOOK CONNECTION:
// This file implements the 'Model Hot-Swap' Hook. The Conductor calls
// `hotel_swap()` during ADDIECRAPEYE phase transitions to ensure the
// right cognitive tool is loaded for the right task.
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system.
// Failed swaps are reported as obstacles but never block the system —
// Tempo (E4B) handles everything in degraded "Lone Wolf" mode.
//
// ARCHITECTURE:
//   • `hotel_swap(target_role)` — the single entry point
//   • Phase 1: Check if target is already loaded (skip if so)
//   • Phase 2: Kill any existing occupant on the target port
//   • Phase 3: Launch the target model via its launch script (--bg)
//   • Phase 4: Wait for /health to respond (timeout: 90s)
//   • Fallback: If swap fails, Tempo handles the request
//
// CHANGES:
//   2026-04-14  Cascade  Initial implementation — Gemma 4 P-ART-Y Hotel Protocol
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::inference_router::PartyRole;
use std::time::Duration;
use tracing::{error, info, warn};

/// Result of a Hotel swap attempt
#[derive(Debug, Clone)]
pub struct HotelSwapResult {
    /// The role that was requested
    pub requested: PartyRole,
    /// Whether the swap succeeded (model is healthy)
    pub success: bool,
    /// The URL of the loaded model (if successful)
    pub url: Option<String>,
    /// How long the swap took
    pub duration: Duration,
    /// Human-readable message
    pub message: String,
}

/// Configuration for each P-ART-Y Hotel guest
struct HotelGuest {
    role: PartyRole,
    port: u16,
    launch_script: &'static str,
    name: &'static str,
}

/// All known Hotel guests (models that can be swapped in/out)
const HOTEL_GUESTS: &[HotelGuest] = &[
    HotelGuest {
        role: PartyRole::Programming,
        port: 8000,
        launch_script: "scripts/launch/launch_pete_coder.sh",
        name: "P (Programming — Gemma 4 26B A4B)",
    },
    HotelGuest {
        role: PartyRole::Reasoning,
        port: 8002,
        launch_script: "scripts/launch/launch_recycler_dense.sh",
        name: "R (Reasoning — Gemma 4 31B Dense)",
    },
    HotelGuest {
        role: PartyRole::Aesthetics,
        port: 8003,
        launch_script: "scripts/launch/janus_sidecar.py",
        name: "A (Aesthetics — Janus Pro 7B)",
    },
];

/// The current Hotel occupant — tracked across swaps to avoid redundant restarts.
/// This is a simple in-memory state; it resets on server restart.
static CURRENT_OCCUPANT: std::sync::OnceLock<tokio::sync::Mutex<Option<PartyRole>>> =
    std::sync::OnceLock::new();

fn occupant_lock() -> &'static tokio::sync::Mutex<Option<PartyRole>> {
    CURRENT_OCCUPANT.get_or_init(|| tokio::sync::Mutex::new(None))
}

/// Execute a Hotel swap: unload the current occupant, load the target model,
/// wait for health, and return the result.
///
/// This is the single entry point for all model hot-swapping in TRINITY.
/// It is called by `conductor_leader.rs::manage_hotel_sidecars()`.
///
/// # Behavior
/// - If the target role is already loaded (and healthy), returns immediately.
/// - If a different model occupies the swap zone, it is killed first.
/// - The target model is launched in the background via its launch script.
/// - Health is verified via HTTP GET to /health (timeout: 60s).
/// - On failure, the system operates in Lone Wolf mode (Tempo handles everything).
pub async fn hotel_swap(target: PartyRole) -> HotelSwapResult {
    let start = std::time::Instant::now();

    // Find the guest config for this role
    let guest = match HOTEL_GUESTS.iter().find(|g| g.role == target) {
        Some(g) => g,
        None => {
            return HotelSwapResult {
                requested: target,
                success: false,
                url: None,
                duration: start.elapsed(),
                message: format!("No Hotel guest config for role {}", target),
            };
        }
    };

    let target_url = format!("http://127.0.0.1:{}", guest.port);

    // ── Phase 1: Check if target is already the occupant and is healthy ──
    {
        let occupant = occupant_lock().lock().await;
        if *occupant == Some(target) {
            // Already loaded — verify health
            if check_health(&target_url).await {
                info!(
                    "🏨 Hotel: {} already checked in and healthy — skipping swap",
                    guest.name
                );
                return HotelSwapResult {
                    requested: target,
                    success: true,
                    url: Some(target_url),
                    duration: start.elapsed(),
                    message: format!("{} already loaded", guest.name),
                };
            }
            // Occupant was set but model crashed — fall through to re-launch
            warn!(
                "🏨 Hotel: {} was occupant but is unhealthy — re-launching",
                guest.name
            );
        }
    }

    // ── Phase 2: Kill ALL Hotel ports (checkout any current occupant) ──
    info!("🏨 Hotel checkout: clearing swap zone for {}", guest.name);
    for other_guest in HOTEL_GUESTS {
        kill_port(other_guest.port).await;
    }

    // Brief pause for port release
    tokio::time::sleep(Duration::from_secs(1)).await;

    // ── Phase 3: Launch the target model ──
    let workspace = std::env::current_dir().unwrap_or_default();
    let script_path = workspace.join(guest.launch_script);

    if !script_path.exists() {
        warn!(
            "🏨 Hotel: Launch script not found: {}",
            script_path.display()
        );
        // Update occupant to None
        *occupant_lock().lock().await = None;
        return HotelSwapResult {
            requested: target,
            success: false,
            url: None,
            duration: start.elapsed(),
            message: format!(
                "Launch script not found: {}. Falling back to Lone Wolf.",
                script_path.display()
            ),
        };
    }

    info!(
        "🏨 Hotel check-in: launching {} via {}",
        guest.name,
        script_path.display()
    );

    // Determine launch command based on script type
    let launch_result = if guest.launch_script.ends_with(".py") {
        // Python sidecar (Janus Pro)
        tokio::process::Command::new("python3")
            .arg(&script_path)
            .current_dir(&workspace)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
    } else {
        // Bash script with --bg flag for background launch
        tokio::process::Command::new("bash")
            .arg(&script_path)
            .arg("--bg")
            .current_dir(&workspace)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
    };

    if let Err(e) = launch_result {
        error!("🏨 Hotel: Failed to spawn {}: {}", guest.name, e);
        *occupant_lock().lock().await = None;
        return HotelSwapResult {
            requested: target,
            success: false,
            url: None,
            duration: start.elapsed(),
            message: format!("Failed to spawn {}: {}", guest.name, e),
        };
    }

    // ── Phase 4: Wait for health ──
    info!(
        "🏨 Hotel: Waiting for {} health on port {}...",
        guest.name, guest.port
    );
    let healthy = wait_for_health(&target_url, Duration::from_secs(90)).await;

    if healthy {
        info!(
            "🏨 Hotel: ✅ {} is ONLINE (took {:.1}s)",
            guest.name,
            start.elapsed().as_secs_f64()
        );
        *occupant_lock().lock().await = Some(target);
        HotelSwapResult {
            requested: target,
            success: true,
            url: Some(target_url),
            duration: start.elapsed(),
            message: format!("{} loaded successfully", guest.name),
        }
    } else {
        warn!(
            "🏨 Hotel: ⚠️ {} failed health check after {:.1}s — Lone Wolf mode",
            guest.name,
            start.elapsed().as_secs_f64()
        );
        *occupant_lock().lock().await = None;
        HotelSwapResult {
            requested: target,
            success: false,
            url: None,
            duration: start.elapsed(),
            message: format!(
                "{} failed to come online. Tempo handles this phase.",
                guest.name
            ),
        }
    }
}

/// Evict the current Hotel occupant without loading a new one.
/// Used when transitioning to Tempo-only phases (Analysis, Implementation, etc.)
pub async fn hotel_checkout() {
    let mut occupant = occupant_lock().lock().await;
    if let Some(role) = *occupant {
        info!("🏨 Hotel checkout: evicting {} to free VRAM", role);
        for guest in HOTEL_GUESTS {
            kill_port(guest.port).await;
        }
        *occupant = None;
    }
}

/// Get the current Hotel occupant role, if any
pub async fn current_occupant() -> Option<PartyRole> {
    *occupant_lock().lock().await
}

// ─── Internal helpers ──────────────────────────────────────────────────────────

/// Kill whatever process is listening on a given port
async fn kill_port(port: u16) {
    let result = tokio::process::Command::new("bash")
        .arg("-c")
        .arg(format!(
            "lsof -ti:{} 2>/dev/null | xargs -r kill -9 2>/dev/null",
            port
        ))
        .output()
        .await;

    match result {
        Ok(output) => {
            if output.status.success() {
                let pids = String::from_utf8_lossy(&output.stdout);
                if !pids.trim().is_empty() {
                    info!("🏨 Killed process on port {}", port);
                }
            }
        }
        Err(e) => {
            warn!("🏨 Failed to kill port {}: {}", port, e);
        }
    }
}

/// Check if a URL responds with HTTP 200
async fn check_health(url: &str) -> bool {
    crate::http::QUICK
        .get(format!("{}/health", url))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

/// Wait for a URL to become healthy, polling every 2 seconds
async fn wait_for_health(base_url: &str, timeout: Duration) -> bool {
    let start = std::time::Instant::now();
    let health_url = format!("{}/health", base_url);

    while start.elapsed() < timeout {
        if crate::http::QUICK
            .get(&health_url)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
        {
            return true;
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotel_guest_config() {
        // Verify all roles have a guest config
        assert!(HOTEL_GUESTS
            .iter()
            .any(|g| g.role == PartyRole::Programming));
        assert!(HOTEL_GUESTS
            .iter()
            .any(|g| g.role == PartyRole::Reasoning));
        assert!(HOTEL_GUESTS
            .iter()
            .any(|g| g.role == PartyRole::Aesthetics));

        // Verify ports don't conflict
        let ports: Vec<u16> = HOTEL_GUESTS.iter().map(|g| g.port).collect();
        let unique_ports: std::collections::HashSet<u16> = ports.iter().copied().collect();
        assert_eq!(ports.len(), unique_ports.len(), "Hotel ports must be unique");

        // Verify no guest uses the Tempo port (8001)
        assert!(
            !ports.contains(&8001),
            "Hotel guests must not use Tempo's port 8001"
        );
    }

    #[test]
    fn test_hotel_swap_result_debug() {
        let result = HotelSwapResult {
            requested: PartyRole::Programming,
            success: true,
            url: Some("http://127.0.0.1:8000".to_string()),
            duration: Duration::from_secs(8),
            message: "P (Programming) loaded successfully".to_string(),
        };
        // Should not panic
        let _ = format!("{:?}", result);
        assert!(result.success);
    }
}
