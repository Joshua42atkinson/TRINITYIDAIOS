// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Inference Fleet Manager
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        vllm_fleet.rs
// PURPOSE:     Auto-detect, log, and optionally launch inference sidecars
//
// ARCHITECTURE (P.A.R.T.Y. Framework — April 2026):
//
//   Port 8010 — Pete / LongCat-Next 74B MoE (sglang-engine distrobox, GPU, ~38-84GB NF4)
//               Handles: text (Socratic/LitRPG), images (DiNA), TTS (CosyVoice),
//               audio understanding, music (Acestep 1.5)
//               Launched via: distrobox enter sglang-engine -- bash ./longcat_omni_sidecar/launch_engine.sh
//
//   Port 8000 — A.R.T.Y. Hub (FastAPI reverse proxy on host)
//               Routes to downstream vLLM instances:
//     Port 8005 — R: nomic-embed-text-v1.5-AWQ (embeddings for RAG semantic search)
//     Port 8009 — Y: Yardmaster / Qwen REAP (coding subagent — optional)
//               Launched via: ./scripts/launch/launch_arty_hub.sh
//
// NOTE: Inference sidecars are launched externally (via distrobox, scripts).
//       This module checks their health and can optionally auto-launch the
//       A.R.T.Y. Hub if the launch script exists.
//
// MATURITY:    L3 — Functional fleet management with optional auto-launch (was L2)
//
// CHANGES:
//   2026-04-09  Cascade  Upgraded: fleet status struct, optional auto-launch, diagnostic API
//   2026-04-09  Cascade  Aligned to definitive P.A.R.T.Y. dual-brain architecture
//   2026-04-08  Cascade  Rewritten for LongCat-Next dual-agent architecture
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use tracing::info;

/// Pete / LongCat-Next Omni-Brain (SGLang sidecar)
const LONGCAT_PORT: u16 = 8010;
/// A.R.T.Y. Hub reverse proxy (routes to nomic-embed, Yardmaster, etc.)
const ARTY_HUB_PORT: u16 = 8000;
/// nomic-embed direct port (behind the A.R.T.Y. hub)
const NOMIC_EMBED_PORT: u16 = 8005;

/// Status of the inference fleet — used by /api/inference/status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetStatus {
    /// Pete / LongCat-Next on :8010
    pub longcat_online: bool,
    /// A.R.T.Y. Hub on :8000
    pub arty_hub_online: bool,
    /// nomic-embed on :8005 (behind A.R.T.Y.)
    pub nomic_embed_online: bool,
    /// Timestamp of last check
    pub last_checked: String,
    /// Human-readable diagnostic messages
    pub diagnostics: Vec<String>,
}

/// Check status of all inference sidecars on startup.
/// Returns the fleet status for diagnostic reporting.
pub async fn start_fleet() -> FleetStatus {
    info!("🔍 Inference Fleet: checking P.A.R.T.Y. sidecar status...");

    let mut diagnostics = Vec::new();

    // 1. LongCat-Next Omni-Brain (Pete — primary brain)
    let longcat_url = format!("http://127.0.0.1:{}/health", LONGCAT_PORT);
    let longcat_online = crate::http::check_health(&longcat_url).await;
    if longcat_online {
        info!("✅ Pete / LongCat-Next Omni-Brain running on :{}", LONGCAT_PORT);
        diagnostics.push(format!("Pete online on :{}", LONGCAT_PORT));
    } else {
        info!(
            "⬚  LongCat-Next not detected on :{}. Launch via: distrobox enter sglang-engine -- bash ./longcat_omni_sidecar/launch_engine.sh",
            LONGCAT_PORT
        );
        diagnostics.push(format!("Pete offline — start with: distrobox enter sglang-engine -- bash ./longcat_omni_sidecar/launch_engine.sh"));
    }

    // 2. A.R.T.Y. Hub — try auto-launch if script exists and hub is down
    let arty_url = format!("http://127.0.0.1:{}/health", ARTY_HUB_PORT);
    let mut arty_hub_online = crate::http::check_health(&arty_url).await;
    if arty_hub_online {
        info!("✅ A.R.T.Y. Hub running on :{}", ARTY_HUB_PORT);
        diagnostics.push(format!("A.R.T.Y. Hub online on :{}", ARTY_HUB_PORT));
    } else {
        // Attempt auto-launch if script exists
        let launch_script = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("scripts/launch/launch_arty_hub.sh");

        if launch_script.exists() {
            info!("🚀 A.R.T.Y. Hub not running — attempting auto-launch from {}...", launch_script.display());
            match std::process::Command::new("bash")
                .arg(&launch_script)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
            {
                Ok(_) => {
                    info!("   A.R.T.Y. Hub auto-launch initiated (port {})", ARTY_HUB_PORT);
                    diagnostics.push("A.R.T.Y. Hub auto-launch initiated".to_string());

                    // Wait briefly for startup
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                    arty_hub_online = crate::http::check_health(&arty_url).await;
                    if arty_hub_online {
                        info!("   ✅ A.R.T.Y. Hub came online after auto-launch!");
                        diagnostics.push("A.R.T.Y. Hub auto-launched successfully".to_string());
                    }
                }
                Err(e) => {
                    info!("   ⚠️ Failed to auto-launch A.R.T.Y. Hub: {}", e);
                    diagnostics.push(format!("A.R.T.Y. auto-launch failed: {}", e));
                }
            }
        }

        if !arty_hub_online {
            info!(
                "⬚  A.R.T.Y. Hub not detected on :{}. Launch via: ./scripts/launch/launch_arty_hub.sh",
                ARTY_HUB_PORT
            );
            diagnostics.push(format!("A.R.T.Y. Hub offline — RAG uses text-only fallback"));
        }
    }

    // 3. nomic-embed (direct check — behind A.R.T.Y. hub)
    let nomic_url = format!("http://127.0.0.1:{}/health", NOMIC_EMBED_PORT);
    let nomic_embed_online = crate::http::check_health(&nomic_url).await;
    if nomic_embed_online {
        info!("  ✅ R (Research): nomic-embed running on :{}", NOMIC_EMBED_PORT);
        diagnostics.push(format!("nomic-embed online on :{}", NOMIC_EMBED_PORT));
    } else {
        info!(
            "  ⬚  R (Research): nomic-embed not detected on :{} — RAG will use text-only fallback",
            NOMIC_EMBED_PORT
        );
        diagnostics.push("nomic-embed offline — semantic search degraded".to_string());
    }

    FleetStatus {
        longcat_online,
        arty_hub_online,
        nomic_embed_online,
        last_checked: chrono::Utc::now().to_rfc3339(),
        diagnostics,
    }
}

/// Re-check fleet status without auto-launching anything.
/// Useful for the /api/inference/status endpoint.
pub async fn fleet_status() -> FleetStatus {
    let longcat_online = crate::http::check_health(&format!("http://127.0.0.1:{}/health", LONGCAT_PORT)).await;
    let arty_hub_online = crate::http::check_health(&format!("http://127.0.0.1:{}/health", ARTY_HUB_PORT)).await;
    let nomic_embed_online = crate::http::check_health(&format!("http://127.0.0.1:{}/health", NOMIC_EMBED_PORT)).await;

    let mut diagnostics = Vec::new();
    if longcat_online { diagnostics.push("Pete online".to_string()); }
    else { diagnostics.push("Pete offline".to_string()); }
    if arty_hub_online { diagnostics.push("A.R.T.Y. Hub online".to_string()); }
    else { diagnostics.push("A.R.T.Y. Hub offline".to_string()); }
    if nomic_embed_online { diagnostics.push("nomic-embed online".to_string()); }
    else { diagnostics.push("nomic-embed offline".to_string()); }

    FleetStatus {
        longcat_online,
        arty_hub_online,
        nomic_embed_online,
        last_checked: chrono::Utc::now().to_rfc3339(),
        diagnostics,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fleet_status_serializes() {
        let status = FleetStatus {
            longcat_online: true,
            arty_hub_online: false,
            nomic_embed_online: false,
            last_checked: "2026-04-09T12:00:00Z".to_string(),
            diagnostics: vec!["Pete online".to_string()],
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"longcat_online\":true"));
        assert!(json.contains("\"arty_hub_online\":false"));
    }
}
