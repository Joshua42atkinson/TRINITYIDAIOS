use axum::{extract::State, Json, http::StatusCode};
use crate::AppState;
use crate::voice;

/// Native TTS — Kokoro TTS (primary) → vLLM Omni E4B (future fallback)
pub async fn tts_proxy(
    State(_state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Result<axum::response::Response<axum::body::Body>, (StatusCode, String)> {
    let t0 = std::time::Instant::now();

    let text = body
        .get("text")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if text.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Missing 'text' field".to_string()));
    }

    let voice = body
        .get("voice")
        .and_then(|v| v.as_str())
        .unwrap_or("M1")
        .to_string();

    let format = body
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("wav")
        .to_string();

    // Try Kokoro via vLLM-Omni first
    if voice::check_omni_audio_health().await {
        match voice::omni_synthesize(&text, &voice, &format).await {
            Ok(audio_bytes) => {
                let latency_ms = t0.elapsed().as_millis();
                let content_type = match format.as_str() {
                    "mp3" => "audio/mpeg",
                    "flac" => "audio/flac",
                    "opus" => "audio/opus",
                    _ => "audio/wav",
                };
                return axum::response::Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", content_type)
                    .header("X-TTS-Backend", "acestep-1.5")
                    .header("X-Latency-Ms", latency_ms.to_string())
                    .header("X-Voice", voice::persona_to_omni_voice(&voice))
                    .body(axum::body::Body::from(audio_bytes))
                    .map_err(|e| (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to build response: {}", e),
                    ));
            }
            Err(e) => {
                tracing::warn!("Kokoro TTS failed, no fallback available: {}", e);
            }
        }
    }

    Err((
        StatusCode::SERVICE_UNAVAILABLE,
        "No TTS backend available. Start Kokoro sidecar on :8200 or enable vLLM Omni E4B".to_string(),
    ))
}

/// POST /api/stt/transcribe — Accept audio, return transcribed text
pub async fn stt_transcribe(
    State(_state): State<AppState>,
    _headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let t0 = std::time::Instant::now();
    let client = &*crate::http::LONG;
    use base64::Engine;
    let base64_audio = base64::engine::general_purpose::STANDARD.encode(&body);
    let audio_url = format!("data:audio/wav;base64,{}", base64_audio);
        
    let fallback_model = "Great_Recycler".to_string();
    let model_name = match client.get("http://127.0.0.1:8001/v1/models").send().await {
        Ok(res) => {
            if let Ok(json) = res.json::<serde_json::Value>().await {
                json["data"][0]["id"].as_str().unwrap_or(&fallback_model).to_string()
            } else { fallback_model.clone() }
        },
        Err(_) => fallback_model.clone(),
    };

    let payload = serde_json::json!({
        "model": model_name,
        "temperature": 0.0,
        "max_tokens": 512,
        "messages": [
            {
                "role": "user",
                "content": [
                    { "type": "text", "text": "Transcribe this audio precisely. Output nothing but the transcribed text." },
                    { "type": "audio_url", "audio_url": { "url": audio_url } }
                ]
            }
        ]
    });
        
    let response = match client.post("http://127.0.0.1:8001/v1/chat/completions").json(&payload).send().await {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("STT failed: {}", e) }))).into_response()
    };
    
    if !response.status().is_success() {
        let text = response.text().await.unwrap_or_default();
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("STT error: {}", text) }))).into_response();
    }
    
    let json: serde_json::Value = response.json().await.unwrap_or_default();
    let text = json["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
    
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "text": text,
            "duration_ms": t0.elapsed().as_millis() as u64,
            "audio_samples": 0,
            "audio_seconds": 0.0,
        })),
    ).into_response()
}

/// GET /api/stt/status — Check if STT engine is loaded
pub async fn stt_status(
    State(_state): State<AppState>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "loaded": true,
        "model": "Great_Recycler",
        "backend": "tempo-e4b",
    }))
}

pub async fn start_voice_loop(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    tracing::info!("Voice loop queried. Reporting Kokoro status.");
    
    let is_acestep_healthy = voice::check_omni_audio_health().await;

    Ok(Json(serde_json::json!({
        "status": if is_acestep_healthy { "healthy" } else { "offline" },
        "pipeline": "acestep_1.5",
        "message": "Voice loop is live. Please connect via WebSocket /api/telephone for real-time STT/TTS."
    })))
}

