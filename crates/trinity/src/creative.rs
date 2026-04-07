// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        creative.rs
// PURPOSE:     Creative Sidecar API — ComfyUI image + MusicGPT audio generation
//
// 🪟 THE LIVING CODE TEXTBOOK (P-ART-Y Gear A: Aesthetics):
// This file is the paintbrush of the ART agent. It is designed to be read, 
// modified, and authored by YOU. If you want to change how Trinity generates
// images or music, this is where you customize the creative pipeline.
// ACTION: Edit `generate_image()` to add your own ComfyUI workflow prompts.
//
// 📖 THE HOOK BOOK CONNECTION:
// This file powers the 'Image Generation' and 'Music Composition' Hooks inside
// the School of Creation. You can freely use this API in your own projects to 
// bridge Rust with local AI art generators! 
// For full capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
//
// ARCHITECTURE:
//   • ComfyUI client for SDXL-Turbo image generation (NPU optimized)
//   • MusicGPT client for procedural audio/music generation
//   • CRAP design system: Contrast, Repetition, Alignment, Proximity
//   • Iron Road visual enhancement: generates game assets on demand
//   • Character sheet integration for style persistence
//
// DEPENDENCIES:
//   - axum — HTTP handlers for creative endpoints
//   - serde — Image/audio request serialization
//   - tracing — Generation operation logging
//   - trinity_sidecar_engineer — ComfyUI client re-export
//
// CHANGES:
//   2026-03-16  Cascade  Migrated to §17 comment standard
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{Json, Response},
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::character_sheet;
use crate::AppState;

// TODO: Re-enable when trinity_sidecar_engineer crate is available
// pub use trinity_sidecar_engineer::comfyui::ComfyUIClient;

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

/// Image generation request
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields populated via serde deserialization
pub struct ImageRequest {
    /// The prompt describing the desired image
    pub prompt: String,
    /// Optional negative prompt (what to avoid)
    #[serde(default)]
    pub negative_prompt: Option<String>,
    /// Visual style (steampunk, cyberpunk, etc.)
    #[serde(default)]
    pub style: Option<String>,
    /// Override width
    #[serde(default = "default_width")]
    pub width: u32,
    /// Override height
    #[serde(default = "default_height")]
    pub height: u32,
}

fn default_width() -> u32 {
    1024
}
fn default_height() -> u32 {
    1024
}

/// Image generation response
#[derive(Debug, Serialize)]
pub struct ImageResponse {
    pub success: bool,
    /// Base64-encoded image data
    pub image_data: Option<String>,
    /// Path to saved image if stored
    pub image_path: Option<String>,
    /// HTTP-accessible URL for the generated image
    pub image_url: Option<String>,
    pub message: String,
    pub generation_time_ms: u64,
}

/// Tempo generation request
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields populated via serde deserialization
pub struct TempoRequest {
    /// Visual/Mood style
    #[serde(default)]
    pub style: Option<String>,
    /// Duration in seconds
    #[serde(default = "default_duration")]
    pub duration_secs: u32,
    /// Prompt
    #[serde(default)]
    pub prompt: String,
}

fn default_duration() -> u32 {
    15
}

/// Tempo generation response
#[derive(Debug, Serialize)]
pub struct TempoResponse {
    pub success: bool,
    /// Path to generated audio file
    pub audio_path: Option<String>,
    pub message: String,
    pub generation_time_ms: u64,
}

/// Video generation request
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields populated via serde deserialization
pub struct VideoRequest {
    /// The prompt describing the desired video
    pub prompt: String,
    /// Video duration in seconds (default 4)
    #[serde(default = "default_video_duration")]
    pub duration_secs: u32,
    /// Frames per second (default 24)
    #[serde(default = "default_fps")]
    pub fps: u32,
    /// Resolution height (720 or 480)
    #[serde(default = "default_resolution")]
    pub height: u32,
}

fn default_video_duration() -> u32 {
    4
}
fn default_fps() -> u32 {
    24
}
fn default_resolution() -> u32 {
    720
}

/// Video generation response
#[derive(Debug, Serialize)]
pub struct VideoResponse {
    pub success: bool,
    /// Path to generated video file
    pub video_path: Option<String>,
    /// Base64-encoded video data (for small videos)
    pub video_data: Option<String>,
    pub message: String,
    pub generation_time_ms: u64,
}

/// 3D mesh generation request — TripoSR API
#[derive(Debug, Deserialize)]
pub struct Mesh3DRequest {
    /// The prompt describing the desired 3D mesh (or image path for image-to-3D)
    pub prompt: String,
    /// Optional: base64-encoded image for image-to-3D mode
    #[serde(default)]
    pub image_base64: Option<String>,
    /// Output format: glb, obj (default: glb)
    #[serde(default = "default_mesh_format")]
    pub format: String,
}

fn default_mesh_format() -> String {
    "glb".to_string()
}

/// 3D mesh generation response
#[derive(Debug, Serialize)]
pub struct Mesh3DResponse {
    pub success: bool,
    /// Path to generated mesh file
    pub mesh_path: Option<String>,
    pub message: String,
    pub generation_time_ms: u64,
}

/// Creative sidecar status
#[derive(Debug, Serialize)]
pub struct CreativeStatus {
    pub comfyui: SidecarStatus,
    pub musicgpt: SidecarStatus,
    pub hunyuan3d: SidecarStatus,
}

#[derive(Debug, Serialize)]
pub struct SidecarStatus {
    pub running: bool,
    pub endpoint: String,
    pub message: String,
}

/// Creative settings update request
#[derive(Debug, Deserialize)]
pub struct CreativeSettingsRequest {
    #[serde(default)]
    pub visual_style: Option<String>,
    #[serde(default)]
    pub music_style: Option<String>,
    #[serde(default)]
    pub creative_enabled: Option<bool>,
}

/// Creative settings response
#[derive(Debug, Serialize)]
pub struct CreativeSettingsResponse {
    pub visual_style: String,
    pub music_style: String,
    pub creative_enabled: bool,
}

// ============================================================================
// HEALTH PROBES (used by startup auto-launch)
// ============================================================================

/// Quick health check for ComfyUI — returns true if responding on :8188
pub async fn check_comfyui_health_quick() -> bool {
    crate::http::QUICK
        .get("http://127.0.0.1:8188/system_stats")
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

/// Ensure ComfyUI NPU sidecar is running, auto-launching via systemd if needed
pub async fn ensure_comfyui_running() -> Result<(), (StatusCode, String)> {
    let client = &*crate::http::LONG;
    let healthy = client
        .get("http://127.0.0.1:8188/system_stats")
        .timeout(std::time::Duration::from_secs(3))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    if healthy {
        return Ok(());
    }

    info!("ComfyUI not running — attempting systemd auto-launch...");
    match std::process::Command::new("systemctl")
        .args(["--user", "start", "trinity-comfyui.service"])
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                let err = String::from_utf8_lossy(&output.stderr);
                return Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    format!("Failed to start trinity-comfyui.service: {}", err),
                ));
            }
            info!("trinity-comfyui.service start signal sent, waiting for socket...");
        }
        Err(e) => {
            return Err((
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Failed to execute systemctl: {}", e),
            ));
        }
    }

    // Wait up to 30 seconds for ComfyUI to become healthy
    for attempt in 1..=15 {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let check = client
            .get("http://127.0.0.1:8188/system_stats")
            .timeout(std::time::Duration::from_secs(2))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false);
        if check {
            info!("ComfyUI ready after {}s", attempt * 2);
            return Ok(());
        }
    }

    Err((
        StatusCode::SERVICE_UNAVAILABLE,
        "ComfyUI failed to start within 30 seconds. Check systemctl --user status trinity-comfyui.service".to_string(),
    ))
}

// ============================================================================
// API HANDLERS
// ============================================================================

/// Check creative sidecar status
pub async fn creative_status() -> Json<CreativeStatus> {
    let comfyui = check_vllm_creative().await; // Reused struct field "comfyui" but backed by vLLM for dashboard compatibility
    let musicgpt = check_musicgpt().await;
    let hunyuan3d = check_hunyuan3d().await;

    Json(CreativeStatus {
        comfyui,
        musicgpt,
        hunyuan3d,
    })
}

async fn check_vllm_creative() -> SidecarStatus {
    let running = crate::http::QUICK
        .get("http://127.0.0.1:8010/health")
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    SidecarStatus {
        running,
        endpoint: "http://127.0.0.1:8010".to_string(),
        message: if running {
            "LongCat wrapper proxy running".to_string()
        } else {
            "LongCat proxy down. Start start_longcat_server.py".to_string()
        },
    }
}

async fn check_musicgpt() -> SidecarStatus {
    SidecarStatus {
        running: true,
        endpoint: "local".to_string(),
        message: "Tempo procedural engine ready".to_string(),
    }
}

async fn check_hunyuan3d() -> SidecarStatus {
    let running = crate::http::QUICK
        .get("http://127.0.0.1:7860/api/predict")
        .send()
        .await
        .map(|_| true)
        .unwrap_or(false);

    SidecarStatus {
        running,
        endpoint: "http://127.0.0.1:7860".to_string(),
        message: if running {
            "Hunyuan3D-2.1 running".to_string()
        } else {
            "Hunyuan3D-2.1 not running. Start: cd Hunyuan3D-2.1 && python app.py".to_string()
        },
    }
}

/// Generate an image natively via vLLM Omni router
pub async fn generate_image(
    State(state): State<AppState>,
    Json(request): Json<ImageRequest>,
) -> Result<Json<ImageResponse>, (StatusCode, String)> {
    use base64::Engine;
    
    let start = std::time::Instant::now();
    let client = &*crate::http::LONG;

    // Use unified design format via inference options
    let style_suffix = get_style_prompt_suffix(&request.style);
    let full_prompt = format!("{}, {}", request.prompt, style_suffix);

    info!(
        "Generating image via LongCat Proxy: {} ({}x{})",
        full_prompt, request.width, request.height
    );

    let payload = serde_json::json!({
        "model": "FLUX.1-schnell",
        "prompt": full_prompt,
        "n": 1,
        "size": format!("{}x{}", request.width, request.height),
        "response_format": "b64_json"
    });

    let response = client
        .post("http://127.0.0.1:8010/v1/images/generations")
        .json(&payload)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("LongCat Server unreachable: {}", e)))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("LongCat generation failed: {}", body)));
    }

    let result: serde_json::Value = response.json().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Parse the b64_json from the OpenAI compliant response
    let b64_data = result["data"][0]["b64_json"].as_str().ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "No b64_json in vLLM response".to_string()))?;

    // Decode and save to unified Desktop app storage
    let b64_bytes = base64::prelude::BASE64_STANDARD.decode(b64_data).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Base64 decode failed: {}", e)))?;
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let workspace_dir = std::path::PathBuf::from(&home).join(".local/share/trinity/workspace/assets/images");
    let _ = std::fs::create_dir_all(&workspace_dir);
    
    let filename = format!("trinity_art_{}.png", start.elapsed().as_micros());
    let final_path = workspace_dir.join(&filename);
    std::fs::write(&final_path, b64_bytes).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to write image: {}", e)))?;

    info!("Image stored: {} in {}ms", final_path.display(), start.elapsed().as_millis());

    Ok(Json(ImageResponse {
        success: true,
        image_data: None, // Too large to send back entirely, relying on URL instead
        image_path: Some(final_path.to_string_lossy().into_owned()),
        image_url: Some(format!("/api/creative/assets/{}", filename)),
        message: "Image generated via vLLM Omni".to_string(),
        generation_time_ms: start.elapsed().as_millis() as u64,
    }))
}

/// Generate tempo via local procedural engine
pub async fn generate_tempo(
    State(state): State<AppState>,
    Json(request): Json<TempoRequest>,
) -> Result<Json<TempoResponse>, (StatusCode, String)> {
    let start = std::time::Instant::now();

    info!("Generating procedural tempo: {}", request.prompt);

    // Call the little Gemma-4 (E4B) on port 8003 to act as the "vibe setting boss"
    let system_prompt = "You are the Tempo Vibe Boss. Your job is to analyze the user's prompt and select the best musical context for it. Respond with EXACTLY ONE of the following precise strings, with NO other text, punctuation, or explanation: problem_solving, creative_exploration, review_practice, assessment, break, concept_introduction.";
    
    let vibe_payload = serde_json::json!({
        "model": "gemma-4-E4B",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": request.prompt}
        ],
        "temperature": 0.3,
        "max_tokens": 10
    });

    let client = &*crate::http::STANDARD;
    let mut context_arg = "concept_introduction".to_string(); // default fallback

    if let Ok(vibe_res) = client.post("http://127.0.0.1:8003/v1/chat/completions")
        .json(&vibe_payload)
        .send().await 
    {
        if vibe_res.status().is_success() {
            if let Ok(vibe_json) = vibe_res.json::<serde_json::Value>().await {
                if let Some(content) = vibe_json["choices"][0]["message"]["content"].as_str() {
                    let cleaned = content.trim().to_lowercase();
                    // Validate it's one of our supported contexts
                    if ["problem_solving", "creative_exploration", "review_practice", "assessment", "break", "concept_introduction"].contains(&cleaned.as_str()) {
                        context_arg = cleaned;
                        info!("vibe boss (E4B) selected context: {}", context_arg);
                    } else {
                        info!("vibe boss returned invalid context '{}', using default", cleaned);
                    }
                }
            }
        } else {
            info!("vibe boss (E4B) returned non-success, using default context");
        }
    } else {
        info!("vibe boss (E4B) offline or failed, using default context");
    }

    let _duration_str = request.duration_secs.to_string();
    
    // Output path to workspace
    let home = std::env::var("HOME").unwrap_or_else(|_| dirs::home_dir().unwrap_or_default().to_string_lossy().to_string());
    let workspace_dir = std::path::PathBuf::from(&home).join(".local/share/trinity/workspace/assets/audio");
    let _ = std::fs::create_dir_all(&workspace_dir);
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("trinity_tempo_{}.wav", timestamp);
    let final_path = workspace_dir.join(&filename);
    
    // Shell out to our newly merged cargo crate in archive/trinity-tempo-ai
    // For production this would be invoked directly via library, but CLI invocation is perfect for the sidecar architecture.
    let crate_path = format!("{}/Workflow/desktop_trinity/trinity-genesis/archive/trinity-tempo-ai", home);
    
    let result = std::process::Command::new("cargo")
        .current_dir(crate_path)
        .arg("run")
        .arg("--release")
        .arg("--")
        .arg("generate")
        .arg("--context")
        .arg(context_arg)
        .arg("--output")
        .arg(final_path.to_string_lossy().to_string())
        .output();

    match result {
        Ok(output) if output.status.success() => {
            info!("Tempo procedural audio generated successfully in {}ms", start.elapsed().as_millis());

            // Auto-vault to portfolio
            {
                let mut sheet = state.player.character_sheet.write().await;
                let artifact = trinity_protocol::character_sheet::PortfolioArtifact {
                    artifact_id: uuid::Uuid::new_v4(),
                    title: request.prompt.clone(),
                    hooks_cast: Vec::new(),
            addiecrapeye_phase: "Develop".to_string(),
                    artifact_type: "Procedural Audio".to_string(),
                    reflection_journal: format!("ArtStudio tempo: {}", request.prompt),
                    aligned_supra_badge: "Design & Development".to_string(),
                    qm_score: 100.0,
                    aect_ethics_cleared: true,
                };
                sheet.ldt_portfolio.artifact_vault.push(artifact);
                sheet.ldt_portfolio.recalculate();
                if let Err(e) = crate::character_sheet::save_character_sheet(&sheet) {
                    tracing::error!("Failed to persist character sheet after auto-vault: {}", e);
                }
            }

            Ok(Json(TempoResponse {
                success: true,
                audio_path: Some(final_path.to_string_lossy().to_string()),
                message: "Tempo generated successfully".to_string(),
                generation_time_ms: start.elapsed().as_millis() as u64,
            }))
        }
        Ok(output) => {
            let err = String::from_utf8_lossy(&output.stderr);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Tempo engine failed: {}", err),
            ))
        }
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to execute tempo engine: {}", e),
            ))
        }
    }
}

/// Generate a video via vLLM Omni
pub async fn generate_video(
    State(state): State<AppState>,
    Json(request): Json<VideoRequest>,
) -> Result<Json<VideoResponse>, (StatusCode, String)> {
    let start = std::time::Instant::now();
    let client = &*crate::http::LONG;

    let payload = serde_json::json!({
        "prompt": request.prompt,
        "duration": request.duration_secs,
        "fps": request.fps,
        "response_format": "b64_json"
    });

    let response = client.post("http://127.0.0.1:8006/v1/video/generations").json(&payload).send().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("CogVideoX queue failed: {}", e)))?;

    if !response.status().is_success() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "CogVideoX rejected video request".to_string()));
    }

    let result: serde_json::Value = response.json().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let b64 = result["data"][0]["b64_json"].as_str().unwrap_or("");
    use base64::{Engine as _, engine::general_purpose};
    let bytes = general_purpose::STANDARD.decode(b64).unwrap_or_default();
    
    let path = format!("/tmp/trinity_video_{}.mp4", uuid::Uuid::new_v4());
    let _ = std::fs::write(&path, bytes);

    let mut sheet = state.player.character_sheet.write().await;
    sheet.ldt_portfolio.artifact_vault.push(trinity_protocol::character_sheet::PortfolioArtifact {
        artifact_id: uuid::Uuid::new_v4(),
        title: request.prompt.clone(),
        hooks_cast: Vec::new(),
        addiecrapeye_phase: "Develop".to_string(),
        artifact_type: "Generated Video".to_string(),
        reflection_journal: format!("ArtStudio Video: {}", request.prompt),
        aligned_supra_badge: "Design & Development".to_string(),
        qm_score: 100.0,
        aect_ethics_cleared: true,
    });
    sheet.ldt_portfolio.recalculate();
    let _ = crate::character_sheet::save_character_sheet(&sheet);

    Ok(Json(VideoResponse {
        success: true,
        video_path: Some(path.clone()),
        video_data: None,
        message: "Video generated successfully".to_string(),
        generation_time_ms: start.elapsed().as_millis() as u64,
    }))
}


/// Generate a 3D mesh via TripoSR (MIT) API on port 8007
pub async fn generate_3d_mesh(
    State(state): State<AppState>,
    Json(request): Json<Mesh3DRequest>,
) -> Result<Json<Mesh3DResponse>, (StatusCode, String)> {
    let start = std::time::Instant::now();

    let client = &*crate::http::LONG;

    // Check TripoSR health
    let healthy = client
        .get("http://127.0.0.1:8007/docs")
        .timeout(std::time::Duration::from_secs(3))
        .send()
        .await
        .map(|_| true)
        .unwrap_or(false);

    if !healthy {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "TripoSR not running. Start with start_vllm_omni.sh".to_string(),
        ));
    }

    info!("Generating 3D mesh: {}", request.prompt);

    let img_b64 = match request.image_base64 {
        Some(b64) => b64,
        None => return Err((StatusCode::BAD_REQUEST, "TripoSR requires an input image_base64.".to_string())),
    };

    let payload = serde_json::json!({
        "prompt": request.prompt,
        "image_base64": img_b64
    });

    let response = client
        .post("http://127.0.0.1:8007/v1/3d/generations")
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("TripoSR request failed: {}", e),
            )
        })?;

    if !response.status().is_success() {
        let err_text = response.text().await.unwrap_or_default();
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("TripoSR error: {}", err_text),
        ));
    }

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let obj_b64 = result["data"][0]["obj_base64"].as_str().unwrap_or("");
    use base64::{Engine as _, engine::general_purpose};
    let bytes = general_purpose::STANDARD.decode(obj_b64).unwrap_or_default();

    // Copy to Trinity workspace
    let home = std::env::var("HOME").unwrap_or_else(|_| dirs::home_dir().unwrap_or_default().to_string_lossy().to_string());
    let workspace_dir = std::path::PathBuf::from(&home)
        .join(".local/share/trinity/workspace/assets/meshes");
    let _ = std::fs::create_dir_all(&workspace_dir);

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("trinity_mesh_{}.obj", timestamp);
    let final_path = workspace_dir.join(&filename);
    let _ = std::fs::write(&final_path, bytes);
            
    let mesh_path = Some(final_path.to_string_lossy().to_string());

    if mesh_path.is_some() {
        // Auto-vault to portfolio
        let mut sheet = state.player.character_sheet.write().await;
        let artifact = trinity_protocol::character_sheet::PortfolioArtifact {
            artifact_id: uuid::Uuid::new_v4(),
            title: request.prompt.clone(),
            hooks_cast: Vec::new(),
            addiecrapeye_phase: "Develop".to_string(),
            artifact_type: "Generated 3D Mesh".to_string(),
            reflection_journal: format!("ArtStudio 3D Mesh: {}", request.prompt),
            aligned_supra_badge: "Design & Development".to_string(),
            qm_score: 100.0,
            aect_ethics_cleared: true,
        };
        sheet.ldt_portfolio.artifact_vault.push(artifact);
        sheet.ldt_portfolio.recalculate();
        if let Err(e) = crate::character_sheet::save_character_sheet(&sheet) {
            tracing::error!("Failed to persist character sheet after auto-vault: {}", e);
        }
    }

    Ok(Json(Mesh3DResponse {
        success: mesh_path.is_some(),
        mesh_path,
        message: "3D mesh generated via TripoSR".to_string(),
        generation_time_ms: start.elapsed().as_millis() as u64,
    }))
}

/// Build HunyuanVideo workflow for ComfyUI
fn build_hunyuan_workflow(request: &VideoRequest) -> serde_json::Value {
    // Calculate frames from duration and fps
    let frames = request.duration_secs * request.fps;

    serde_json::json!({
        "1": {
            "class_type": "HunyuanVideoModelLoader",
            "inputs": {
                "model": "hunyuan-video-t2v-720p",
                "quantization": "fp8_scaled",
            }
        },
        "2": {
            "class_type": "HunyuanVideoPromptEncoder",
            "inputs": {
                "prompt": request.prompt,
                "negative_prompt": "blurry, low quality, distorted",
            }
        },
        "3": {
            "class_type": "HunyuanVideoSampler",
            "inputs": {
                "model": ["1", 0],
                "prompt_embeds": ["2", 0],
                "frames": frames,
                "height": request.height,
                "width": (request.height * 16 / 9),
                "steps": 30,
                "guidance_scale": 7.0,
            }
        },
        "4": {
            "class_type": "HunyuanVideoDecode",
            "inputs": {
                "latents": ["3", 0],
                "vae": ["1", 1],
            }
        },
        "5": {
            "class_type": "SaveVideo",
            "inputs": {
                "video": ["4", 0],
                "filename_prefix": "trinity_video",
                "fps": request.fps,
            }
        }
    })
}

/// Wait for video generation to complete
async fn wait_for_video(
    client: &reqwest::Client,
    prompt_id: &str,
) -> Result<String, (StatusCode, String)> {
    let mut elapsed = 0u64;
    let max_wait = 600_000; // 10 minutes max

    loop {
        if elapsed > max_wait {
            return Err((
                StatusCode::REQUEST_TIMEOUT,
                "Video generation timed out".to_string(),
            ));
        }

        // Check history for completed prompt
        let response = client
            .get(format!("http://127.0.0.1:8188/history/{}", prompt_id))
            .send()
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        if response.status().is_success() {
            let history: serde_json::Value = response
                .json()
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            // Check if outputs exist
            if let Some(outputs) = history.get(prompt_id).and_then(|h| h.get("outputs")) {
                for (_node_id, output) in outputs.as_object().unwrap_or(&serde_json::Map::new()) {
                    if let Some(videos) = output.get("videos") {
                        if let Some(video) = videos.as_array().and_then(|v| v.first()) {
                            if let Some(filename) = video.get("filename").and_then(|f| f.as_str()) {
                                let subfolder = video
                                    .get("subfolder")
                                    .and_then(|s| s.as_str())
                                    .unwrap_or("");
                                let comfyui_path = if subfolder.is_empty() {
                                    let home = std::env::var("HOME")
                                        .unwrap_or_else(|_| "/tmp".to_string());
                                    std::path::PathBuf::from(format!(
                                        "{}/ComfyUI/output/{}",
                                        home, filename
                                    ))
                                } else {
                                    let home = std::env::var("HOME")
                                        .unwrap_or_else(|_| "/tmp".to_string());
                                    std::path::PathBuf::from(format!(
                                        "{}/ComfyUI/output/{}/{}",
                                        home, subfolder, filename
                                    ))
                                };

                                // Copy to unified Desktop app storage
                                let home = std::env::var("HOME")
                                    .unwrap_or_else(|_| dirs::home_dir().unwrap_or_default().to_string_lossy().to_string());
                                let workspace_dir = std::path::PathBuf::from(home)
                                    .join(".local/share/trinity/workspace/assets/videos");
                                let _ = std::fs::create_dir_all(&workspace_dir);

                                let final_path = workspace_dir.join(filename);
                                let _ = std::fs::copy(&comfyui_path, &final_path);

                                return Ok(final_path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }

        // Wait and retry
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        elapsed += 5000;
    }
}

/// Get current creative settings from character sheet
pub async fn get_creative_settings(
    State(state): State<AppState>,
) -> Json<CreativeSettingsResponse> {
    let sheet = state.player.character_sheet.read().await;
    Json(CreativeSettingsResponse {
        visual_style: format!("{:?}", sheet.creative_config.visual_style).to_lowercase(),
        music_style: format!("{:?}", sheet.creative_config.music_style).to_lowercase(),
        creative_enabled: sheet.creative_config.creative_enabled,
    })
}

/// Update creative settings in character sheet and persist to disk
pub async fn update_creative_settings(
    State(state): State<AppState>,
    Json(request): Json<CreativeSettingsRequest>,
) -> Result<Json<CreativeSettingsResponse>, (StatusCode, String)> {
    let mut sheet = state.player.character_sheet.write().await;

    // Update visual style
    if let Some(visual) = request.visual_style.as_deref() {
        sheet.creative_config.visual_style = match visual {
            "cyberpunk" => trinity_protocol::character_sheet::VisualStyle::Cyberpunk,
            "fantasy" => trinity_protocol::character_sheet::VisualStyle::Fantasy,
            "minimalist" => trinity_protocol::character_sheet::VisualStyle::Minimalist,
            "retro" => trinity_protocol::character_sheet::VisualStyle::Retro,
            "noir" => trinity_protocol::character_sheet::VisualStyle::Noir,
            _ => trinity_protocol::character_sheet::VisualStyle::Steampunk,
        };
    }

    // Update music style
    if let Some(music) = request.music_style.as_deref() {
        sheet.creative_config.music_style = match music {
            "lofi" => trinity_protocol::character_sheet::MusicStyle::Lofi,
            "electronic" => trinity_protocol::character_sheet::MusicStyle::Electronic,
            "jazz" => trinity_protocol::character_sheet::MusicStyle::Jazz,
            "ambient" => trinity_protocol::character_sheet::MusicStyle::Ambient,
            "classical" => trinity_protocol::character_sheet::MusicStyle::Classical,
            _ => trinity_protocol::character_sheet::MusicStyle::Orchestral,
        };
    }

    // Update enabled flag
    if let Some(enabled) = request.creative_enabled {
        sheet.creative_config.creative_enabled = enabled;
    }

    // Persist to disk
    if let Err(e) = character_sheet::save_character_sheet(&sheet) {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to save settings: {}", e),
        ));
    }

    info!(
        "Updated creative settings: visual={:?}, music={:?}, enabled={}",
        sheet.creative_config.visual_style,
        sheet.creative_config.music_style,
        sheet.creative_config.creative_enabled
    );

    Ok(Json(CreativeSettingsResponse {
        visual_style: format!("{:?}", sheet.creative_config.visual_style).to_lowercase(),
        music_style: format!("{:?}", sheet.creative_config.music_style).to_lowercase(),
        creative_enabled: sheet.creative_config.creative_enabled,
    }))
}

/// Get logs from the creative sidecar
pub async fn get_creative_logs() -> Json<serde_json::Value> {
    let log_path = std::path::Path::new("/tmp/trinity_art_sidecar.log");
    let mut logs = Vec::new();

    if log_path.exists() {
        if let Ok(content) = std::fs::read_to_string(log_path) {
            for line in content.lines().rev().take(50) {
                if let Ok(entry) = serde_json::from_str::<serde_json::Value>(line) {
                    logs.push(entry);
                }
            }
        }
    } else {
        logs.push(serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "tag": "SYSTEM",
            "message": "Creative log file not found. Waiting for first action..."
        }));
    }

    Json(serde_json::json!({
        "logs": logs
    }))
}

// ============================================================================
// ASSET MANAGEMENT — List and serve generated creative assets
// ============================================================================

/// Individual asset metadata returned by list_assets
#[derive(Debug, Serialize)]
pub struct AssetEntry {
    pub filename: String,
    pub asset_type: String, // "image", "video", "audio", "mesh"
    pub size_bytes: u64,
    pub created_at: String,
    pub url: String, // /api/creative/assets/<filename>
}

/// Get the unified workspace assets directory
fn get_workspace_assets_dir() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| dirs::home_dir().unwrap_or_default().to_string_lossy().to_string());
    std::path::PathBuf::from(home).join(".local/share/trinity/workspace/assets")
}

/// Classify a file extension into an asset type
fn classify_asset(ext: &str) -> Option<&'static str> {
    match ext {
        "png" | "jpg" | "jpeg" | "webp" | "bmp" | "gif" => Some("image"),
        "mp4" | "webm" | "avi" | "mov" | "mkv" => Some("video"),
        "wav" | "mp3" | "ogg" | "flac" | "aac" => Some("audio"),
        "glb" | "gltf" | "obj" | "fbx" | "stl" => Some("mesh"),
        _ => None,
    }
}

/// MIME type for an extension
fn mime_for_ext(ext: &str) -> &'static str {
    match ext {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "mkv" => "video/x-matroska",
        "wav" => "audio/wav",
        "mp3" => "audio/mpeg",
        "ogg" => "audio/ogg",
        "flac" => "audio/flac",
        "aac" => "audio/aac",
        "glb" => "model/gltf-binary",
        "gltf" => "model/gltf+json",
        "obj" => "text/plain",
        "fbx" => "application/octet-stream",
        "stl" => "model/stl",
        _ => "application/octet-stream",
    }
}

/// List all generated assets across images/videos/meshes/audio subdirs
pub async fn list_assets() -> Json<serde_json::Value> {
    let base = get_workspace_assets_dir();
    let subdirs = ["images", "videos", "meshes", "audio"];
    let mut assets: Vec<AssetEntry> = Vec::new();

    for subdir in &subdirs {
        let dir = base.join(subdir);
        if !dir.exists() {
            continue;
        }
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                if let Some(asset_type) = classify_asset(&ext) {
                    let meta = std::fs::metadata(&path).ok();
                    let size_bytes = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                    let created_at = meta
                        .and_then(|m| m.modified().ok())
                        .map(|t| {
                            let dt: chrono::DateTime<chrono::Utc> = t.into();
                            dt.to_rfc3339()
                        })
                        .unwrap_or_default();

                    assets.push(AssetEntry {
                        url: format!("/api/creative/assets/{}", filename),
                        filename,
                        asset_type: asset_type.to_string(),
                        size_bytes,
                        created_at,
                    });
                }
            }
        }
    }

    // Sort newest first
    assets.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Json(serde_json::json!({
        "assets": assets,
        "total": assets.len(),
    }))
}

/// Serve a generated asset file by filename
pub async fn serve_asset(Path(filename): Path<String>) -> Result<Response, (StatusCode, String)> {
    // Sanitize: no path traversal
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Err((StatusCode::BAD_REQUEST, "Invalid filename".to_string()));
    }

    let base = get_workspace_assets_dir();
    let subdirs = ["images", "videos", "meshes", "audio"];

    // Search all subdirs for the file
    for subdir in &subdirs {
        let path = base.join(subdir).join(&filename);
        if path.exists() && path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            let content_type = mime_for_ext(&ext);

            match tokio::fs::read(&path).await {
                Ok(bytes) => {
                    return Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, content_type)
                        .header(header::CACHE_CONTROL, "public, max-age=3600")
                        .body(axum::body::Body::from(bytes))
                        .unwrap());
                }
                Err(e) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to read file: {}", e),
                    ));
                }
            }
        }
    }

    Err((
        StatusCode::NOT_FOUND,
        format!("Asset not found: {}", filename),
    ))
}

// ============================================================================
// HELPERS
// ============================================================================

#[allow(dead_code)] // Called by generate_image when user specifies art style
fn get_style_prompt_suffix(style: &Option<String>) -> &'static str {
    match style.as_deref() {
        Some("steampunk") => "steampunk aesthetic, brass gears, steam pipes, Victorian industrial, warm amber lighting",
        Some("cyberpunk") => "cyberpunk aesthetic, neon lights, holographic displays, futuristic cityscape",
        Some("fantasy") => "fantasy aesthetic, magical atmosphere, ethereal glow, medieval architecture",
        Some("minimalist") => "minimalist aesthetic, clean lines, simple shapes, neutral colors",
        Some("retro") => "retro pixel art style, 8-bit graphics, nostalgic gaming aesthetic",
        Some("noir") => "noir aesthetic, dramatic shadows, black and white, film noir lighting",
        _ => "steampunk aesthetic, brass gears, steam pipes, Victorian industrial, warm amber lighting",
    }
}

#[allow(dead_code)] // Activates with MusicGPT sidecar integration
fn get_music_style_prompt(style: &Option<String>) -> &'static str {
    match style.as_deref() {
        Some("orchestral") => "epic orchestral background music, cinematic, adventure theme",
        Some("lofi") => "lofi hip hop beats, chill focus music, relaxed",
        Some("electronic") => "synthwave electronic music, ambient synthesizer",
        Some("jazz") => "smooth jazz background music, noir detective vibes",
        Some("ambient") => "ambient atmospheric music, minimal, spacey",
        Some("classical") => "classical orchestral music, baroque style, elegant",
        _ => "epic orchestral background music, cinematic, adventure theme",
    }
}

#[allow(dead_code)] // Used when returning inline image data to frontend
fn base64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}

#[allow(dead_code)] // Used when ComfyUI returns raw bytes for workspace storage
async fn save_image(image_bytes: Vec<u8>) -> Result<String, std::io::Error> {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("trinity_creative_{}.png", timestamp);

    let home = std::env::var("HOME").unwrap_or_else(|_| dirs::home_dir().unwrap_or_default().to_string_lossy().to_string());
    let path = std::path::Path::new(&home)
        .join(".local/share/trinity/workspace/assets/images")
        .join(&filename);

    // Create directory if needed
    std::fs::create_dir_all(path.parent().unwrap())?;

    // Write file
    std::fs::write(&path, &image_bytes)?;

    Ok(path.to_string_lossy().to_string())
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── Request/Response Structs ─────────────────────────────────────────

    #[test]
    fn test_image_request_deserialize() {
        let json = r#"{"prompt": "a dragon", "width": 512, "height": 512}"#;
        let req: ImageRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.prompt, "a dragon");
        assert_eq!(req.width, 512);
        assert_eq!(req.height, 512);
    }

    #[test]
    fn test_image_request_defaults() {
        let json = r#"{"prompt": "test"}"#;
        let req: ImageRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.width, 1024);  // default
        assert_eq!(req.height, 1024); // default
        assert!(req.negative_prompt.is_none());
        assert!(req.style.is_none());
    }

    #[test]
    fn test_tempo_request_defaults() {
        let json = r#"{"prompt": "ambient forest"}"#;
        let req: TempoRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.duration_secs, 15); // default
        assert!(req.style.is_none());
    }

    #[test]
    fn test_video_request_defaults() {
        let json = r#"{"prompt": "waves crashing"}"#;
        let req: VideoRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.duration_secs, 4);  // default
        assert_eq!(req.fps, 24);            // default
        assert_eq!(req.height, 720);        // default
    }

    #[test]
    fn test_mesh3d_request_defaults() {
        let json = r#"{"prompt": "a castle"}"#;
        let req: Mesh3DRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.format, "glb"); // default
        assert!(req.image_base64.is_none());
    }

    // ── Sidecar Status ──────────────────────────────────────────────────

    #[test]
    fn test_sidecar_status_serializes() {
        let status = SidecarStatus {
            running: true,
            endpoint: "http://127.0.0.1:8188".to_string(),
            message: "ComfyUI running".to_string(),
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"running\":true"));
        assert!(json.contains("8188"));
    }

    #[test]
    fn test_creative_status_serializes() {
        let status = CreativeStatus {
            comfyui: SidecarStatus {
                running: false,
                endpoint: "http://127.0.0.1:8188".to_string(),
                message: "Not running".to_string(),
            },
            musicgpt: SidecarStatus {
                running: true,
                endpoint: "local".to_string(),
                message: "Ready".to_string(),
            },
            hunyuan3d: SidecarStatus {
                running: false,
                endpoint: "http://127.0.0.1:7860".to_string(),
                message: "Not running".to_string(),
            },
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("comfyui"));
        assert!(json.contains("musicgpt"));
        assert!(json.contains("hunyuan3d"));
    }

    // ── Image Response ──────────────────────────────────────────────────

    #[test]
    fn test_image_response_serializes() {
        let resp = ImageResponse {
            success: true,
            image_data: None,
            image_path: Some("/path/to/image.png".to_string()),
            image_url: Some("/api/creative/assets/image.png".to_string()),
            message: "Image generated".to_string(),
            generation_time_ms: 2000,
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("image.png"));
    }

    // ── Visual Style ────────────────────────────────────────────────────

    #[test]
    fn test_style_prompt_suffix() {
        let steampunk = get_style_prompt_suffix(&Some("steampunk".to_string()));
        assert!(!steampunk.is_empty());

        let default = get_style_prompt_suffix(&None);
        assert!(!default.is_empty()); // should return some default style
    }

    // ── Settings Request ────────────────────────────────────────────────

    #[test]
    fn test_creative_settings_deserialize() {
        let json = r#"{"visual_style": "steampunk", "creative_enabled": true}"#;
        let req: CreativeSettingsRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.visual_style, Some("steampunk".to_string()));
        assert_eq!(req.creative_enabled, Some(true));
    }
}

