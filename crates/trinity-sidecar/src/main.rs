// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-sidecar-engineer
// ═══════════════════════════════════════════════════════════════════════════════
// 
// FILE:        main.rs
// PURPOSE:     Role-based AI sidecar — 6 party member roles in one binary
// 
// ARCHITECTURE:
//   • Single binary runs as ANY party member via --role flag
//   • Engineer: Dual-model (Opus 27B + REAP 25B) Sword & Shield
//   • Evaluator: Opus 65K ctx for QM rubrics and audits
//   • Artist: Opus 32K ctx for game design and wireframes
//   • Brakeman: REAP 16K ctx for QA and security
//   • Pete: Opus 16K ctx fallback for Socratic dialogue
//   • Ports: 8081 (primary), 8082 (secondary/Engineer), 8090 (API)
//   • ONLY ONE sidecar runs at a time (shared ports)
//
// DEPENDENCIES:
//   - tokio — Async runtime
//   - axum — HTTP API server (port 8090)
//   - clap — CLI argument parsing (--role)
//   - llama-cpp — GGUF model inference
//
// CHANGES:
//   2026-03-16  Cascade  Migrated to §17 comment standard
//
// ═══════════════════════════════════════════════════════════════════════════════

mod api;
mod comfyui;

mod cow_catcher;
mod llama;
mod prompts;
mod quest;
mod roles;
mod workflow;

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

use llama::{LlamaClient, LlamaConfig, LlamaProcess};
use workflow::{WorkflowEngine, WorkflowState};

const API_PORT: u16 = 8090;
const DEFAULT_NGL: i32 = 99; // GPU offload — free on unified memory (128GB LPDDR5X shared)

fn default_threads() -> u32 {
    let cpus = std::thread::available_parallelism()
        .map(|p| p.get() as u32)
        .unwrap_or(8);
    (cpus / 2).max(4)
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent() // crates/
        .and_then(|p| p.parent()) // trinity-genesis/
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Parse --role from command line args
fn parse_role() -> String {
    let args: Vec<String> = std::env::args().collect();
    for i in 0..args.len() {
        if args[i] == "--role" {
            if let Some(role) = args.get(i + 1) {
                return role.clone();
            }
        }
    }
    // Also check TRINITY_ROLE env var
    if let Ok(role) = std::env::var("TRINITY_ROLE") {
        return role;
    }
    "tempo".to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("trinity_sidecar_engineer=info")
        .init();

    let role_id = parse_role();
    let role = roles::get_role(&role_id).ok_or_else(|| {
        anyhow::anyhow!(
            "Unknown role: '{}'. Available: engineer, evaluator, artist, brakeman, pete",
            role_id
        )
    })?;

    info!("╔══════════════════════════════════════════════════════════════╗");
    info!(
        "║  TRINITY PARTY SIDECAR — {} {}",
        role.icon,
        format!("{:<44}║", role.name)
    );
    info!("║  {}", format!("{:<57}║", role.description));
    info!(
        "║  Skill: {}",
        format!(
            "{:<49}║",
            &role.party_skill[..role.party_skill.len().min(49)]
        )
    );
    info!("╚══════════════════════════════════════════════════════════════╝");

    let ws_root = workspace_root();
    let llama_bin = ws_root.join("llama.cpp/build/bin/llama-server");
    let quest_dir = ws_root.join("quests");
    let threads = default_threads();
    let model_dir = roles::model_dir();

    info!("Role: {} ({})", role.name, role.id);
    info!("Workspace: {}", ws_root.display());
    info!("Quest board: {}", quest_dir.display());
    info!("Threads per model: {}", threads);

    // Validate models exist
    roles::validate_models(&role).map_err(|e| anyhow::anyhow!(e))?;

    // ── Start primary model ─────────────────────────────────────────

    let primary_model_path = model_dir.join(role.primary.filename);
    info!(
        "Primary: {} ({:.1} GB) on port {}",
        role.primary.name,
        std::fs::metadata(&primary_model_path)
            .map(|m| m.len() as f64 / 1_073_741_824.0)
            .unwrap_or(0.0),
        role.primary.port
    );

    let mut primary_proc = LlamaProcess::start(
        LlamaConfig {
            name: role.primary.name.to_string(),
            model_path: primary_model_path,
            port: role.primary.port,
            context_size: role.primary.context_size,
            n_gpu_layers: DEFAULT_NGL,
            threads,
        },
        &llama_bin,
    )
    .await?;

    let primary_url = format!("http://127.0.0.1:{}", role.primary.port);
    let primary_client = LlamaClient::new(&primary_url);

    info!(
        "Waiting for {} to be ready (up to 180s)...",
        role.primary.name
    );
    if !primary_client.wait_ready(180).await {
        error!("{} failed to start", role.primary.name);
        primary_proc.stop().await.ok();
        anyhow::bail!("{} failed to start", role.primary.name);
    }
    info!(
        "{} is READY on port {}",
        role.primary.name, role.primary.port
    );

    // ── Start secondary model (if dual-model role) ──────────────────

    let mut secondary_proc: Option<LlamaProcess> = None;
    let secondary_url;

    if let Some(ref secondary) = role.secondary {
        let secondary_model_path = model_dir.join(secondary.filename);
        info!(
            "Secondary: {} ({:.1} GB) on port {}",
            secondary.name,
            std::fs::metadata(&secondary_model_path)
                .map(|m| m.len() as f64 / 1_073_741_824.0)
                .unwrap_or(0.0),
            secondary.port
        );

        let mut proc = LlamaProcess::start(
            LlamaConfig {
                name: secondary.name.to_string(),
                model_path: secondary_model_path,
                port: secondary.port,
                context_size: secondary.context_size,
                n_gpu_layers: DEFAULT_NGL,
                threads,
            },
            &llama_bin,
        )
        .await?;

        secondary_url = format!("http://127.0.0.1:{}", secondary.port);
        let secondary_client = LlamaClient::new(&secondary_url);

        info!("Waiting for {} to be ready (up to 180s)...", secondary.name);
        if !secondary_client.wait_ready(180).await {
            error!("{} failed to start", secondary.name);
            primary_proc.stop().await.ok();
            proc.stop().await.ok();
            anyhow::bail!("{} failed to start", secondary.name);
        }
        info!("{} is READY on port {}", secondary.name, secondary.port);

        secondary_proc = Some(proc);
    } else {
        // Single-model: both opus and reap point to the same model
        secondary_url = primary_url.clone();
    }

    info!("All models loaded and healthy!");

    // ── Initialize quest board ──────────────────────────────────────

    let quest_board = quest::QuestBoard::new(quest_dir.clone());
    quest_board.init().await?;

    let available = quest_board.list_available().await?;
    info!("Quest board: {} quests available", available.len());

    // ── Build workflow engine ───────────────────────────────────────

    let workflow_state = Arc::new(RwLock::new(WorkflowState {
        role_id: role.id.to_string(),
        role_name: role.name.to_string(),
        ..WorkflowState::default()
    }));
    {
        let mut ws = workflow_state.write().await;
        ws.opus_healthy = true;
        ws.reap_healthy = true;
        ws.status = workflow::EngineStatus::Idle;
    }

    let engine = Arc::new(WorkflowEngine::new(
        &primary_url,
        &secondary_url,
        quest_dir,
        ws_root,
        workflow_state.clone(),
        role.id.to_string(),
    ));

    // ── Start API server ────────────────────────────────────────────

    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);

    let api_state = api::ApiState {
        engine: engine.clone(),
        workflow_state: workflow_state.clone(),
        shutdown_tx: shutdown_tx.clone(),
    };

    let router = api::build_router(api_state);
    let api_addr = format!("127.0.0.1:{}", API_PORT);

    info!("{} Sidecar API listening on http://{}", role.name, api_addr);
    info!("");
    info!("  Party Skill: {}", role.party_skill);
    info!("  ADDIE Phases: {:?}", role.addie_phases);
    info!("  Quest Types: {:?}", role.quest_specialties);
    info!("");
    info!("  Endpoints:");
    info!("    GET  /status            — Sidecar health + quest progress");
    info!("    GET  /quests            — List all quests");
    info!("    POST /quest/execute     — Execute a quest (manual trigger)");
    info!("    POST /autonomous/start  — Start 24/7 work loop");
    info!("    POST /autonomous/stop   — Stop work loop");
    info!("    POST /think             — Direct prompt to primary model");
    info!("    POST /code              — Direct prompt to secondary model");
    info!("    POST /shutdown          — Graceful shutdown");
    info!("");
    info!("Ready for quests! Drop JSON files in quests/board/ or POST /autonomous/start");

    let listener = tokio::net::TcpListener::bind(&api_addr).await?;

    // ── Run until shutdown ──────────────────────────────────────────

    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;

    tokio::select! {
        result = axum::serve(listener, router) => {
            if let Err(e) = result {
                error!("API server error: {}", e);
            }
        }
        _ = sigint.recv() => {
            info!("Received SIGINT");
        }
        _ = sigterm.recv() => {
            info!("Received SIGTERM");
        }
        _ = shutdown_rx.recv() => {
            info!("Shutdown requested via API");
        }
    }

    // ── Cleanup ─────────────────────────────────────────────────────

    info!("Shutting down {} Sidecar...", role.name);
    engine.stop_autonomous().await;
    primary_proc.stop().await.ok();
    if let Some(mut proc) = secondary_proc {
        proc.stop().await.ok();
    }
    info!("{} Sidecar shut down cleanly.", role.name);

    Ok(())
}
