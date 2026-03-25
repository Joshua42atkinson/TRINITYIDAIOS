// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Creative Bridge (ART Sidecar HTTP Client)
// ═══════════════════════════════════════════════════════════════════════════════
//
// Async HTTP client for talking to Trinity's ART sidecars:
//   • ComfyUI (:8188) — image generation
//   • Trinity server (:3000) — creative API proxy
//
// Runs as a Bevy Resource, polled from systems via channels.
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Status of an ART sidecar service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SidecarStatus {
    Unknown,
    Healthy,
    Unhealthy(String),
}

impl std::fmt::Display for SidecarStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SidecarStatus::Unknown => write!(f, "⚪ Unknown"),
            SidecarStatus::Healthy => write!(f, "🟢 Healthy"),
            SidecarStatus::Unhealthy(e) => write!(f, "🔴 {}", e),
        }
    }
}

/// Tracks health of all ART sidecars.
#[derive(Resource, Debug, Clone)]
pub struct ArtSidecarState {
    /// ComfyUI image generation (:8188)
    pub comfyui: SidecarStatus,
    /// Trinity server (:3000)
    pub trinity: SidecarStatus,
    /// LLM server (:8080)
    pub llm: SidecarStatus,
}

impl Default for ArtSidecarState {
    fn default() -> Self {
        Self {
            comfyui: SidecarStatus::Unknown,
            trinity: SidecarStatus::Unknown,
            llm: SidecarStatus::Unknown,
        }
    }
}

/// Request to generate an image via the creative pipeline.
#[derive(Debug, Clone, Serialize)]
pub struct ImageGenRequest {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub width: u32,
    pub height: u32,
}

/// Response from image generation.
#[derive(Debug, Clone, Deserialize)]
pub struct ImageGenResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub url: Option<String>,
}

/// Request to generate tempo/procedural audio via creative pipeline.
#[derive(Debug, Clone, Serialize)]
pub struct TempoGenRequest {
    pub prompt: String,
    pub style: Option<String>,
    pub duration_secs: u32,
}

/// Response from tempo generation.
#[derive(Debug, Clone, Deserialize)]
pub struct TempoGenResponse {
    pub success: bool,
    pub audio_path: Option<String>,
    pub message: String,
}

/// Shared mailbox for async results coming back from HTTP calls.
#[derive(Resource, Clone)]
pub struct CreativeMailbox {
    pub image_results: Arc<Mutex<Vec<ImageGenResponse>>>,
    pub tempo_results: Arc<Mutex<Vec<TempoGenResponse>>>,
    pub pending_count: Arc<Mutex<u32>>,
}

impl Default for CreativeMailbox {
    fn default() -> Self {
        Self {
            image_results: Arc::new(Mutex::new(Vec::new())),
            tempo_results: Arc::new(Mutex::new(Vec::new())),
            pending_count: Arc::new(Mutex::new(0)),
        }
    }
}

/// Plugin that manages ART sidecar health polling.
pub struct CreativeBridgePlugin;

impl Plugin for CreativeBridgePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ArtSidecarState::default())
            .insert_resource(CreativeMailbox::default())
            .insert_resource(CreativePollTimer(Timer::from_seconds(
                5.0,
                TimerMode::Repeating,
            )))
            .add_systems(Update, poll_sidecar_health);
    }
}

#[derive(Resource)]
struct CreativePollTimer(Timer);

/// Periodically check sidecar health via HTTP.
fn poll_sidecar_health(
    time: Res<Time>,
    mut timer: ResMut<CreativePollTimer>,
    mut sidecar_state: ResMut<ArtSidecarState>,
    mailbox: Res<CreativeMailbox>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    // Clone what we need for the async task
    let trinity_status = sidecar_state.trinity.clone();
    let comfyui_status = sidecar_state.comfyui.clone();
    let llm_status = sidecar_state.llm.clone();
    let _ = (trinity_status, comfyui_status, llm_status);

    // Spawn async health checks
    // NOTE: We use a non-blocking approach — fire HTTP requests on a thread pool
    // and write results back via shared state. In a real deployment, this would use
    // bevy's AsyncComputeTaskPool, but for now we use std::thread to avoid
    // runtime conflicts between bevy and tokio.

    let state_clone = ArtSidecarState {
        comfyui: sidecar_state.comfyui.clone(),
        trinity: sidecar_state.trinity.clone(),
        llm: sidecar_state.llm.clone(),
    };

    // Use a simple blocking approach on a background thread
    let results: Arc<Mutex<Option<ArtSidecarState>>> = Arc::new(Mutex::new(None));
    let results_clone = results.clone();

    std::thread::spawn(move || {
        let _state = state_clone;
        let mut new_state = ArtSidecarState::default();

        // Check Trinity server
        match ureq_get("http://localhost:3000/api/health") {
            Ok(_) => new_state.trinity = SidecarStatus::Healthy,
            Err(e) => new_state.trinity = SidecarStatus::Unhealthy(e),
        }

        // Check ComfyUI
        match ureq_get("http://localhost:8188/system_stats") {
            Ok(_) => new_state.comfyui = SidecarStatus::Healthy,
            Err(e) => new_state.comfyui = SidecarStatus::Unhealthy(e),
        }

        // Check LLM
        match ureq_get("http://localhost:8080/health") {
            Ok(_) => new_state.llm = SidecarStatus::Healthy,
            Err(e) => new_state.llm = SidecarStatus::Unhealthy(e),
        }

        if let Ok(mut lock) = results_clone.lock() {
            *lock = Some(new_state);
        }
    });

    // Try to read results (non-blocking) — they'll be ready next frame
    if let Ok(mut lock) = results.try_lock() {
        if let Some(new_state) = lock.take() {
            *sidecar_state = new_state;
        }
    }

    // Also check the creative mailbox for completed image results
    if let Ok(lock) = mailbox.pending_count.try_lock() {
        if *lock > 0 {
            info!("🎨 {} image generation(s) pending", *lock);
        }
    }
}

/// Simple blocking HTTP GET — used from background threads only.
fn ureq_get(url: &str) -> Result<String, String> {
    // Use reqwest blocking client with a short timeout
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let resp = client
        .get(url)
        .send()
        .map_err(|e| format!("connection refused: {}", e))?;

    if resp.status().is_success() {
        Ok("ok".to_string())
    } else {
        Err(format!("HTTP {}", resp.status()))
    }
}

/// Fire an image generation request to the Trinity creative endpoint.
/// This is called from UI code and runs asynchronously.
pub fn request_image_generation(mailbox: &CreativeMailbox, request: ImageGenRequest) {
    let results = mailbox.image_results.clone();
    let pending = mailbox.pending_count.clone();

    // Increment pending count
    if let Ok(mut count) = pending.lock() {
        *count += 1;
    }

    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build();

        let client = match client {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to create HTTP client: {}", e);
                if let Ok(mut count) = pending.lock() {
                    *count = count.saturating_sub(1);
                }
                return;
            }
        };

        let resp = client
            .post("http://localhost:3000/api/creative/image")
            .json(&request)
            .send();

        match resp {
            Ok(r) => {
                if let Ok(gen_resp) = r.json::<ImageGenResponse>() {
                    if let Ok(mut lock) = results.lock() {
                        lock.push(gen_resp);
                    }
                }
            }
            Err(e) => {
                tracing::error!("Image generation request failed: {}", e);
            }
        }

        if let Ok(mut count) = pending.lock() {
            *count = count.saturating_sub(1);
        }
    });
}

/// Fire a tempo generation request to the Trinity creative endpoint.
pub fn request_tempo_generation(mailbox: &CreativeMailbox, request: TempoGenRequest) {
    let results = mailbox.tempo_results.clone();
    let pending = mailbox.pending_count.clone();

    if let Ok(mut count) = pending.lock() {
        *count += 1;
    }

    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build();

        let client = match client {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to create HTTP client: {}", e);
                if let Ok(mut count) = pending.lock() {
                    *count = count.saturating_sub(1);
                }
                return;
            }
        };

        let resp = client
            .post("http://localhost:3000/api/creative/tempo")
            .json(&request)
            .send();

        match resp {
            Ok(r) => {
                if let Ok(gen_resp) = r.json::<TempoGenResponse>() {
                    if let Ok(mut lock) = results.lock() {
                        lock.push(gen_resp);
                    }
                }
            }
            Err(e) => {
                tracing::error!("Tempo generation request failed: {}", e);
            }
        }

        if let Ok(mut count) = pending.lock() {
            *count = count.saturating_sub(1);
        }
    });
}
