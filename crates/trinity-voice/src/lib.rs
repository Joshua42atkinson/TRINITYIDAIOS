// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-voice
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        lib.rs
// PURPOSE:     Voice interface types and Rust integration layer
//
// ARCHITECTURE:
//   • WORKING PIPELINE: Python trinity_voice_server.py on :7777
//     openwakeword → faster-whisper → Mistral Small 4 → Kokoro TTS
//   • This Rust crate provides shared types (VoiceMode, VoiceState, etc.)
//     and future Rust-native voice integration
//   • PersonaPlex ONNX types retained for future NPU path
//
// CHANGES:
//   2026-03-19  Cascade  Updated docs — Python pipeline is the working impl
//   2026-03-16  Cascade  Full PersonaPlex integration (types)
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod audio;
pub mod personaplex;
pub mod tts;

pub use audio::*;
pub use personaplex::*;
pub use tts::*;

/// Voice engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    pub model_path: PathBuf,
    pub mimi_encoder_path: PathBuf,
    pub mimi_decoder_path: PathBuf,
    pub context_size: usize,
    pub sample_rate: u32,
    pub channels: u16,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            model_path: PathBuf::from("models/personaplex"),
            mimi_encoder_path: PathBuf::from("models/personaplex/mimi_encoder.onnx"),
            mimi_decoder_path: PathBuf::from("models/personaplex/mimi_decoder.onnx"),
            context_size: 16384,
            sample_rate: 24000,
            channels: 1,
        }
    }
}

/// Voice engine for PersonaPlex-7B
#[derive(Debug)]
pub struct VoiceEngine {
    pub config: VoiceConfig,
    pub state: VoiceState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceState {
    Uninitialized,
    Loading,
    Ready,
    Listening,
    Processing,
    Speaking,
    Error,
}

impl VoiceEngine {
    pub fn new(config: VoiceConfig) -> Self {
        Self {
            config,
            state: VoiceState::Uninitialized,
        }
    }

    /// Initialize the voice engine
    pub async fn initialize(&mut self) -> anyhow::Result<()> {
        self.state = VoiceState::Loading;

        // Check if model files exist
        if !self.config.model_path.exists() {
            return Err(anyhow::anyhow!(
                "PersonaPlex model not found at {:?}",
                self.config.model_path
            ));
        }

        // TODO: Load ONNX models
        // TODO: Initialize audio I/O

        self.state = VoiceState::Ready;
        Ok(())
    }

    /// Check if voice engine is ready
    pub fn is_ready(&self) -> bool {
        self.state == VoiceState::Ready
    }

    /// Get current state
    pub fn state(&self) -> VoiceState {
        self.state
    }
}

/// Audio chunk for streaming
#[derive(Debug, Clone)]
pub struct AudioChunk {
    pub data: Vec<f32>,
    pub timestamp_ms: u64,
    pub sample_rate: u32,
}

/// Voice command recognized from audio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommand {
    pub text: String,
    pub confidence: f32,
    pub screen: String, // "pete", "mini", "yardmaster"
    pub latency_ms: u64,
}

/// Voice response (audio + text)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceResponse {
    pub audio_base64: String,
    pub text: String,
    pub latency_ms: u64,
}

/// Screen-specific voice modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VoiceMode {
    /// ID/Pete: Socratic teaching, questions not answers
    PeteTeaching,
    /// AI/Mini: Creative commands, image generation
    CreativeCommand,
    /// OS/YARDMASTER: Voice coding, terminal commands
    DevCommand,
}

impl VoiceMode {
    pub fn system_prompt(&self) -> &'static str {
        match self {
            VoiceMode::PeteTeaching => {
                "You are Pete in voice mode. Be warm, concise, and Socratic. \
                 Ask one clarifying question per response. Use railroad metaphors."
            }
            VoiceMode::CreativeCommand => {
                "You are the Artist voice assistant. Generate ready-to-use prompts \
                 for image/video/music creation. Be specific and technical."
            }
            VoiceMode::DevCommand => {
                "You are the Engineer voice assistant. Generate Rust/Bevy code \
                 from voice commands. Use idiomatic patterns with §17 headers."
            }
        }
    }

    pub fn screen(&self) -> &'static str {
        match self {
            VoiceMode::PeteTeaching => "pete",
            VoiceMode::CreativeCommand => "mini",
            VoiceMode::DevCommand => "yardmaster",
        }
    }
}

/// Wake word detection
#[derive(Debug, Clone)]
pub struct WakeWordDetector {
    pub wake_word: String,
    pub threshold: f32,
}

impl WakeWordDetector {
    pub fn new(wake_word: impl Into<String>) -> Self {
        Self {
            wake_word: wake_word.into(),
            threshold: 0.85,
        }
    }

    /// Check if audio contains wake word
    pub fn detect(&self, _audio: &AudioChunk) -> bool {
        // TODO: Implement actual wake word detection
        // For now, stub - always return false
        false
    }
}

/// Voice pipeline result
#[derive(Debug, Clone)]
pub enum VoiceResult {
    /// Wake word detected
    WakeWord,
    /// Command recognized
    Command(VoiceCommand),
    /// No speech detected
    Silence,
    /// Error processing
    Error(String),
}
pub mod ssml;
