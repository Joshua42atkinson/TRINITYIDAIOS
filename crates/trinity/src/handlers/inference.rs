use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use tracing::{info, warn};
use std::path::PathBuf;

use crate::AppState;
use crate::inference_router;
use crate::hotel_manager;

fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home/joshua"))
}

fn gguf_model_path(filename: &str) -> PathBuf {
    home_dir().join("trinity-models/gguf").join(filename)
}

#[allow(dead_code)]
fn conductor_model_path(filename: &str) -> PathBuf {
    home_dir().join("ai_models/gguf/conductor").join(filename)
}

#[allow(dead_code)]
fn voice_model_path(filename: &str) -> PathBuf {
    home_dir()
        .join("trinity-models/voice/personaplex-7b")
        .join(filename)
}

#[allow(dead_code)]
fn onnx_model_path(relative_path: &str) -> PathBuf {
    home_dir().join("trinity-models/onnx").join(relative_path)
}

fn safetensors_model_path(repo: &str) -> PathBuf {
    home_dir().join("trinity-models/vllm").join(repo).join("config.json")
}

pub(crate) fn installed_model_inventory() -> Vec<(&'static str, PathBuf)> {
    vec![
        (
            "🧠 TRINITY: Gemma 4 E4B AWQ [15GB, vision+text]",
            safetensors_model_path("gemma-4-E4B-it-AWQ-4bit"),
        ),
        (
            "🧠 TRINITY (Large): Gemma 4 26B-A4B AWQ [17GB, vision+text]",
            safetensors_model_path("gemma-4-26B-A4B-it-AWQ-4bit"),
        ),
        (
            "⚙️ Yardmaster: Qwen3-Coder-REAP-25B-A3B [Q4_K_M GGUF]",
            gguf_model_path("Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf"),
        ),
        (
            "🎨 Image Gen: FLUX.1-schnell [Q4_K_S GGUF, 6.4GB]",
            gguf_model_path("flux1-schnell-Q4_K_S.gguf"),
        ),
        (
            "🎵 Music Gen: ACE-Step v1 3.5B [safetensors, 7.8GB]",
            home_dir().join("trinity-models/safetensors/ACE-Step-v1-3.5B/config.json"),
        ),
        (
            "🎤 Voice: Kokoro TTS [ONNX, 338MB]",
            home_dir().join("trinity-models/tts/kokoro/config.json"),
        ),
        (
            "👂 STT: Whisper Base [ONNX, 280MB]",
            home_dir().join("trinity-models/stt/whisper-base/config.json"),
        ),
        (
            "🔍 RAG: all-MiniLM-L6-v2 [ONNX, 23MB]",
            home_dir().join("trinity-models/onnx/embeddings/model.onnx"),
        ),
        (
            "📐 ONNX: Qwen2.5-7B [ONNX AMD, 8.3GB]",
            home_dir().join("trinity-models/onnx/qwen-2.5-7b-onnx-amd/config.json"),
        ),
    ]
}

pub async fn list_models() -> Json<serde_json::Value> {
    let mut models = Vec::new();

    for (name, path) in installed_model_inventory() {
        if path.exists() {
            let size: u64 = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            models.push(serde_json::json!({
                "name": name,
                "path": path.display().to_string(),
                "size_gb": format!("{:.1}", size as f64 / 1_073_741_824.0),
            }));
        }
    }

    Json(serde_json::json!({ "models": models }))
}

/// Get the active inference URL and backend info
pub async fn active_model(State(state): State<AppState>) -> Json<serde_json::Value> {
    let router = state.inference_router.read().await;
    let url = router.active_url().to_string();
    let name = router.active_name().to_string();
    let healthy = router.is_healthy();
    let supports_tools = router.supports_tools();
    let supports_vision = router.supports_vision();
    drop(router);
    Json(serde_json::json!({
        "url": url,
        "backend": name,
        "healthy": healthy,
        "supports_tools": supports_tools,
        "supports_vision": supports_vision,
    }))
}

/// Model status endpoint — polled by Yardmaster model bar every 5s
/// Returns mounted/unmounted status plus model name from the inference router.
pub async fn model_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let router = state.inference_router.read().await;
    let healthy = router.is_healthy();
    let backend = router.active_backend();
    let model_name = backend.and_then(|b| b.model_name.clone());
    let inference_mode = router.active_name().to_string();
    let base_url = router.active_url().to_string();
    drop(router);

    Json(serde_json::json!({
        "status": if healthy { "mounted" } else { "unmounted" },
        "inference_mode": inference_mode,
        "model_name": model_name,
        "model_path": model_name,
        "base_url": base_url,
    }))
}

/// Switch the active inference backend at runtime
#[derive(Debug, Deserialize)]
pub struct SwitchModelRequest {
    /// Backend name (e.g. "tempo-e4b", "pete-coder", "recycler-dense") OR URL
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub backend: Option<String>,
}

pub async fn switch_model(
    State(state): State<AppState>,
    Json(request): Json<SwitchModelRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut router = state.inference_router.write().await;
    let old_name = router.active_name().to_string();
    let old_url = router.active_url().to_string();

    // Try switching by backend name first, then by URL
    if let Some(ref name) = request.backend {
        if !router.switch_backend(name) {
            return Err((
                StatusCode::NOT_FOUND,
                format!("Backend '{}' not found", name),
            ));
        }
    } else if !request.url.is_empty() {
        if !crate::auth::is_url_allowed_for_inference(&request.url) {
            return Err((
                StatusCode::FORBIDDEN,
                "Inference URL exfiltration prevented: URL must point to localhost or Tailscale host only".to_string(),
            ));
        }
        router.set_active_url(request.url.clone());
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Provide 'backend' name or 'url'".to_string(),
        ));
    }

    let new_url = router.active_url().to_string();
    let new_name = router.active_name().to_string();
    info!(
        "Model hot-swap: {} ({}) -> {} ({})",
        old_name, old_url, new_name, new_url
    );

    Ok(Json(serde_json::json!({
        "previous": { "backend": old_name, "url": old_url },
        "current": { "backend": new_name, "url": new_url },
    })))
}

/// GET /api/inference/status — full inference router status
pub async fn inference_status(State(state): State<AppState>) -> Json<inference_router::RouterStatus> {
    let router = state.inference_router.read().await;
    Json(router.status())
}

/// GET /api/inference/fleet — Deprecated, returns empty status
pub async fn fleet_status_endpoint() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "deprecated", "fleet": []}))
}

/// GET /api/inference/hotel — Hotel Swap zone status
/// Returns current occupant, mode, available guests, and swap zone state
pub async fn hotel_status_endpoint() -> Json<serde_json::Value> {
    let occupant = hotel_manager::current_occupant().await;
    let mode = hotel_manager::current_hotel_mode().await;

    Json(serde_json::json!({
        "occupant": occupant.map(|r| format!("{}", r)),
        "mode": mode,
        "day_mode": {
            "P": {"port": 8000, "model": "DiffusionGemma 26B A4B", "status": "always-on (podman)", "threads": "0-3", "vram_mb": 42496},
            "R": {"port": 8002, "model": "Qwythos-9B-Claude-Mythos", "status": "hotel-swapped", "threads": "4-15", "vram_mb": 13312},
            "A": {"port": 8188, "model": "ComfyUI (FLUX/SDXL/ACE)", "status": "hotel-swapped", "threads": "16-23", "vram_mb": 16384},
            "T": {"port": 8001, "model": "zen-musician", "status": "hotel-swapped", "threads": "24-27", "vram_mb": 6144}
        },
        "modes": {
            "team": "P + R + T all running (~61GB VRAM, threads 0-27)",
            "solo": "P only (~42GB VRAM, threads 4-27 free for IDE agents)",
            "swap": "One model at a time (legacy)",
            "closed": "All models off (night shift or full IDE)"
        },
        "note": "POST /hotel/team for full power, /hotel/solo for IDE agents, /hotel/close for night shift."
    }))
}

/// POST /api/inference/hotel/swap — Trigger a Hotel swap by role
/// Body: { "role": "P" } or { "role": "R" } or { "role": "A" } or { "role": "checkout" }
pub async fn hotel_swap_endpoint(
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let role_str = body["role"]
        .as_str()
        .ok_or((StatusCode::BAD_REQUEST, "Missing 'role' field".to_string()))?;

    if role_str == "checkout" || role_str == "none" {
        hotel_manager::hotel_checkout().await;
        return Ok(Json(serde_json::json!({
            "status": "checked_out",
            "message": "Hotel swap zone cleared — Tempo handles all tasks",
        })));
    }

    let target = match role_str {
        "P" | "programming" | "Programming" => inference_router::PartyRole::Programming,
        "R" | "reasoning" | "Reasoning" => inference_router::PartyRole::Reasoning,
        "A" | "aesthetics" | "Aesthetics" => inference_router::PartyRole::Aesthetics,
        other => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid role '{}'. Use P, R, A, or checkout", other),
            ))
        }
    };

    let result = hotel_manager::hotel_swap(target).await;

    Ok(Json(serde_json::json!({
        "status": if result.success { "swapped" } else { "failed" },
        "role": format!("{}", result.requested),
        "url": result.url,
        "duration_secs": result.duration.as_secs_f64(),
        "message": result.message,
    })))
}

/// POST /api/inference/hotel/studio — Start Studio mode
/// Launches all always_resident guests (P + A). Tier 2 models load on demand via ComfyUI.
/// Uses ~59GB VRAM for Tier 1, up to 53GB for Tier 2. Threads 0-7 for residents.
pub async fn hotel_studio_endpoint() -> Json<serde_json::Value> {
    let results = hotel_manager::hotel_studio_start().await;
    let success_count = results.iter().filter(|r| r.success).count();
    let total = results.len();
    let results_json: Vec<serde_json::Value> = results
        .iter()
        .map(|r| serde_json::json!({
            "role": format!("{}", r.requested),
            "success": r.success,
            "url": r.url,
            "duration_secs": r.duration.as_secs_f64(),
            "message": r.message,
        }))
        .collect();

    Json(serde_json::json!({
        "status": if success_count == total { "studio_ready" } else { "partial" },
        "models_online": success_count,
        "models": results_json,
        "message": format!("{}/{} residents online", success_count, total),
    }))
}

/// POST /api/inference/hotel/solo — Solo mode for IDE agents
/// Keeps P running, evicts A. Frees threads 4-27 and ~17GB VRAM.
pub async fn hotel_solo_endpoint() -> Json<serde_json::Value> {
    hotel_manager::hotel_solo().await;
    Json(serde_json::json!({
        "status": "solo",
        "message": "P only — A evicted. Threads 4-27 free for IDE agents. ~17GB VRAM freed."
    }))
}

/// POST /api/inference/hotel/close — Close the Hotel for shift change
/// Evicts all Hotel guests, frees all VRAM. Transition to night mode (LM Studio).
pub async fn hotel_close_endpoint() -> Json<serde_json::Value> {
    hotel_manager::hotel_close_all().await;
    Json(serde_json::json!({
        "status": "closed",
        "message": "Hotel closed — all guests evicted, VRAM freed. Ready for night shift (LM Studio)."
    }))
}

/// POST /api/inference/hotel/open — Open the Hotel for day shift
/// Resets occupant state. First swap call will launch the needed model.
pub async fn hotel_open_endpoint() -> Json<serde_json::Value> {
    hotel_manager::hotel_open().await;
    Json(serde_json::json!({
        "status": "open",
        "message": "Hotel open — day shift ready. Start DiffusionGemma with start-diffusiongemma.sh."
    }))
}

/// GET /api/inference/resources — System resources + model memory estimates
pub async fn inference_resources_endpoint(State(state): State<AppState>) -> Json<serde_json::Value> {
    use sysinfo::System;
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total_ram_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let used_ram_gb = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let available_ram_gb = total_ram_gb - used_ram_gb;

    let router = state.inference_router.read().await;
    let router_status = router.status();

    let model_profiles = serde_json::json!([
        {
            "id": "tempo-e4b",
            "name": "Gemma 4 E4B AWQ",
            "role": "T (Tempo)",
            "port": 8001,
            "ram_gb": 6.0,
            "context_len": 131072,
            "quantization": "AWQ INT4",
            "capabilities": ["text", "vision", "tools"],
            "status": if router_status.backends.iter().any(|b| b.base_url.contains("8001")) { "online" } else { "offline" },
            "always_resident": true,
            "description": "Always-on fast brain — Socratic chat, NPC dialog, TTS routing"
        },
        {
            "id": "pete-coder",
            "name": "Gemma 4 26B A4B AWQ",
            "role": "P (Programming)",
            "port": 8000,
            "ram_gb": 16.0,
            "context_len": 262144,
            "quantization": "AWQ INT4",
            "capabilities": ["code", "tools", "vision"],
            "status": if router_status.backends.iter().any(|b| b.base_url.contains("8000")) { "online" } else { "offline" },
            "always_resident": false,
            "description": "MoE coding brain — code gen, tool calling, React/Rust scaffolding"
        },
        {
            "id": "recycler-dense",
            "name": "Gemma 4 31B AWQ",
            "role": "R (Reasoning)",
            "port": 8002,
            "ram_gb": 18.0,
            "context_len": 262144,
            "quantization": "AWQ INT4",
            "capabilities": ["reasoning", "tools"],
            "status": if router_status.backends.iter().any(|b| b.base_url.contains("8002")) { "online" } else { "offline" },
            "always_resident": false,
            "description": "Dense reasoning — evaluation, QM rubrics, PEARL alignment"
        },
        {
            "id": "janus-pro",
            "name": "Janus Pro 7B",
            "role": "A (Aesthetics)",
            "port": 8003,
            "ram_gb": 4.0,
            "context_len": 0,
            "quantization": "FP16",
            "capabilities": ["vision", "image-critique"],
            "status": if router_status.backends.iter().any(|b| b.base_url.contains("8188")) { "online" } else { "offline" },
            "always_resident": false,
            "description": "Vision-Language CRAP critique of UI screenshots"
        },
        {
            "id": "flux-schnell",
            "name": "FLUX.1-schnell",
            "role": "A (Aesthetics)",
            "port": 0,
            "ram_gb": 7.0,
            "context_len": 0,
            "quantization": "GGUF Q4",
            "capabilities": ["image-gen"],
            "status": "embedded",
            "always_resident": true,
            "description": "2D image generation via embedded Candle crate"
        },
        {
            "id": "kokoro-tts",
            "name": "Kokoro TTS",
            "role": "Voice",
            "port": 0,
            "ram_gb": 1.0,
            "context_len": 0,
            "quantization": "ONNX",
            "capabilities": ["tts"],
            "status": "embedded",
            "always_resident": true,
            "description": "Text-to-speech via embedded ORT crate"
        },
        {
            "id": "nomic-embed",
            "name": "nomic-embed-text-v1.5",
            "role": "Embeddings",
            "port": 0,
            "ram_gb": 1.0,
            "context_len": 8192,
            "quantization": "ONNX",
            "capabilities": ["embeddings"],
            "status": "embedded",
            "always_resident": true,
            "description": "RAG semantic search via embedded ORT crate"
        },
        {
            "id": "acestep-1.5",
            "name": "ACE-Step 1.5",
            "role": "T (Tempo/Music)",
            "port": 8008,
            "ram_gb": 7.8,
            "context_len": 0,
            "quantization": "BF16",
            "capabilities": ["music-gen"],
            "status": "offline",
            "always_resident": false,
            "description": "Ambient music generation — optional Python sidecar"
        }
    ]);

    Json(serde_json::json!({
        "system": {
            "total_ram_gb": (total_ram_gb * 10.0).round() / 10.0,
            "used_ram_gb": (used_ram_gb * 10.0).round() / 10.0,
            "available_ram_gb": (available_ram_gb * 10.0).round() / 10.0,
            "ram_percent": ((used_ram_gb / total_ram_gb * 100.0) * 10.0).round() / 10.0,
            "gpu": "AMD Strix Halo — 128GB Unified LPDDR5x (shared CPU+GPU)",
            "npu": "Ryzen AI — XDNA2 (not yet bound via ONNX RT)"
        },
        "router": router_status,
        "models": model_profiles,
        "constraints": {
            "note": "RAM is unified — CPU and GPU share 128GB LPDDR5x. Loading one model reduces available RAM for all.",
            "max_concurrent_estimate_gb": available_ram_gb,
            "recommendation": if available_ram_gb > 50.0 {
                "Headroom available — can load additional models"
            } else if available_ram_gb > 20.0 {
                "Moderate load — choose models carefully"
            } else {
                "RAM is tight — unload before loading new models"
            }
        }
    }))
}

/// POST /api/inference/switch — switch active backend by name
#[derive(Debug, Deserialize)]
pub struct InferenceSwitchRequest {
    pub backend: String,
}

pub async fn inference_switch(
    State(state): State<AppState>,
    Json(request): Json<InferenceSwitchRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut router = state.inference_router.write().await;
    if router.switch_backend(&request.backend) {
        Ok(Json(serde_json::json!({
            "active": router.active_name(),
            "url": router.active_url(),
        })))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("Backend '{}' not found", request.backend),
        ))
    }
}

/// POST /api/inference/refresh — re-probe all backends
pub async fn inference_refresh(State(state): State<AppState>) -> Json<inference_router::RouterStatus> {
    let mut router = state.inference_router.write().await;
    router.auto_detect().await;
    Json(router.status())
}

/// POST /api/inference/start — launch T (Tempo) from web UI
pub async fn inference_start_endpoint(State(state): State<AppState>) -> Json<serde_json::Value> {
    let workspace = std::env::current_dir().unwrap_or_default();

    let script = {
        let new = workspace.join("scripts/launch/launch_tempo_e4b.sh");
        let legacy = workspace.join("scripts/launch/launch_pete.sh");
        if new.exists() {
            new
        } else if legacy.exists() {
            legacy
        } else {
            return Json(serde_json::json!({
                "status": "error",
                "message": "No Tempo launch script found (tried launch_tempo_e4b.sh, launch_pete.sh)"
            }));
        }
    };

    match tokio::process::Command::new("bash")
        .arg(&script)
        .current_dir(&workspace)
        .spawn()
    {
        Ok(_child) => {
            info!("🚀 T (Tempo) launch initiated via {}", script.display());
            
            let state_clone = state.clone();
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                let mut router = state_clone.inference_router.write().await;
                router.auto_detect().await;
            });
            Json(serde_json::json!({
                "status": "starting",
                "message": "T (Tempo — Gemma 4 E4B AWQ) launch initiated on port 8001",
                "script": script.display().to_string()
            }))
        }
        Err(e) => {
            warn!("❌ Failed to start Tempo: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to execute launch script: {}", e)
            }))
        }
    }
}

/// POST /api/inference/stop — stop T (Tempo) from web UI
pub async fn inference_stop_endpoint(State(state): State<AppState>) -> Json<serde_json::Value> {
    match tokio::process::Command::new("bash")
        .arg("-c")
        .arg("lsof -ti:8001 2>/dev/null | xargs -r kill 2>/dev/null; echo '{\"status\":\"stopped\"}'")
        .output()
        .await
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            info!("🛑 Tempo stop: {}", stdout.trim());
            
            hotel_manager::hotel_checkout().await;
            
            let mut router = state.inference_router.write().await;
            router.auto_detect().await;

            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(stdout.trim()) {
                Json(parsed)
            } else {
                Json(serde_json::json!({
                    "status": "stopped",
                    "output": stdout.trim()
                }))
            }
        }
        Err(e) => {
            warn!("❌ Failed to stop Tempo: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to stop: {}", e)
            }))
        }
    }
}
