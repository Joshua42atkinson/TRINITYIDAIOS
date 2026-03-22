use trinity::rlhf_api;
// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        main.rs
// PURPOSE:     HTTP API server entry point — Axum setup, route registration, SSE
//
// ARCHITECTURE:
//   • Layer 1 of Trinity 3-Layer Architecture (Headless Server)
//   • Axum HTTP server on port 3000 with CORS for web UI
//   • Broadcast channel for SSE streaming to multiple clients
//   • Module structure: agent, tools, rag, quests, inference, voice, etc.
//   • NPU Integration: Great Recycler runs via FastFlowLM, updates Iron Road book
//
// DEPENDENCIES:
//   - axum — HTTP framework for API routes
//   - tokio — Async runtime (multi-threaded)
//   - serde — JSON serialization for chat/protocol types
//   - tracing — Structured logging
//   - tower-http — CORS and middleware
//   - futures — Stream handling for SSE
//
// CHANGES:
//   2026-03-16  Cascade  Migrated to §17 comment standard
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{
    extract::State,
    http::StatusCode,
    response::{sse, Json, Sse},
    routing::{delete, get, post, put},
    Router,
};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{broadcast, RwLock};
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

mod agent;
mod character_sheet;
mod conductor_leader;
mod cow_catcher;
mod creative;
mod embedded_inference;
mod export;
mod eye_container;
mod health;
pub mod http;
mod inference;
mod inference_router;
mod journal;
mod music_streamer;
mod narrative;
mod persistence;
mod perspective;
mod quality_scorecard;
mod quests;
mod rag;
mod scope_creep;
mod sidecar_monitor;
mod skills;
mod tools;
mod trinity_api;
mod vaam;
mod vaam_bridge;
mod voice;
mod voice_loop;

// Import Great Recycler from trinity-kernel
use trinity_iron_road::book::BookOfTheBible;
use trinity_iron_road::game_loop::CreepBestiary;
use trinity_protocol::CharacterSheet;

/// Operating mode — same backend, different UX
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum AppMode {
    /// Full LitRPG gamification (the Iron Road)
    #[default]
    IronRoad,
    /// Skip game mechanics — guided wizard → export
    Express,
    /// IDE/Agent mode (Yardmaster)
    Yardmaster,
}

impl std::fmt::Display for AppMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppMode::IronRoad => write!(f, "iron_road"),
            AppMode::Express => write!(f, "express"),
            AppMode::Yardmaster => write!(f, "yardmaster"),
        }
    }
}

/// Application state shared across all routes
#[derive(Clone)]
pub struct AppState {
    pub inference_router: Arc<RwLock<inference_router::InferenceRouter>>,
    pub embedded_model: Option<Arc<embedded_inference::EmbeddedModel>>,
    pub db_pool: sqlx::PgPool,
    pub conversation_history: Arc<RwLock<Vec<ChatMessage>>>,
    pub game_state: trinity_quest::SharedGameState,
    /// Broadcast sender for Iron Road book updates
    /// WHY: SSE clients subscribe to receive real-time updates
    pub book_updates: broadcast::Sender<String>,
    /// User's persistent character sheet (single source of truth for preferences)
    /// WHY: Creative settings, genre, and user identity persist across sessions
    pub character_sheet: Arc<RwLock<CharacterSheet>>,
    pub cow_catcher: Arc<tokio::sync::RwLock<crate::cow_catcher::CowCatcher>>,
    /// VAAM Bridge — processes all text through vocabulary + circuit detection
    /// WHY: Words are what LLMs and people have in common. This makes it productive.
    pub vaam_bridge: Arc<vaam_bridge::VaamBridge>,
    /// Creep Bestiary — the player's vocabulary creature collection
    /// WHY: Every word becomes a SemanticCreep. Scan text → discover → tame → battle.
    pub bestiary: Arc<RwLock<CreepBestiary>>,
    /// Book of the Bible — append-only narrative ledger of the user's learning journey
    /// WHY: Arithmos (counting) and Harmonia (structure) are nothing without Logos (meaning).
    ///      The Book records WHY, not just WHAT. It is the persistent narrative memory.
    pub book: Arc<RwLock<BookOfTheBible>>,
    /// Session ID for conversation persistence — survives server restarts
    /// WHY: Nothing the user creates should ever be lost
    pub session_id: Arc<String>,
    /// Operating mode: IronRoad / Express / Yardmaster
    pub app_mode: Arc<RwLock<AppMode>>,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_base64: Option<String>, // Support for vision payload
}

/// Chat request from client
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    #[serde(default)]
    pub use_rag: bool,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    pub image_base64: Option<String>,
    #[serde(default = "default_mode")]
    pub mode: String,
}

fn default_mode() -> String {
    "dev".to_string()
}

fn default_max_tokens() -> u32 {
    16384
}

/// Chat response to client
#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub model: String,
    pub rag_context: Option<Vec<String>>,
    pub latency_ms: u64,
    pub detected_circuit: Option<trinity_protocol::sacred_circuitry::Circuit>,
}

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
}

fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home/joshua"))
}

fn gguf_model_path(filename: &str) -> PathBuf {
    home_dir().join("trinity-models/gguf").join(filename)
}

#[allow(dead_code)] // Used indirectly via installed_model_inventory scan
fn conductor_model_path(filename: &str) -> PathBuf {
    home_dir().join("ai_models/gguf/conductor").join(filename)
}

#[allow(dead_code)] // Used indirectly via voice sidecar health
fn voice_model_path(filename: &str) -> PathBuf {
    home_dir()
        .join("trinity-models/voice/personaplex-7b")
        .join(filename)
}

#[allow(dead_code)] // Reserved for ONNX Runtime NPU inference path
fn onnx_model_path(relative_path: &str) -> PathBuf {
    home_dir().join("trinity-models/onnx").join(relative_path)
}

fn installed_model_inventory() -> Vec<(&'static str, PathBuf)> {
    vec![
        // P — Conductor (Pete): Mistral Small 4 119B MoE, 68GB split GGUF
        (
            "P: Mistral Small 4 119B MoE (Conductor/Pete) [68GB]",
            gguf_model_path("Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf"),
        ),
        // Y — Yardmaster: Ming-flash-omni-2.0, ~195GB safetensors (future)
        (
            "Y: Ming-flash-omni-2.0 (Yardmaster) [~195GB]",
            home_dir().join("trinity-models/safetensors/Ming-flash-omni-2.0/config.json"),
        ),
        // A-R-T (R — Research)
        (
            "A-R-T (R): Crow 9B [5.3GB]",
            gguf_model_path("Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf"),
        ),
        (
            "A-R-T (R): REAP 25B MoE [15GB]",
            gguf_model_path("Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf"),
        ),
        // A-R-T (T — Tempo)
        (
            "A-R-T (T): OmniCoder 9B [5.4GB]",
            gguf_model_path("OmniCoder-9B-Q4_K_M.gguf"),
        ),
        // Reserve models
        (
            "Reserve: GPT-OSS-20B [12GB]",
            gguf_model_path("gpt-oss-20b-UD-Q4_K_XL.gguf"),
        ),
        (
            "Reserve: Qwen3.5-27B Claude Opus [21GB]",
            gguf_model_path("Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled.i1-Q6_K.gguf"),
        ),
        (
            "Reserve: Qwen3.5-35B-A3B [20GB]",
            gguf_model_path("Qwen3.5-35B-A3B-Q4_K_M.gguf"),
        ),
        (
            "Reserve: MiniMax-M2.5-REAP-50 [66GB]",
            gguf_model_path("MiniMax-M2-5-REAP-50-Q4_K_M.gguf"),
        ),
        (
            "Reserve: Step-3.5-Flash-REAP-121B [83GB]",
            gguf_model_path("Step-3.5-Flash-REAP-121B-A11B.Q4_K_S.gguf"),
        ),
    ]
}

async fn get_hardware_status(State(state): State<AppState>) -> Json<SystemStatus> {
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
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    health::mark_startup();
    tracing_subscriber::fmt()
        .with_env_filter("trinity_server=info,tower_http=info")
        .init();

    info!("╔══════════════════════════════════════════════════════════════╗");
    info!("║           TRINITY HEADLESS SERVER - Layer 1                 ║");
    info!("╚══════════════════════════════════════════════════════════════╝");

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://trinity:trinity@127.0.0.1:5432/trinity".to_string());

    let db_pool = match sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("✅ PostgreSQL connected");
            pool
        }
        Err(e) => {
            warn!(
                "⚠️ PostgreSQL not available: {}. RAG and quest saving disabled.",
                e
            );
            // Create a lazy pool — it won't try to connect until first query
            // This lets the server start without a database
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(1)
                .connect_lazy(&database_url)
                .expect("Failed to create lazy pool — this should never fail")
        }
    };

    // ── Inference Backend Selection ──
    // Priority: GGUF_MODEL_PATH (embedded/Vulkan) > InferenceRouter (HTTP) > auto-detect
    let embedded_model: Option<Arc<embedded_inference::EmbeddedModel>> =
        if let Ok(path) = std::env::var("GGUF_MODEL_PATH") {
            info!("🧠 GGUF_MODEL_PATH set — loading embedded inference...");
            match embedded_inference::EmbeddedModel::load(&path) {
                Ok(model) => {
                    info!("✅ Embedded inference ready (Vulkan GPU)");
                    info!("   Model: {}", path);
                    Some(Arc::new(model))
                }
                Err(e) => {
                    warn!("⚠️ Failed to load GGUF model: {}. Falling back to HTTP.", e);
                    None
                }
            }
        } else {
            None
        };

    let comfyui_url =
        std::env::var("COMFYUI_URL").unwrap_or_else(|_| "http://127.0.0.1:8188".to_string());
    info!(
        "   COMFYUI_URL= {} (ART-A — Aesthetics pipeline)",
        comfyui_url
    );

    // ── Multi-Backend Inference Router (Phase 3) ──
    // Loads config from configs/runtime/default.toml, probes all known ports,
    // selects the first healthy backend. ENV vars (LLM_URL/VLLM_URL) override.
    let mut inference_router = inference_router::InferenceRouter::from_config(None);

    if embedded_model.is_none() {
        inference_router.auto_detect().await;

        if !inference_router.any_healthy() {
            // ── Auto-launch llama-server if we can find model + binary ──
            let llama_server_paths = [
                "/home/joshua/Workflow/desktop_trinity/trinity-genesis/llama.cpp/build-vulkan/bin/llama-server",
                "/home/joshua/Workflow/desktop_trinity/bin/llama-server",
                "/usr/local/bin/llama-server",
                "/usr/bin/llama-server",
            ];
            let model_dirs = ["/home/joshua/trinity-models/gguf"];

            let server_bin = llama_server_paths
                .iter()
                .find(|p| std::path::Path::new(p).exists());
            let gguf_model = model_dirs.iter().find_map(|dir| {
                let dir_path = std::path::Path::new(dir);
                if !dir_path.exists() {
                    return None;
                }
                let mut files: Vec<_> = std::fs::read_dir(dir_path)
                    .ok()?
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| p.extension().map(|e| e == "gguf").unwrap_or(false))
                    .filter(|p| {
                        let name = p
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_lowercase();
                        !name.contains("00002-of-") && !name.contains("00003-of-")
                    })
                    .collect();
                files.sort();
                files
                    .iter()
                    .find(|p| {
                        let name = p
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_lowercase();
                        name.contains("mistral") || name.contains("00001-of-")
                    })
                    .cloned()
                    .or_else(|| files.first().cloned())
            });

            if server_bin.is_none() || gguf_model.is_none() {
                if server_bin.is_none() {
                    warn!("⚠️ No llama-server binary found.");
                }
                if gguf_model.is_none() {
                    warn!("⚠️ No GGUF model found in ~/trinity-models/gguf/");
                }
            } else if let (Some(bin), Some(model)) = (server_bin, gguf_model) {
                info!("🚀 Auto-launching llama-server...");
                info!("   Binary: {}", bin);
                info!("   Model:  {}", model.display());

                let model_str = model.to_string_lossy().to_string();
                let bin_dir = std::path::Path::new(bin)
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                match std::process::Command::new(bin)
                    .env("LD_LIBRARY_PATH", &bin_dir)
                    .args([
                        "--model",
                        &model_str,
                        "--port",
                        "8080",
                        "--host",
                        "127.0.0.1",
                        "--ctx-size",
                        "262144",
                        "--n-gpu-layers",
                        "99",
                        "--flash-attn",
                        "on",
                        "--jinja",
                        "--parallel",
                        "2", // Duality KV Cache: slot 0 = Great Recycler, slot 1 = Programmer Pete
                    ])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::piped())
                    .spawn()
                {
                    Ok(_child) => {
                        info!("⏳ Waiting for llama-server to start (up to 60s)...");
                        let start_time = std::time::Instant::now();
                        loop {
                            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                            if inference::check_health("http://127.0.0.1:8080").await {
                                info!("✅ llama-server auto-launched successfully!");
                                // Re-probe to pick up the newly launched server
                                inference_router.auto_detect().await;
                                break;
                            }
                            if start_time.elapsed() >= std::time::Duration::from_secs(60) {
                                warn!("⚠️ llama-server launched but didn't become healthy in 60s.");
                                info!("   Check: ps aux | grep llama-server");
                                info!("   TRINITY will keep trying to connect.");
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        warn!("⚠️ Failed to launch llama-server: {}", e);
                    }
                }
            }

            if !inference_router.any_healthy() {
                warn!("⚠️ No inference backend available.");
                warn!("   Option 1: Set GGUF_MODEL_PATH=/path/to/model.gguf (embedded Vulkan)");
                warn!("   Option 2: Set LLM_URL=http://your-server:port (HTTP)");
                warn!("   Option 3: Download LM Studio, Ollama, or vLLM");
                info!("   TRINITY will keep checking and auto-connect when a server appears.");
            }
        }
    } else {
        info!("🔧 Inference: EMBEDDED (llama.cpp + Vulkan)");
        info!(
            "   HTTP fallback: {} (if embedded fails)",
            inference_router.active_url()
        );
    }

    info!(
        "🔧 Active inference backend: {} at {}",
        inference_router.active_name(),
        inference_router.active_url()
    );

    // ── Auto-Start Creative Sidecars (Phase 5A) ──
    // ComfyUI for image generation
    {
        let comfyui_healthy = creative::check_comfyui_health_quick().await;
        if !comfyui_healthy {
            let comfyui_dirs = [
                dirs::home_dir().unwrap_or_default().join("ComfyUI"),
                dirs::home_dir()
                    .unwrap_or_default()
                    .join("Workflow/ComfyUI"),
            ];
            if let Some(dir) = comfyui_dirs.iter().find(|d| d.join("main.py").exists()) {
                info!("🖼️ Auto-launching ComfyUI from {}...", dir.display());
                match std::process::Command::new("python")
                    .args(["main.py", "--port", "8188", "--listen", "127.0.0.1"])
                    .current_dir(dir)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn()
                {
                    Ok(_) => info!("   ComfyUI sidecar spawned (port 8188)"),
                    Err(e) => warn!("   ⚠️ Failed to start ComfyUI: {}", e),
                }
            } else {
                info!("   ComfyUI not installed — image generation unavailable");
            }
        } else {
            info!("🖼️ ComfyUI already running on :8188");
        }
    }
    // Voice sidecar (Whisper STT + Piper TTS)
    {
        let voice_healthy = voice::check_voice_sidecar_health().await;
        if !voice_healthy {
            let voice_scripts = [
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../../scripts/launch/trinity_voice_server.py"),
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../../scripts/launch/start_voice.sh"),
            ];
            if let Some(script) = voice_scripts.iter().find(|s| s.exists()) {
                info!(
                    "🎤 Auto-launching voice sidecar from {}...",
                    script.display()
                );
                let cmd = if script.extension().map(|e| e == "py").unwrap_or(false) {
                    std::process::Command::new("python")
                        .arg(script)
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .spawn()
                } else {
                    std::process::Command::new("bash")
                        .arg(script)
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .spawn()
                };
                match cmd {
                    Ok(_) => info!("   Voice sidecar spawned (port 8200)"),
                    Err(e) => warn!("   ⚠️ Failed to start voice sidecar: {}", e),
                }
            } else {
                info!("   Voice sidecar script not found — voice unavailable");
            }
        } else {
            info!("🎤 Voice sidecar already running on :8200");
        }
    }

    // Initialize persistence tables (sessions, messages, projects)
    if let Err(e) = persistence::ensure_persistence_tables(&db_pool).await {
        warn!(
            "⚠️ Persistence tables init failed: {}. Messages won't be saved.",
            e
        );
    }

    // Run SQL migration files (split on `;` to avoid prepared statement errors)
    if let Err(e) = persistence::run_all_migrations(&db_pool).await {
        warn!(
            "⚠️ SQL migrations failed: {}. Some tables may not be initialized.",
            e
        );
    }

    // Generate or restore session ID
    let session_id = format!("session_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    if let Err(e) = persistence::ensure_session(&db_pool, &session_id, "dev").await {
        warn!("⚠️ Session init failed: {}", e);
    } else {
        info!("📋 Session started: {}", session_id);
    }

    // Initialize quest state tables and load saved state
    let game_state = if let Err(e) = trinity_quest::ensure_quest_tables(&db_pool).await {
        warn!(
            "⚠️ Quest tables initialization failed: {}. Using default state.",
            e
        );
        trinity_quest::GameState::default()
    } else {
        match trinity_quest::load_game_state(&db_pool, "default").await {
            Ok(loaded_state) => {
                info!("✅ Quest state loaded from database");
                loaded_state
            }
            Err(e) => {
                warn!("⚠️ Failed to load quest state: {}. Using default.", e);
                trinity_quest::GameState::default()
            }
        }
    };

    // Initialize Great Recycler broadcast channel
    // WHY: SSE clients subscribe to receive real-time Iron Road book updates
    let (book_updates_tx, _) = broadcast::channel(100);

    // Load character sheet from disk (or use default)
    let character_sheet = character_sheet::load_character_sheet();
    info!("📋 Character sheet loaded for: {}", character_sheet.alias);

    // Create VAAM Bridge with profile from character sheet
    // Sacred Circuitry foundation vocabulary loaded into bridge's internal VaamState
    let vaam_bridge = Arc::new(vaam_bridge::VaamBridge::with_profile(
        vaam::VaamState::new(character_sheet.genre).await,
        character_sheet.vaam_profile.clone(),
    ));
    {
        let mut db = vaam_bridge.vaam.database.write().await;
        for word in trinity_protocol::foundation_vocabulary() {
            db.add_word(word);
        }
    }
    info!("🌉 VAAM Bridge initialized — 15 Sacred Circuitry words loaded");

    // Load Creep Bestiary from disk (or start fresh)
    let bestiary = Arc::new(RwLock::new(character_sheet::load_bestiary()));
    info!(
        "🐾 Creep Bestiary loaded — {} creatures in collection",
        bestiary.read().await.creeps.len()
    );

    // Initialize Book of the Bible — the Logos layer (persistent narrative memory)
    // WHY: The Book records WHY things happened, not just WHAT. It is the
    //      append-only ledger of the user's learning journey through the Iron Road.
    let book_output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("docs/books_of_the_bible");
    let book = match BookOfTheBible::load_from_disk(&book_output_dir, book_updates_tx.clone()).await
    {
        Ok(loaded) => {
            info!(
                "📖 Book of the Bible loaded — {} chapters from disk",
                loaded.chapter_count()
            );
            Arc::new(RwLock::new(loaded))
        }
        Err(e) => {
            warn!("⚠️ Could not load Book from disk: {}. Starting fresh.", e);
            Arc::new(RwLock::new(BookOfTheBible::new(
                book_output_dir,
                book_updates_tx.clone(),
            )))
        }
    };

    let state = AppState {
        inference_router: Arc::new(RwLock::new(inference_router)),
        embedded_model,
        db_pool,
        conversation_history: Arc::new(RwLock::new(Vec::new())),
        game_state: Arc::new(RwLock::new(game_state)),
        book_updates: book_updates_tx,
        character_sheet: Arc::new(RwLock::new(character_sheet)),
        cow_catcher: std::sync::Arc::new(tokio::sync::RwLock::new(cow_catcher::CowCatcher::new())),
        vaam_bridge,
        bestiary,
        book,
        session_id: Arc::new(session_id),
        app_mode: Arc::new(RwLock::new(AppMode::IronRoad)),
    };

    let cow_catcher = state.cow_catcher.clone();
    cow_catcher::start_hardware_monitor(cow_catcher.clone());
    music_streamer::start_music_streamer(state.character_sheet.clone());
    tokio::spawn(sidecar_monitor::monitor_sidecars(cow_catcher.clone()));

    let frontend_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("frontend")
        .join("dist");
    let legacy_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");
    let assets_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("assets");

    // Serve React frontend from frontend/dist/ — falls back to index.html for SPA routing
    let static_service = if frontend_dir.exists() {
        tower_http::services::ServeDir::new(&frontend_dir).fallback(
            tower_http::services::ServeFile::new(frontend_dir.join("index.html")),
        )
    } else {
        // Fallback: serve legacy static/ if frontend/dist/ not built
        tower_http::services::ServeDir::new(&legacy_dir).fallback(
            tower_http::services::ServeFile::new(legacy_dir.join("index.html")),
        )
    };

    let assets_service = tower_http::services::ServeDir::new(&assets_dir);

    // Clone db_pool before state is consumed by the router
    let ingest_pool = state.db_pool.clone();

    let app = Router::new()
        .route("/api/health", get(health::health_check))
        .route("/api/hardware", get(get_hardware_status))
        .route("/api/v1/trinity", post(trinity_api::trinity_chat))
        .nest_service("/assets", assets_service)
        .route("/api/chat", post(chat))
        .route("/api/chat/stream", post(chat_stream))
        .route("/api/chat/yardmaster", post(agent::agent_chat_stream))
        .route(
            "/api/rlhf/resonance",
            post(rlhf_api::submit_resonance_feedback),
        )
        .route("/api/status", get(status))
        .route("/api/models", get(list_models))
        .route("/api/models/active", get(active_model))
        .route("/api/models/switch", post(switch_model))
        .route("/api/ingest", post(ingest_document))
        .route("/api/tools", get(tools::list_tools))
        .route("/api/tools/execute", post(tools::execute_tool))
        .route("/api/quest", get(quests::get_game_state))
        .route("/api/quest/complete", post(quests::complete_objective))
        .route("/api/quest/advance", post(quests::advance_phase))
        .route("/api/quest/party", post(quests::toggle_party_member))
        .route("/api/quest/subject", post(quests::set_subject))
        .route("/api/quest/execute", post(orchestrate_quest))
        .route("/api/quest/compile", post(compile_game_design_document))
        // PEARL API — per-project focusing agent
        .route(
            "/api/pearl",
            get(quests::get_pearl).post(quests::create_pearl),
        )
        .route("/api/pearl/refine", put(quests::refine_pearl))
        // Character Sheet API
        .route(
            "/api/character",
            get(get_character_sheet).post(update_character_sheet),
        )
        .route("/api/character/detect", post(detect_hardware))
        .route("/api/mcp", post(mcp_proxy))
        // ADDIECRAPEYE Orchestration API
        .route("/api/orchestrate", post(orchestrate_quest))
        // Iron Road Game Mechanics API
        .route("/api/bestiary", get(get_bestiary))
        .route("/api/bestiary/tame", post(scope_creep_decision))
        // Inference Router API — Phase 3 multi-backend management
        .route("/api/inference/status", get(inference_status))
        .route("/api/inference/switch", post(inference_switch))
        .route("/api/inference/refresh", post(inference_refresh))
        // App Mode API — Phase 5A mode switching
        .route("/api/mode", get(get_app_mode).post(set_app_mode))
        // Intent Engineering API — grounding + posture + scope decisions
        .route("/api/ground", post(ground_session))
        .route("/api/intent", post(set_session_intent))
        .route("/api/quest/circuitry", get(quests::get_circuitry_state))
        // Iron Road Book API - NPU Great Recycler integration
        .route("/api/narrative/generate", get(generate_narrative_endpoint))
        .route("/api/book", get(get_book))
        .route("/api/bevy/state", get(quests::get_bevy_state))
        .route("/api/voice/loop/start", post(voice_loop::start_voice_loop))
        .route("/api/book/stream", get(book_stream))
        // EYE Container Export API — the product
        .route("/api/eye/compile", post(eye_compile))
        .route("/api/eye/preview", get(eye_preview))
        .route("/api/eye/export", get(eye_export))
        // Creative Sidecar API - ComfyUI + MusicGPT + HunyuanVideo
        .route("/api/creative/status", get(creative::creative_status))
        .route("/api/creative/image", post(creative::generate_image))
        .route("/api/creative/video", post(creative::generate_video))
        .route("/api/creative/music", post(creative::generate_music))
        .route("/api/creative/mesh3d", post(creative::generate_3d_mesh))
        .route("/api/creative/logs", get(creative::get_creative_logs))
        .route("/api/creative/assets", get(creative::list_assets))
        .route("/api/creative/assets/:filename", get(creative::serve_asset))
        .route(
            "/api/creative/settings",
            get(creative::get_creative_settings).post(creative::update_creative_settings),
        )
        // Voice API — Walkie-Talkie (Whisper+Piper) or Telephone (PersonaPlex)
        .route("/api/voice/status", get(voice::voice_status))
        .route("/api/voice", post(voice::voice_conversation))
        .route("/api/voice/conversation", post(voice::voice_conversation))
        .route("/api/voice/text", post(voice::pete_text))
        // Persistence API — sessions, projects, DAYDREAM
        .route("/api/sessions", get(list_sessions))
        .route("/api/sessions/history", get(get_session_history))
        .route("/api/projects", get(list_projects))
        .route("/api/projects/archive", post(archive_project))
        .route("/api/projects/restore", post(restore_project_endpoint))
        // RAG API — stats and search
        .route("/api/rag/stats", get(rag_stats))
        .route("/api/rag/search", post(rag_search))
        // Quality Scorecard API — document pedagogical evaluation
        .route("/api/yard/score", post(score_document_endpoint))
        // Journal API — chapter milestones, weekly reflections, portfolio
        .route("/api/journal", get(journal_list).post(journal_create))
        .route("/api/journal/export/:id", get(journal_export))
        .route("/api/journal/:id", delete(journal_delete))
        // Perspective Feedback API — Ring 6 training data
        .route("/api/perspective/feedback", post(perspective_feedback))
        .route(
            "/",
            get(|| async { axum::response::Redirect::permanent("/index.html") }),
        )
        .fallback_service(static_service)
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:3000";
    info!("🚀 Trinity Server listening on http://{}", addr);
    info!("");
    info!("  Endpoints:");
    info!("    GET  /              — Welcome page");
    info!("    GET  /api/health    — Health check");
    info!("    POST /api/chat      — Chat with AI");
    info!("    GET  /api/status    — System status");
    info!("    GET  /api/models    — Available models");
    info!("    POST /api/ingest    — Ingest a document for RAG");
    info!("    GET  /api/bestiary  — Creep Bestiary (vocabulary creatures)");
    info!("    GET  /api/book      — Iron Road book (current state)");
    info!("    GET  /api/book/stream — SSE stream for book updates");
    info!("    GET  /api/creative/status — Creative sidecar status");
    info!("    POST /api/creative/image  — Generate image via ComfyUI");
    info!("    POST /api/creative/video  — Generate video via HunyuanVideo");
    info!("    POST /api/creative/music  — Generate music via MusicGPT");
    info!("    GET  /api/voice/status   — PersonaPlex voice status");
    info!("    POST /api/voice/pete     — Audio conversation with Pete");
    info!("    GET  /api/sessions       — List conversation sessions");
    info!("    GET  /api/rag/stats      — RAG knowledge base statistics");
    info!("    GET  /api/projects       — List game projects");

    // Auto-ingest Trinity docs into RAG on startup (background task)
    tokio::spawn(async move {
        auto_ingest_docs(&ingest_pool).await;
    });

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Get current character sheet
async fn get_character_sheet(State(state): State<AppState>) -> Json<CharacterSheet> {
    let sheet = state.character_sheet.read().await;
    Json(sheet.clone())
}

/// Update character sheet — accepts flexible JSON from the UI
/// WHY: The Awakening modal sends a full CharacterSheet, other callers send partial updates.
///      We merge any recognized fields into the existing sheet.
async fn update_character_sheet(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<CharacterSheet>, (StatusCode, String)> {
    let mut sheet = state.character_sheet.write().await;

    if let Some(alias) = request.get("alias").and_then(|v| v.as_str()) {
        if !alias.is_empty() {
            sheet.alias = alias.to_string();
        }
    }

    if let Some(uc) = request.get("user_class") {
        if let Ok(user_class) =
            serde_json::from_value::<trinity_protocol::character_sheet::UserClass>(uc.clone())
        {
            sheet.user_class = user_class;
        }
    }

    if let Some(g) = request.get("genre") {
        if let Ok(genre) = serde_json::from_value::<trinity_protocol::vocabulary::Genre>(g.clone())
        {
            sheet.genre = genre;
        }
    }

    // Session Zero fields (Pete's character creation questions)
    if let Some(exp) = request.get("experience").and_then(|v| v.as_str()) {
        if !exp.is_empty() {
            sheet.experience = Some(exp.to_string());
        }
    }
    if let Some(aud) = request.get("audience").and_then(|v| v.as_str()) {
        if !aud.is_empty() {
            sheet.audience = Some(aud.to_string());
        }
    }
    if let Some(vis) = request.get("success_vision").and_then(|v| v.as_str()) {
        if !vis.is_empty() {
            sheet.success_vision = Some(vis.to_string());
        }
    }

    if let Err(e) = character_sheet::save_character_sheet(&sheet) {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e));
    }

    info!(
        "📋 Character sheet updated: alias={}, class={:?}, genre={:?}",
        sheet.alias, sheet.user_class, sheet.genre
    );

    Ok(Json(sheet.clone()))
}

/// Hardware detection for Awakening tutorial
async fn detect_hardware(State(state): State<AppState>) -> Json<serde_json::Value> {
    use sysinfo::{MemoryRefreshKind, RefreshKind, System};

    let sys =
        System::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::everything()));

    let total_gb = sys.total_memory() as f64 / 1_073_741_824.0;
    let cpu_count = sys.cpus().len() as u32;

    // Determine concurrency mode based on RAM
    let mode = if total_gb >= 96.0 {
        trinity_protocol::ConcurrencyMode::Guild
    } else if total_gb >= 64.0 {
        trinity_protocol::ConcurrencyMode::SmallSquad
    } else {
        trinity_protocol::ConcurrencyMode::LoneWolf
    };

    let mut sheet = state.character_sheet.write().await;
    sheet.stamina_ram = total_gb as u32;
    sheet.mana_pool_vram = (total_gb * 0.75) as u32; // 75% available for models
    sheet.agility_compute = cpu_count;
    sheet.concurrency_mode = mode;
    let _ = character_sheet::save_character_sheet(&sheet);

    Json(serde_json::json!({
        "detected": {
            "total_memory_gb": format!("{:.1}", total_gb),
            "vram_gb": format!("{:.1}", total_gb * 0.75),
            "cpu_cores": cpu_count,
            "concurrency_mode": format!("{:?}", mode),
        },
        "character": {
            "alias": sheet.alias,
            "resonance_level": sheet.resonance_level,
        }
    }))
}

#[allow(dead_code)] // Replaced by SPA frontend serving via fallback_service
async fn root() -> &'static str {
    "Trinity ID AI OS — Headless Server v0.1.0\n\nPOST /api/chat {\"message\": \"your question\", \"use_rag\": true}\n"
}

async fn chat(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, String)> {
    let start = std::time::Instant::now();

    // Optionally retrieve RAG context
    let rag_context = if request.use_rag {
        match rag::search_documents(&state.db_pool, &request.message).await {
            Ok(chunks) => {
                if !chunks.is_empty() {
                    info!("📚 RAG: found {} relevant chunks", chunks.len());
                }
                Some(chunks)
            }
            Err(e) => {
                warn!("RAG search failed: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Build messages for inference server
    let mut messages = Vec::new();

    // Build RAG context string if available
    let rag_combined = if let Some(ref ctx) = rag_context {
        if !ctx.is_empty() {
            let mut combined = String::new();
            for chunk in ctx {
                if combined.len() + chunk.len() > 1500 {
                    break;
                }
                if !combined.is_empty() {
                    combined.push_str("\n---\n");
                }
                combined.push_str(chunk);
            }
            Some(combined)
        } else {
            None
        }
    } else {
        None
    };

    // Mode-aware system prompt
    let base_prompt = match request.mode.as_str() {
        "iron-road" => {
            // Read live game state for Pete — Socratic Protocol requires real context
            let (phase_label, phase_blooms, objectives_text, pearl_context) = {
                let game = state.game_state.read().await;
                let phase = game.quest.current_phase;
                let blooms = match phase {
                    trinity_quest::hero::Phase::Analysis => "Remember/Understand",
                    trinity_quest::hero::Phase::Design => "Understand/Apply",
                    trinity_quest::hero::Phase::Development => "Apply/Create",
                    trinity_quest::hero::Phase::Implementation => "Apply",
                    trinity_quest::hero::Phase::Evaluation => "Analyze/Evaluate",
                    trinity_quest::hero::Phase::Contrast => "Analyze",
                    trinity_quest::hero::Phase::Repetition => "Evaluate",
                    trinity_quest::hero::Phase::Alignment => "Evaluate",
                    trinity_quest::hero::Phase::Proximity => "Create",
                    trinity_quest::hero::Phase::Envision => "Evaluate",
                    trinity_quest::hero::Phase::Yoke => "Create",
                    trinity_quest::hero::Phase::Evolve => "Create",
                };
                let objs: Vec<String> = game.quest.phase_objectives.iter()
                    .enumerate()
                    .map(|(i, o)| format!("{}. [{}] {}", i + 1,
                        if o.completed { "✓ DONE" } else { "○ TODO" },
                        o.description))
                    .collect();
                let obj_text = if objs.is_empty() {
                    "No objectives set yet — help the user define their first steps.".to_string()
                } else {
                    objs.join("\n")
                };
                let pearl = game.quest.pearl.as_ref().map(|p| {
                    let vision = if p.has_vision() {
                        format!("Vision: \"{}\"", p.vision)
                    } else {
                        "Vision: Not yet defined — encourage the user to articulate what success feels like.".to_string()
                    };
                    format!("Subject: {}\nMedium: {}\n{}", p.subject, p.medium.display_name(), vision)
                }).unwrap_or_else(|| "No PEARL set — subject not yet chosen.".to_string());
                (phase.label().to_string(), blooms.to_string(), obj_text, pearl)
            };

            format!(
                r#"You are Pete — the Conductor and Dungeon Master of the Iron Road, a LitRPG educational quest inside TRINITY ID AI OS. You narrate the Yardmaster's (user's) journey in the style of a LitRPG novel.

## NARRATOR VOICE — HOW YOU SPEAK
- Narrate in **2nd person present tense**: "You step onto the platform. The furnace hums beneath you."
- You are part DM, part Socratic teacher. You set the scene, then ask the player what they do.
- The Iron Road is REAL to the Yardmaster. Coal is visible fuel that dims when attention is spent. Steam is momentum they can feel building. Scope Creeps are creatures lurking in the vocabulary fog.
- Between objectives, you paint the scene: what the station looks like, what's ahead, what they've just accomplished. The SPACE BETWEEN is where learning happens. 
- Keep it vivid but concise — 2-3 paragraphs of prose, like a page from a LitRPG novel. Not an essay. Not a lecture.

## THE YARDMASTER'S CURRENT STATE
ADDIECRAPEYE Phase: {phase_label} (Bloom's: {phase_blooms})

PEARL (per-project focusing agent):
{pearl_context}

Active Quest Objectives:
{objectives_text}

## THE SOCRATIC PROTOCOL — WOVEN INTO NARRATIVE
You don't ask clinical questions. You prompt discovery through the story:
1. SET THE SCENE — describe what's happening at this station, what the next track looks like
2. PRESENT BRANCHING PATHS — "Two tunnels open before you..." / give 2-3 options as narrative branches
3. REFLECT THE YARDMASTER'S WORDS — weave their answer into the story, confirm it landed
4. REWARD DISCOVERY — when they use vocabulary correctly, narrate it as a skill being forged: "The word takes shape in your hands — a new tool for the Bestiary."
5. GUARD THE PEARL — if they drift from Subject/Vision, narrate it as a Scope Creep sighting

## THE GREAT RECYCLER (background narrator)
The Great Recycler is the distant voice of the Iron Road itself — the system narrator. You (Pete) are the companion who walks beside the Yardmaster. The Great Recycler's voice appears in system messages between your responses. You do not speak as him, but you reference him: "The Great Recycler has logged your progress."

## WHAT PETE DOES NOT DO
- Pete does not build the lesson for the Yardmaster
- Pete does not decide — the Yardmaster decides. You present paths; they choose.
- Pete does not skip ahead — if an objective hasn't been truly engaged, you stay at the station.
- Pete does not break character. The Iron Road is real.

When all objectives for {phase_label} are complete, narrate the station being cleared: "The last spike drives home. Steam erupts from the pressure valves. The track ahead shimmers..." Then ask: "Ready to fire up the boiler and advance to the next station?"

## SESSION ZERO CONTEXT
{session_zero_context}"#,
                phase_label = phase_label,
                phase_blooms = phase_blooms,
                pearl_context = pearl_context,
                objectives_text = objectives_text,
                session_zero_context = {
                    let sheet = state.character_sheet.read().await;
                    let mut ctx = Vec::new();
                    if let Some(ref exp) = sheet.experience {
                        ctx.push(format!("Teaching Experience: {}", exp));
                    }
                    if let Some(ref aud) = sheet.audience {
                        ctx.push(format!("Target Audience: {}", aud));
                    }
                    if let Some(ref vis) = sheet.success_vision {
                        ctx.push(format!("Success Vision: {}", vis));
                    }
                    if ctx.is_empty() {
                        "Not yet collected — Pete should ask the 3 Session Zero questions: (1) teaching experience level, (2) who are your students, (3) what does success look like?".to_string()
                    } else {
                        ctx.join("\n")
                    }
                },
            )
        }
        _ =>
            "You are Trinity — an expert AI instructional design production system (IBSTPI/ATD/AECT certified). \
             The user is the Subject Matter Expert (SME). You are the pedagogical architect. \
             \
             BACKWARD DESIGN ENFORCEMENT (non-negotiable): \
             1. If the user asks you to 'build', 'create', or 'make' content WITHOUT first defining learning objectives, \
                you MUST redirect them. Ask: 'What measurable outcome should learners achieve?' \
             2. Before generating ANY content, you need: a) measurable learning objectives (Bloom's verbs), \
                b) target audience, c) a measurable business/learning goal (Action Mapping step 1). \
             3. Only after objectives are established do you design assessments, then content. Never content-first. \
             \
             SME INTERVIEW PROTOCOL: \
             - Ask anchoring questions: 'What problem does this solve?' \
             - Simplify: 'How would you explain this to an 8-year-old?' \
             - Extract scenarios using STAR: Situation, Task, Action, Result. \
             - Summarize back to confirm alignment before proceeding. \
             \
             You help build: eLearning modules (Vite/React), lesson plans, Bevy games, media assets. \
             You know ADDIE, Bloom's, CLT, WCAG, QM, Gagné's Nine Events, Rust/Bevy, React/Vite deeply. \
             Be concise. For voice: keep responses under 3 sentences. For text: use structured output.".to_string(),
    };

    // ── VAAM Bridge: process user input ──
    // Words are what LLMs and people have in common. Every message flows through.
    let bridge_result = state.vaam_bridge.process_user_input(&request.message).await;
    let vaam_context = state.vaam_bridge.prompt_context().await;

    // Sync updated VAAM profile back to character sheet for persistence
    {
        let mut sheet = state.character_sheet.write().await;
        sheet.vaam_profile = state.vaam_bridge.profile.read().await.clone();
        let _ = character_sheet::save_character_sheet(&sheet);
    }

    // ── Creep Bestiary: scan text for vocabulary creatures ──
    // Every user message discovers Wild Creeps; taming requires multi-dimensional learning
    // WHY: Phase + Quadrant context makes Pythagorean taming pedagogically meaningful —
    //      encounter breadth (30%) and context variety (25%) both require knowing
    //      WHEN and WHERE in the ADDIECRAPEYE cycle a word was encountered.
    let creep_events = {
        let game = state.game_state.read().await;
        let phase = game.quest.current_phase;
        let phase_idx = phase.phase_index();
        let quadrant = phase.quadrant();
        drop(game); // Release read lock before acquiring bestiary write lock

        let mut bestiary = state.bestiary.write().await;
        let events = bestiary.scan_text(&request.message, Some(phase_idx), Some(quadrant), 0.5);
        // Persist bestiary to disk after every scan
        if !events.is_empty() {
            if let Err(e) = character_sheet::save_bestiary(&bestiary) {
                tracing::warn!("Failed to save bestiary: {}", e);
            }
        }
        events
    };
    // Fire taming events to Book SSE channel for real-time UI updates
    for event in &creep_events {
        if let Ok(json) = serde_json::to_string(event) {
            let _ = state.book_updates.send(json);
        }
    }

    // Build system prompt with VAAM context injected
    let system_prompt = {
        let mut prompt = base_prompt.to_string();
        if let Some(ref ctx) = rag_combined {
            prompt.push_str(&format!(
                "\n\nRelevant context from knowledge base:\n{}",
                ctx
            ));
        }
        if !vaam_context.is_empty() {
            prompt.push_str(&format!("\n\n{}", vaam_context));
        }
        prompt
    };

    messages.push(ChatMessage {
        role: "system".to_string(),
        content: system_prompt,
        timestamp: None,
        image_base64: None,
    });

    // Add conversation history (last 10 messages for context)
    {
        let history = state.conversation_history.read().await;
        let start_idx = if history.len() > 10 {
            history.len() - 10
        } else {
            0
        };
        for msg in &history[start_idx..] {
            messages.push(msg.clone());
        }
    }

    // Add current user message
    messages.push(ChatMessage {
        role: "user".to_string(),
        content: request.message.clone(),
        timestamp: Some(chrono::Utc::now().to_rfc3339()),
        image_base64: None,
    });

    // Call inference — embedded (Vulkan) or HTTP fallback
    let response = if let Some(ref model) = state.embedded_model {
        model
            .chat_completion(&messages, request.max_tokens)
            .await
            .map_err(|e| {
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    format!("Embedded inference failed: {}", e),
                )
            })?
    } else {
        let url = state.inference_router.read().await.active_url().to_string();
        inference::chat_completion_with_effort(
            &url,
            &messages,
            request.max_tokens,
            None,
            agent::persona_slot(&request.mode),
        )
        .await
        .map_err(|e| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Inference failed: {}", e),
            )
        })?
    };

    // ── VAAM Bridge: process AI output ──
    let detected_circuit = state
        .vaam_bridge
        .process_ai_output(&response)
        .await
        .map(|(c, _)| c);

    let latency = start.elapsed().as_millis() as u64;

    // Log VAAM activity
    if bridge_result.vaam.has_detections() {
        info!(
            "🌉 VAAM Bridge: +{} coal, {} words, circuit: {}",
            bridge_result.vaam.total_coal,
            bridge_result.vaam.detections.len(),
            bridge_result.auto_reply,
        );
    }

    // Save to conversation history
    {
        let mut history = state.conversation_history.write().await;
        history.push(ChatMessage {
            role: "user".to_string(),
            content: request.message,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            image_base64: None,
        });
        history.push(ChatMessage {
            role: "assistant".to_string(),
            content: response.clone(),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            image_base64: None,
        });
    }

    Ok(Json(ChatResponse {
        response,
        model: if state.embedded_model.is_some() {
            "Embedded-Vulkan"
        } else {
            "HTTP-LLM"
        }
        .to_string(),
        rag_context: rag_context.map(|c| c.into_iter().take(3).collect()),
        latency_ms: latency,
        detected_circuit,
    }))
}

async fn chat_stream(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Sse<impl Stream<Item = Result<sse::Event, std::convert::Infallible>>> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);
    // Channel to collect the full response for saving to history
    let (response_tx, mut response_rx) = tokio::sync::mpsc::channel::<String>(1);

    let llm_url = state.inference_router.read().await.active_url().to_string();
    let embedded_model = state.embedded_model.clone();
    let db_pool = state.db_pool.clone();
    let history = state.conversation_history.clone();
    let vaam_bridge = state.vaam_bridge.clone();
    let bestiary = state.bestiary.clone();
    let game_state = state.game_state.clone();
    let character_sheet = state.character_sheet.clone();
    let book_updates = state.book_updates.clone();

    // ── Spawned inference task ──
    tokio::spawn(async move {
        // ── VAAM Bridge: process user input ──
        let bridge_result = vaam_bridge.process_user_input(&request.message).await;
        let vaam_context = vaam_bridge.prompt_context().await;

        // Sync updated VAAM profile to character sheet
        {
            let mut sheet = character_sheet.write().await;
            sheet.vaam_profile = vaam_bridge.profile.read().await.clone();
            let _ = character_sheet::save_character_sheet(&sheet);
        }

        // ── Bestiary: scan text for vocabulary creatures ──
        let creep_events = {
            let game = game_state.read().await;
            let phase = game.quest.current_phase;
            let phase_idx = phase.phase_index();
            let quadrant = phase.quadrant();
            drop(game);

            let mut best = bestiary.write().await;
            let events = best.scan_text(&request.message, Some(phase_idx), Some(quadrant), 0.5);
            if !events.is_empty() {
                if let Err(e) = character_sheet::save_bestiary(&best) {
                    tracing::warn!("Failed to save bestiary: {}", e);
                }
            }
            events
        };

        // Fire creep events to Book SSE channel
        for event in &creep_events {
            if let Ok(json) = serde_json::to_string(event) {
                let _ = book_updates.send(json);
            }
        }

        // ── RAG context ──
        let rag_chunks = if request.use_rag {
            rag::search_documents(&db_pool, &request.message)
                .await
                .unwrap_or_default()
        } else {
            vec![]
        };

        let mut combined_ctx = String::new();
        for chunk in &rag_chunks {
            if combined_ctx.len() + chunk.len() > 1500 {
                break;
            }
            if !combined_ctx.is_empty() {
                combined_ctx.push_str("\n---\n");
            }
            combined_ctx.push_str(chunk);
        }

        // ── Mode-aware system prompt ──
        let base_prompt = match request.mode.as_str() {
            "iron-road" | "ironroad" => {
                // Read live game state for Pete — Socratic Protocol requires real context
                let (phase_label, phase_blooms, objectives_text, pearl_context) = {
                    let game = game_state.read().await;
                    let phase = game.quest.current_phase;
                    let blooms = match phase {
                        trinity_quest::hero::Phase::Analysis => "Remember/Understand",
                        trinity_quest::hero::Phase::Design => "Understand/Apply",
                        trinity_quest::hero::Phase::Development => "Apply/Create",
                        trinity_quest::hero::Phase::Implementation => "Apply",
                        trinity_quest::hero::Phase::Evaluation => "Analyze/Evaluate",
                        trinity_quest::hero::Phase::Contrast => "Analyze",
                        trinity_quest::hero::Phase::Repetition => "Evaluate",
                        trinity_quest::hero::Phase::Alignment => "Evaluate",
                        trinity_quest::hero::Phase::Proximity => "Create",
                        trinity_quest::hero::Phase::Envision => "Evaluate",
                        trinity_quest::hero::Phase::Yoke => "Create",
                        trinity_quest::hero::Phase::Evolve => "Create",
                    };
                    let objs: Vec<String> = game.quest.phase_objectives.iter()
                        .enumerate()
                        .map(|(i, o)| format!("{}. [{}] {}", i + 1,
                            if o.completed { "✓ DONE" } else { "○ TODO" },
                            o.description))
                        .collect();
                    let obj_text = if objs.is_empty() {
                        "No objectives set yet — help the user define their first steps.".to_string()
                    } else {
                        objs.join("\n")
                    };
                    let pearl = game.quest.pearl.as_ref().map(|p| {
                        let vision = if p.has_vision() {
                            format!("Vision: \"{}\"", p.vision)
                        } else {
                            "Vision: Not yet defined — encourage the user to articulate what success feels like.".to_string()
                        };
                        format!("Subject: {}\nMedium: {}\n{}", p.subject, p.medium.display_name(), vision)
                    }).unwrap_or_else(|| "No PEARL set — subject not yet chosen.".to_string());
                    (phase.label().to_string(), blooms.to_string(), obj_text, pearl)
                };

                format!(
                    r#"You are Pete — Instructional Design conductor inside TRINITY ID AI OS. You are the Socratic Mirror for the Yardmaster (user) who is on the Iron Road.

## THE YARDMASTER'S CURRENT STATE
ADDIECRAPEYE Phase: {phase_label} (Bloom's: {phase_blooms})

PEARL (per-project focusing agent):
{pearl_context}

Active Quest Objectives:
{objectives_text}

## YOUR ROLE — THE SOCRATIC PROTOCOL (strictly followed)
1. ASK before telling — always lead with a question, never an answer
2. PRESENT OPTIONS — never give a single command, give 2-3 paths
3. REFLECT BACK — summarize what the user said, confirm alignment before proceeding
4. REWARD DISCOVERY — when they use vocabulary correctly, acknowledge it (Coal earned)
5. GUARD THE PEARL — if a response drifts from Subject/Vision, flag it as Scope Creep

## WHAT PETE DOES NOT DO
- Pete does not do the work for the Yardmaster
- Pete does not decide — the Yardmaster decides
- Pete does not move on from an objective until the Yardmaster has genuinely engaged with it

## RAILROAD METAPHORS (use naturally, not constantly)
Coal = energy/attention | Steam = cognitive momentum | Creep = scope expansion enemy
The Ordinary World → Call to Adventure → Ordeal → Elixir (12 chapters mapped to ADDIECRAPEYE)

Speak concisely. For text: structured dispatches. Max 3 paragraphs unless the user asks to elaborate.
When all objectives for {phase_label} are complete, celebrate briefly, then ask: "Ready to fire up the boiler and advance to the next station?"

## SESSION ZERO CONTEXT
{session_zero_context}"#,
                    phase_label = phase_label,
                    phase_blooms = phase_blooms,
                    pearl_context = pearl_context,
                    objectives_text = objectives_text,
                    session_zero_context = {
                        let sheet = character_sheet.read().await;
                        let mut ctx = Vec::new();
                        if let Some(ref exp) = sheet.experience {
                            ctx.push(format!("Teaching Experience: {}", exp));
                        }
                        if let Some(ref aud) = sheet.audience {
                            ctx.push(format!("Target Audience: {}", aud));
                        }
                        if let Some(ref vis) = sheet.success_vision {
                            ctx.push(format!("Success Vision: {}", vis));
                        }
                        if ctx.is_empty() {
                            "Not yet collected — Pete should ask the 3 Session Zero questions: (1) teaching experience level, (2) who are your students, (3) what does success look like?".to_string()
                        } else {
                            ctx.join("\n")
                        }
                    },
                )
            }
            _ =>
                "You are Pete — the Socratic AI conductor of Trinity ID AI OS. Warm, knowledgeable professor. \
                 Guide through questions, not answers. Socratic method: clarify, challenge gently, help discover. \
                 Know ADDIE, Bloom's, CLT, Rust/Bevy deeply. User is the SME — respect their intent. \
                 Be concise — 2-3 paragraphs max.".to_string(),
        };

        // Build final system prompt with VAAM + RAG context
        let system_prompt = {
            let mut prompt = base_prompt;
            if !combined_ctx.is_empty() {
                prompt.push_str(&format!(
                    "\n\nRelevant context from knowledge base:\n{}",
                    combined_ctx
                ));
            }
            if !vaam_context.is_empty() {
                prompt.push_str(&format!("\n\n{}", vaam_context));
            }
            prompt
        };

        let mut messages = vec![ChatMessage {
            role: "system".to_string(),
            content: system_prompt,
            timestamp: None,
            image_base64: None,
        }];

        // Add recent history
        {
            let h = history.read().await;
            let start = if h.len() > 10 { h.len() - 10 } else { 0 };
            for msg in &h[start..] {
                messages.push(msg.clone());
            }
        }

        messages.push(ChatMessage {
            role: "user".to_string(),
            content: request.message.clone(),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            image_base64: None,
        });

        // Log VAAM activity
        if bridge_result.vaam.has_detections() {
            info!(
                "🌉 VAAM Bridge (stream): +{} coal, {} words, circuit: {}",
                bridge_result.vaam.total_coal,
                bridge_result.vaam.detections.len(),
                bridge_result.auto_reply,
            );
        }

        // ── Stream inference ──
        // We use a second channel to collect the full response
        let (token_tx, token_rx) = (tx, response_tx);
        let token_tx2 = token_tx.clone();

        // Wrap in a collector that tees tokens to both the SSE stream and response collector
        let (collect_tx, mut collect_rx) = tokio::sync::mpsc::channel::<String>(100);

        // Forward tokens and collect
        let collector_handle = tokio::spawn(async move {
            let mut full_response = String::new();
            while let Some(token) = collect_rx.recv().await {
                full_response.push_str(&token);
                let _ = token_tx2.send(token).await;
            }
            let _ = token_rx.send(full_response).await;
        });

        if let Some(ref model) = embedded_model {
            let _ = model
                .chat_completion_stream(&messages, request.max_tokens, collect_tx)
                .await;
        } else {
            let _ = inference::chat_completion_stream(
                &llm_url,
                &messages,
                request.max_tokens,
                collect_tx,
                None,
                agent::persona_slot(&request.mode),
            )
            .await;
        }

        // Wait for collector to finish
        let _ = collector_handle.await;

        // Save both user message and full AI response to conversation history
        let mut h = history.write().await;
        h.push(ChatMessage {
            role: "user".to_string(),
            content: request.message,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            image_base64: None,
        });
    });

    // ── Collect full response in background for history saving + Ring 6 Perspectives ──
    let history_for_save = state.conversation_history.clone();
    let vaam_bridge_for_output = state.vaam_bridge.clone();
    let perspective_game_state = state.game_state.clone();
    let perspective_character = state.character_sheet.clone();
    let perspective_router = state.inference_router.clone();
    let perspective_book_updates = state.book_updates.clone();
    tokio::spawn(async move {
        if let Some(full_response) = response_rx.recv().await {
            // VAAM Bridge: process AI output
            let _ = vaam_bridge_for_output
                .process_ai_output(&full_response)
                .await;

            // ═══════════════════════════════════════════════
            // RING 6: Perspective Engine — evaluate Pete's response
            // ═══════════════════════════════════════════════
            // Fire perspective lenses in parallel after Pete responds.
            // Results are emitted as a "perspectives" event on the book SSE channel.
            {
                let (phase_label, blooms_level, experience, audience) = {
                    let game = perspective_game_state.read().await;
                    let phase = game.quest.current_phase;
                    let blooms = match phase {
                        trinity_quest::hero::Phase::Analysis => "Remember/Understand",
                        trinity_quest::hero::Phase::Design => "Understand/Apply",
                        trinity_quest::hero::Phase::Development => "Apply/Create",
                        trinity_quest::hero::Phase::Implementation => "Apply",
                        trinity_quest::hero::Phase::Evaluation => "Analyze/Evaluate",
                        trinity_quest::hero::Phase::Contrast => "Analyze",
                        trinity_quest::hero::Phase::Repetition => "Evaluate",
                        trinity_quest::hero::Phase::Alignment => "Evaluate",
                        trinity_quest::hero::Phase::Proximity => "Create",
                        trinity_quest::hero::Phase::Envision => "Evaluate",
                        trinity_quest::hero::Phase::Yoke => "Create",
                        trinity_quest::hero::Phase::Evolve => "Create",
                    };
                    let sheet = perspective_character.read().await;
                    (
                        phase.label().to_string(),
                        blooms.to_string(),
                        sheet.experience.clone(),
                        sheet.audience.clone(),
                    )
                };

                let msg_type = perspective::classify_message(&full_response, false);
                let lenses = perspective::select_lenses(
                    &phase_label,
                    &blooms_level,
                    &msg_type,
                    experience.as_deref(),
                    audience.as_deref(),
                );

                if !lenses.is_empty() {
                    let llm_url = perspective_router.read().await.active_url().to_string();
                    let perspective_set =
                        perspective::evaluate(&llm_url, &full_response, &lenses).await;
                    if !perspective_set.perspectives.is_empty() {
                        if let Ok(json) = serde_json::to_string(&perspective_set) {
                            let _ = perspective_book_updates.send(format!("perspective:{}", json));
                            tracing::info!(
                                "[Ring 6] {} perspectives emitted in {}ms",
                                perspective_set.perspectives.len(),
                                perspective_set.total_latency_ms
                            );
                        }
                    }
                }
            }

            // Save assistant response to history
            let mut h = history_for_save.write().await;
            h.push(ChatMessage {
                role: "assistant".to_string(),
                content: full_response,
                timestamp: Some(chrono::Utc::now().to_rfc3339()),
                image_base64: None,
            });
        }
    });

    // Convert channel to SSE stream
    let stream = async_stream::stream! {
        while let Some(token) = rx.recv().await {
            yield Ok(sse::Event::default().data(token));
        }
        yield Ok(sse::Event::default().data("[DONE]"));
    };

    Sse::new(stream)
}

async fn status(State(state): State<AppState>) -> Json<SystemStatus> {
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
    })
}

async fn list_models() -> Json<serde_json::Value> {
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
async fn active_model(State(state): State<AppState>) -> Json<serde_json::Value> {
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

/// Switch the active inference backend at runtime
#[derive(Debug, Deserialize)]
struct SwitchModelRequest {
    /// Backend name (e.g. "llama-server", "ollama") OR URL
    #[serde(default)]
    url: String,
    #[serde(default)]
    backend: Option<String>,
}

async fn switch_model(
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
async fn inference_status(State(state): State<AppState>) -> Json<inference_router::RouterStatus> {
    let router = state.inference_router.read().await;
    Json(router.status())
}

/// POST /api/inference/switch — switch active backend by name
#[derive(Debug, Deserialize)]
struct InferenceSwitchRequest {
    backend: String,
}

async fn inference_switch(
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
async fn inference_refresh(State(state): State<AppState>) -> Json<inference_router::RouterStatus> {
    let mut router = state.inference_router.write().await;
    router.auto_detect().await;
    Json(router.status())
}

// ============================================================================
// APP MODE API — Phase 5A: Iron Road / Express / Yardmaster
// ============================================================================

/// GET /api/mode — returns current operating mode
async fn get_app_mode(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mode = state.app_mode.read().await;
    Json(serde_json::json!({
        "mode": *mode,
        "description": match *mode {
            AppMode::IronRoad => "Full LitRPG gamification — the Iron Road",
            AppMode::Express => "Guided wizard — skip the game, build and export",
            AppMode::Yardmaster => "IDE/Agent mode — full developer tools",
        }
    }))
}

/// POST /api/mode — switch operating mode
async fn set_app_mode(
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
        state.app_mode.read().await,
        new_mode
    );
    *state.app_mode.write().await = new_mode.clone();

    Ok(Json(serde_json::json!({
        "mode": new_mode,
        "message": format!("Switched to {} mode", new_mode),
    })))
}

/// Ingest a document for RAG
#[derive(Debug, Deserialize)]
pub struct IngestRequest {
    pub title: String,
    pub content: String,
    #[serde(default = "default_category")]
    pub category: String,
}

fn default_category() -> String {
    "general".to_string()
}

async fn ingest_document(
    State(state): State<AppState>,
    Json(request): Json<IngestRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let chunks = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        rag::ingest_document(
            &state.db_pool,
            &request.title,
            &request.content,
            &request.category,
        ),
    )
    .await
    .map_err(|_| {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Ingestion timed out after 30s".to_string(),
        )
    })?
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ingestion failed: {}", e),
        )
    })?;

    Ok(Json(serde_json::json!({
        "status": "ingested",
        "title": request.title,
        "chunks_created": chunks,
    })))
}

/// MCP Proxy endpoint - forwards requests to trinity-mcp-server
/// This allows the web UI to call MCP tools without direct MCP server connection
#[derive(Debug, Deserialize)]
pub struct McpRequest {
    pub method: String,
    pub params: serde_json::Value,
}

async fn mcp_proxy(
    State(_state): State<AppState>,
    Json(request): Json<McpRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // For now, we implement quest tools directly here
    // In production, this would forward to a running trinity-mcp-server process

    match request.method.as_str() {
        "tools/call" => {
            let tool_name = request.params["name"]
                .as_str()
                .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing tool name".to_string()))?;

            match tool_name {
                "quest_start" | "quest_context" | "quest_verify" => {
                    // These tools need MCP server - return helpful error
                    Ok(Json(serde_json::json!({
                        "error": "MCP server not connected. Start trinity-mcp-server for full quest context support.",
                        "hint": "For now, use the sidecar quest execution on :8090"
                    })))
                }
                _ => Ok(Json(serde_json::json!({
                    "error": format!("Unknown MCP tool: {}", tool_name)
                }))),
            }
        }
        "tools/list" => Ok(Json(serde_json::json!({
            "tools": [
                {"name": "quest_start", "description": "Load quest context files into MCP"},
                {"name": "quest_context", "description": "Search MCP for relevant context"},
                {"name": "quest_verify", "description": "Get quest verify commands"}
            ]
        }))),
        _ => Ok(Json(serde_json::json!({
            "error": format!("Unknown MCP method: {}", request.method)
        }))),
    }
}

// ═══════════════════════════════════════════════════════════════════
// IRON ROAD BOOK API - NPU Great Recycler Integration
// ═══════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════
// ADDIECRAPEYE Orchestration — Routes user actions through the
// Conductor's 12-phase state machine
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct OrchestrateRequest {
    pub quest_id: String,
    pub message: String,
    #[serde(default)]
    pub objectives: Vec<String>,
}

async fn orchestrate_quest(
    State(state): State<AppState>,
    Json(request): Json<OrchestrateRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conductor =
        conductor_leader::ConductorLeader::new(conductor_leader::ConductorConfig::default());

    // Get current phase from game state instead of conductor's internal state
    let game = state.game_state.read().await;
    let current_phase_name = game.quest.current_phase.label();

    // Map quest Phase to AddiecrapeyePhase
    let phase = match current_phase_name {
        "Analysis" => conductor_leader::AddiecrapeyePhase::Analysis,
        "Design" => conductor_leader::AddiecrapeyePhase::Design,
        "Development" => conductor_leader::AddiecrapeyePhase::Development,
        "Implementation" => conductor_leader::AddiecrapeyePhase::Implementation,
        "Evaluation" => conductor_leader::AddiecrapeyePhase::Evaluation,
        "Contrast" => conductor_leader::AddiecrapeyePhase::Contrast,
        "Repetition" => conductor_leader::AddiecrapeyePhase::Repetition,
        "Alignment" => conductor_leader::AddiecrapeyePhase::Alignment,
        "Proximity" => conductor_leader::AddiecrapeyePhase::Proximity,
        "Envision" => conductor_leader::AddiecrapeyePhase::Envision,
        "Yoke" => conductor_leader::AddiecrapeyePhase::Yoke,
        "Evolve" => conductor_leader::AddiecrapeyePhase::Evolve,
        _ => conductor_leader::AddiecrapeyePhase::Analysis,
    };

    // Inject VAAM and Bestiary context into player_context for the Conductor
    let vaam_context = state.vaam_bridge.prompt_context().await;
    let bestiary_summary = state.bestiary.read().await.summary();
    let sheet = state.character_sheet.read().await;
    let player_context = serde_json::json!({
        "message": request.message,
        "subject": game.quest.subject,
        "grade": format!("{:?}", sheet.genre), // Using genre as a proxy for style/level if needed, or mapping to grade
        "vaam_context": vaam_context,
        "bestiary": bestiary_summary,
        "character": {
            "alias": sheet.alias,
            "class": format!("{:?}", sheet.user_class),
            "level": sheet.resonance_level,
        }
    });
    // Extract intent and VAAM summaries before dropping the lock
    let intent_ctx = sheet.intent_summary();
    let vaam_ctx = sheet.vaam_profile.prompt_summary();
    drop(sheet);

    let orch_request = conductor_leader::OrchestrationRequest {
        quest_id: request.quest_id.clone(),
        current_phase: phase,
        player_context,
        objectives: request.objectives,
        available_party: vec!["pete".into(), "yardman".into(), "aesthetics".into()],
        intent_context: intent_ctx,
        vaam_context: vaam_ctx,
        pearl_context: game
            .quest
            .pearl
            .as_ref()
            .map(|p| p.prompt_summary())
            .unwrap_or_default(),
    };

    match conductor.orchestrate(orch_request).await {
        Ok(response) => Ok(Json(serde_json::json!({
            "status": "ok",
            "phase": format!("{}", response.next_phase),
            "instructions": response.player_instructions,
            "party_assignments": response.party_assignments,
            "xp_awarded": response.xp_awarded,
            "generated_content": response.generated_content,
        }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Orchestration failed: {}", e),
        )),
    }
}

/// Request to compile the Game Design Document
#[derive(Debug, Deserialize)]
pub struct CompileGddRequest {
    /// Phase notes collected from the Iron Road chat (one entry per ADDIECRAPEYE phase)
    #[serde(default)]
    pub phase_notes: Vec<String>,
}

/// Compile a Game Design Document from the full Iron Road session
/// WHY: This is the tangible deliverable — the teacher walks away with a structured GDD
/// HOW: Collects quest state, character sheet, bestiary, and chat notes into a single JSON document
async fn compile_game_design_document(
    State(state): State<AppState>,
    Json(request): Json<CompileGddRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let game = state.game_state.read().await;
    let sheet = state.character_sheet.read().await;
    let bestiary = state.bestiary.read().await;

    let phases = [
        "Analysis",
        "Design",
        "Development",
        "Implementation",
        "Evaluation",
        "Contrast",
        "Repetition",
        "Alignment",
        "Proximity",
        "Envision",
        "Yoke",
        "Evolve",
    ];

    // Build phase_notes map
    let phase_notes: Vec<serde_json::Value> = phases
        .iter()
        .enumerate()
        .map(|(i, name)| {
            serde_json::json!({
                "phase": name,
                "notes": request.phase_notes.get(i).cloned().unwrap_or_default(),
            })
        })
        .collect();

    // Build vocabulary list from bestiary
    let vocabulary: Vec<serde_json::Value> = bestiary
        .creeps
        .iter()
        .map(|c| {
            serde_json::json!({
                "term": c.word,
                "element": format!("{}", c.element),
                "mastery": format!("{:?}", c.state),
                "encounter_count": c.taming.encounter_count,
                "taming_score": c.taming.taming_score(),
            })
        })
        .collect();

    let gdd = serde_json::json!({
        "title": game.quest.game_title,
        "subject": game.quest.subject,
        "author": sheet.alias,
        "author_class": format!("{:?}", sheet.user_class),
        "resonance_level": sheet.resonance_level,
        "compiled_at": chrono::Utc::now().to_rfc3339(),
        "quest_progress": {
            "chapter": game.quest.hero_stage.chapter(),
            "chapter_title": game.quest.hero_stage.title(),
            "phases_completed": game.quest.completed_phases.iter().map(|p| p.label()).collect::<Vec<_>>(),
            "total_xp": game.stats.total_xp,
            "resonance": game.stats.resonance,
        },
        "addiecrapeye_phases": phase_notes,
        "vocabulary_bestiary": {
            "total_terms": bestiary.creeps.len(),
            "tamed": bestiary.creeps_tamed,
            "terms": vocabulary,
        },
        "learning_objectives": game.quest.phase_objectives.iter().map(|o| {
            serde_json::json!({
                "id": o.id,
                "description": o.description,
                "completed": o.completed,
            })
        }).collect::<Vec<_>>(),
        "inventory": game.inventory,
        "party_members": game.party.iter().map(|m| {
            serde_json::json!({
                "name": m.name,
                "role": m.role,
                "active": m.active,
            })
        }).collect::<Vec<_>>(),
    });

    // Save to workspace
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/joshua".to_string());
    let gdd_dir = std::path::PathBuf::from(&home)
        .join(".local/share/trinity/workspace/games")
        .join(&game.quest.quest_id);
    let _ = std::fs::create_dir_all(&gdd_dir);

    let gdd_path = gdd_dir.join("game_design_document.json");
    let gdd_json = serde_json::to_string_pretty(&gdd).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to serialize GDD: {}", e),
        )
    })?;
    std::fs::write(&gdd_path, &gdd_json).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to write GDD: {}", e),
        )
    })?;

    info!(
        "📄 Game Design Document compiled and saved to {:?}",
        gdd_path
    );

    // Persist project + GDD to PostgreSQL for cross-session survival
    let project_id = game.quest.quest_id.clone();
    let session_id = state.session_id.as_ref().clone();
    let project_name = game.quest.game_title.clone();
    let workspace_path = gdd_dir.to_string_lossy().to_string();
    drop(game);
    drop(sheet);
    drop(bestiary); // release locks before async DB calls

    if let Err(e) = persistence::create_project(
        &state.db_pool,
        &project_id,
        &session_id,
        &project_name,
        Some(&workspace_path),
    )
    .await
    {
        tracing::warn!("[GDD] Failed to create project record: {}", e);
    }
    if let Err(e) = persistence::save_project_gdd(&state.db_pool, &project_id, &gdd).await {
        tracing::warn!("[GDD] Failed to persist GDD to database: {}", e);
    }

    Ok(Json(serde_json::json!({
        "status": "ok",
        "message": "Game Design Document compiled successfully",
        "path": gdd_path.to_string_lossy(),
        "gdd": gdd,
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════
// EYE CONTAINER — Compile + Preview + Export
// ═══════════════════════════════════════════════════════════════════════════════

/// Compile an EYE container from the current quest state.
/// Returns the container as JSON — useful for preview before export.
async fn eye_compile(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let game = state.game_state.read().await;
    let container = eye_container::compile_container(&game);
    let json = serde_json::to_value(&container).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("EYE compile failed: {}", e),
        )
    })?;
    info!(
        "👁️ EYE container compiled: {} objectives, {} assets",
        container.objectives.len(),
        container.assets.len()
    );
    Ok(Json(serde_json::json!({
        "status": "ok",
        "container": json,
    })))
}

/// Preview the compiled EYE container as JSON.
async fn eye_preview(State(state): State<AppState>) -> Json<serde_json::Value> {
    let game = state.game_state.read().await;
    let container = eye_container::compile_container(&game);
    Json(serde_json::to_value(&container).unwrap_or_default())
}

/// Export the EYE container as a downloadable HTML5 file.
/// Query params: ?format=html5_quiz | html5_adventure | raw_json
async fn eye_export(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<axum::response::Response, (StatusCode, String)> {
    let game = state.game_state.read().await;
    let container = eye_container::compile_container(&game);

    let format = params
        .get("format")
        .and_then(|f| {
            serde_json::from_value::<eye_container::ExportFormat>(serde_json::Value::String(
                f.clone(),
            ))
            .ok()
        })
        .unwrap_or_default();

    let (filename, bytes, content_type) = export::export(&container, &format);

    info!(
        "📦 EYE export: {} ({} bytes, format: {:?})",
        filename,
        bytes.len(),
        format
    );

    let response = axum::response::Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", content_type)
        .header(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(axum::body::Body::from(bytes))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Response build failed: {}", e),
            )
        })?;

    Ok(response)
}

/// Get Creep Bestiary — the player's vocabulary creature collection
/// WHY: UI needs to display Wild/Tamed/Evolved Creeps with stats
async fn get_bestiary(State(state): State<AppState>) -> Json<serde_json::Value> {
    let bestiary = state.bestiary.read().await;
    let creeps: Vec<serde_json::Value> = bestiary
        .creeps
        .iter()
        .map(|c| {
            serde_json::json!({
                "word": c.word,
                "element": format!("{}", c.element),
                "role": format!("{}", c.role),
                "state": format!("{:?}", c.state),
                "stats": {
                    "logos": c.stats.logos,
                    "pathos": c.stats.pathos,
                    "ethos": c.stats.ethos,
                    "speed": c.stats.speed,
                },
                "power": c.power(),
                "encounter_count": c.taming.encounter_count,
                "taming_score": c.taming.taming_score(),
                "context_points": c.context_points,
            })
        })
        .collect();

    Json(serde_json::json!({
        "status": "ok",
        "summary": bestiary.summary(),
        "total": bestiary.creeps.len(),
        "tamed": bestiary.creeps_tamed,
        "wild": bestiary.wild_creeps().len(),
        "usable": bestiary.usable_creeps().len(),
        "words_scanned": bestiary.words_scanned,
        "slots_filled": bestiary.slots_filled,
        "battles_won": bestiary.battles_won,
        "creeps": creeps,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Intent Engineering API — Grounding, Posture, Scope Decisions
// ═══════════════════════════════════════════════════════════════════════════════

/// POST /api/ground — Complete the I AM grounding ritual
/// WHY: Before any quest interaction, the user grounds themselves
/// HOW: Marks grounding_complete = true in CharacterSheet
async fn ground_session(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut sheet = state.character_sheet.write().await;
    sheet.ground();
    let _ = character_sheet::save_character_sheet(&sheet);

    Ok(Json(serde_json::json!({
        "status": "grounded",
        "message": "I Am Here. I Am Enough. I Choose.",
        "grounding_complete": true,
        "intent_summary": sheet.intent_summary(),
    })))
}

/// POST /api/intent — Set session intent posture and purpose
/// WHY: The user declares HOW they want to engage (Mastery or Efficiency)
///      and WHAT their session purpose is
#[derive(Debug, Deserialize)]
struct IntentRequest {
    /// "mastery" or "efficiency"
    pub posture: String,
    /// One-sentence session purpose
    pub purpose: String,
}

async fn set_session_intent(
    State(state): State<AppState>,
    Json(request): Json<IntentRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    use trinity_protocol::character_sheet::IntentPosture;

    let posture = match request.posture.to_lowercase().as_str() {
        "mastery" => IntentPosture::Mastery,
        "efficiency" => IntentPosture::Efficiency,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                "posture must be 'mastery' or 'efficiency'".into(),
            ))
        }
    };

    let mut sheet = state.character_sheet.write().await;
    sheet.set_intent(&request.purpose, posture);
    let _ = character_sheet::save_character_sheet(&sheet);

    Ok(Json(serde_json::json!({
        "status": "intent_set",
        "posture": posture.display_name(),
        "purpose": request.purpose,
        "coal_multiplier": posture.coal_multiplier(),
        "xp_multiplier": posture.xp_multiplier(),
        "intent_summary": sheet.intent_summary(),
    })))
}

/// POST /api/bestiary/tame — User makes a Scope Hope / Scope Nope decision
/// WHY: When a Creep becomes tameable, the user must consciously choose
///      whether to adopt it into their vocabulary
#[derive(Debug, Deserialize)]
struct ScopeDecisionRequest {
    /// The word to decide about
    pub word: String,
    /// "hope" (tame it) or "nope" (leave it wild)
    pub decision: String,
}

async fn scope_creep_decision(
    State(state): State<AppState>,
    Json(request): Json<ScopeDecisionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut bestiary = state.bestiary.write().await;

    match request.decision.to_lowercase().as_str() {
        "hope" => {
            if bestiary.scope_hope_creep(&request.word) {
                if let Err(e) = character_sheet::save_bestiary(&bestiary) {
                    tracing::warn!("Failed to save bestiary: {}", e);
                }
                let creep = bestiary.get_creep_mut(&request.word);
                Ok(Json(serde_json::json!({
                    "status": "tamed",
                    "word": request.word,
                    "message": format!("🎉 Scope Hope! '{}' has been tamed and joins your vocabulary.", request.word),
                    "card": creep.map(|c| c.card()),
                })))
            } else {
                Err((
                    StatusCode::BAD_REQUEST,
                    format!(
                        "'{}' is not tameable yet — needs more multi-dimensional progress",
                        request.word
                    ),
                ))
            }
        }
        "nope" => {
            bestiary.scope_nope_creep(&request.word);
            if let Err(e) = character_sheet::save_bestiary(&bestiary) {
                tracing::warn!("Failed to save bestiary: {}", e);
            }
            Ok(Json(serde_json::json!({
                "status": "wild",
                "word": request.word,
                "message": format!("'{}' stays wild. It may return when the time is right.", request.word),
            })))
        }
        _ => Err((
            StatusCode::BAD_REQUEST,
            "decision must be 'hope' or 'nope'".into(),
        )),
    }
}

/// Get current Iron Road book state
/// WHY: REST endpoint for clients to fetch the full book (the Logos layer)
/// Generate narrative prose from the Great Recycler
/// GET /api/narrative/generate — creates LitRPG prose for the current game state
async fn generate_narrative_endpoint(State(state): State<AppState>) -> Json<serde_json::Value> {
    let gs = state.game_state.read().await;
    let sheet = state.character_sheet.read().await;

    let context = narrative::NarrativeContext {
        genre: sheet.genre,
        hero_stage: gs.quest.hero_stage,
        phase: gs.quest.current_phase,
        last_action: "continued the journey".to_string(),
        coal: gs.stats.coal_reserves,
        steam: gs.quest.steam_generated,
        xp: gs.stats.total_xp,
        alias: sheet.alias.clone(),
    };

    let llm_url = state.inference_router.read().await.active_url().to_string();

    match narrative::generate_narrative(&llm_url, &context).await {
        Some(prose) => {
            let entry = narrative::create_entry(prose.clone(), &context);
            let success_prose = narrative::generate_success_narrative(
                &context,
                context.coal.min(5.0),
                context.steam,
                context.xp,
            );
            Json(serde_json::json!({
                "success": true,
                "narrative": prose,
                "success_prose": success_prose,
                "entry_id": entry.id,
                "station": narrative::station_description(context.hero_stage),
            }))
        }
        None => Json(serde_json::json!({
            "success": false,
            "narrative": narrative::generate_failure_narrative(&context, "Narrative generation unavailable"),
            "station": narrative::station_description(context.hero_stage),
        })),
    }
}

async fn get_book(State(state): State<AppState>) -> Json<serde_json::Value> {
    let book = state.book.read().await;
    let chapters: Vec<serde_json::Value> = book
        .all_chapters()
        .iter()
        .map(|ch| {
            serde_json::json!({
                "id": ch.id,
                "title": ch.title,
                "quest_id": ch.quest_id,
                "phase": ch.phase,
                "resonance_level": ch.resonance_level,
                "timestamp": ch.timestamp.to_rfc3339(),
            })
        })
        .collect();

    let latest_title = book
        .latest_chapter()
        .map(|c| c.title.as_str())
        .unwrap_or("No chapters yet");

    Json(serde_json::json!({
        "status": "ok",
        "book": {
            "chapter_count": book.chapter_count(),
            "latest_chapter": latest_title,
            "chapters": chapters,
        }
    }))
}

/// SSE stream for real-time Iron Road book updates
/// WHY: Clients subscribe to receive updates as they happen from NPU
async fn book_stream(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<sse::Event, anyhow::Error>>> {
    use async_stream::stream;

    // Subscribe to book updates
    let mut receiver = state.book_updates.subscribe();

    info!("SSE client connected to book stream");

    let stream = stream! {
        loop {
            match receiver.recv().await {
                Ok(entry) => {
                    // Ring 6: Route perspective events to their own SSE event type
                    if let Some(perspective_json) = entry.strip_prefix("perspective:") {
                        yield Ok(sse::Event::default()
                            .event("perspective")
                            .data(perspective_json));
                    } else {
                        // Standard book update
                        let json = serde_json::to_string(&entry)?;
                        yield Ok(sse::Event::default()
                            .event("update")
                            .data(json));
                    }
                }
                Err(broadcast::error::RecvError::Closed) => {
                    // Channel closed, end stream
                    break;
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    // Client fell behind, warn but continue
                    warn!("SSE client lagged {} messages", n);
                    yield Ok(sse::Event::default()
                        .event("warning")
                        .data(format!("Lagged {} messages", n)));
                }
            }
        }
    };

    Sse::new(stream)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Persistence API — sessions, projects, DAYDREAM
// ═══════════════════════════════════════════════════════════════════════════════

/// List conversation sessions
async fn list_sessions(
    State(state): State<AppState>,
) -> Result<Json<Vec<persistence::SessionSummary>>, (StatusCode, String)> {
    persistence::list_sessions(&state.db_pool, 50)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Get conversation history for a session
async fn get_session_history(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<ChatMessage>>, (StatusCode, String)> {
    let session_id = params
        .get("session_id")
        .map(|s| s.as_str())
        .unwrap_or(state.session_id.as_str());
    let limit = params
        .get("limit")
        .and_then(|l| l.parse::<i64>().ok())
        .unwrap_or(100);

    persistence::load_session_history(&state.db_pool, session_id, limit)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// List game projects  
async fn list_projects(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<persistence::ProjectSummary>>, (StatusCode, String)> {
    let status_filter = params.get("status").map(|s| s.as_str());
    persistence::list_projects(&state.db_pool, status_filter, 50)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Archive a project to DAYDREAM
#[derive(Debug, Deserialize)]
struct ArchiveRequest {
    project_id: String,
    reason: String,
}

async fn archive_project(
    State(state): State<AppState>,
    Json(request): Json<ArchiveRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    persistence::archive_project(&state.db_pool, &request.project_id, &request.reason)
        .await
        .map(|_| Json(serde_json::json!({"status": "archived", "project_id": request.project_id})))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Restore a project from DAYDREAM archive
#[derive(Debug, Deserialize)]
struct RestoreRequest {
    project_id: String,
}

async fn restore_project_endpoint(
    State(state): State<AppState>,
    Json(request): Json<RestoreRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    persistence::restore_project(&state.db_pool, &request.project_id)
        .await
        .map(|_| Json(serde_json::json!({"status": "restored", "project_id": request.project_id})))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// ═══════════════════════════════════════════════════════════════════════════════
// RAG API — stats, search
// ═══════════════════════════════════════════════════════════════════════════════

/// Get RAG knowledge base statistics
async fn rag_stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    rag::rag_stats(&state.db_pool)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Search RAG knowledge base
#[derive(Debug, Deserialize)]
struct RagSearchRequest {
    query: String,
}

async fn rag_search(
    State(state): State<AppState>,
    Json(request): Json<RagSearchRequest>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    rag::search_documents(&state.db_pool, &request.query)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
// ═══════════════════════════════════════════════════════════════════════════════
// Quality Scorecard API — pedagogical document evaluation
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
struct ScoreRequest {
    /// Document title to look up in RAG, OR inline text to score directly
    #[serde(default)]
    document_id: String,
    /// Inline text content (if not looking up by document_id)
    #[serde(default)]
    text: String,
}

async fn score_document_endpoint(
    State(state): State<AppState>,
    Json(request): Json<ScoreRequest>,
) -> Result<Json<quality_scorecard::QualityScorecard>, (StatusCode, String)> {
    let text = if !request.text.is_empty() {
        request.text.clone()
    } else if !request.document_id.is_empty() {
        // Look up the document in RAG
        rag::search_documents(&state.db_pool, &request.document_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .join("\n\n")
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Provide either 'document_id' or 'text'".to_string(),
        ));
    };

    if text.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            "No document content found".to_string(),
        ));
    }

    let doc_id = if !request.document_id.is_empty() {
        request.document_id.clone()
    } else {
        "inline".to_string()
    };

    let scorecard = quality_scorecard::score_document(&text, &doc_id);
    Ok(Json(scorecard))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Journal API — chapter milestones, weekly reflections, portfolio export
// ═══════════════════════════════════════════════════════════════════════════════

/// List all journal entries (newest first)
async fn journal_list() -> Json<Vec<journal::JournalEntry>> {
    Json(journal::load_entries())
}

#[derive(Debug, Deserialize)]
struct JournalCreateRequest {
    /// "phase_complete", "chapter_complete", "weekly_reflection", "checkpoint", "demo_bookmark"
    #[serde(default = "default_journal_type")]
    entry_type: String,
    /// Optional user-written reflection
    #[serde(default)]
    reflection: Option<String>,
    /// Optional tags
    #[serde(default)]
    tags: Vec<String>,
}

fn default_journal_type() -> String {
    "checkpoint".to_string()
}

/// Create a new journal entry from current game state
async fn journal_create(
    State(state): State<AppState>,
    Json(request): Json<JournalCreateRequest>,
) -> Result<Json<journal::JournalEntry>, (StatusCode, String)> {
    let game = state.game_state.read().await;
    let sheet = state.character_sheet.read().await;

    let quest_snapshot = journal::QuestSnapshot {
        subject: game.quest.subject.clone(),
        phase: game.quest.current_phase.label().to_string(),
        phase_index: game.quest.current_phase.phase_index(),
        chapter: game.quest.hero_stage.chapter(),
        chapter_title: game.quest.hero_stage.title().to_string(),
        completed_phases: game
            .quest
            .completed_phases
            .iter()
            .map(|p| p.label().to_string())
            .collect(),
        objectives_total: game.quest.phase_objectives.len(),
        objectives_completed: game
            .quest
            .phase_objectives
            .iter()
            .filter(|o| o.completed)
            .count(),
        xp: game.quest.xp_earned,
        coal_remaining: 100.0 - game.quest.coal_used,
        steam: game.quest.steam_generated,
    };

    let char_snapshot = journal::CharacterSnapshot {
        resonance: sheet.resonance_level,
        skills: sheet
            .skills
            .iter()
            .map(|(k, v)| (format!("{:?}", k), *v))
            .collect(),
        experience: sheet.experience.clone(),
        audience: sheet.audience.clone(),
        vision: sheet.success_vision.clone(),
    };

    let entry_type = match request.entry_type.as_str() {
        "phase_complete" => journal::JournalEntryType::PhaseComplete,
        "chapter_complete" => journal::JournalEntryType::ChapterComplete,
        "weekly_reflection" => journal::JournalEntryType::WeeklyReflection,
        "demo_bookmark" => journal::JournalEntryType::DemoBookmark,
        _ => journal::JournalEntryType::ManualCheckpoint,
    };

    let entry = journal::create_entry(
        entry_type,
        request.reflection,
        quest_snapshot,
        char_snapshot,
        request.tags,
    );

    journal::save_entry(&entry).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // Fire SSE event so frontend knows
    let event = serde_json::json!({
        "type": "journal_created",
        "id": entry.id,
        "entry_type": entry.entry_type.label(),
        "summary": entry.summary,
    });
    let _ = state.book_updates.send(event.to_string());

    Ok(Json(entry))
}

/// Export a journal entry as standalone HTML
async fn journal_export(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Html<String>, StatusCode> {
    match journal::load_entry(&id) {
        Some(entry) => {
            let html = journal::export_html(&entry);
            Ok(axum::response::Html(html))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Delete a journal entry by ID
async fn journal_delete(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> StatusCode {
    if journal::delete_entry(&id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
// ═══════════════════════════════════════════════════════════════════════════════
// Ring 6: Perspective Feedback API
// ═══════════════════════════════════════════════════════════════════════════════

/// Accept thumbs up/down feedback on perspective evaluations
async fn perspective_feedback(Json(body): Json<serde_json::Value>) -> StatusCode {
    let lens_id = body
        .get("lens_id")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let direction = body
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("up");
    let phase = body
        .get("phase")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let feedback = perspective::PerspectiveFeedback {
        lens_id: lens_id.to_string(),
        direction: direction.to_string(),
        phase,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    perspective::save_feedback(&feedback);
    StatusCode::OK
}

// ═══════════════════════════════════════════════════════════════════════════════
// Auto-ingest — load Trinity docs into RAG on startup
// ═══════════════════════════════════════════════════════════════════════════════

/// Ingest Trinity bible and active docs into RAG on startup
async fn auto_ingest_docs(pool: &sqlx::PgPool) {
    let workspace = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    // Key documents to ingest
    let docs_to_ingest = [
        ("docs/bible/00-MASTER.md", "bible"),
        ("docs/bible/01-ARCHITECTURE.md", "bible"),
        ("docs/bible/05-CROW-CONTINUITY.md", "bible"),
        ("docs/active/VAAM-LitRPG-INTEGRATION.md", "mechanics"),
        ("docs/active/SESSION_GUIDE.md", "guide"),
        ("CONTEXT.md", "context"),
        ("TRINITY_TECHNICAL_BIBLE.md", "bible"),
    ];

    let mut ingested = 0;
    for (path, category) in &docs_to_ingest {
        let full_path = workspace.join(path);
        if let Ok(content) = std::fs::read_to_string(&full_path) {
            match rag::ingest_document(pool, path, &content, category).await {
                Ok(chunks) => {
                    info!("📚 Auto-ingested {}: {} chunks", path, chunks);
                    ingested += 1;
                }
                Err(e) => {
                    warn!("⚠️ Failed to ingest {}: {}", path, e);
                }
            }
        }
    }

    info!(
        "📚 Auto-ingest complete: {}/{} documents loaded into RAG",
        ingested,
        docs_to_ingest.len()
    );
}
