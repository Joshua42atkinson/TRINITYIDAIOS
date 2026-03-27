// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Sidecar Manager
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:     api.rs
// PURPOSE:  HTTP API for sidecar lifecycle management (start/stop/health)
// BIBLE:    Car 3 — DEVELOPMENT (Hotel Management, §3.3)
//
// ═══════════════════════════════════════════════════════════════════════════════

//! Axum HTTP API for the Sidecar
//!
//! Port 8090 — used by the conductor and web UI to control sidecars.
//!
//! Endpoints:
//!   GET  /status          — Sidecar health, model status, quest progress
//!   GET  /quests           — List all quests (board, active, complete, failed)
//!   POST /quest/claim      — Claim a quest by ID
//!   POST /quest/execute    — Execute a quest by ID (manual trigger)
//!   POST /autonomous/start — Start autonomous work loop
//!   POST /autonomous/stop  — Stop autonomous work loop
//!   POST /think            — Send a prompt directly to primary model
//!   POST /code             — Send a prompt directly to secondary model
//!   POST /shutdown         — Graceful shutdown
//!
//! Artist-specific endpoints:
//!   GET  /creative/status  — Check ComfyUI, ACE-Step, Trellis health
//!   POST /creative/image   — Generate image via ComfyUI
//!   POST /creative/music   — Generate music via ACE-Step
//!   POST /creative/assets  — Generate full game asset bundle

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::llama::ChatMessage;
use crate::prompts;
use crate::workflow::{WorkflowEngine, WorkflowState};

/// Shared application state for API handlers
#[derive(Clone)]
pub struct ApiState {
    pub engine: Arc<WorkflowEngine>,
    pub workflow_state: Arc<RwLock<WorkflowState>>,
    pub shutdown_tx: tokio::sync::broadcast::Sender<()>,
}

/// Build the Axum router
pub fn build_router(state: ApiState) -> Router {
    Router::new()
        .route("/status", get(get_status))
        .route("/quests", get(list_quests))
        .route("/quest/:id", get(get_quest_by_id))
        .route("/quest/claim", post(claim_quest))
        .route("/quest/execute", post(execute_quest))
        .route("/autonomous/start", post(start_autonomous))
        .route("/autonomous/stop", post(stop_autonomous))
        .route("/think", post(think))
        .route("/code", post(code))
        .route("/shutdown", post(shutdown))
        // Artist creative endpoints
        .route("/creative/status", get(get_creative_status))
        .route("/creative/image", post(generate_image))
        .route("/creative/music", post(generate_music))
        .route("/creative/assets", post(generate_assets))
        // Blender 3D endpoints
        .route("/creative/3d/model", post(generate_3d_model))
        .route("/creative/3d/scene", post(compose_3d_scene))
        .route("/creative/3d/render", post(render_frame))
        .route("/creative/3d/animation", post(render_animation))
        .route("/creative/3d/material", post(create_material))
        .route("/creative/3d/game-asset", post(create_game_asset))
        .route("/creative/full-bundle", post(create_full_bundle))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

// ─── Status ─────────────────────────────────────────────────────────

async fn get_status(State(state): State<ApiState>) -> Json<WorkflowState> {
    let ws = state.workflow_state.read().await;
    Json(ws.clone())
}

// ─── Quest Management ───────────────────────────────────────────────

#[derive(Serialize)]
struct QuestList {
    board: Vec<QuestSummary>,
    active: Vec<QuestSummary>,
    complete: Vec<QuestSummary>,
    failed: Vec<QuestSummary>,
}

#[derive(Serialize)]
struct QuestSummary {
    id: String,
    title: String,
    priority: u8,
    status: String,
    quest_type: String,
}

fn summarize(q: &crate::quest::Quest) -> QuestSummary {
    QuestSummary {
        id: q.id.clone(),
        title: q.title.clone(),
        priority: q.priority,
        status: format!("{:?}", q.status),
        quest_type: format!("{:?}", q.quest_type),
    }
}

async fn list_quests(
    State(state): State<ApiState>,
) -> Result<Json<QuestList>, (StatusCode, String)> {
    let board = &state.engine.quest_board;

    let available = board
        .list_available()
        .await
        .map_err(|e: anyhow::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let active = board
        .list_active()
        .await
        .map_err(|e: anyhow::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let complete = board
        .list_complete()
        .await
        .map_err(|e: anyhow::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(QuestList {
        board: available.iter().map(summarize).collect(),
        active: active.iter().map(summarize).collect(),
        complete: complete.iter().map(summarize).collect(),
        failed: Vec::new(),
    }))
}

async fn get_quest_by_id(
    State(state): State<ApiState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<crate::quest::Quest>, (StatusCode, String)> {
    let quest = state
        .engine
        .quest_board
        .get_quest(&id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match quest {
        Some(q) => Ok(Json(q)),
        None => Err((StatusCode::NOT_FOUND, format!("Quest not found: {}", id))),
    }
}

#[derive(Deserialize)]
struct QuestIdRequest {
    quest_id: String,
}

async fn claim_quest(
    State(state): State<ApiState>,
    Json(request): Json<QuestIdRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let quest = state
        .engine
        .quest_board
        .claim(&request.quest_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "status": "claimed",
        "quest_id": quest.id,
        "title": quest.title,
    })))
}

async fn execute_quest(
    State(state): State<ApiState>,
    Json(request): Json<QuestIdRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let results = state
        .engine
        .execute_quest_by_id(&request.quest_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "status": "complete",
        "quest_id": request.quest_id,
        "files_modified": results.files_modified,
        "files_created": results.files_created,
        "verdict": results.opus_verdict,
        "xp_earned": results.xp_earned,
    })))
}

// ─── Autonomous Control ─────────────────────────────────────────────

async fn start_autonomous(State(state): State<ApiState>) -> Json<serde_json::Value> {
    let engine = state.engine.clone();
    tokio::spawn(async move {
        engine.run_autonomous().await;
    });
    Json(serde_json::json!({"status": "autonomous_started"}))
}

async fn stop_autonomous(State(state): State<ApiState>) -> Json<serde_json::Value> {
    state.engine.stop_autonomous().await;
    Json(serde_json::json!({"status": "autonomous_stopped"}))
}

// ─── Direct Model Access ────────────────────────────────────────────

#[derive(Deserialize)]
struct PromptRequest {
    prompt: String,
    #[serde(default = "default_max_tokens")]
    max_tokens: u32,
    #[serde(default = "default_temperature")]
    temperature: f32,
}

fn default_max_tokens() -> u32 {
    4096
}
fn default_temperature() -> f32 {
    0.3
}

async fn think(
    State(state): State<ApiState>,
    Json(request): Json<PromptRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let response = state
        .engine
        .opus
        .chat(
            &[
                ChatMessage::system(prompts::OPUS_SYSTEM),
                ChatMessage::user(&request.prompt),
            ],
            request.max_tokens,
            request.temperature,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Opus error: {}", e),
            )
        })?;

    Ok(Json(serde_json::json!({
        "model": "opus",
        "response": response,
    })))
}

async fn code(
    State(state): State<ApiState>,
    Json(request): Json<PromptRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let response = state
        .engine
        .reap
        .chat(
            &[
                ChatMessage::system(prompts::REAP_SYSTEM),
                ChatMessage::user(&request.prompt),
            ],
            request.max_tokens,
            request.temperature,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("REAP error: {}", e),
            )
        })?;

    Ok(Json(serde_json::json!({
        "model": "reap",
        "response": response,
    })))
}

// ─── Shutdown ───────────────────────────────────────────────────────

async fn shutdown(State(state): State<ApiState>) -> Json<serde_json::Value> {
    info!("Shutdown requested via API");
    let _ = state.shutdown_tx.send(());
    Json(serde_json::json!({"status": "shutting_down"}))
}

// ─── Creative Endpoints (Artist Sidecar) ──────────────────────────────

#[derive(Serialize)]
struct CreativeStatusResponse {
    comfyui_healthy: bool,
    music_healthy: bool,
    trellis_healthy: bool,
    role: String,
}

async fn get_creative_status(
    State(state): State<ApiState>,
) -> Json<CreativeStatusResponse> {
    let status = state.engine.creative.status().await;
    Json(CreativeStatusResponse {
        comfyui_healthy: status.comfyui_healthy,
        music_healthy: status.music_healthy,
        trellis_healthy: status.trellis_healthy,
        role: state.engine.role_id.clone(),
    })
}

#[derive(Deserialize)]
struct ImageRequest {
    prompt: String,
    negative_prompt: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    steps: Option<u32>,
    seed: Option<u64>,
}

#[derive(Serialize)]
struct ImageResponse {
    filename: String,
    width: u32,
    height: u32,
    size_bytes: usize,
}

async fn generate_image(
    State(state): State<ApiState>,
    Json(request): Json<ImageRequest>,
) -> Result<Json<ImageResponse>, (StatusCode, String)> {
    let img = state
        .engine
        .creative
        .comfyui
        .generate_image(&trinity_comfy::ImageRequest {
            prompt: request.prompt,
            negative_prompt: request.negative_prompt,
            width: request.width.unwrap_or(512),
            height: request.height.unwrap_or(512),
            steps: request.steps.unwrap_or(4),
            seed: request.seed,
        })
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(ImageResponse {
        filename: img.filename,
        width: img.width,
        height: img.height,
        size_bytes: img.data.len(),
    }))
}

#[derive(Deserialize)]
struct MusicRequest {
    prompt: String,
    duration_secs: Option<u32>,
    style: Option<String>,
}

#[derive(Serialize)]
struct MusicResponse {
    filename: String,
    duration_secs: u32,
    size_bytes: usize,
}

async fn generate_music(
    State(state): State<ApiState>,
    Json(request): Json<MusicRequest>,
) -> Result<Json<MusicResponse>, (StatusCode, String)> {
    let music = state
        .engine
        .creative
        .music
        .generate_music(&trinity_comfy::MusicRequest {
            prompt: request.prompt,
            duration_secs: request.duration_secs.unwrap_or(30),
            style: request.style,
            tempo: None,
        })
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(MusicResponse {
        filename: music.filename,
        duration_secs: music.duration_secs,
        size_bytes: music.data.len(),
    }))
}

#[derive(Deserialize)]
struct AssetsRequest {
    concept: String,
    include_music: Option<bool>,
}

#[derive(Serialize)]
struct AssetsResponse {
    image_filename: Option<String>,
    music_filename: Option<String>,
    has_3d_model: bool,
}

async fn generate_assets(
    State(state): State<ApiState>,
    Json(request): Json<AssetsRequest>,
) -> Result<Json<AssetsResponse>, (StatusCode, String)> {
    let bundle = state
        .engine
        .creative
        .generate_game_assets(&request.concept, request.include_music.unwrap_or(false))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AssetsResponse {
        image_filename: bundle.image.map(|i| i.filename),
        music_filename: bundle.music.map(|m| m.filename),
        has_3d_model: bundle.model_3d.is_some(),
    }))
}

// ─── Blender 3D Endpoints ────────────────────────────────────────────────

#[derive(Deserialize)]
struct Model3DRequest {
    mesh_type: String,  // "cube", "sphere", "cylinder", "torus", "plane", "ico_sphere", "suzanne", "grid"
    name: String,
    size: Option<f32>,
    radius: Option<f32>,
    segments: Option<u32>,
    format: Option<String>,  // "glb", "fbx", "obj", "stl"
    material_color: Option<[f32; 4]>,
    metallic: Option<f32>,
    roughness: Option<f32>,
}

#[derive(Serialize)]
struct Model3DResponse {
    filename: String,
    format: String,
    vertex_count: u32,
    material_count: u32,
    size_bytes: usize,
}

async fn generate_3d_model(
    State(state): State<ApiState>,
    Json(request): Json<Model3DRequest>,
) -> Result<Json<Model3DResponse>, (StatusCode, String)> {
    use trinity_comfy::{MeshType, MeshRequest, MaterialDef, ExportFormat};
    
    let mesh_type = match request.mesh_type.as_str() {
        "cube" => MeshType::Cube { size: request.size.unwrap_or(1.0) },
        "sphere" => MeshType::Sphere { 
            radius: request.radius.unwrap_or(1.0) as u32, 
            segments: request.segments.unwrap_or(32) 
        },
        "cylinder" => MeshType::Cylinder { 
            radius: request.radius.unwrap_or(0.5), 
            depth: request.size.unwrap_or(2.0) 
        },
        "torus" => MeshType::Torus { 
            major_radius: request.radius.unwrap_or(1.0), 
            minor_radius: request.size.unwrap_or(0.3) 
        },
        "plane" => MeshType::Plane { size: request.size.unwrap_or(2.0) },
        "ico_sphere" => MeshType::IcoSphere { 
            radius: request.radius.unwrap_or(1.0), 
            subdivisions: request.segments.unwrap_or(2) 
        },
        "suzanne" => MeshType::Suzanne,
        "grid" => MeshType::Grid { 
            size_x: request.segments.unwrap_or(10), 
            size_y: request.size.unwrap_or(10.0) as u32 
        },
        _ => return Err((StatusCode::BAD_REQUEST, format!("Unknown mesh type: {}", request.mesh_type))),
    };
    
    let format = match request.format.as_deref() {
        Some("glb") | None => ExportFormat::Glb,
        Some("fbx") => ExportFormat::Fbx,
        Some("obj") => ExportFormat::Obj,
        Some("stl") => ExportFormat::Stl,
        Some(f) => return Err((StatusCode::BAD_REQUEST, format!("Unknown format: {}", f))),
    };
    
    let material = request.material_color.map(|c| MaterialDef {
        name: format!("{}_mat", request.name),
        base_color: c,
        metallic: request.metallic.unwrap_or(0.0),
        roughness: request.roughness.unwrap_or(0.5),
        emissive: None,
    });
    
    let mesh_request = MeshRequest {
        mesh_type,
        name: request.name,
        material,
        modifiers: vec![],
    };
    
    let model = state
        .engine
        .creative
        .generate_3d_model(&mesh_request, format)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(Model3DResponse {
        filename: model.filename,
        format: model.format.extension().to_string(),
        vertex_count: model.vertex_count,
        material_count: model.material_count,
        size_bytes: model.data.len(),
    }))
}

#[derive(Deserialize)]
struct Scene3DRequest {
    name: String,
    objects: Vec<ObjectPlacementRequest>,
    lighting: Option<String>,  // "three_point", "studio", "outdoor"
}

#[derive(Deserialize)]
struct ObjectPlacementRequest {
    mesh_type: String,
    name: String,
    position: [f32; 3],
    rotation: Option<[f32; 3]>,
    scale: Option<[f32; 3]>,
}

#[derive(Serialize)]
struct Scene3DResponse {
    blend_path: String,
    object_count: usize,
}

async fn compose_3d_scene(
    State(state): State<ApiState>,
    Json(_request): Json<Scene3DRequest>,
) -> Result<Json<Scene3DResponse>, (StatusCode, String)> {
    // For now, return a placeholder - full scene composition requires more complex request parsing
    Err((StatusCode::NOT_IMPLEMENTED, "Scene composition requires full SceneRequest - use generate_3d_model for individual objects".to_string()))
}

#[derive(Deserialize)]
struct RenderRequest {
    blend_path: String,
    frame: Option<u32>,
}

#[derive(Serialize)]
struct RenderResponse {
    filename: String,
    width: u32,
    height: u32,
    size_bytes: usize,
}

async fn render_frame(
    State(state): State<ApiState>,
    Json(request): Json<RenderRequest>,
) -> Result<Json<RenderResponse>, (StatusCode, String)> {
    let blend_path = std::path::PathBuf::from(&request.blend_path);
    let frame = request.frame.unwrap_or(1);
    
    let output = state
        .engine
        .creative
        .render_frame(&blend_path, frame)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(RenderResponse {
        filename: output.filename,
        width: output.width,
        height: output.height,
        size_bytes: output.data.len(),
    }))
}

#[derive(Deserialize)]
struct AnimationRenderRequest {
    blend_path: Option<String>,
    frame_start: u32,
    frame_end: u32,
    fps: Option<u32>,
}

async fn render_animation(
    State(state): State<ApiState>,
    Json(_request): Json<AnimationRenderRequest>,
) -> Result<Json<RenderResponse>, (StatusCode, String)> {
    // Requires AnimationRequest with scene_file
    Err((StatusCode::NOT_IMPLEMENTED, "Animation rendering requires AnimationRequest with scene_file".to_string()))
}

#[derive(Deserialize)]
struct MaterialRequest {
    name: String,
    base_color: [f32; 4],
    metallic: Option<f32>,
    roughness: Option<f32>,
    emissive: Option<[f32; 3]>,
}

#[derive(Serialize)]
struct MaterialResponse {
    name: String,
    created: bool,
}

async fn create_material(
    State(state): State<ApiState>,
    Json(request): Json<MaterialRequest>,
) -> Result<Json<MaterialResponse>, (StatusCode, String)> {
    use trinity_comfy::MaterialDef;
    
    let mat = MaterialDef {
        name: request.name.clone(),
        base_color: request.base_color,
        metallic: request.metallic.unwrap_or(0.0),
        roughness: request.roughness.unwrap_or(0.5),
        emissive: request.emissive,
    };
    
    state
        .engine
        .creative
        .create_material(&mat)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(MaterialResponse {
        name: request.name,
        created: true,
    }))
}

#[derive(Deserialize)]
struct GameAssetRequest {
    mesh_type: String,
    name: String,
    include_collision: Option<bool>,
    include_lods: Option<bool>,
}

#[derive(Serialize)]
struct GameAssetResponse {
    main_model: Option<String>,
    lods: Vec<String>,
    collision: Option<String>,
}

async fn create_game_asset(
    State(state): State<ApiState>,
    Json(request): Json<GameAssetRequest>,
) -> Result<Json<GameAssetResponse>, (StatusCode, String)> {
    use trinity_comfy::{MeshType, MeshRequest};
    
    let mesh_type = match request.mesh_type.as_str() {
        "cube" => MeshType::Cube { size: 1.0 },
        "sphere" => MeshType::Sphere { radius: 1, segments: 32 },
        "cylinder" => MeshType::Cylinder { radius: 0.5, depth: 2.0 },
        "suzanne" => MeshType::Suzanne,
        _ => MeshType::Cube { size: 1.0 },
    };
    
    let mesh = MeshRequest {
        mesh_type,
        name: request.name,
        material: None,
        modifiers: vec![],
    };
    
    let asset = state
        .engine
        .creative
        .create_game_asset_3d(&mesh, request.include_collision.unwrap_or(false), request.include_lods.unwrap_or(false))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(GameAssetResponse {
        main_model: asset.main_model.map(|m| m.filename),
        lods: asset.lods.iter().map(|m| m.filename.clone()).collect(),
        collision: asset.collision.map(|m| m.filename),
    }))
}

#[derive(Deserialize)]
struct FullBundleRequest {
    concept: String,
    include_animation: Option<bool>,
}

#[derive(Serialize)]
struct FullBundleResponse {
    image: Option<String>,
    music: Option<String>,
    model_3d: Option<String>,
    animation: Option<String>,
}

async fn create_full_bundle(
    State(state): State<ApiState>,
    Json(request): Json<FullBundleRequest>,
) -> Result<Json<FullBundleResponse>, (StatusCode, String)> {
    let bundle = state
        .engine
        .creative
        .create_full_asset_bundle(&request.concept, request.include_animation.unwrap_or(false))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(FullBundleResponse {
        image: bundle.image.map(|i| i.filename),
        music: bundle.music.map(|m| m.filename),
        model_3d: bundle.model_3d.map(|m| m.filename),
        animation: bundle.animation.map(|a| a.filename),
    }))
}
