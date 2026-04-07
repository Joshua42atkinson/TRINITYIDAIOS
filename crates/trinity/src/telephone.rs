// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Telephone Line
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        telephone.rs
// PURPOSE:     Headless audio-only WebSocket route for hands-free education
//
// ARCHITECTURE:
//   Client connects via `ws://host:3000/api/telephone`
//   1. Client sends Binary frames (WAV/PCM audio chunks)
//   2. Server routes audio → STT (Whisper ONNX)
//   3. Server routes transcript → LLM (Pete, via inference_router)
//   4. Server routes response → TTS (Kokoro or Omni)
//   5. Server sends Binary frames (WAV audio) back to client
//
//   Text frames carry JSON control messages:
//     → { "type": "config", "persona": "pete", "mode": "iron-road" }
//     ← { "type": "transcript", "text": "..." }
//     ← { "type": "response", "text": "..." }
//     ← { "type": "status", "pipeline": "supertonic", "latency_ms": 123 }
//
// MATURITY:    L2 (core structure, STT integration pending runtime test)
// QUEST_PHASE: supports all ADDIECRAPEYE phases
//
// CHANGES:
//   2026-03-27  Cascade  Initial Telephone Line implementation
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

use crate::AppState;

// ============================================================================
// TYPES
// ============================================================================

/// Client → Server control message
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum TelephoneCommand {
    /// Configure the session
    #[serde(rename = "config")]
    Config {
        /// Pete persona or voice variant
        persona: Option<String>,
        /// Mode: "dev" or "iron-road"
        mode: Option<String>,
        /// Voice: which TTS voice to use
        voice: Option<String>,
    },
    /// End the call
    #[serde(rename = "hangup")]
    Hangup,
}

/// Server → Client control message
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum TelephoneEvent {
    /// User speech transcription
    #[serde(rename = "transcript")]
    Transcript { text: String },
    /// Pete's text response
    #[serde(rename = "response")]
    Response { text: String },
    /// Status update
    #[serde(rename = "status")]
    Status {
        pipeline: String,
        latency_ms: u64,
        message: String,
    },
    /// Error
    #[serde(rename = "error")]
    Error { message: String },
    /// Call ended
    #[serde(rename = "hangup")]
    Hangup { reason: String },
}

/// Session configuration
struct TelephoneSession {
    persona: String,
    mode: String,
    voice: String,
}

impl Default for TelephoneSession {
    fn default() -> Self {
        Self {
            persona: "pete".to_string(),
            mode: "iron-road".to_string(),
            voice: "pete".to_string(),
        }
    }
}

// ============================================================================
// WEBSOCKET HANDLER
// ============================================================================

/// WebSocket upgrade handler for the Telephone Line
/// Route: GET /api/telephone → WebSocket
pub async fn telephone_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    info!("📞 Telephone Line: incoming call");
    ws.on_upgrade(move |socket| telephone_handler(socket, state))
}

/// Main WebSocket loop — bidirectional audio streaming
async fn telephone_handler(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut session = TelephoneSession::default();

    // Send welcome status
    let welcome = TelephoneEvent::Status {
        pipeline: detect_voice_pipeline().await,
        latency_ms: 0,
        message: "Telephone Line connected. Send audio frames to begin.".to_string(),
    };
    if let Ok(json) = serde_json::to_string(&welcome) {
        let _ = sender.send(Message::Text(json.into())).await;
    }

    info!("📞 Telephone Line: call started (persona={}, mode={})", session.persona, session.mode);

    // Process incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Binary(audio_data)) => {
                // Audio frame received — process through the pipeline
                let start = std::time::Instant::now();

                match process_audio_frame(&audio_data, &session, &state).await {
                    Ok((transcript, response_text, response_audio)) => {
                        let latency = start.elapsed().as_millis() as u64;

                        // Send transcript
                        if let Ok(json) = serde_json::to_string(&TelephoneEvent::Transcript {
                            text: transcript.clone(),
                        }) {
                            let _ = sender.send(Message::Text(json.into())).await;
                        }

                        // Send response text
                        if let Ok(json) = serde_json::to_string(&TelephoneEvent::Response {
                            text: response_text.clone(),
                        }) {
                            let _ = sender.send(Message::Text(json.into())).await;
                        }

                        // Send response audio
                        if !response_audio.is_empty() {
                            let _ = sender.send(Message::Binary(response_audio.into())).await;
                        }

                        // Send latency status
                        if let Ok(json) = serde_json::to_string(&TelephoneEvent::Status {
                            pipeline: detect_voice_pipeline().await,
                            latency_ms: latency,
                            message: format!("Processed {} bytes in {}ms", audio_data.len(), latency),
                        }) {
                            let _ = sender.send(Message::Text(json.into())).await;
                        }

                        info!("📞 Telephone: {}ms | {} → {}", latency,
                            truncate(&transcript, 50), truncate(&response_text, 50));
                    }
                    Err(e) => {
                        error!("📞 Telephone pipeline error: {}", e);
                        if let Ok(json) = serde_json::to_string(&TelephoneEvent::Error {
                            message: format!("Pipeline error: {}", e),
                        }) {
                            let _ = sender.send(Message::Text(json.into())).await;
                        }
                    }
                }
            }
            Ok(Message::Text(text)) => {
                // JSON control message
                match serde_json::from_str::<TelephoneCommand>(&text) {
                    Ok(TelephoneCommand::Config { persona, mode, voice }) => {
                        if let Some(p) = persona { session.persona = p; }
                        if let Some(m) = mode { session.mode = m; }
                        if let Some(v) = voice { session.voice = v; }
                        info!("📞 Telephone config: persona={}, mode={}, voice={}",
                            session.persona, session.mode, session.voice);
                    }
                    Ok(TelephoneCommand::Hangup) => {
                        info!("📞 Telephone: client hung up");
                        let _ = sender.send(Message::Text(
                            serde_json::to_string(&TelephoneEvent::Hangup {
                                reason: "Client ended call".to_string(),
                            }).unwrap_or_default().into()
                        )).await;
                        break;
                    }
                    Err(e) => {
                        warn!("📞 Telephone: invalid control message: {}", e);
                    }
                }
            }
            Ok(Message::Close(_)) => {
                info!("📞 Telephone: connection closed");
                break;
            }
            Ok(Message::Ping(data)) => {
                let _ = sender.send(Message::Pong(data)).await;
            }
            Err(e) => {
                error!("📞 Telephone: WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    info!("📞 Telephone Line: call ended");
}

// ============================================================================
// PIPELINE
// ============================================================================

/// Process a single audio frame through STT → LLM → TTS
async fn process_audio_frame(
    audio_data: &[u8],
    session: &TelephoneSession,
    state: &AppState,
) -> anyhow::Result<(String, String, Vec<u8>)> {
    // ── Step 1: STT — Whisper ONNX (or future Cohere Transcribe) ──
    let client = &*crate::http::LONG;
    let part = reqwest::multipart::Part::bytes(audio_data.to_vec())
        .file_name("call.wav").mime_str("audio/wav").unwrap();
    let form = reqwest::multipart::Form::new().text("model", "Gemma-4-E4B-Omni").part("file", part);
    let transcript = match client.post("http://127.0.0.1:8000/v1/audio/transcriptions").multipart(form).send().await {
        Ok(res) if res.status().is_success() => {
            let json: serde_json::Value = res.json().await.unwrap_or_default();
            json["text"].as_str().unwrap_or("[Silence]").to_string()
        },
        _ => "[Silence]".to_string()
    };

    // ── Step 2: LLM — Pete chat completion ──
    let system_prompt = build_telephone_system_prompt(&session.persona, &session.mode);
    let messages = vec![
        crate::ChatMessage {
            role: "system".to_string(),
            content: system_prompt,
            timestamp: None,
            image_base64: None,
        },
        crate::ChatMessage {
            role: "user".to_string(),
            content: transcript.clone(),
            timestamp: None,
            image_base64: None,
        },
    ];

    let url = state.inference_router.read().await.active_url().to_string();
    let response_text = crate::inference::chat_completion(&url, &messages, 256)
        .await
        .map_err(|e| anyhow::anyhow!("LLM failed: {}", e))?;

    // ── Step 3: TTS — Omni (if available) or Kokoro (fallback) ──
    let response_audio = synthesize_response(&response_text, &session.voice, state).await
        .unwrap_or_else(|e| {
            warn!("📞 TTS failed (sending text only): {}", e);
            vec![]
        });

    Ok((transcript, response_text, response_audio))
}

/// Synthesize response text via best available TTS
/// Priority: Kokoro (:8200, Apache 2.0) → silent fallback
async fn synthesize_response(text: &str, voice: &str, _state: &AppState) -> anyhow::Result<Vec<u8>> {
    // Kokoro is our primary TTS — it's live on :8200 via omni_synthesize
    if crate::voice::check_omni_audio_health().await {
        return crate::voice::omni_synthesize(text, voice, "wav").await;
    }
    Err(anyhow::anyhow!("No TTS available (Kokoro :8200 unreachable)"))
}

/// Build a voice-optimized system prompt for the Telephone Line
fn build_telephone_system_prompt(persona: &str, mode: &str) -> String {
    let persona_name = match persona {
        "recycler" | "narrator" | "dm" => "Great Recycler",
        _ => "Programmer Pete",
    };

    let mode_context = match mode {
        "iron-road" => "The user is on the Iron Road — a gamified instructional design quest. \
            They are using voice-only mode (Telephone Line). Keep responses conversational \
            and under 3 sentences. Guide them through the ADDIECRAPEYE lifecycle.",
        "dev" => "The user is in development mode, building with Trinity. \
            Keep responses technical but concise — they're hands-free.",
        _ => "Keep responses concise and voice-friendly.",
    };

    format!(
        "You are {persona_name}, a Socratic AI companion in Trinity ID AI OS.\n\
        \n\
        VOICE MODE ACTIVE — Telephone Line\n\
        {mode_context}\n\
        \n\
        Rules for voice:\n\
        - Maximum 3 sentences per response\n\
        - Use conversational language, not academic prose\n\
        - Pause naturally between ideas\n\
        - Never output markdown, code blocks, or formatting\n\
        - Never output lists or bullet points — speak naturally\n\
        - Ask one question at a time, wait for the response"
    )
}

/// Detect which voice pipeline is active
async fn detect_voice_pipeline() -> String {
    if crate::voice::check_omni_audio_health().await {
        "kokoro".to_string()
    } else {
        "unavailable".to_string()
    }
}

/// Truncate a string for logging
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telephone_system_prompt_pete() {
        let prompt = build_telephone_system_prompt("pete", "iron-road");
        assert!(prompt.contains("Programmer Pete"));
        assert!(prompt.contains("Telephone Line"));
        assert!(prompt.contains("3 sentences"));
    }

    #[test]
    fn test_telephone_system_prompt_recycler() {
        let prompt = build_telephone_system_prompt("recycler", "dev");
        assert!(prompt.contains("Great Recycler"));
        assert!(prompt.contains("development mode"));
    }

    #[test]
    fn test_telephone_session_defaults() {
        let session = TelephoneSession::default();
        assert_eq!(session.persona, "pete");
        assert_eq!(session.mode, "iron-road");
        assert_eq!(session.voice, "pete");
    }

    #[test]
    fn test_truncate_short() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_long() {
        let long = "a".repeat(100);
        let result = truncate(&long, 10);
        assert!(result.len() < 20); // 10 chars + ellipsis
        assert!(result.ends_with('…'));
    }

    #[test]
    fn test_telephone_command_deserialize_config() {
        let json = r#"{"type":"config","persona":"recycler","mode":"dev"}"#;
        let cmd: TelephoneCommand = serde_json::from_str(json).unwrap();
        match cmd {
            TelephoneCommand::Config { persona, mode, .. } => {
                assert_eq!(persona.unwrap(), "recycler");
                assert_eq!(mode.unwrap(), "dev");
            }
            _ => panic!("Expected Config"),
        }
    }

    #[test]
    fn test_telephone_command_deserialize_hangup() {
        let json = r#"{"type":"hangup"}"#;
        let cmd: TelephoneCommand = serde_json::from_str(json).unwrap();
        assert!(matches!(cmd, TelephoneCommand::Hangup));
    }

    #[test]
    fn test_telephone_event_serialize_transcript() {
        let event = TelephoneEvent::Transcript { text: "hello world".to_string() };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"transcript\""));
        assert!(json.contains("hello world"));
    }

    #[test]
    fn test_telephone_event_serialize_status() {
        let event = TelephoneEvent::Status {
            pipeline: "omni".to_string(),
            latency_ms: 150,
            message: "ok".to_string(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"pipeline\":\"omni\""));
        assert!(json.contains("150"));
    }
}
