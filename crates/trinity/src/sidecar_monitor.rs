// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        sidecar_monitor.rs
// PURPOSE:     Sidecar health monitor — checks real sidecars, not phantom ports
//
// 🪟 THE LIVING CODE TEXTBOOK (P-ART-Y Infrastructure):
// This file is the immune system of the OS. It is designed to be read, modified, 
// and authored by YOU. It monitors the external AI workers (ComfyUI, Voice, etc.)
// and reports real, hard crashes directly to the Cow Catcher system.
// ACTION: Edit `targets: Vec<SidecarTarget>` to add your own external AI models.
//
// 📖 THE HOOK BOOK CONNECTION:
// This file bridges the '30 Agentic Tools' Hook and the external Python ecosystems.
// By mastering this pattern, you can connect your Rust app to any external AI.
// For a full catalogue of system capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
//
// ARCHITECTURE:
//   • Monitors the P.A.R.T.Y. sidecars: LongCat-Next (:8010), A.R.T.Y. Hub (:8000)
//   • Only reports to CowCatcher if a sidecar was previously healthy and
//     then went down — avoids false obstacles from optional/uninstalled services
//   • The LLM backend is handled by the InferenceRouter's own health loop
//
// CHANGES:
//   2026-03-22  Cascade  Fixed phantom port pinging (8090-8092 → real sidecars)
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::cow_catcher::CowCatcher;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// A sidecar we monitor — `required=false` means it's optional (no obstacle if down)
struct SidecarTarget {
    name: &'static str,
    url: &'static str,
    /// Has this sidecar ever responded healthy during this server's lifetime?
    was_healthy: bool,
}

pub async fn monitor_sidecars(cow_catcher: Arc<RwLock<CowCatcher>>) {
    info!("Starting Sidecar Health Monitor — real targets only");

    // The sidecars we actually run. These complement the InferenceRouter's /v1 health checks.
    // P.A.R.T.Y. Architecture (April 2026):
    //   - longcat-omni (8010): Pete / Great Recycler — SGLang sidecar
    //   - vllm-arty (8000): A.R.T.Y. Hub — vLLM reverse proxy for Aesthetics, Research, Tempo, Yardmaster
    let mut targets: Vec<SidecarTarget> = vec![
        SidecarTarget {
            name: "longcat-omni",
            url: "http://127.0.0.1:8010/health",
            was_healthy: false,
        },
        SidecarTarget {
            name: "vllm-arty-hub",
            url: "http://127.0.0.1:8000/health",
            was_healthy: false,
        },
    ];

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        let client = &*crate::http::QUICK;

        for target in targets.iter_mut() {
            let healthy = client
                .get(target.url)
                .send()
                .await
                .map(|r| r.status().is_success())
                .unwrap_or(false);

            if healthy {
                if !target.was_healthy {
                    info!("✅ Sidecar {} is now healthy", target.name);
                }
                target.was_healthy = true;
            } else if target.was_healthy {
                // Was healthy, now down — this is a real crash, report it
                warn!(
                    "⚠️ Sidecar {} went down (was previously healthy)",
                    target.name
                );
                let mut cc = cow_catcher.write().await;
                cc.report_sidecar_crash(target.name, None, "Sidecar went down after being healthy");
                // Don't keep re-reporting — reset until it comes back
                target.was_healthy = false;
            }
            // If was_healthy == false and still not healthy, that's fine — it's just not running.
            // No obstacle generated. Optional sidecars shouldn't pollute the CowCatcher.
        }
    }
}
