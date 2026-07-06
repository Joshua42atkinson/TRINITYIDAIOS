// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        lm_studio_client.rs
// PURPOSE:     Proxy client for LM Studio REST API — model load/unload/download
//
// ARCHITECTURE:
//   • LM Studio runs on :1234 with an OpenAI-compatible API
//   • This client proxies LM Studio's native model management endpoints
//   • Trinity routes chat completions through inference_router.rs (active_url)
//   • This module is for model lifecycle management only
//
// LM Studio API endpoints used:
//   GET  /v1/models                    — list loaded models
//   POST /api/v1/models/load           — load a model by identifier
//   POST /api/v1/models/unload         — unload a model by identifier
//   POST /api/v1/models/download       — download a model from HuggingFace
//   GET  /api/v1/models/download/status — check download progress
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::http;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

const LM_STUDIO_URL: &str = "http://127.0.0.1:1234";

// ═══════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub object: String,
    pub owned_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelList {
    pub object: String,
    pub data: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadStatus {
    pub completed: bool,
    pub progress: f64,
    pub error: Option<String>,
}

// ═══════════════════════════════════════════════════
// API Functions
// ═══════════════════════════════════════════════════

/// List all models currently loaded in LM Studio.
/// Uses the OpenAI-compatible /v1/models endpoint.
pub async fn list_models() -> Result<ModelList, String> {
    let res = http::QUICK
        .get(&format!("{}/v1/models", LM_STUDIO_URL))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to LM Studio: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("LM Studio returned status {}", res.status()));
    }

    res.json::<ModelList>()
        .await
        .map_err(|e| format!("Failed to parse LM Studio response: {}", e))
}

/// Load a model in LM Studio by its identifier.
/// The model must already be downloaded and available in LM Studio's model directory.
pub async fn load_model(model_id: &str) -> Result<(), String> {
    info!("Loading model in LM Studio: {}", model_id);

    let res = http::LONG
        .post(&format!("{}/api/v1/models/load", LM_STUDIO_URL))
        .json(&serde_json::json!({ "model": model_id }))
        .send()
        .await
        .map_err(|e| format!("Failed to send load request: {}", e))?;

    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(format!("LM Studio load failed ({}): {}", status, body));
    }

    info!("Model loaded successfully: {}", model_id);
    Ok(())
}

/// Unload a model from LM Studio by its identifier.
pub async fn unload_model(model_id: &str) -> Result<(), String> {
    info!("Unloading model from LM Studio: {}", model_id);

    let res = http::STANDARD
        .post(&format!("{}/api/v1/models/unload", LM_STUDIO_URL))
        .json(&serde_json::json!({ "model": model_id }))
        .send()
        .await
        .map_err(|e| format!("Failed to send unload request: {}", e))?;

    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(format!("LM Studio unload failed ({}): {}", status, body));
    }

    info!("Model unloaded successfully: {}", model_id);
    Ok(())
}

/// Download a model from HuggingFace to LM Studio's model directory.
/// This is a long-running operation — check status with `download_status()`.
pub async fn download_model(hf_repo: &str) -> Result<(), String> {
    info!("Downloading model from HuggingFace: {}", hf_repo);

    let res = http::LONG
        .post(&format!("{}/api/v1/models/download", LM_STUDIO_URL))
        .json(&serde_json::json!({ "repo": hf_repo }))
        .send()
        .await
        .map_err(|e| format!("Failed to send download request: {}", e))?;

    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(format!("LM Studio download failed ({}): {}", status, body));
    }

    info!("Download started for: {}", hf_repo);
    Ok(())
}

/// Check the status of an ongoing model download.
pub async fn download_status() -> Result<DownloadStatus, String> {
    let res = http::QUICK
        .get(&format!(
            "{}/api/v1/models/download/status",
            LM_STUDIO_URL
        ))
        .send()
        .await
        .map_err(|e| format!("Failed to check download status: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("LM Studio returned status {}", res.status()));
    }

    res.json::<DownloadStatus>()
        .await
        .map_err(|e| format!("Failed to parse download status: {}", e))
}

/// Check if LM Studio is reachable and healthy.
pub async fn is_healthy() -> bool {
    http::check_health(LM_STUDIO_URL).await
        || http::QUICK
            .get(&format!("{}/v1/models", LM_STUDIO_URL))
            .timeout(std::time::Duration::from_secs(3))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
}
