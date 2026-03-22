//! Vision integration for graphics generation
//!
//! Uses Qwen3.5-35B-A3B + mmproj via llama-server for vision understanding.
//! Connects to llama-server's OpenAI-compatible API with image support.

use crate::*;
use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::{DynamicImage, ImageFormat};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;

use image::GenericImageView;

/// Vision processor for understanding graphics requirements
/// Connects to llama-server with mmproj for vision inference
pub struct VisionProcessor {
    /// HTTP client for llama-server
    client: reqwest::Client,
    /// Server URL (e.g., http://127.0.0.1:8081)
    server_url: String,
    /// Model configuration
    config: VisionConfig,
    /// Server health status (cached)
    healthy: Arc<RwLock<bool>>,
}

/// Configuration for vision processing
#[derive(Debug, Clone)]
pub struct VisionConfig {
    /// Path to vision model (for reference)
    pub model_path: std::path::PathBuf,
    /// Path to mmproj file (for reference)
    pub mmproj_path: std::path::PathBuf,
    /// Maximum image size for preprocessing
    pub max_image_size: u32,
    /// Number of vision tokens
    pub vision_tokens: u32,
    /// llama-server port
    pub server_port: u16,
}

impl Default for VisionConfig {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_default();
        Self {
            model_path: home
                .join(".lmstudio/models/lmstudio-community/Qwen3.5-35B-A3B-GGUF/Qwen3.5-35B-A3B-Q4_K_M.gguf"),
            mmproj_path: home.join("ai_models/gguf/mmproj-Qwen3.5-35B-A3B-BF16.gguf"),
            max_image_size: 1024,
            vision_tokens: 256,
            server_port: 8081,
        }
    }
}

impl VisionProcessor {
    /// Create new vision processor connected to llama-server
    pub fn new(config: VisionConfig) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;

        Ok(Self {
            client,
            server_url: format!("http://127.0.0.1:{}", config.server_port),
            config,
            healthy: Arc::new(RwLock::new(false)),
        })
    }

    /// Check if llama-server is healthy
    pub async fn is_healthy(&self) -> bool {
        match self
            .client
            .get(format!("{}/health", self.server_url))
            .timeout(std::time::Duration::from_secs(3))
            .send()
            .await
        {
            Ok(resp) => {
                let healthy = resp.status().is_success();
                *self.healthy.write().await = healthy;
                healthy
            }
            Err(_) => {
                *self.healthy.write().await = false;
                false
            }
        }
    }

    /// Analyze prompt and extract visual requirements using vision model
    pub async fn analyze_prompt_async(&self, prompt: &str) -> Result<VisualRequirements> {
        // Try vision model first
        if self.is_healthy().await {
            match self.vision_analyze_prompt(prompt).await {
                Ok(req) => return Ok(req),
                Err(e) => warn!("Vision model failed, falling back: {}", e),
            }
        }

        // Fallback to keyword extraction
        self.fallback_analyze_prompt(prompt)
    }

    /// Analyze reference image using vision model
    pub async fn analyze_reference_image_async(&self, image_path: &Path) -> Result<ImageAnalysis> {
        // Load and preprocess image
        let img = image::open(image_path)?;
        let processed = self.preprocess_image(img)?;

        // Try vision model
        if self.is_healthy().await {
            match self.vision_analyze_image(&processed).await {
                Ok(analysis) => return Ok(analysis),
                Err(e) => warn!("Vision model failed, falling back: {}", e),
            }
        }

        // Fallback to basic analysis
        self.fallback_analyze_image(&processed)
    }

    /// Generate detailed description from vision analysis
    pub async fn generate_description_async(
        &self,
        prompt: &str,
        reference_images: &[std::path::PathBuf],
    ) -> Result<String> {
        if !reference_images.is_empty() && self.is_healthy().await {
            // Use vision model with images
            let images_base64: Vec<String> = reference_images
                .iter()
                .filter_map(|p| self.load_image_base64(p).ok())
                .collect();

            if !images_base64.is_empty() {
                return self
                    .vision_generate_description(prompt, &images_base64)
                    .await;
            }
        }

        // Fallback
        let mut description = prompt.to_string();
        for image_path in reference_images {
            if let Ok(analysis) = self.analyze_reference_image_async(image_path).await {
                description.push_str(&format!(
                    "\n- Style: {}, Colors: {:?}",
                    analysis.style, analysis.dominant_colors
                ));
            }
        }
        Ok(description)
    }

    // ========================================================================
    // VISION MODEL INFERENCE (via llama-server)
    // ========================================================================

    /// Use vision model to analyze a prompt
    async fn vision_analyze_prompt(&self, prompt: &str) -> Result<VisualRequirements> {
        let system = "You are a graphics asset analyzer. Extract visual requirements from the prompt.
Return JSON with: asset_type (mesh/material/shader/scene), style, complexity (0.0-1.0), colors (hex codes), mood.";

        let response = self.chat_completion(system, prompt, None).await?;

        // Parse JSON response
        self.parse_visual_requirements(&response, prompt)
    }

    /// Use vision model to analyze an image
    async fn vision_analyze_image(&self, img: &DynamicImage) -> Result<ImageAnalysis> {
        let base64 = self.image_to_base64(img)?;

        let system = "You are an image analyzer for graphics assets. Analyze the image and return JSON with:
dominant_colors (hex codes), composition (rule_of_thirds, balance, focus_point), style, objects (label, confidence), lighting (direction, intensity, temperature, softness).";

        let user = "Analyze this image for graphics generation purposes.";

        let response = self
            .chat_completion_with_image(system, user, &base64)
            .await?;

        // Parse JSON response
        self.parse_image_analysis(&response, img)
    }

    /// Generate description using vision model with images
    async fn vision_generate_description(
        &self,
        prompt: &str,
        images_base64: &[String],
    ) -> Result<String> {
        let system = "You are a graphics description generator. Given a prompt and reference images, generate a detailed description for asset generation.";

        // For now, use first image only (llama-server supports single image)
        let image_data = images_base64.first().map(|s| s.as_str());

        let response = self.chat_completion(system, prompt, image_data).await?;
        Ok(response)
    }

    /// Send chat completion request to llama-server
    async fn chat_completion(
        &self,
        system: &str,
        user: &str,
        image_base64: Option<&str>,
    ) -> Result<String> {
        let messages = if let Some(img) = image_base64 {
            vec![
                serde_json::json!({"role": "system", "content": system}),
                serde_json::json!({
                    "role": "user",
                    "content": [
                        {"type": "text", "text": user},
                        {"type": "image_url", "image_url": {"url": format!("data:image/png;base64,{}", img)}}
                    ]
                }),
            ]
        } else {
            vec![
                serde_json::json!({"role": "system", "content": system}),
                serde_json::json!({"role": "user", "content": user}),
            ]
        };

        let request = serde_json::json!({
            "messages": messages,
            "max_tokens": 1024,
            "temperature": 0.3,
        });

        let response = self
            .client
            .post(format!("{}/v1/chat/completions", self.server_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("llama-server returned {}: {}", status, body);
        }

        let json: serde_json::Value = response.json().await?;
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(content)
    }

    /// Chat completion with image helper
    async fn chat_completion_with_image(
        &self,
        system: &str,
        user: &str,
        image_base64: &str,
    ) -> Result<String> {
        self.chat_completion(system, user, Some(image_base64)).await
    }

    // ========================================================================
    // FALLBACK METHODS (keyword-based, no vision model)
    // ========================================================================

    /// Fallback prompt analysis using keywords
    fn fallback_analyze_prompt(&self, prompt: &str) -> Result<VisualRequirements> {
        Ok(VisualRequirements {
            asset_type: self.extract_asset_type(prompt)?,
            style: self.extract_style(prompt)?,
            complexity: self.estimate_complexity(prompt)?,
            colors: self.extract_colors(prompt)?,
            mood: self.extract_mood(prompt)?,
        })
    }

    /// Fallback image analysis - requires vision model
    fn fallback_analyze_image(&self, _img: &DynamicImage) -> Result<ImageAnalysis> {
        anyhow::bail!("Image analysis requires vision model - no fallback available")
    }

    /// Preprocess image for vision model
    fn preprocess_image(&self, img: DynamicImage) -> Result<DynamicImage> {
        let size = img.dimensions();
        let max_dim = size.0.max(size.1);

        if max_dim > self.config.max_image_size {
            let scale = self.config.max_image_size as f32 / max_dim as f32;
            let new_width = (size.0 as f32 * scale) as u32;
            let new_height = (size.1 as f32 * scale) as u32;
            Ok(img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3))
        } else {
            Ok(img)
        }
    }

    /// Extract asset type from prompt
    fn extract_asset_type(&self, prompt: &str) -> Result<AssetType> {
        let prompt_lower = prompt.to_lowercase();

        if prompt_lower.contains("mesh")
            || prompt_lower.contains("3d")
            || prompt_lower.contains("model")
        {
            Ok(AssetType::Mesh {
                complexity: MeshComplexity::Medium { vertices: 1000 },
                style: MeshStyle::Realistic,
            })
        } else if prompt_lower.contains("material") || prompt_lower.contains("texture") {
            Ok(AssetType::Material {
                material_type: MaterialType::Pbr,
                properties: MaterialProperties::default(),
            })
        } else if prompt_lower.contains("shader") {
            Ok(AssetType::Shader {
                shader_type: ShaderType::Fragment,
                language: ShaderLanguage::Wgsl,
            })
        } else if prompt_lower.contains("scene") {
            Ok(AssetType::Scene {
                elements: Vec::new(),
                environment: EnvironmentSettings::default(),
            })
        } else {
            // Default to mesh
            Ok(AssetType::Mesh {
                complexity: MeshComplexity::Medium { vertices: 1000 },
                style: MeshStyle::Realistic,
            })
        }
    }

    /// Extract style from prompt
    fn extract_style(&self, prompt: &str) -> Result<String> {
        let prompt_lower = prompt.to_lowercase();

        if prompt_lower.contains("realistic") {
            Ok("realistic".to_string())
        } else if prompt_lower.contains("stylized") || prompt_lower.contains("cartoon") {
            Ok("stylized".to_string())
        } else if prompt_lower.contains("low poly") {
            Ok("low_poly".to_string())
        } else if prompt_lower.contains("voxel") {
            Ok("voxel".to_string())
        } else {
            Ok("neutral".to_string())
        }
    }

    /// Estimate complexity from prompt
    fn estimate_complexity(&self, prompt: &str) -> Result<f32> {
        let mut complexity: f32 = 0.5; // Base complexity

        // Increase complexity based on keywords
        if prompt.to_lowercase().contains("detailed") {
            complexity += 0.3;
        }
        if prompt.to_lowercase().contains("intricate") {
            complexity += 0.3;
        }
        if prompt.to_lowercase().contains("simple") {
            complexity -= 0.3;
        }

        Ok(complexity.clamp(0.0_f32, 1.0_f32))
    }

    /// Extract colors from prompt
    fn extract_colors(&self, prompt: &str) -> Result<Vec<bevy::color::Color>> {
        let mut colors = Vec::new();
        let prompt_lower = prompt.to_lowercase();

        // Simple color extraction
        if prompt_lower.contains("red") {
            colors.push(bevy::color::Color::srgb(1.0, 0.0, 0.0));
        }
        if prompt_lower.contains("blue") {
            colors.push(bevy::color::Color::srgb(0.0, 0.0, 1.0));
        }
        if prompt_lower.contains("green") {
            colors.push(bevy::color::Color::srgb(0.0, 1.0, 0.0));
        }
        if prompt_lower.contains("yellow") {
            colors.push(bevy::color::Color::srgb(1.0, 1.0, 0.0));
        }
        if prompt_lower.contains("purple") {
            colors.push(bevy::color::Color::srgb(0.5, 0.0, 0.5));
        }
        if prompt_lower.contains("orange") {
            colors.push(bevy::color::Color::srgb(1.0, 0.5, 0.0));
        }

        Ok(colors)
    }

    /// Extract mood from prompt
    fn extract_mood(&self, prompt: &str) -> Result<String> {
        let prompt_lower = prompt.to_lowercase();

        if prompt_lower.contains("dark") || prompt_lower.contains("somber") {
            Ok("dark".to_string())
        } else if prompt_lower.contains("bright") || prompt_lower.contains("cheerful") {
            Ok("bright".to_string())
        } else if prompt_lower.contains("mysterious") {
            Ok("mysterious".to_string())
        } else if prompt_lower.contains("peaceful") || prompt_lower.contains("calm") {
            Ok("peaceful".to_string())
        } else {
            Ok("neutral".to_string())
        }
    }

    /// Extract dominant colors from image (requires vision model)
    fn extract_dominant_colors(&self, _img: &DynamicImage) -> Result<Vec<bevy::color::Color>> {
        // No fallback - requires vision model for accurate color extraction
        anyhow::bail!("Color extraction requires vision model")
    }

    /// Analyze image composition (requires vision model)
    fn analyze_composition(&self, _img: &DynamicImage) -> Result<CompositionAnalysis> {
        // No fallback - requires vision model for composition analysis
        anyhow::bail!("Composition analysis requires vision model")
    }

    /// Infer style from image (requires vision model)
    #[allow(dead_code)] // Called via parse_image_analysis when vision model is online
    fn infer_style(&self, _img: &DynamicImage) -> Result<String> {
        // No fallback - requires vision model for style inference
        anyhow::bail!("Style inference requires vision model")
    }

    /// Detect objects in image (requires vision model)
    #[allow(dead_code)] // Called via parse_image_analysis when vision model is online
    fn detect_objects(&self, _img: &DynamicImage) -> Result<Vec<DetectedObject>> {
        // No fallback - requires vision model for object detection
        anyhow::bail!("Object detection requires vision model")
    }

    /// Analyze lighting in image (requires vision model)
    fn analyze_lighting(&self, _img: &DynamicImage) -> Result<LightingAnalysis> {
        // No fallback - requires vision model for lighting analysis
        anyhow::bail!("Lighting analysis requires vision model")
    }

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Convert image to base64 string
    fn image_to_base64(&self, img: &DynamicImage) -> Result<String> {
        let mut buffer = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(STANDARD.encode(&buffer))
    }

    /// Load image file and convert to base64
    fn load_image_base64(&self, path: &Path) -> Result<String> {
        let img = image::open(path)?;
        let processed = self.preprocess_image(img)?;
        self.image_to_base64(&processed)
    }

    /// Parse visual requirements from vision model JSON response
    fn parse_visual_requirements(
        &self,
        response: &str,
        original_prompt: &str,
    ) -> Result<VisualRequirements> {
        // Try to parse as JSON, fall back to keyword extraction
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(response) {
            let asset_type = self
                .parse_asset_type_json(&json)
                .unwrap_or_else(|_| self.extract_asset_type(original_prompt).unwrap());
            let style = json["style"].as_str().unwrap_or("neutral").to_string();
            let complexity = json["complexity"].as_f64().unwrap_or(0.5) as f32;
            let colors = self.parse_colors_json(&json).unwrap_or_default();
            let mood = json["mood"].as_str().unwrap_or("neutral").to_string();

            return Ok(VisualRequirements {
                asset_type,
                style,
                complexity,
                colors,
                mood,
            });
        }

        // Fallback to keyword extraction
        self.fallback_analyze_prompt(original_prompt)
    }

    /// Parse image analysis from vision model JSON response
    fn parse_image_analysis(&self, response: &str, img: &DynamicImage) -> Result<ImageAnalysis> {
        // Try to parse as JSON, fall back to basic analysis
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(response) {
            let dominant_colors = self
                .parse_colors_json(&json)
                .unwrap_or_else(|_| self.extract_dominant_colors(img).unwrap());
            let composition = self
                .parse_composition_json(&json)
                .unwrap_or_else(|_| self.analyze_composition(img).unwrap());
            let style = json["style"].as_str().unwrap_or("realistic").to_string();
            let objects = self.parse_objects_json(&json).unwrap_or_default();
            let lighting = self
                .parse_lighting_json(&json)
                .unwrap_or_else(|_| self.analyze_lighting(img).unwrap());

            return Ok(ImageAnalysis {
                dominant_colors,
                composition,
                style,
                objects,
                lighting,
            });
        }

        // Fallback to basic analysis
        self.fallback_analyze_image(img)
    }

    /// Parse asset type from JSON
    fn parse_asset_type_json(&self, json: &serde_json::Value) -> Result<AssetType> {
        let asset_type_str = json["asset_type"].as_str().unwrap_or("mesh").to_lowercase();

        match asset_type_str.as_str() {
            "mesh" | "3d" | "model" => Ok(AssetType::Mesh {
                complexity: MeshComplexity::Medium { vertices: 1000 },
                style: MeshStyle::Realistic,
            }),
            "material" | "texture" => Ok(AssetType::Material {
                material_type: MaterialType::Pbr,
                properties: MaterialProperties::default(),
            }),
            "shader" => Ok(AssetType::Shader {
                shader_type: ShaderType::Fragment,
                language: ShaderLanguage::Wgsl,
            }),
            "scene" => Ok(AssetType::Scene {
                elements: Vec::new(),
                environment: EnvironmentSettings::default(),
            }),
            _ => Ok(AssetType::Mesh {
                complexity: MeshComplexity::Medium { vertices: 1000 },
                style: MeshStyle::Realistic,
            }),
        }
    }

    /// Parse colors from JSON array of hex codes
    fn parse_colors_json(&self, json: &serde_json::Value) -> Result<Vec<bevy::color::Color>> {
        let empty = Vec::new();
        let colors_json = json["colors"]
            .as_array()
            .unwrap_or_else(|| json["dominant_colors"].as_array().unwrap_or(&empty));

        let mut colors = Vec::new();
        for color_val in colors_json {
            if let Some(hex) = color_val.as_str() {
                if let Some(color) = self.parse_hex_color(hex) {
                    colors.push(color);
                }
            }
        }

        Ok(colors)
    }

    /// Parse hex color string to Bevy color
    fn parse_hex_color(&self, hex: &str) -> Option<bevy::color::Color> {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            return Some(bevy::color::Color::srgb_u8(r, g, b));
        }
        None
    }

    /// Parse composition from JSON
    fn parse_composition_json(&self, json: &serde_json::Value) -> Result<CompositionAnalysis> {
        let comp = &json["composition"];
        Ok(CompositionAnalysis {
            rule_of_thirds: comp["rule_of_thirds"].as_f64().unwrap_or(0.7) as f32,
            balance: comp["balance"].as_f64().unwrap_or(0.8) as f32,
            focus_point: bevy::math::Vec2::new(
                comp["focus_point"][0].as_f64().unwrap_or(0.5) as f32,
                comp["focus_point"][1].as_f64().unwrap_or(0.5) as f32,
            ),
        })
    }

    /// Parse detected objects from JSON
    fn parse_objects_json(&self, json: &serde_json::Value) -> Result<Vec<DetectedObject>> {
        let empty = Vec::new();
        let objects_json = json["objects"].as_array().unwrap_or(&empty);

        let mut objects = Vec::new();
        for obj in objects_json {
            objects.push(DetectedObject {
                label: obj["label"].as_str().unwrap_or("unknown").to_string(),
                confidence: obj["confidence"].as_f64().unwrap_or(0.5) as f32,
                bounding_box: bevy::math::Rect::new(
                    obj["bounding_box"][0].as_f64().unwrap_or(0.0) as f32,
                    obj["bounding_box"][1].as_f64().unwrap_or(0.0) as f32,
                    obj["bounding_box"][2].as_f64().unwrap_or(1.0) as f32,
                    obj["bounding_box"][3].as_f64().unwrap_or(1.0) as f32,
                ),
            });
        }

        Ok(objects)
    }

    /// Parse lighting from JSON
    fn parse_lighting_json(&self, json: &serde_json::Value) -> Result<LightingAnalysis> {
        let light = &json["lighting"];
        Ok(LightingAnalysis {
            primary_direction: bevy::math::Vec3::new(
                light["direction"][0].as_f64().unwrap_or(1.0) as f32,
                light["direction"][1].as_f64().unwrap_or(1.0) as f32,
                light["direction"][2].as_f64().unwrap_or(0.5) as f32,
            )
            .normalize(),
            intensity: light["intensity"].as_f64().unwrap_or(0.8) as f32,
            color_temperature: light["temperature"].as_f64().unwrap_or(5500.0) as f32,
            softness: light["softness"].as_f64().unwrap_or(0.6) as f32,
        })
    }
}

/// Visual requirements extracted from prompt
#[derive(Debug, Clone)]
pub struct VisualRequirements {
    pub asset_type: AssetType,
    pub style: String,
    pub complexity: f32,
    pub colors: Vec<bevy::color::Color>,
    pub mood: String,
}

/// Image analysis results
#[derive(Debug, Clone)]
pub struct ImageAnalysis {
    pub dominant_colors: Vec<bevy::color::Color>,
    pub composition: CompositionAnalysis,
    pub style: String,
    pub objects: Vec<DetectedObject>,
    pub lighting: LightingAnalysis,
}

/// Composition analysis
#[derive(Debug, Clone)]
pub struct CompositionAnalysis {
    pub rule_of_thirds: f32,
    pub balance: f32,
    pub focus_point: bevy::math::Vec2,
}

/// Detected object
#[derive(Debug, Clone)]
pub struct DetectedObject {
    pub label: String,
    pub confidence: f32,
    pub bounding_box: bevy::math::Rect,
}

/// Lighting analysis
#[derive(Debug, Clone)]
pub struct LightingAnalysis {
    pub primary_direction: bevy::math::Vec3,
    pub intensity: f32,
    pub color_temperature: f32,
    pub softness: f32,
}
