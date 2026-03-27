// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Protocol Layer
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:     sidecars.rs
// PURPOSE:  Sidecar RPC protocol types for vLLM, llama.cpp, and ORT model backends
// BIBLE:    Car 3 — DEVELOPMENT (Hotel Management, §3.3)
//
// ═══════════════════════════════════════════════════════════════════════════════

//! Trinity Sidecar RPC Protocol
//!
//! Defines the shared services for vLLM, Llama.cpp, and ORT sidecars.

use serde::{Deserialize, Serialize};

/// Comprehensive sidecar status for process management and UI
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SidecarStatus {
    Stopped,
    Starting,
    Ready, // Renamed from Running to Ready for RPC clarity
    Busy,  // Processing an inference task
    Stopping,
    Error,
    Restarting,
    Hibernating, // Process alive, model unloaded
}

/// Generic health status for system components
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Starting,
    Stopped,
}

/// Sidecar types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SidecarType {
    /// vLLM-OMNI server for 97B Qwen Conductor and SDXL
    VllmOmni,
    /// NPU audio sidecar for Personaplex-7B
    NpuAudio,
    /// LFM2.5-Audio sidecar for speech-to-speech (CPU AVX-512 or GPU ROCm)
    LfmAudio,
    /// Document management sidecar
    DocumentManager,
    /// Data pipeline sidecar
    DataPipeline,
    /// Blueprint reviewer sidecar
    BlueprintReviewer,
    /// Music AI sidecar
    MusicAi,
    /// Agent steward sidecar
    AgentSteward,
    /// Graphics generation sidecar
    BevyGraphics,
    /// Skills processing sidecar
    Skills,
    /// Memory management sidecar
    Memory,
    /// llama.cpp sidecar
    LlamaCpp,
    /// ORT sidecar
    Ort,
    /// Yardmaster sidecar (Project Management / RAG)
    Yardmaster,
    /// Brakeman sidecar (Security Testing)
    Brakeman,
    /// Nitrogen sidecar (Code Optimization)
    Nitrogen,
    /// Dispatcher sidecar (Knowledge Routing)
    Dispatcher,
    /// Draftsman sidecar (Creative Layouts)
    Draftsman,
    /// Engineer sidecar (Code Synthesis)
    Engineer,
    /// Diffusion sidecar (Image Generation)
    Diffusion,
    /// Omni sidecar (Multimodal)
    Omni,
}

/// Sidecar health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarHealthStatus {
    pub overall_healthy: bool,
    pub available_sidecars: Vec<SidecarType>,
    pub unavailable_sidecars: Vec<SidecarType>,
    pub last_check_secs: u64,
}

/// vLLM Manager Service
#[tarpc::service]
pub trait VllmManagerService {
    /// Start the vLLM server with specific model and GPU utilization
    async fn start_server(model_id: String, gpu_utilization: f32) -> Result<(), String>;
    /// Stop the running vLLM server
    async fn stop_server() -> Result<(), String>;
    /// Get the current status of the vLLM process
    async fn get_status() -> SidecarStatus;
}

/// Llama.cpp Inference Service
#[tarpc::service]
pub trait LlamaInferenceService {
    /// Load a GGUF model from path
    async fn load_model(path: String, n_gpu_layers: u32, context_size: u32) -> Result<(), String>;
    /// Unload the currently loaded model to free memory
    async fn unload_model() -> Result<(), String>;
    /// Generate text based on a prompt
    async fn generate(prompt: String, params: InferenceParams) -> Result<String, String>;
    /// Get current engine status
    async fn get_status() -> SidecarStatus;
}

/// ONNX Runtime (ORT) Multi-modal Service
#[tarpc::service]
pub trait OrtInferenceService {
    /// Generate semantic embeddings for a list of strings
    async fn embed(texts: Vec<String>) -> Result<Vec<Vec<f32>>, String>;
    /// Transcribe audio data using Whisper
    async fn transcribe(audio_data: Vec<u8>) -> Result<String, String>;
    /// Perform vision analysis on an image
    async fn analyze_image(image_data: Vec<u8>, task: String) -> Result<String, String>;
    /// Get current status
    async fn get_status() -> SidecarStatus;
}

/// Common inference parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceParams {
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: u32,
    pub stop_sequences: Vec<String>,
    pub repeat_penalty: f32,
}

impl Default for InferenceParams {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            top_p: 0.9,
            max_tokens: 2048,
            stop_sequences: vec!["<|endoftext|>".to_string()],
            repeat_penalty: 1.1,
        }
    }
}
