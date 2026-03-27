// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Sidecar Manager
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:     comfyui.rs
// PURPOSE:  ComfyUI workflow orchestration for image/video generation sidecar
// BIBLE:    Car 11 — YOKE (ART Creative Pipeline, §11.2)
//
// ═══════════════════════════════════════════════════════════════════════════════

//! ComfyUI HTTP Bridge
//!
//! Connects Artist and Visionary sidecars to ComfyUI for real image generation.
//! ComfyUI runs as a separate Python process on localhost:8188.

use serde::Deserialize;
use serde_json::json;
use tracing::info;

/// ComfyUI client for image generation
pub struct ComfyUIClient {
    http: reqwest::Client,
    base_url: String,
}

impl ComfyUIClient {
    pub fn new(base_url: &str) -> Self {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("Failed to build HTTP client");
        Self {
            http,
            base_url: base_url.to_string(),
        }
    }

    /// Check if ComfyUI server is running
    #[allow(dead_code)] // Used in tests; will be wired to /api/health creative section
    pub async fn is_healthy(&self) -> bool {
        match self
            .http
            .get(format!("{}/system_stats", self.base_url))
            .timeout(std::time::Duration::from_secs(3))
            .send()
            .await
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Generate an image from a text prompt using SDXL Turbo
    pub async fn generate_image(
        &self,
        prompt: &str,
        negative_prompt: Option<&str>,
    ) -> anyhow::Result<Vec<u8>> {
        info!("Generating image: {}", prompt);

        // Build ComfyUI workflow JSON for SDXL Turbo
        let workflow = self.build_sdxl_turbo_workflow(prompt, negative_prompt.unwrap_or(""));

        // Submit workflow
        let prompt_response: PromptResponse = self
            .http
            .post(format!("{}/prompt", self.base_url))
            .json(&json!({ "prompt": workflow }))
            .send()
            .await?
            .json()
            .await?;

        let prompt_id = prompt_response.prompt_id;
        info!("ComfyUI prompt submitted: {}", prompt_id);

        // Poll for completion
        let image_data = self.poll_for_completion(&prompt_id).await?;

        Ok(image_data)
    }

    /// Build SDXL Turbo workflow JSON
    fn build_sdxl_turbo_workflow(&self, prompt: &str, negative_prompt: &str) -> serde_json::Value {
        json!({
            "3": {
                "class_type": "KSampler",
                "inputs": {
                    "seed": chrono::Utc::now().timestamp() as u64,
                    "steps": 4,
                    "cfg": 1.0,
                    "sampler_name": "euler",
                    "scheduler": "normal",
                    "denoise": 1.0,
                    "model": ["4", 0],
                    "positive": ["6", 0],
                    "negative": ["7", 0],
                    "latent_image": ["5", 0]
                }
            },
            "4": {
                "class_type": "CheckpointLoaderSimple",
                "inputs": {
                    "ckpt_name": "sd_xl_turbo_1.0_fp16.safetensors"
                }
            },
            "5": {
                "class_type": "EmptyLatentImage",
                "inputs": {
                    "width": 512,
                    "height": 512,
                    "batch_size": 1
                }
            },
            "6": {
                "class_type": "CLIPTextEncode",
                "inputs": {
                    "text": prompt,
                    "clip": ["4", 1]
                }
            },
            "7": {
                "class_type": "CLIPTextEncode",
                "inputs": {
                    "text": negative_prompt,
                    "clip": ["4", 1]
                }
            },
            "8": {
                "class_type": "VAEDecode",
                "inputs": {
                    "samples": ["3", 0],
                    "vae": ["4", 2]
                }
            },
            "9": {
                "class_type": "SaveImage",
                "inputs": {
                    "filename_prefix": "trinity",
                    "images": ["8", 0]
                }
            }
        })
    }

    /// Poll ComfyUI for workflow completion and retrieve image
    async fn poll_for_completion(&self, prompt_id: &str) -> anyhow::Result<Vec<u8>> {
        for _ in 0..60 {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            let history: serde_json::Value = self
                .http
                .get(format!("{}/history/{}", self.base_url, prompt_id))
                .send()
                .await?
                .json()
                .await?;

            if let Some(outputs) = history[prompt_id]["outputs"].as_object() {
                // Find SaveImage node output
                for (_, node_output) in outputs {
                    if let Some(images) = node_output["images"].as_array() {
                        if let Some(first_image) = images.first() {
                            let filename = first_image["filename"].as_str().unwrap();
                            let subfolder = first_image["subfolder"].as_str().unwrap_or("");

                            // Download image
                            let image_url = format!(
                                "{}/view?filename={}&subfolder={}&type=output",
                                self.base_url, filename, subfolder
                            );

                            let image_bytes = self
                                .http
                                .get(&image_url)
                                .send()
                                .await?
                                .bytes()
                                .await?
                                .to_vec();

                            info!("Image generated: {} bytes", image_bytes.len());
                            return Ok(image_bytes);
                        }
                    }
                }
            }
        }

        anyhow::bail!("ComfyUI workflow timed out after 120s")
    }
}

#[derive(Debug, Deserialize)]
struct PromptResponse {
    prompt_id: String,
}

impl Default for ComfyUIClient {
    fn default() -> Self {
        Self::new("http://127.0.0.1:8188")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_comfyui_health() {
        let client = ComfyUIClient::default();
        let healthy = client.is_healthy().await;
        println!("ComfyUI healthy: {}", healthy);
    }
}
