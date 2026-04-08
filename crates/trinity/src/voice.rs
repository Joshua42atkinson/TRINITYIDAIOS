// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        voice.rs
// PURPOSE:     Voice conversation API — Kokoro TTS + Walkie-Talkie + Telephone
//
// 🪟 THE LIVING CODE TEXTBOOK (P-ART-Y Gear T: Tempo):
// This file is the vocal cords of the OS. It is designed to be read, modified, 
// and authored by YOU. If you want to change Pete's voice or integrate a new TTS 
// engine, this is the file to edit.
// ACTION: Edit `persona_to_omni_voice()` to build custom emotional voices.
//
// 📖 THE HOOK BOOK CONNECTION:
// This file powers the 'Voice Narration' Hook. By mastering this file, you can 
// build your own local-first, low-latency Audio and Telephone interfaces. 
// For a full catalogue of system capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
//
// MATURITY:     L2 → L3 (Kokoro wired, needs model download)
// QUEST_PHASE:  supports all ADDIECRAPEYE phases (narration)
//
// ARCHITECTURE:
//   • "Kokoro" (PRIMARY): Kokoro TTS (Apache 2.0) via FastAPI sidecar
//     - 6 preset voices, American English, low-latency
//     - POST /tts endpoint returning WAV audio
//   • "Walkie-Talkie" (FALLBACK): Whisper STT + Piper TTS via voice sidecar (:8200)
//     - STT/TTS run on NPU, leaving 100% GPU for LongCat-Next 74B MoE
//   • "Kokoro" (ALWAYS-ON): Native ONNX TTS (66M params, CPU-capable)
//   • "Telephone" (FUTURE): PersonaPlex/Moshi audio-to-audio on GPU
//   • Two modes: DEV (production agent) and IRON ROAD (gamified roleplay)
//
// DEPENDENCIES:
//   - axum — HTTP handlers for voice endpoints
//   - serde — Voice status/request serialization
//   - tracing — Voice operation logging
//
// CHANGES:
//   2026-03-26  Cascade  Kokoro TTS integration (vLLM-Omni primary)
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
use trinity_protocol::character_sheet::VoiceEmotion;

use crate::AppState;

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

/// Voice status response
#[derive(Debug, Serialize)]
pub struct VoiceStatus {
    pub sidecar_running: bool,
    pub personaplex_available: bool,
    pub omni_available: bool,
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
    let omni_available = check_omni_audio_health().await;

    let (pipeline, message) = if omni_available {
        (
            "acestep-1.5",
            "Acestep 1.5 native audio generation via LongCat SGLang".to_string(),
        )
    } else if personaplex_available {
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
            "acestep-native",
            "Acestep 1.5 native (always-on fallback)".to_string(),
        )
    };

    Json(VoiceStatus {
        sidecar_running,
        personaplex_available,
        omni_available,
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

// ============================================================================
// KOKORO TTS (Apache 2.0) — Narrator Voice Acting Engine
// ============================================================================
//
// The voice acting system supports:
//   1. Persona voices     — Pete (am_adam), Recycler (am_fenrir), NPC (am_echo)
//   2. Emotion detection  — infer happy/sarcastic/contemplative from text
//   3. Narrator mode      — Great Recycler can break character (DM mode)
//   4. Voice toggle       — Recycler narration can be on/off per user pref
//
// Kokoro voices: af_heart, af_bella, am_adam, am_echo, am_michael, am_fenrir
// License: Apache 2.0 — stress-free for everyone.

// ─── Kokoro TTS Acting Subsystem ─────────────────────────────────────────────
// Built and ready — activates when kokoro_sidecar.py goes live on :8200.
// Suppress dead_code warnings for the entire subsystem until then.

/// LongCat proxy port — CosyVoice TTS (future, currently returns mock audio)
const LONGCAT_PORT: u16 = 8010;
/// Kokoro TTS sidecar port — PRIMARY working TTS (Apache 2.0)
const KOKORO_PORT: u16 = 8200;

/// Voice acting emotion — detected from text content
// VoiceEmotion and detect_emotion have been relocated to trinity_protocol::character_sheet
// VoiceEmotion and detect_emotion have been relocated to trinity_protocol::character_sheet
// granting the Omni NPC (Gemma-4) dynamic overriding control over the narrator's emotional state natively through the Vibe Tool ecosystem.

/// Narrator mode — the Great Recycler can DM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)] // Activates with Kokoro
pub enum NarratorMode {
    /// Story Mode — in-character narration (LitRPG audiobook voice)
    InCharacter,
    /// DM Mode — out-of-character design recommendations
    /// "Pausing story mode: Here's a direct design recommendation..."
    OutOfCharacter,
    /// Silent — narrator text appears but no voice (reading mode)
    Silent,
}

/// Detect narrator mode from text markers.
/// The Great Recycler can embed `[DM]` or `[OOC]` tags to break character.
#[allow(dead_code)] // Activates with Kokoro
pub fn detect_narrator_mode(text: &str) -> (NarratorMode, String) {
    // Check for DM/OOC tags
    if text.starts_with("[DM]") || text.starts_with("[OOC]") {
        let clean = text
            .trim_start_matches("[DM]")
            .trim_start_matches("[OOC]")
            .trim()
            .to_string();
        return (NarratorMode::OutOfCharacter, clean);
    }

    // Check for silent tag
    if text.starts_with("[SILENT]") || text.starts_with("[MUTE]") {
        let clean = text
            .trim_start_matches("[SILENT]")
            .trim_start_matches("[MUTE]")
            .trim()
            .to_string();
        return (NarratorMode::Silent, clean);
    }

    (NarratorMode::InCharacter, text.to_string())
}

/// Generate a DM voice cue prefix for out-of-character narration
#[allow(dead_code)] // Activates with Kokoro
fn dm_voice_cue() -> &'static str {
    "Pausing story mode. "
}

/// Check if the Acestep 1.5 LongCat backend is available
pub async fn check_omni_audio_health() -> bool {
    // Primary: LongCat Acestep 1.5 on port 8010
    crate::http::QUICK
        .get(format!("http://127.0.0.1:{}/health", LONGCAT_PORT))
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

/// Map Trinity persona names to Kokoro preset voices
/// Kokoro voices: af_heart, af_bella, am_adam, am_echo, am_michael, am_fenrir
pub fn persona_to_omni_voice(persona: &str) -> String {
    let lower = persona.to_lowercase();
    
    // Allow pass-through for custom cloned voices
    if lower == "joshua" || lower.starts_with("clone_") {
        return lower;
    }

    match lower.as_str() {
        // Pete — warm, confident, mentor
        "pete" | "conductor" | "m1" | "causal_male" => "am_adam".to_string(),
        // Great Recycler — authoritative narrator (DM voice) using Custom User Clone
        "recycler" | "narrator" | "alloy" | "dm" => "joshua".to_string(),
        // NPCs — varied voices
        "npc" | "default" | "echo" => "am_echo".to_string(),
        // Youser feedback — encouraging, warm
        "youser" | "student" | "nova" => "af_heart".to_string(),
        // Female voices
        "f1" | "f2" | "f3" | "shimmer" => "af_bella".to_string(),
        "fable" | "onyx" => "am_michael".to_string(),
        // Fallback
        _ => "am_adam".to_string(),
    }
}

/// Full narrated synthesis — persona + emotion + narrator mode
/// This is the main entry point for voice-acted TTS.
#[allow(dead_code)] // Activates with LongCat CosyVoice
pub async fn omni_synthesize_narrated(
    text: &str,
    persona: &str,
    format: &str,
    emotion: VoiceEmotion,
) -> anyhow::Result<VoiceActResult> {
    let (narrator_mode, clean_text) = detect_narrator_mode(text);

    // Silent mode — return text only, no audio
    if narrator_mode == NarratorMode::Silent {
        return Ok(VoiceActResult {
            audio: None,
            emotion: VoiceEmotion::Neutral,
            narrator_mode,
            voice_used: "silent".to_string(),
            text: clean_text,
        });
    }

    let omni_voice = persona_to_omni_voice(persona);

    // Prepend DM cue for out-of-character narration
    let speak_text = if narrator_mode == NarratorMode::OutOfCharacter {
        format!("{}{}", dm_voice_cue(), clean_text)
    } else {
        clean_text.clone()
    };

    info!(
        "🎭 Voice Act: persona={} voice={} emotion={:?} mode={:?} len={}",
        persona, omni_voice, emotion, narrator_mode, speak_text.len()
    );

    let audio = omni_synthesize(&speak_text, persona, format).await?;

    Ok(VoiceActResult {
        audio: Some(audio),
        emotion,
        narrator_mode,
        voice_used: omni_voice.to_string(),
        text: clean_text,
    })
}

/// Result of a voice-acted synthesis
#[derive(Debug, Serialize)]
#[allow(dead_code)] // Activates with LongCat CosyVoice
pub struct VoiceActResult {
    /// Audio bytes (None if Silent mode)
    #[serde(skip)]
    pub audio: Option<Vec<u8>>,
    /// Detected emotion
    pub emotion: VoiceEmotion,
    /// Narrator mode used
    pub narrator_mode: NarratorMode,
    /// Omni voice name used
    pub voice_used: String,
    /// Clean text (tags stripped)
    pub text: String,
}

/// Synthesize text via Acestep 1.5 on LongCat
/// Returns raw WAV audio bytes
pub async fn omni_synthesize(
    text: &str,
    voice: &str,
    _format: &str,
) -> anyhow::Result<Vec<u8>> {
    let omni_voice = persona_to_omni_voice(voice);

    let payload = serde_json::json!({
        "text": text,
        "voice": omni_voice,
    });

    info!("🎙️ LongCat Acestep 1.5 TTS (port {}): voice={} len={}", LONGCAT_PORT, omni_voice, text.len());

    let response = crate::http::LONG
        .post(format!("http://127.0.0.1:{}/tts", LONGCAT_PORT))
        .json(&payload)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Acestep 1.5 TTS failed: {} — {}", status, body);
    }

    let audio_bytes = response.bytes().await?;
    info!("🎙️ LongCat Acestep 1.5 returned {} bytes", audio_bytes.len());
    Ok(audio_bytes.to_vec())
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

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use trinity_protocol::character_sheet::detect_emotion;

    // ── Omni Persona Mapping ─────────────────────────────────────────

    #[test]
    fn test_persona_pete_maps_to_causal_male() {
        assert_eq!(persona_to_omni_voice("pete"), "am_adam");
        assert_eq!(persona_to_omni_voice("conductor"), "am_adam");
        assert_eq!(persona_to_omni_voice("M1"), "am_adam");
    }

    #[test]
    fn test_persona_recycler_maps_to_alloy() {
        assert_eq!(persona_to_omni_voice("recycler"), "joshua");
        assert_eq!(persona_to_omni_voice("narrator"), "joshua");
        assert_eq!(persona_to_omni_voice("dm"), "joshua");
    }

    #[test]
    fn test_persona_npc_maps_to_echo() {
        assert_eq!(persona_to_omni_voice("npc"), "am_echo");
        assert_eq!(persona_to_omni_voice("default"), "am_echo");
    }

    #[test]
    fn test_persona_youser_maps_to_nova() {
        assert_eq!(persona_to_omni_voice("youser"), "af_heart");
        assert_eq!(persona_to_omni_voice("student"), "af_heart");
    }

    #[test]
    fn test_persona_unknown_falls_back_to_causal_male() {
        assert_eq!(persona_to_omni_voice("unknown_voice"), "am_adam");
        assert_eq!(persona_to_omni_voice(""), "am_adam");
    }

    #[test]
    fn test_persona_case_insensitive() {
        assert_eq!(persona_to_omni_voice("PETE"), "am_adam");
        assert_eq!(persona_to_omni_voice("Recycler"), "joshua");
        assert_eq!(persona_to_omni_voice("NPC"), "am_echo");
    }

    // ── Emotion Detection ───────────────────────────────────────────────

    #[test]
    fn test_emotion_celebratory() {
        assert_eq!(detect_emotion("Congratulations! Quest complete!"), VoiceEmotion::Celebratory);
        assert_eq!(detect_emotion("You leveled up! XP awarded."), VoiceEmotion::Celebratory);
        assert_eq!(detect_emotion("Excellent work on the milestone."), VoiceEmotion::Celebratory);
    }

    #[test]
    fn test_emotion_urgent() {
        assert_eq!(detect_emotion("Warning: deadline approaching!"), VoiceEmotion::Urgent);
        assert_eq!(detect_emotion("CRITICAL: system error detected"), VoiceEmotion::Urgent);
        assert_eq!(detect_emotion("Caution: this cannot be undone"), VoiceEmotion::Urgent);
    }

    #[test]
    fn test_emotion_sarcastic() {
        assert_eq!(detect_emotion("Oh really? Interesting choice."), VoiceEmotion::Sarcastic);
        assert_eq!(detect_emotion("Bold move, let's see how that works."), VoiceEmotion::Sarcastic);
    }

    #[test]
    fn test_emotion_contemplative() {
        assert_eq!(detect_emotion("Have you considered another approach?"), VoiceEmotion::Contemplative);
        assert_eq!(detect_emotion("What if we thought about this differently?"), VoiceEmotion::Contemplative);
        assert_eq!(detect_emotion("Perhaps there's a better path."), VoiceEmotion::Contemplative);
    }

    #[test]
    fn test_emotion_warm() {
        assert_eq!(detect_emotion("Great job on this section!"), VoiceEmotion::Warm);
        assert_eq!(detect_emotion("You're making progress, keep going!"), VoiceEmotion::Warm);
    }

    #[test]
    fn test_emotion_neutral_default() {
        assert_eq!(detect_emotion("The lesson plan has three sections."), VoiceEmotion::Neutral);
        assert_eq!(detect_emotion("Click the button to proceed."), VoiceEmotion::Neutral);
    }

    #[test]
    fn test_emotion_to_omni_tag() {
        assert_eq!(VoiceEmotion::Celebratory.to_voxtral_tag(), "happy");
        assert_eq!(VoiceEmotion::Sarcastic.to_voxtral_tag(), "sarcastic");
        assert_eq!(VoiceEmotion::Neutral.to_voxtral_tag(), "neutral");
        assert_eq!(VoiceEmotion::Warm.to_voxtral_tag(), "happy");
    }

    // ── Narrator Mode Detection ─────────────────────────────────────────

    #[test]
    fn test_narrator_dm_mode() {
        let (mode, text) = detect_narrator_mode("[DM] Your rubric needs a column for criteria.");
        assert_eq!(mode, NarratorMode::OutOfCharacter);
        assert_eq!(text, "Your rubric needs a column for criteria.");
    }

    #[test]
    fn test_narrator_ooc_mode() {
        let (mode, text) = detect_narrator_mode("[OOC] Consider using Bloom's taxonomy here.");
        assert_eq!(mode, NarratorMode::OutOfCharacter);
        assert_eq!(text, "Consider using Bloom's taxonomy here.");
    }

    #[test]
    fn test_narrator_silent_mode() {
        let (mode, text) = detect_narrator_mode("[SILENT] System checkpoint saved.");
        assert_eq!(mode, NarratorMode::Silent);
        assert_eq!(text, "System checkpoint saved.");
    }

    #[test]
    fn test_narrator_mute_mode() {
        let (mode, text) = detect_narrator_mode("[MUTE] Internal process complete.");
        assert_eq!(mode, NarratorMode::Silent);
        assert_eq!(text, "Internal process complete.");
    }

    #[test]
    fn test_narrator_in_character_default() {
        let (mode, text) = detect_narrator_mode("The Iron Road stretches before you.");
        assert_eq!(mode, NarratorMode::InCharacter);
        assert_eq!(text, "The Iron Road stretches before you.");
    }

    // ── Voice Status Struct ─────────────────────────────────────────────

    #[test]
    fn test_voice_status_serializes() {
        let status = VoiceStatus {
            sidecar_running: false,
            personaplex_available: false,
            omni_available: true,
            npu_available: false,
            active_pipeline: "omni".to_string(),
            mode: "dev".to_string(),
            message: "Omni ready".to_string(),
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"omni_available\":true"));
        assert!(json.contains("\"active_pipeline\":\"omni\""));
    }

    // ── Omni Port ────────────────────────────────────────────────────

    #[test]
    fn test_omni_port_constant() {
        assert_eq!(OMNI_PORT, 8200);
    }

    // ── NPU Check (doesn't panic) ──────────────────────────────────────

    #[test]
    fn test_npu_check_no_panic() {
        let _ = check_npu_availability();
    }
}


