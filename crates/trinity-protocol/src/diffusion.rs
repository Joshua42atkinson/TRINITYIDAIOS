// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Protocol Layer
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:     diffusion.rs
// PURPOSE:  Diffusion model types for NPU-optimized image generation pipeline
// BIBLE:    Car 11 — YOKE (ART Creative Pipeline, §11.2)
//
// ═══════════════════════════════════════════════════════════════════════════════

// Trinity Diffusion Model Types
// Shared types for NPU-optimized diffusion pipeline.

use serde::{Deserialize, Serialize};

/// Request for diffusion model inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffusionRequest {
    pub request_id: String,
    pub prompt: String,
    pub style: String,
    pub complexity: String,
    pub cognitive_load: f32,
    pub created_at: f64,
}

/// A generated or generating diffusion asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffusionAsset {
    pub request_id: String,
    pub prompt: String,
    pub status: AssetStatus,
    pub progress: f32,
    pub result_data: Option<Vec<u8>>,
    pub created_at: f64,
    pub completed_at: Option<f64>,
}

/// Status of a diffusion asset generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AssetStatus {
    #[default]
    Queued,
    Generating,
    Completed,
    Failed,
}

/// User interaction with a diffusion asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiffusionInteraction {
    PreviewRequest { asset_id: String },
    ParameterAdjust { asset_id: String, params: String },
}

/// Performance telemetry for NPU diffusion
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub npu_utilization: f32,
    pub temperature_celsius: f32,
    pub memory_used_mb: u32,
    pub inference_hz: f32,
}
