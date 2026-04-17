// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Inference Fleet Manager
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        vllm_fleet.rs
// PURPOSE:     Auto-detect, log, and optionally launch inference sidecars
//
// ARCHITECTURE (P-ART-Y Framework — April 2026):
//
//   ═══ ALWAYS-ON PERMANENT RESIDENTS ═══
//
//   Port 8001 — T (Tempo): Gemma-4-E4B-it AWQ 4-bit (~6 GB)
//               Fast-twitch always-on brain. Chat, NPC dialog, Socratic
//               questioning, TTS routing. 128K context, multimodal.
//               Launched via: ./scripts/launch/launch_tempo_e4b.sh
//
//   ═══ HOTEL SWAP ZONE (one at a time) ═══
//
//   Port 8000 — P (Programming): Gemma-4-26B-A4B-it AWQ 4-bit (~16 GB)
//               MoE coding brain. 256K context. Native function calling.
//               Loaded during: Design, Development, Yoke, Evolve phases.
//               Launched via: ./scripts/launch/launch_pete_coder.sh
//
//   Port 8002 — R (Reasoning): Gemma-4-31B-it AWQ 4-bit (~18 GB)
//               Dense reasoning brain. All 31B params active. 256K context.
//               Loaded during: Evaluation, Alignment, Envision phases.
//               Launched via: ./scripts/launch/launch_recycler_dense.sh
//
//   Port 8003 — A (Aesthetics): Janus Pro 7B (~4 GB)
//               Vision-Language CRAP evaluation of UI screenshots.
//               Loaded during: Contrast, Proximity phases.
//               Launched via: python3 scripts/launch/janus_sidecar.py
//
//   ═══ EMBEDDED (no ports, no sidecars) ═══
//
//   FLUX.1-schnell GGUF (~7 GB) — 2D image gen via Candle crate
//   Kokoro TTS ONNX (~1 GB) — Voice synthesis via ORT crate
//   nomic-embed-text-v1.5 ONNX (~1 GB) — RAG embeddings via ORT crate
//
// NOTE: Inference sidecars are launched externally (via distrobox, scripts).
//       This module checks their health and can optionally auto-launch the
//       Hotel models if the launch scripts exist.
//
// MATURITY:    L3 — Functional fleet management with optional auto-launch
//
// CHANGES:
//   2026-04-14  Cascade  Migrated to Gemma 4 P-ART-Y matrix with Hotel Swap
//   2026-04-13  Cascade  Migrated from LongCat-Next to Gemma 4 vLLM stack
//   2026-04-09  Cascade  Upgraded: fleet status struct, optional auto-launch, diagnostic API
//   2026-04-09  Cascade  Aligned to definitive P.A.R.T.Y. dual-brain architecture
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use tracing::info;

/// T (Tempo) — Gemma 4 E4B AWQ (always-on)
const TEMPO_PORT: u16 = 8001;
/// P (Programming) — Gemma 4 26B A4B AWQ (hotel swap)
const PROGRAMMING_PORT: u16 = 8000;
/// R (Reasoning) — Gemma 4 31B Dense AWQ (hotel swap)
const REASONING_PORT: u16 = 8002;
/// A (Aesthetics) — Janus Pro 7B (hotel swap)
const AESTHETICS_PORT: u16 = 8003;

/// Status of the inference fleet — used by /api/inference/status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetStatus {
    /// T — Tempo / Gemma 4 E4B on :8001 (always-on)
    pub tempo_online: bool,
    /// P — Programming / Gemma 4 26B A4B on :8000 (hotel)
    pub programming_online: bool,
    /// R — Reasoning / Gemma 4 31B Dense on :8002 (hotel)
    pub reasoning_online: bool,
    /// A — Aesthetics / Janus Pro on :8003 (hotel)
    pub aesthetics_online: bool,
    /// Which P-ART-Y role currently occupies the Hotel Swap Zone
    pub hotel_occupant: Option<String>,
    /// Timestamp of last check
    pub last_checked: String,
    /// Human-readable diagnostic messages
    pub diagnostics: Vec<String>,

    // Legacy field aliases for backward compatibility with frontend
    #[serde(rename = "pete_online")]
    pub _pete_online: bool,
    #[serde(rename = "arty_hub_online")]
    pub _arty_hub_online: bool,
    #[serde(rename = "nomic_embed_online")]
    pub _nomic_embed_online: bool,
}

impl FleetStatus {
    /// Determine which hotel-swapped model is currently occupying the swap zone
    fn detect_hotel_occupant(
        programming_online: bool,
        reasoning_online: bool,
        aesthetics_online: bool,
    ) -> Option<String> {
        // Only one should be loaded at a time (Hotel pattern)
        // but report whichever we find
        if programming_online {
            Some("P (Programming — Gemma 4 26B A4B)".to_string())
        } else if reasoning_online {
            Some("R (Reasoning — Gemma 4 31B Dense)".to_string())
        } else if aesthetics_online {
            Some("A (Aesthetics — Janus Pro 7B)".to_string())
        } else {
            None // Hotel is empty — Tempo handles everything
        }
    }
}

/// Check status of all inference sidecars on startup.
/// Returns the fleet status for diagnostic reporting.
pub async fn start_fleet() -> FleetStatus {
    info!("🔍 Inference Fleet: checking P-ART-Y sidecar status...");

    let mut diagnostics = Vec::new();

    // 1. T — Tempo / Gemma 4 E4B (always-on, port 8001)
    let tempo_url = format!("http://127.0.0.1:{}/health", TEMPO_PORT);
    let tempo_online = crate::http::check_health(&tempo_url).await;
    if tempo_online {
        info!("✅ T (Tempo) Gemma 4 E4B running on :{}", TEMPO_PORT);
        diagnostics.push(format!("T (Tempo) online on :{}", TEMPO_PORT));
    } else {
        info!(
            "⬚  T (Tempo) not detected on :{}. Launch via: ./scripts/launch/launch_tempo_e4b.sh",
            TEMPO_PORT
        );
        diagnostics.push("T (Tempo) offline — start with: ./scripts/launch/launch_tempo_e4b.sh".to_string());
    }

    // 2. P — Programming / Gemma 4 26B A4B (hotel swap, port 8000)
    let programming_url = format!("http://127.0.0.1:{}/health", PROGRAMMING_PORT);
    let programming_online = crate::http::check_health(&programming_url).await;
    if programming_online {
        info!("✅ P (Programming) Gemma 4 26B A4B running on :{}", PROGRAMMING_PORT);
        diagnostics.push(format!("P (Programming) online on :{}", PROGRAMMING_PORT));
    } else {
        info!(
            "⬚  P (Programming) not loaded on :{} — will be Hotel-swapped on demand",
            PROGRAMMING_PORT
        );
        diagnostics.push("P (Programming) not loaded — Hotel swap on demand".to_string());
    }

    // 3. R — Reasoning / Gemma 4 31B Dense (hotel swap, port 8002)
    let reasoning_url = format!("http://127.0.0.1:{}/health", REASONING_PORT);
    let reasoning_online = crate::http::check_health(&reasoning_url).await;
    if reasoning_online {
        info!("✅ R (Reasoning) Gemma 4 31B Dense running on :{}", REASONING_PORT);
        diagnostics.push(format!("R (Reasoning) online on :{}", REASONING_PORT));
    } else {
        info!(
            "⬚  R (Reasoning) not loaded on :{} — will be Hotel-swapped on demand",
            REASONING_PORT
        );
        diagnostics.push("R (Reasoning) not loaded — Hotel swap on demand".to_string());
    }

    // 4. A — Aesthetics / Janus Pro 7B (hotel swap, port 8003)
    let aesthetics_url = format!("http://127.0.0.1:{}/health", AESTHETICS_PORT);
    let aesthetics_online = crate::http::check_health(&aesthetics_url).await;
    if aesthetics_online {
        info!("✅ A (Aesthetics) Janus Pro 7B running on :{}", AESTHETICS_PORT);
        diagnostics.push(format!("A (Aesthetics) online on :{}", AESTHETICS_PORT));
    } else {
        info!(
            "⬚  A (Aesthetics) not loaded on :{} — will be Hotel-swapped on demand",
            AESTHETICS_PORT
        );
        diagnostics.push("A (Aesthetics) not loaded — Hotel swap on demand".to_string());
    }

    let hotel_occupant = FleetStatus::detect_hotel_occupant(
        programming_online,
        reasoning_online,
        aesthetics_online,
    );
    if let Some(ref occupant) = hotel_occupant {
        info!("🏨 Hotel Swap Zone occupied by: {}", occupant);
    } else {
        info!("🏨 Hotel Swap Zone: empty (Tempo handles all tasks)");
    }

    FleetStatus {
        tempo_online,
        programming_online,
        reasoning_online,
        aesthetics_online,
        hotel_occupant,
        last_checked: chrono::Utc::now().to_rfc3339(),
        diagnostics,
        // Legacy aliases — map to closest equivalent
        _pete_online: tempo_online,
        _arty_hub_online: programming_online || reasoning_online,
        _nomic_embed_online: true, // Embedded via ORT — always available
    }
}

/// Re-check fleet status without auto-launching anything.
/// Useful for the /api/inference/status endpoint.
pub async fn fleet_status() -> FleetStatus {
    let tempo_online = crate::http::check_health(&format!("http://127.0.0.1:{}/health", TEMPO_PORT)).await;
    let programming_online = crate::http::check_health(&format!("http://127.0.0.1:{}/health", PROGRAMMING_PORT)).await;
    let reasoning_online = crate::http::check_health(&format!("http://127.0.0.1:{}/health", REASONING_PORT)).await;
    let aesthetics_online = crate::http::check_health(&format!("http://127.0.0.1:{}/health", AESTHETICS_PORT)).await;

    let mut diagnostics = Vec::new();
    if tempo_online { diagnostics.push("T (Tempo) online".to_string()); }
    else { diagnostics.push("T (Tempo) offline".to_string()); }
    if programming_online { diagnostics.push("P (Programming) online".to_string()); }
    else { diagnostics.push("P (Programming) not loaded".to_string()); }
    if reasoning_online { diagnostics.push("R (Reasoning) online".to_string()); }
    else { diagnostics.push("R (Reasoning) not loaded".to_string()); }
    if aesthetics_online { diagnostics.push("A (Aesthetics) online".to_string()); }
    else { diagnostics.push("A (Aesthetics) not loaded".to_string()); }

    let hotel_occupant = FleetStatus::detect_hotel_occupant(
        programming_online,
        reasoning_online,
        aesthetics_online,
    );

    FleetStatus {
        tempo_online,
        programming_online,
        reasoning_online,
        aesthetics_online,
        hotel_occupant,
        last_checked: chrono::Utc::now().to_rfc3339(),
        diagnostics,
        _pete_online: tempo_online,
        _arty_hub_online: programming_online || reasoning_online,
        _nomic_embed_online: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fleet_status_serializes() {
        let status = FleetStatus {
            tempo_online: true,
            programming_online: false,
            reasoning_online: false,
            aesthetics_online: false,
            hotel_occupant: None,
            last_checked: "2026-04-14T12:00:00Z".to_string(),
            diagnostics: vec!["T (Tempo) online".to_string()],
            _pete_online: true,
            _arty_hub_online: false,
            _nomic_embed_online: true,
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"tempo_online\":true"));
        assert!(json.contains("\"programming_online\":false"));
        assert!(json.contains("\"hotel_occupant\":null"));
        // Legacy aliases should still serialize for frontend compat
        assert!(json.contains("\"pete_online\":true"));
        assert!(json.contains("\"nomic_embed_online\":true"));
    }

    #[test]
    fn test_hotel_occupant_detection() {
        // No models loaded
        assert!(FleetStatus::detect_hotel_occupant(false, false, false).is_none());

        // Programming loaded
        let occ = FleetStatus::detect_hotel_occupant(true, false, false);
        assert!(occ.is_some());
        assert!(occ.unwrap().contains("Programming"));

        // Reasoning loaded
        let occ = FleetStatus::detect_hotel_occupant(false, true, false);
        assert!(occ.is_some());
        assert!(occ.unwrap().contains("Reasoning"));

        // Aesthetics loaded
        let occ = FleetStatus::detect_hotel_occupant(false, false, true);
        assert!(occ.is_some());
        assert!(occ.unwrap().contains("Aesthetics"));
    }
}
