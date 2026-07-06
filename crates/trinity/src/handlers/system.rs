use axum::{extract::State, Json, http::StatusCode, response::sse::{self, Sse}, response::Response, body::Body};
use serde::{Deserialize, Serialize};
use futures::Stream;
use tokio::sync::broadcast;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::AppState;
use crate::handlers::inference::installed_model_inventory;
use crate::AppMode;
use crate::persistence;
use crate::ChatMessage;

/// System status — fields must match what index.html pollHardware() expects
#[derive(Debug, Serialize)]
pub struct SystemStatus {
    pub server: String,
    pub inference_server: String,
    pub inference_connected: bool,
    pub database: String,
    pub models_available: Vec<String>,
    pub memory_usage_mb: u64,
    pub cpu_load: f32,
    pub mem_used_gb: f32,
    pub mem_total_gb: f32,
    pub mem_percent: f32,
    pub gpu_load: f32,
    pub npu_load: f32,
    /// Ignition state machine: idle | launching | daemon_up | server_starting | polling | loading_model | ready | failed
    pub ignition_status: String,
}

#[derive(serde::Deserialize)]
pub struct BackendStartRequest {
    pub backend: String,
}

#[derive(serde::Deserialize)]
pub struct SetupConfig {
    pub backend: String,
    pub custom_url: Option<String>,
}

/// Helper: set ignition status atomically
pub async fn set_ignition(status: &Arc<RwLock<String>>, value: &str) {
    *status.write().await = value.to_string();
    info!("🔥 Ignition State → {}", value);
}

pub async fn get_hardware_status(State(state): State<AppState>) -> Json<SystemStatus> {
    use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_memory(MemoryRefreshKind::everything())
            .with_cpu(CpuRefreshKind::everything()),
    );
    sys.refresh_all();

    let cpu_load = sys.global_cpu_info().cpu_usage();
    let total_mem = sys.total_memory() as f32 / 1_073_741_824.0;
    let used_mem = sys.used_memory() as f32 / 1_073_741_824.0;
    let mem_percent = (used_mem / total_mem) * 100.0;

    // Read real GPU load from sysfs (ROCm/AMDGPU)
    let gpu_load = std::fs::read_to_string("/sys/class/drm/renderD128/device/gpu_busy_percent")
        .ok()
        .and_then(|s| s.trim().parse::<f32>().ok())
        .unwrap_or(0.0);

    // Read NPU load if available (accel0)
    let npu_load = std::fs::read_to_string("/sys/class/accel/accel0/device/busy_percent")
        .ok()
        .or_else(|| std::fs::read_to_string("/sys/class/accel/accel0/busy_percent").ok())
        .and_then(|s| s.trim().parse::<f32>().ok())
        .unwrap_or(0.0);

    let router = state.inference_router.read().await;
    let _llm_url = router.active_url().to_string();
    let inference_connected = router.is_healthy();
    drop(router);

    let db_connected = sqlx::query("SELECT 1")
        .execute(&state.db_pool)
        .await
        .is_ok();

    let models: Vec<String> = installed_model_inventory()
        .into_iter()
        .filter(|(_, path)| path.exists())
        .map(|(name, _)| name.to_string())
        .collect();

    let ignition_status = state.ignition_status.read().await.clone();

    Json(SystemStatus {
        server: "running".to_string(),
        inference_server: if inference_connected {
            "connected".to_string()
        } else {
            "disconnected".to_string()
        },
        inference_connected,
        database: if db_connected {
            "connected".to_string()
        } else {
            "not configured".to_string()
        },
        models_available: models,
        memory_usage_mb: (used_mem * 1024.0) as u64,
        cpu_load,
        mem_used_gb: used_mem,
        mem_total_gb: total_mem,
        mem_percent,
        gpu_load,
        npu_load,
        ignition_status,
    })
}

pub async fn backend_start(
    State(state): State<AppState>,
    Json(payload): Json<BackendStartRequest>,
) -> impl axum::response::IntoResponse {
    let backend_name = payload.backend.clone();
    let ignition = state.ignition_status.clone();

    // Prevent double-ignition
    {
        let current = ignition.read().await;
        if *current != "idle" && *current != "failed" && *current != "ready" {
            return Json(serde_json::json!({
                "status": "already_running",
                "ignition_status": *current,
                "message": format!("Ignition already in progress: {}", *current)
            }));
        }
    }

    match payload.backend.as_str() {
        "vllm-omni" => {
            set_ignition(&ignition, "launching").await;
            let ignition_bg = ignition.clone();
            let inference_router = state.inference_router.clone();

            tokio::spawn(async move {
                let client = reqwest::Client::new();
                
                // ═══ Phase 1: Fast-path check
                let mut server_already_up = false;
                match client.get("http://127.0.0.1:8001/v1/models")
                    .timeout(std::time::Duration::from_secs(2))
                    .send().await
                {
                    Ok(resp) if resp.status().is_success() => {
                        info!("🔥 Fast-path: vLLM API server is already running on :8001");
                        server_already_up = true;
                    }
                    _ => {}
                }

                if !server_already_up {
                    info!("🔥 TRINITY/vLLM not running — launching in background...");
                    
                    let trinity_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    let launch_script = trinity_dir.join("scripts/launch/launch_pete.sh");
                    
                    match tokio::process::Command::new("bash")
                        .arg(launch_script)
                        .spawn()
                    {
                        Ok(mut child) => {
                            info!("🔥 Launched TRINITY/vLLM script in background!");
                            tokio::spawn(async move {
                                let _ = child.wait().await;
                            });
                        },
                        Err(e) => {
                            warn!("❌ Failed to launch TRINITY script: {}", e);
                            set_ignition(&ignition_bg, "failed").await;
                            return;
                        }
                    }

                    // ═══ Phase 2: Poll :8001 until the server is healthy
                    set_ignition(&ignition_bg, "polling").await;
                    let mut server_ready = false;
                    for attempt in 1..=300 {
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        match client.get("http://127.0.0.1:8001/v1/models")
                            .timeout(std::time::Duration::from_secs(2))
                            .send().await
                        {
                            Ok(resp) if resp.status().is_success() => {
                                info!("🔥 TRINITY/vLLM API server ready after {}s", attempt);
                                server_ready = true;
                                break;
                            }
                            _ => {
                                if attempt % 10 == 0 {
                                    info!("🔥 Waiting for TRINITY/vLLM API... ({}s)", attempt);
                                }
                            }
                        }
                    }
                    if !server_ready {
                        warn!("❌ TRINITY/vLLM API server did not respond after 300s");
                        set_ignition(&ignition_bg, "failed").await;
                        return;
                    }
                }

                set_ignition(&ignition_bg, "ready").await;
                let mut router = inference_router.write().await;
                router.auto_detect().await;
                info!("🔥 ═══ IGNITION COMPLETE — TRINITY/vLLM is ONLINE ═══");
            });
        },
        _ => {
            info!("🔥 Ignition: Custom backend '{}' — no auto-start", backend_name);
        }
    }

    Json(serde_json::json!({
        "status": "ignition_started",
        "ignition_status": "launching",
        "message": format!("Ignition Sequence for {} started.", backend_name)
    }))
}

pub async fn status(State(state): State<AppState>) -> Json<SystemStatus> {
    let llama_ok = state.inference_router.read().await.is_healthy();
    let db_ok = sqlx::query("SELECT 1")
        .execute(&state.db_pool)
        .await
        .is_ok();

    // Check available models
    let models = installed_model_inventory()
        .into_iter()
        .filter(|(_, path)| path.exists())
        .map(|(name, _)| name.to_string())
        .collect();

    Json(SystemStatus {
        server: "running".to_string(),
        inference_server: if llama_ok {
            "connected".to_string()
        } else {
            "disconnected".to_string()
        },
        inference_connected: llama_ok,
        database: if db_ok {
            "connected".to_string()
        } else {
            "not configured".to_string()
        },
        models_available: models,
        memory_usage_mb: 0,
        cpu_load: 0.0,
        mem_used_gb: 0.0,
        mem_total_gb: 0.0,
        mem_percent: 0.0,
        gpu_load: 0.0,
        npu_load: 0.0,
        ignition_status: "idle".to_string(),
    })
}

/// SSE stream for real-time Cognitive Load tracking
pub async fn telemetry_stream(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<sse::Event, anyhow::Error>>> {
    use async_stream::stream;

    info!("SSE client connected to telemetry stream");
    let mut receiver = state.telemetry_updates.subscribe();

    let stream = stream! {
        loop {
            match receiver.recv().await {
                Ok(trace) => {
                    let json_payload = serde_json::to_string(&trace)?;
                    yield Ok(sse::Event::default().event("trace").data(json_payload));
                }
                Err(broadcast::error::RecvError::Closed) => {
                    break;
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    continue;
                }
            }
        }
    };

    Sse::new(stream)
}

pub async fn setup_config(
    State(state): State<AppState>,
    Json(config): Json<SetupConfig>,
) -> impl axum::response::IntoResponse {
    let url = match config.backend.as_str() {
        "vllm-omni" => "http://127.0.0.1:8001/v1/chat/completions",
        _ => config.custom_url.as_deref().unwrap_or("http://127.0.0.1:8001/v1/chat/completions"),
    };

    // Test the connection BEFORE acknowledging setup is complete
    let test_url = url.replace("/chat/completions", "/models");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .unwrap_or_default();

    if client.get(&test_url).send().await.is_err() {
        tracing::error!("Setup failed: LLM Backend offline at {}", test_url);
        return axum::http::StatusCode::SERVICE_UNAVAILABLE;
    }

    let mut router = state.inference_router.write().await;
    router.set_active_url(url.to_string());
    router.auto_detect().await;
    
    axum::http::StatusCode::OK
}

/// GET /api/mode — returns current operating mode
pub async fn get_app_mode(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mode = state.player.app_mode.read().await;
    Json(serde_json::json!({
        "mode": *mode,
        "description": match *mode {
            AppMode::IronRoad => "Full LitRPG gamification — the Iron Road",
            AppMode::Express => "Guided wizard — skip the game, build and export",
            AppMode::Yardmaster => "IDE/Agent mode — full developer tools",
            AppMode::Demo => "Read-only demo — chat and view, no mutation",
        }
    }))
}

/// POST /api/mode — switch operating mode
pub async fn set_app_mode(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mode_str = body["mode"]
        .as_str()
        .ok_or((StatusCode::BAD_REQUEST, "Missing 'mode' field".to_string()))?;

    let new_mode: AppMode = serde_json::from_value(serde_json::json!(mode_str)).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            format!(
                "Invalid mode '{}'. Use: iron_road, express, or yardmaster",
                mode_str
            ),
        )
    })?;

    info!(
        "🚂 Mode switch: {} → {}",
        state.player.app_mode.read().await,
        new_mode
    );
    *state.player.app_mode.write().await = new_mode.clone();

    Ok(Json(serde_json::json!({
        "mode": new_mode,
        "message": format!("Switched to {} mode", new_mode),
    })))
}

/// MCP Proxy endpoint - forwards requests to trinity-mcp-server
#[derive(Debug, Serialize, Deserialize)]
pub struct McpRequest {
    pub method: String,
    pub params: serde_json::Value,
}

pub async fn mcp_proxy(
    State(_state): State<AppState>,
    Json(request): Json<McpRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let mut stream = match tokio::net::TcpStream::connect("127.0.0.1:8080").await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to connect to MCP server on 8080: {}", e);
            return Ok(Json(serde_json::json!({
                "error": "MCP server not connected. Ensure trinity-mcp-server is running on port 8080.",
                "details": e.to_string(),
                "hint": "Check distrobox or run `cargo run -p trinity-mcp-server`"
            })));
        }
    };

    let mut request_json = serde_json::to_string(&request).unwrap_or_default();
    request_json.push('\n');

    if let Err(e) = stream.write_all(request_json.as_bytes()).await {
        return Ok(Json(serde_json::json!({ "error": format!("Failed to send MCP request: {}", e) })));
    }

    let mut reader = BufReader::new(stream);
    let mut response_line = String::new();
    
    if let Err(e) = reader.read_line(&mut response_line).await {
        return Ok(Json(serde_json::json!({ "error": format!("Failed to read MCP response: {}", e) })));
    }

    if response_line.is_empty() {
        return Ok(Json(serde_json::json!({ "error": "Empty response from MCP server" })));
    }

    match serde_json::from_str::<serde_json::Value>(&response_line) {
        Ok(parsed) => Ok(Json(parsed)),
        Err(e) => Ok(Json(serde_json::json!({ "error": format!("Failed to parse MCP response: {}", e) })))
    }
}

/// List conversation sessions
pub async fn list_sessions(
    State(state): State<AppState>,
) -> Result<Json<Vec<persistence::SessionSummary>>, (StatusCode, String)> {
    persistence::list_sessions(&state.db_pool, 50)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Get conversation history for a session
pub async fn get_session_history(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<ChatMessage>>, (StatusCode, String)> {
    let session_id = params
        .get("session_id")
        .map(|s| s.as_str())
        .unwrap_or(state.project.session_id.as_str());
    let limit = params
        .get("limit")
        .and_then(|l| l.parse::<i64>().ok())
        .unwrap_or(100);

    persistence::load_session_history(&state.db_pool, session_id, limit)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Serve the Five Chariots + Hook Book — root documentation as raw markdown
pub async fn serve_chariot_doc(
    axum::extract::Path(filename): axum::extract::Path<String>,
) -> Result<Response, (StatusCode, String)> {
    const ALLOWED: &[&str] = &[
        "TRINITY_FANCY_BIBLE.md",
        "ASK_PETE_FIELD_MANUAL.md",
        "PROFESSOR.md",
        "README.md",
        "PLAYERS_HANDBOOK.md",
        "HOOK_BOOK.md",
    ];
    if !ALLOWED.contains(&filename.as_str()) {
        return Err((StatusCode::NOT_FOUND, "Document not found".to_string()));
    }
    let path = std::path::PathBuf::from(&filename);
    match tokio::fs::read_to_string(&path).await {
        Ok(content) => Ok(Response::builder()
            .header("Content-Type", "text/markdown; charset=utf-8")
            .body(Body::from(content))
            .unwrap()),
        Err(_) => Err((StatusCode::NOT_FOUND, format!("{} not found on disk", filename))),
    }
}

/// Serve the generated .wav files for the Player Handbook E-Learning module
pub async fn serve_audiobook_audio(
    axum::extract::Path(filename): axum::extract::Path<String>,
) -> Result<Response, (StatusCode, String)> {
    if !filename.ends_with(".wav") {
        return Err((StatusCode::BAD_REQUEST, "Only .wav files are allowed".into()));
    }
    
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("audiobook_output").join(&filename);
        
    match tokio::fs::read(&path).await {
        Ok(bytes) => Ok(Response::builder()
            .header("Content-Type", "audio/wav")
            .header("Accept-Ranges", "bytes")
            .body(Body::from(bytes))
            .unwrap()),
        Err(_) => Err((StatusCode::NOT_FOUND, "Audio file not found".into())),
    }
}

/// Serve the generated artwork (50GB image gen) for the Player Handbook slides
pub async fn serve_audiobook_art(
    axum::extract::Path(filename): axum::extract::Path<String>,
) -> Result<Response, (StatusCode, String)> {
    if !filename.ends_with(".jpg") && !filename.ends_with(".png") && !filename.ends_with(".webp") {
        return Err((StatusCode::BAD_REQUEST, "Only image files are allowed".into()));
    }

    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("images").join("handbook_art").join(&filename);

    match tokio::fs::read(&path).await {
        Ok(bytes) => {
            let content_type = if filename.ends_with(".png") { "image/png" } else if filename.ends_with(".webp") { "image/webp" } else { "image/jpeg" };
            Ok(Response::builder()
                .header("Content-Type", content_type)
                .body(Body::from(bytes))
                .unwrap())
        },
        Err(_) => Err((StatusCode::NOT_FOUND, "Art file not found".into())),
    }
}
