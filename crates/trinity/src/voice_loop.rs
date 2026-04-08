// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Voice Loop
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        voice_loop.rs
// BIBLE CAR:   Car 11 — YOKE (ART Pipeline & Creative Tools)
// HOOK SCHOOL: 🎨 Creation
// PURPOSE:     Status endpoint for the voice pipeline (Kokoro TTS :8200)
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::AppState;
use axum::{extract::State, Json};
use reqwest::StatusCode;
use tracing::info;

pub async fn start_voice_loop(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    info!("Voice loop queried. Reporting Kokoro status.");
    
    let is_acestep_healthy = crate::voice::check_omni_audio_health().await;

    Ok(Json(serde_json::json!({
        "status": if is_acestep_healthy { "healthy" } else { "offline" },
        "pipeline": "acestep_1.5",
        "message": "Voice loop is live. Please connect via WebSocket /api/telephone for real-time STT/TTS."
    })))
}
