// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Inference Fleet Manager
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        vllm_fleet.rs
// PURPOSE:     Auto-detect and log status of inference sidecars
//
// ARCHITECTURE (LongCat-Next Era):
//   Port 8010 — LongCat-Next 74B MoE (sglang-engine, GPU, ~84GB NF4)
//               Handles: text, images (DiNA), audio (CosyVoice)
//               Launched externally via distrobox/sglang container
//
//   Port 8000 — Programmer Pete (Qwen REAP 25B A3B, CPU GGUF via llama-server)
//               Handles: coding tasks, tool calls
//               Launched externally via llama-server CLI
//
//   Port 8200 — Kokoro TTS (Apache 2.0, CPU ONNX)
//               Handles: voice synthesis, 6 preset voices
//               Launched via scripts/launch/kokoro_sidecar.py
//
// NOTE: In the LongCat-Next architecture, inference sidecars are launched
//       externally (via distrobox, llama-server, etc). This module only
//       checks their health on startup and logs their status. It does NOT
//       attempt to auto-launch GPU processes — that was the old vLLM fleet
//       approach which is no longer applicable.
//
// CHANGES:
//   2026-04-08  Cascade  Rewritten for LongCat-Next dual-agent architecture
//   2026-04-06  Cascade  Original vLLM fleet auto-launcher (legacy, removed)
//
// ═══════════════════════════════════════════════════════════════════════════════

use tracing::info;

const LONGCAT_PORT: u16 = 8010;
const PETE_PORT: u16 = 8000;
const KOKORO_PORT: u16 = 8200;

/// Check status of all inference sidecars on startup.
/// Does NOT auto-launch — sidecars are managed externally.
pub async fn start_fleet() {
    info!("🔍 Inference Fleet: checking sidecar status...");

    // 1. LongCat-Next Omni-Brain (primary)
    let longcat_url = format!("http://127.0.0.1:{}/v1/models", LONGCAT_PORT);
    if crate::inference::check_health(&longcat_url).await {
        info!("✅ LongCat-Next Omni-Brain running on :{}", LONGCAT_PORT);
    } else {
        info!(
            "⬚  LongCat-Next not detected on :{}. Launch via: distrobox enter vllm -- ...",
            LONGCAT_PORT
        );
    }

    // 2. Programmer Pete (coding subagent)
    let pete_url = format!("http://127.0.0.1:{}/v1/models", PETE_PORT);
    if crate::inference::check_health(&pete_url).await {
        info!("✅ Programmer Pete (Qwen REAP) running on :{}", PETE_PORT);
    } else {
        info!(
            "⬚  Programmer Pete not detected on :{}. Launch via: llama-server -m <gguf> --port {}",
            PETE_PORT, PETE_PORT
        );
    }

    // 3. Kokoro TTS
    let kokoro_url = format!("http://127.0.0.1:{}/health", KOKORO_PORT);
    if crate::http::check_health(&kokoro_url).await {
        info!("✅ Kokoro TTS running on :{}", KOKORO_PORT);
    } else {
        info!(
            "⬚  Kokoro TTS not detected on :{}. Launch via: python scripts/launch/kokoro_sidecar.py",
            KOKORO_PORT
        );
    }
}
