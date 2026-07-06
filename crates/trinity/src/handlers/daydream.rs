use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct DaydreamCommandRequest {
    pub command: String,
    pub params: serde_json::Value,
}

pub async fn post_daydream_command(
    State(state): State<AppState>,
    Json(payload): Json<DaydreamCommandRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if let Some(tx) = &state.daydream_tx {
        let msg = serde_json::json!({
            "command": payload.command,
            "payload": payload.params
        }).to_string();
        
        if tx.send(msg).await.is_ok() {
            return Ok(Json(serde_json::json!({"success": true})));
        }
    }
    tracing::error!("Daydream TX channel is disconnected or missing.");
    Err(StatusCode::SERVICE_UNAVAILABLE)
}
