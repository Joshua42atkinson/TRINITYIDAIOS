// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        voice.rs
// PURPOSE:     Voice conversation API — Walkie-Talkie + Telephone pipelines
//
// ARCHITECTURE:
//   • "Walkie-Talkie" (NOW): Whisper STT + Piper TTS via voice sidecar (:8200)
//     - STT/TTS run on NPU, leaving 100% GPU for Mistral Small 4
//     - Latency: ~2-4s round trip (speak → pause → think → respond)
//   • "Telephone" (FUTURE): PersonaPlex/Moshi audio-to-audio on GPU
//     - Zero perceived latency, natural interruption
//     - GPU contention with main brain model
//   • Two modes: DEV (production agent) and IRON ROAD (gamified roleplay)
//   • Voice sidecar handles full loop: audio→STT→Trinity chat→TTS→audio
//
// DEPENDENCIES:
//   - axum — HTTP handlers for voice endpoints
//   - serde — Voice status/request serialization
//   - tracing — Voice operation logging
//
// CHANGES:
//   2026-03-18  Cascade  Dual-mode voice (DEV + Iron Road), voice sidecar
//   2026-03-16  Cascade  Migrated to §17 comment standard
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{
    body::Body,
    extract::{Multipart, State},
    http::{header, HeaderValue, StatusCode},
    response::{Json, Response},
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::AppState;

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

/// Voice status response
#[derive(Debug, Serialize)]
pub struct VoiceStatus {
    pub sidecar_running: bool,
    pub personaplex_available: bool,
    pub npu_available: bool,
    pub active_pipeline: String,
    pub mode: String,
    pub message: String,
}

/// Voice conversation request (audio blob)
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields populated via serde deserialization
pub struct VoiceRequest {
    /// Audio data (webm/opus or wav)
    pub audio_data: Vec<u8>,
    /// Format hint
    pub format: Option<String>,
}

/// Voice conversation response
#[derive(Debug, Serialize)]
#[allow(dead_code)] // Fields serialized to JSON for frontend
pub struct VoiceResponse {
    pub success: bool,
    pub transcript: Option<String>,
    pub response_text: Option<String>,
    pub audio_data: Option<String>, // Base64
    pub latency_ms: u64,
}

// ============================================================================
// API HANDLERS
// ============================================================================

/// Check voice system status
pub async fn voice_status() -> Json<VoiceStatus> {
    let sidecar_running = check_voice_sidecar_health().await;
    let personaplex_available = check_personaplex_health().await;

    let (pipeline, message) = if personaplex_available {
        (
            "telephone",
            "PersonaPlex audio-to-audio ready (zero latency)".to_string(),
        )
    } else if sidecar_running {
        (
            "walkie-talkie",
            "Voice sidecar ready (Whisper STT + Piper TTS)".to_string(),
        )
    } else {
        (
            "offline",
            "No voice backend running. Start voice sidecar: python scripts/voice_sidecar.py"
                .to_string(),
        )
    };

    Json(VoiceStatus {
        sidecar_running,
        personaplex_available,
        npu_available: check_npu_availability(),
        active_pipeline: pipeline.to_string(),
        mode: "dev".to_string(),
        message,
    })
}

/// Process audio conversation — routes to best available pipeline
/// Accepts multipart with 'audio' field (WAV) and optional 'mode' field (dev|iron-road)
pub async fn voice_conversation(
    State(_state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Response<Body>, (StatusCode, String)> {
    let start = std::time::Instant::now();

    let mut audio_data: Option<Vec<u8>> = None;
    let mut mode = "dev".to_string();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid multipart payload: {}", e),
        )
    })? {
        let field_name = field.name().unwrap_or_default().to_string();
        if field_name == "mode" {
            if let Ok(bytes) = field.bytes().await {
                mode = String::from_utf8_lossy(&bytes).to_string();
            }
        } else if field_name == "audio" || field_name == "file" || audio_data.is_none() {
            let bytes = field.bytes().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Failed to read uploaded audio: {}", e),
                )
            })?;
            if !bytes.is_empty() {
                audio_data = Some(bytes.to_vec());
            }
        }
    }

    let audio_data = audio_data.ok_or((
        StatusCode::BAD_REQUEST,
        "Missing audio upload field".to_string(),
    ))?;

    // Try PersonaPlex first (telephone pipeline), fall back to voice sidecar (walkie-talkie)
    if check_personaplex_health().await {
        info!("🎤 Using PersonaPlex (telephone pipeline), mode={}", mode);
        let response = call_personaplex(&audio_data).await.map_err(|e| {
            error!("PersonaPlex failed: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                format!("PersonaPlex failed: {}", e),
            )
        })?;

        return build_audio_response(response, start.elapsed().as_millis() as u64);
    }

    if check_voice_sidecar_health().await {
        info!(
            "🎤 Using voice sidecar (walkie-talkie pipeline), mode={}",
            mode
        );
        let response = call_voice_sidecar(&audio_data, &mode).await.map_err(|e| {
            error!("Voice sidecar failed: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                format!("Voice sidecar failed: {}", e),
            )
        })?;

        return build_audio_response(response, start.elapsed().as_millis() as u64);
    }

    Err((
        StatusCode::SERVICE_UNAVAILABLE,
        "No voice backend available. Start voice sidecar: python scripts/voice_sidecar.py"
            .to_string(),
    ))
}

fn build_audio_response(
    response: VoiceConversationResponse,
    latency_ms: u64,
) -> Result<Response<Body>, (StatusCode, String)> {
    let mut builder = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "audio/wav");

    if let Some(transcript) = response.transcript.as_ref() {
        if let Ok(value) = HeaderValue::from_str(transcript) {
            builder = builder.header("X-Transcript", value);
        }
    }

    if let Some(response_text) = response.response_text.as_ref() {
        if let Ok(value) = HeaderValue::from_str(response_text) {
            builder = builder.header("X-Response", value);
        }
    }

    builder = builder.header("X-Latency-Ms", latency_ms.to_string());

    info!("🎤 Voice response completed in {}ms", latency_ms);

    builder.body(Body::from(response.audio_data)).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to build response: {}", e),
        )
    })
}

/// Simple text-to-text Pete conversation (fallback when audio unavailable)
pub async fn pete_text(
    State(state): State<AppState>,
    Json(request): Json<TextRequest>,
) -> Result<Json<TextResponse>, (StatusCode, String)> {
    let start = std::time::Instant::now();

    info!("💬 Pete text conversation: {}", request.message);

    // For now, route through standard chat with Pete persona
    // TODO: Connect to actual PersonaPlex when NPU is ready

    let system_prompt = r#"You are Pete, a Socratic AI companion. Your role is to guide discovery through thoughtful questions, not direct answers.

PERSONALITY:
- Warm but intellectually rigorous
- Uses analogies and thought experiments
- Celebrates mistakes as learning opportunities
- Adapts to the learner's pace

PEDAGOGY:
- Never give direct answers
- Ask clarifying questions first
- Use the Socratic method: question → hypothesis → test → refine
- Connect concepts to practical applications

VOICE STYLE (for text-to-speech):
- Conversational, not academic
- Use pauses for emphasis
- Occasional humor, but never at learner's expense"#;

    // Build messages for chat completion
    let messages = vec![
        crate::ChatMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
            timestamp: None,
            image_base64: None,
        },
        crate::ChatMessage {
            role: "user".to_string(),
            content: request.message.clone(),
            timestamp: None,
            image_base64: None,
        },
    ];

    // Call the local LLM
    let url = state.inference_router.read().await.active_url().to_string();
    let response = crate::inference::chat_completion(&url, &messages, 512)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let latency = start.elapsed().as_millis() as u64;

    Ok(Json(TextResponse {
        success: true,
        response,
        latency_ms: latency,
    }))
}

// ============================================================================
// HELPERS
// ============================================================================

/// Check if voice sidecar (Whisper+Piper) is healthy
pub async fn check_voice_sidecar_health() -> bool {
    crate::http::check_health("http://127.0.0.1:8200").await
}

/// Check if PersonaPlex service is healthy (future telephone pipeline)
async fn check_personaplex_health() -> bool {
    crate::http::check_health("http://127.0.0.1:8190").await
}

/// Check if NPU is available for PersonaPlex
pub fn check_npu_availability() -> bool {
    // Check for XDNA device
    if std::path::Path::new("/dev/xdna").exists() {
        return true;
    }

    // Check for NPU in lspci
    if let Ok(output) = std::process::Command::new("lspci").arg("-v").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("NPU") || output_str.contains("XDNA") {
            return true;
        }
    }

    // Check for models (fallback)
    let models_path = dirs::home_dir()
        .unwrap_or_default()
        .join("models/personaplex");

    models_path.join("lm_backbone.onnx").exists()
}

/// Unified response from any voice pipeline
struct VoiceConversationResponse {
    audio_data: Vec<u8>,
    transcript: Option<String>,
    response_text: Option<String>,
}

/// Call voice sidecar (Whisper STT + Trinity chat + Piper TTS)
async fn call_voice_sidecar(
    audio_data: &[u8],
    mode: &str,
) -> anyhow::Result<VoiceConversationResponse> {
    let response = crate::http::STANDARD
        .post("http://127.0.0.1:8200/conversation")
        .header("Content-Type", "audio/wav")
        .header("X-Mode", mode)
        .body(audio_data.to_vec())
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Voice sidecar returned {}: {}", status, body);
    }

    let transcript = response
        .headers()
        .get("X-Transcript")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let response_text = response
        .headers()
        .get("X-Response")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let audio_response = response.bytes().await?;

    Ok(VoiceConversationResponse {
        audio_data: audio_response.to_vec(),
        transcript,
        response_text,
    })
}

/// Call PersonaPlex API for audio-to-audio conversation (future telephone pipeline)
async fn call_personaplex(audio_data: &[u8]) -> anyhow::Result<VoiceConversationResponse> {
    let response = crate::http::STANDARD
        .post("http://127.0.0.1:8190/converse")
        .header("Content-Type", "audio/webm")
        .body(audio_data.to_vec())
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("PersonaPlex returned {}", response.status());
    }

    let transcript = response
        .headers()
        .get("X-Transcript")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let response_text = response
        .headers()
        .get("X-Response")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let audio_response = response.bytes().await?;

    Ok(VoiceConversationResponse {
        audio_data: audio_response.to_vec(),
        transcript,
        response_text,
    })
}

// ============================================================================
// REQUEST/RESPONSE FOR TEXT
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct TextRequest {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TextResponse {
    pub success: bool,
    pub response: String,
    pub latency_ms: u64,
}
