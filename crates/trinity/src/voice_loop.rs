use crate::AppState;
use axum::{extract::State, Json};
use reqwest::StatusCode;
use tracing::info;

pub async fn start_voice_loop(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Voice loop is now handled by Python sidecar - this is a stub
    info!("Voice loop endpoint called (now handled by Python sidecar)...");

    Ok(Json(serde_json::json!({
        "status": "delegated",
        "message": "Voice loop is now handled by Python sidecar process."
    })))
}
