// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Native SDXL Turbo Image Generation
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        sdxl_native.rs
// PURPOSE:     Pure Rust SDXL Turbo image generation via ONNX Runtime
//
// ARCHITECTURE:
//   text → CLIP tokenizer → text_encoder ONNX → UNet ONNX (4-step Euler)
//     → VAE decoder ONNX → RGB pixels → PNG
//
// This is the TIER 1 (native, offline) image generation path.
// ComfyUI remains as TIER 2 (power user, any model) via HTTP sidecar.
//
// The pipeline follows the exact same pattern as supertonic.rs:
//   Load ONNX sessions → run multi-step inference → output media
//
// CHANGES:
//   2026-04-03  Cascade  Created — native SDXL Turbo ONNX pipeline
//
// ═══════════════════════════════════════════════════════════════════════════════

use anyhow::{Context, Result};
use ndarray::{Array1, Array2, Array4, ArrayD};
use ort::{session::Session, value::Value};
use rand::Rng;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use tokenizers::Tokenizer;
use tracing::{info, warn};

// ============================================================================
// Constants
// ============================================================================

/// SDXL Turbo uses 4 denoising steps (designed for fast generation)
const TURBO_STEPS: usize = 4;

/// Default image dimensions for SDXL Turbo (optimized for 512×512)
const DEFAULT_WIDTH: usize = 512;
const DEFAULT_HEIGHT: usize = 512;

/// Maximum sequence length for CLIP tokenizer
const MAX_SEQ_LEN: usize = 77;

/// CLIP padding token ID
const PAD_TOKEN_ID: u32 = 49407;

/// Latent space dimensions (SDXL uses 4-channel latent at 1/8 resolution)
const LATENT_CHANNELS: usize = 4;
const LATENT_SCALE_FACTOR: usize = 8;

/// VAE scaling factor for SDXL
const VAE_SCALING_FACTOR: f32 = 0.13025;

/// Global flag — set to true if ONNX models not found (don't spam logs)
static SDXL_UNAVAILABLE: AtomicBool = AtomicBool::new(false);

// ============================================================================
// Engine
// ============================================================================

/// Native SDXL Turbo engine — pure Rust, no Python, works offline
pub struct SdxlEngine {
    tokenizer: Tokenizer,
    text_encoder: Session,
    unet: Session,
    vae_decoder: Session,
}

impl SdxlEngine {
    /// Load the SDXL Turbo engine from an ONNX model directory.
    ///
    /// Expected directory structure:
    /// ```
    /// model_dir/
    /// ├── tokenizer/tokenizer.json
    /// ├── text_encoder/model.onnx
    /// ├── unet/model.onnx
    /// └── vae_decoder/model.onnx
    /// ```
    pub fn load<P: AsRef<Path>>(model_dir: P) -> Result<Self> {
        let model_dir = model_dir.as_ref();

        if SDXL_UNAVAILABLE.load(Ordering::Relaxed) {
            anyhow::bail!("SDXL models previously marked unavailable");
        }

        // Check critical files exist
        let tokenizer_path = model_dir.join("tokenizer/tokenizer.json");
        let text_enc_path = model_dir.join("text_encoder/model.onnx");
        let unet_path = model_dir.join("unet/model.onnx");
        let vae_path = model_dir.join("vae_decoder/model.onnx");

        for (name, path) in [
            ("tokenizer", &tokenizer_path),
            ("text_encoder", &text_enc_path),
            ("unet", &unet_path),
            ("vae_decoder", &vae_path),
        ] {
            if !path.exists() {
                SDXL_UNAVAILABLE.store(true, Ordering::Relaxed);
                warn!(
                    "[SDXL] {} not found at {:?} — native image gen disabled. \
                     ComfyUI fallback will be used.",
                    name,
                    path
                );
                anyhow::bail!("Missing SDXL model: {}", name);
            }
        }

        info!("[SDXL] Loading native SDXL Turbo from {}...", model_dir.display());
        let t0 = std::time::Instant::now();

        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {}", e))?;

        let text_encoder = Session::builder()?
            .commit_from_file(&text_enc_path)
            .context("Failed to load text_encoder ONNX")?;

        let unet = Session::builder()?
            .commit_from_file(&unet_path)
            .context("Failed to load UNet ONNX")?;

        let vae_decoder = Session::builder()?
            .commit_from_file(&vae_path)
            .context("Failed to load VAE decoder ONNX")?;

        info!(
            "[SDXL] Engine loaded in {:.1}s (tokenizer + text_encoder + unet + vae_decoder)",
            t0.elapsed().as_secs_f32()
        );

        Ok(SdxlEngine {
            tokenizer,
            text_encoder,
            unet,
            vae_decoder,
        })
    }

    /// Check if native SDXL is available without loading it.
    pub fn is_available() -> bool {
        if SDXL_UNAVAILABLE.load(Ordering::Relaxed) {
            return false;
        }
        let model_dir = Self::default_model_dir();
        model_dir.join("unet/model.onnx").exists()
    }

    /// Default model directory path.
    pub fn default_model_dir() -> std::path::PathBuf {
        dirs::home_dir()
            .unwrap_or_default()
            .join("trinity-models/sdxl-turbo-onnx")
    }

    /// Generate an image from a text prompt.
    ///
    /// Returns PNG bytes.
    pub fn generate(
        &mut self,
        prompt: &str,
        width: Option<usize>,
        height: Option<usize>,
        seed: Option<u64>,
    ) -> Result<Vec<u8>> {
        let width = width.unwrap_or(DEFAULT_WIDTH);
        let height = height.unwrap_or(DEFAULT_HEIGHT);
        let seed = seed.unwrap_or_else(|| rand::thread_rng().gen());

        let t0 = std::time::Instant::now();
        info!("[SDXL] Generating {}×{} image: \"{}\" (seed: {})", width, height, prompt, seed);

        // Step 1: Tokenize
        let token_ids = self.tokenize(prompt)?;

        // Step 2: Text encode
        let text_embeddings = self.encode_text(&token_ids)?;

        // Step 3: Euler denoising loop (4 steps for Turbo)
        // SDXL Turbo uses CFG scale = 1.0, so no unconditional embeddings needed
        let latent_h = height / LATENT_SCALE_FACTOR;
        let latent_w = width / LATENT_SCALE_FACTOR;
        let latents = self.denoise(
            &text_embeddings,
            latent_h,
            latent_w,
            seed,
        )?;

        // Step 4: VAE decode
        let pixels = self.decode_latents(&latents)?;

        // Step 5: Convert to PNG
        let png_bytes = pixels_to_png(&pixels, width, height)?;

        info!(
            "[SDXL] Image generated in {:.1}s ({} bytes PNG)",
            t0.elapsed().as_secs_f32(),
            png_bytes.len()
        );

        Ok(png_bytes)
    }

    /// Tokenize a text prompt using the CLIP tokenizer.
    fn tokenize(&self, text: &str) -> Result<Vec<i64>> {
        let encoding = self.tokenizer.encode(text, true)
            .map_err(|e| anyhow::anyhow!("Tokenization failed: {}", e))?;

        let mut ids: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();

        // Truncate to max length
        if ids.len() > MAX_SEQ_LEN {
            ids.truncate(MAX_SEQ_LEN);
        }

        // Pad to max length
        while ids.len() < MAX_SEQ_LEN {
            ids.push(PAD_TOKEN_ID as i64);
        }

        Ok(ids)
    }

    /// Run text through the CLIP text encoder ONNX model.
    fn encode_text(&mut self, token_ids: &[i64]) -> Result<ArrayD<f32>> {
        let input_ids = Array2::from_shape_vec(
            (1, MAX_SEQ_LEN),
            token_ids.to_vec(),
        )?;

        let input_ids_val = Value::from_array(input_ids)?;

        let outputs = self.text_encoder.run(ort::inputs! {
            "input_ids" => &input_ids_val
        })?;

        // Extract the last hidden state (the text embeddings)
        let (shape, data) = outputs[0].try_extract_tensor::<f32>()?;
        let embeddings = ArrayD::from_shape_vec(
            shape.iter().map(|&s| s as usize).collect::<Vec<_>>(),
            data.to_vec(),
        )?;

        Ok(embeddings)
    }

    /// Run the UNet denoising loop (Euler scheduler, 4 steps for Turbo).
    fn denoise(
        &mut self,
        text_embeddings: &ArrayD<f32>,
        latent_h: usize,
        latent_w: usize,
        seed: u64,
    ) -> Result<Array4<f32>> {
        // Initialize random latents
        let mut latents = random_latents(latent_h, latent_w, seed);

        // Euler scheduler timesteps for SDXL Turbo (4 steps)
        // These are the standard Turbo timesteps
        let timesteps: Vec<f32> = vec![999.0, 749.0, 499.0, 249.0];
        let sigmas = timesteps_to_sigmas(&timesteps);

        for (step, &t) in timesteps.iter().enumerate() {
            // Scale latents for the scheduler
            let sigma = sigmas[step];
            let scaled_latents = scale_model_input(&latents, sigma);

            // Build timestep tensor
            let timestep = Array1::from_vec(vec![t]);

            // Run UNet with text embeddings (CFG scale = 1.0 for Turbo)
            let scaled_val = Value::from_array(scaled_latents.clone())?;
            let t_val = Value::from_array(timestep.clone())?;
            let emb_val = Value::from_array(text_embeddings.clone())?;

            let outputs = self.unet.run(ort::inputs! {
                "sample" => &scaled_val,
                "timestep" => &t_val,
                "encoder_hidden_states" => &emb_val
            })?;

            let (shape, data) = outputs[0].try_extract_tensor::<f32>()?;
            let noise_pred_text = Array4::from_shape_vec(
                (shape[0] as usize, shape[1] as usize, shape[2] as usize, shape[3] as usize),
                data.to_vec(),
            )?;

            // For Turbo, CFG scale = 1.0, so we just use the text-conditioned prediction
            // (no need for classifier-free guidance blending)
            let noise_pred = noise_pred_text;

            // Euler step
            let dt = sigmas[step + 1] - sigma;
            latents = latents + noise_pred * dt;

            info!("[SDXL] Step {}/{} (t={:.0}, σ={:.4})", step + 1, TURBO_STEPS, t, sigma);
        }

        // Scale latents by VAE scaling factor
        latents.mapv_inplace(|v| v / VAE_SCALING_FACTOR);

        Ok(latents)
    }

    /// Decode latent space representation into pixel space using the VAE decoder.
    fn decode_latents(&mut self, latents: &Array4<f32>) -> Result<Array4<f32>> {
        let latent_val = Value::from_array(latents.clone())?;

        let outputs = self.vae_decoder.run(ort::inputs! {
            "latent_sample" => &latent_val
        })?;

        let (shape, data) = outputs[0].try_extract_tensor::<f32>()?;
        let pixels = Array4::from_shape_vec(
            (shape[0] as usize, shape[1] as usize, shape[2] as usize, shape[3] as usize),
            data.to_vec(),
        )?;

        Ok(pixels)
    }
}

// ============================================================================
// Scheduler helpers (Euler Discrete)
// ============================================================================

/// Generate random latent noise from a seed.
fn random_latents(h: usize, w: usize, seed: u64) -> Array4<f32> {
    use rand::SeedableRng;
    use rand_distr::{Distribution, Normal};

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let normal = Normal::new(0.0f32, 1.0).unwrap();

    let total = LATENT_CHANNELS * h * w;
    let data: Vec<f32> = (0..total).map(|_| normal.sample(&mut rng)).collect();

    Array4::from_shape_vec((1, LATENT_CHANNELS, h, w), data)
        .expect("Failed to create latent tensor")
}

/// Convert timesteps to sigma values for the Euler scheduler.
fn timesteps_to_sigmas(timesteps: &[f32]) -> Vec<f32> {
    // Simplified sigma schedule for SDXL Turbo
    // σ(t) = t / 1000 (linear mapping)
    let mut sigmas: Vec<f32> = timesteps.iter().map(|&t| t / 1000.0).collect();
    sigmas.push(0.0); // Final sigma = 0 (fully denoised)
    sigmas
}

/// Scale the model input for the current sigma (Euler scheduler).
fn scale_model_input(latents: &Array4<f32>, sigma: f32) -> Array4<f32> {
    let scale = (sigma * sigma + 1.0).sqrt();
    latents.mapv(|v| v / scale)
}

// ============================================================================
// Image output
// ============================================================================

/// Convert raw pixel tensor (NCHW, f32 [-1, 1]) to PNG bytes.
fn pixels_to_png(pixels: &Array4<f32>, width: usize, height: usize) -> Result<Vec<u8>> {
    // pixels shape: (1, 3, H, W), values in [-1, 1]
    let mut rgb_data: Vec<u8> = Vec::with_capacity(width * height * 3);

    for y in 0..height {
        for x in 0..width {
            for c in 0..3 {
                // Clamp and rescale from [-1, 1] to [0, 255]
                let val = pixels[[0, c, y, x]];
                let byte = ((val + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;
                rgb_data.push(byte);
            }
        }
    }

    // Encode as PNG
    let img = image::RgbImage::from_raw(width as u32, height as u32, rgb_data)
        .context("Failed to create image buffer")?;

    let mut png_bytes: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut png_bytes);
    img.write_to(&mut cursor, image::ImageFormat::Png)
        .context("Failed to encode PNG")?;

    Ok(png_bytes)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_available() {
        // This will return false unless models are installed
        let available = SdxlEngine::is_available();
        eprintln!("SDXL native available: {}", available);
        // Don't assert — depends on model installation
    }

    #[test]
    fn test_random_latents_shape() {
        let latents = random_latents(64, 64, 42);
        assert_eq!(latents.shape(), &[1, 4, 64, 64]);
    }

    #[test]
    fn test_random_latents_deterministic() {
        let a = random_latents(8, 8, 42);
        let b = random_latents(8, 8, 42);
        assert_eq!(a, b, "Same seed should produce same latents");
    }

    #[test]
    fn test_random_latents_different_seeds() {
        let a = random_latents(8, 8, 42);
        let b = random_latents(8, 8, 99);
        assert_ne!(a, b, "Different seeds should produce different latents");
    }

    #[test]
    fn test_timesteps_to_sigmas() {
        let ts = vec![999.0, 749.0, 499.0, 249.0];
        let sigmas = timesteps_to_sigmas(&ts);
        assert_eq!(sigmas.len(), 5); // 4 timesteps + final 0
        assert!((sigmas[0] - 0.999).abs() < 0.001);
        assert_eq!(sigmas[4], 0.0);
    }

    #[test]
    fn test_scale_model_input() {
        let latents = Array4::from_shape_vec((1, 4, 2, 2), vec![1.0; 16]).unwrap();
        let scaled = scale_model_input(&latents, 0.5);
        // scale = sqrt(0.25 + 1.0) = sqrt(1.25) ≈ 1.118
        let expected = 1.0 / (0.5f32 * 0.5 + 1.0).sqrt();
        assert!((scaled[[0, 0, 0, 0]] - expected).abs() < 0.001);
    }

    #[test]
    fn test_pixels_to_png() {
        // Create a tiny 2×2 image
        let pixels = Array4::from_shape_vec(
            (1, 3, 2, 2),
            vec![
                // R channel
                -1.0, 1.0, 0.0, 0.5,
                // G channel
                0.0, -0.5, 1.0, -1.0,
                // B channel
                0.5, 0.0, -1.0, 1.0,
            ],
        )
        .unwrap();

        let png = pixels_to_png(&pixels, 2, 2).unwrap();
        // PNG magic bytes
        assert_eq!(&png[0..4], &[0x89, b'P', b'N', b'G']);
        assert!(png.len() > 20, "PNG should be larger than header");
    }

    #[test]
    fn test_engine_load() {
        let model_dir = SdxlEngine::default_model_dir();
        if !model_dir.join("unet/model.onnx").exists() {
            eprintln!("Skipping SDXL engine test — models not found at {:?}", model_dir);
            return;
        }
        let engine = SdxlEngine::load(&model_dir);
        assert!(engine.is_ok(), "Engine failed to load: {:?}", engine.err());
    }
}
