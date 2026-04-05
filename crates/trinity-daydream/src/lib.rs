// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Bevy 3D Yard
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:     lib.rs
// PURPOSE:  Library root for the Bevy spatial sandbox (Layer 3 of Trinity architecture)
// BIBLE:    Car 12 — EVOLVE (Server Architecture, §12.3)
//
// ═══════════════════════════════════════════════════════════════════════════════

//! 🎨 Trinity Bevy Graphics - Vision Processing Integration
//!
//! Sprint 1 Focus: Vision processing with Qwen-35B + mmproj via llama-server
//! - Prompt analysis for visual requirements extraction
//! - Reference image analysis for style/color/composition
//! - Fallback keyword-based analysis when model unavailable

use std::path::PathBuf;

pub mod addiecrapeye;
#[cfg(feature = "desktop")]
pub mod avatar;
#[cfg(feature = "desktop")]
pub mod bridge;
pub mod meshes;
pub mod vision;

#[cfg(feature = "desktop")]
pub mod art_panels;
#[cfg(feature = "desktop")]
pub mod creative_bridge;

// DAYDREAM — 3D LitRPG world (desktop feature required for physics + camera)
#[cfg(feature = "desktop")]
pub mod daydream;

#[cfg(feature = "desktop")]
pub mod python_bridge;

#[cfg(feature = "desktop")]
pub mod hud;
#[cfg(feature = "desktop")]
pub mod train_car;
#[cfg(feature = "desktop")]
pub mod bridge_client;


pub use meshes::*;
pub use vision::*;

// Re-export Bevy types needed by vision module
pub use bevy::color::Color;
pub use bevy::math::{Vec2, Vec3};

/// Configuration for graphics generation
#[derive(Debug, Clone)]
pub struct GraphicsConfig {
    /// Vision model for understanding prompts
    pub vision_model_path: Option<PathBuf>,
    /// Generation quality preset
    pub quality_preset: QualityPreset,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            vision_model_path: None, // Uses llama-server instead
            quality_preset: QualityPreset::High,
        }
    }
}

/// Quality presets for generation
#[derive(Debug, Clone, PartialEq)]
pub enum QualityPreset {
    Draft,    // Fast, low quality
    Standard, // Balanced
    High,     // High quality
    Ultra,    // Maximum quality
}

/// Mesh complexity levels
#[derive(Debug, Clone, PartialEq)]
pub enum MeshComplexity {
    Low { vertices: u32 },
    Medium { vertices: u32 },
    High { vertices: u32 },
}

/// Mesh styles
#[derive(Debug, Clone, PartialEq)]
pub enum MeshStyle {
    Realistic,
    Stylized { style_name: String },
    LowPoly,
    Voxel,
}

/// Material types
#[derive(Debug, Clone, PartialEq)]
pub enum MaterialType {
    Pbr,
    Toon,
    Unlit,
    Glass,
    Metallic,
}

/// Material properties
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MaterialProperties {
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
}

/// Shader types
#[derive(Debug, Clone, PartialEq)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Compute,
}

/// Shader languages
#[derive(Debug, Clone, PartialEq)]
pub enum ShaderLanguage {
    Wgsl,
    Glsl,
    Hlsl,
}

/// Environment settings
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EnvironmentSettings {
    pub skybox: Option<String>,
}

/// Types of generatable assets
#[derive(Debug, Clone, PartialEq)]
pub enum AssetType {
    /// 3D mesh with materials
    Mesh {
        complexity: MeshComplexity,
        style: MeshStyle,
    },
    /// PBR material
    Material {
        material_type: MaterialType,
        properties: MaterialProperties,
    },
    /// Shader code
    Shader {
        shader_type: ShaderType,
        language: ShaderLanguage,
    },
    /// Complete scene
    Scene {
        elements: Vec<String>,
        environment: EnvironmentSettings,
    },
}
