// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        jobs.rs
// PURPOSE:     Background Job Runner — fire-and-forget autonomous agent work
//
// ARCHITECTURE:
//   • Jobs are submitted via POST /api/jobs with a task description
//   • Each job spawns the standard agent loop (same as chat) with a channel
//   • A consumer task reads from the channel and writes to disk + job state
//   • Jobs survive browser tab closes — results saved to ~/Workflow/trinity-reports/
//   • Job status is queryable via GET /api/jobs and GET /api/jobs/:id
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::agent::AgentRequest;
use crate::AppState;

// ═══════════════════════════════════════════════════
// Job Data Structures
// ═══════════════════════════════════════════════════

/// A background job submitted for autonomous execution
#[derive(Debug, Clone, Serialize)]
pub struct BackgroundJob {
    pub id: String,
    pub message: String,
    pub mode: String,
    pub status: String, // "running" | "complete" | "failed"
    pub created_at: String,
    pub completed_at: Option<String>,
    pub turns_used: u32,
    pub tools_called: Vec<String>,
    pub output_path: Option<String>,
    pub log: Vec<String>,
    pub final_response: Option<String>,
}

/// Thread-safe job queue
pub type JobQueue = Arc<RwLock<Vec<BackgroundJob>>>;

/// Create a new empty job queue
pub fn new_job_queue() -> JobQueue {
    Arc::new(RwLock::new(Vec::new()))
}

// ═══════════════════════════════════════════════════
// Persistence
// ═══════════════════════════════════════════════════

/// Load all jobs from DB on server startup
pub async fn load_jobs(pool: &sqlx::SqlitePool) -> anyhow::Result<JobQueue> {
    let rows: Vec<(
        String,         // id
        String,         // message
        String,         // mode
        String,         // status
        i64,            // turns_used
        String,         // tools_called
        Option<String>, // output_path
        String,         // log
        Option<String>, // final_response
        String,         // created_at
        Option<String>, // completed_at
    )> = sqlx::query_as(
        r#"
        SELECT id, message, mode, status, turns_used, tools_called, output_path, log, final_response, created_at, completed_at
        FROM trinity_background_jobs ORDER BY created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut jobs = Vec::new();
    for r in rows {
        jobs.push(BackgroundJob {
            id: r.0,
            message: r.1,
            mode: r.2,
            // If the server crashed while a job was running, mark it as failed
            status: if r.3 == "running" { "failed (server restart)".to_string() } else { r.3 },
            turns_used: r.4 as u32,
            tools_called: serde_json::from_str(&r.5).unwrap_or_default(),
            output_path: r.6,
            log: serde_json::from_str(&r.7).unwrap_or_default(),
            final_response: r.8,
            created_at: r.9,
            completed_at: r.10,
        });
    }
    
    info!("📂 Loaded {} background jobs from database", jobs.len());
    Ok(Arc::new(RwLock::new(jobs)))
}

/// Save or update a job in the database
async fn save_job_to_db(pool: &sqlx::SqlitePool, job: &BackgroundJob) {
    let tools = serde_json::to_string(&job.tools_called).unwrap_or_else(|_| "[]".to_string());
    let log_str = serde_json::to_string(&job.log).unwrap_or_else(|_| "[]".to_string());
    
    let res = sqlx::query(
        r#"
        INSERT INTO trinity_background_jobs (
            id, message, mode, status, turns_used, tools_called, output_path, log, final_response, created_at, completed_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (id) DO UPDATE SET
            status = EXCLUDED.status,
            turns_used = EXCLUDED.turns_used,
            tools_called = EXCLUDED.tools_called,
            output_path = EXCLUDED.output_path,
            log = EXCLUDED.log,
            final_response = EXCLUDED.final_response,
            completed_at = EXCLUDED.completed_at
        "#,
    )
    .bind(&job.id)
    .bind(&job.message)
    .bind(&job.mode)
    .bind(&job.status)
    .bind(job.turns_used as i64)
    .bind(&tools)
    .bind(&job.output_path)
    .bind(&log_str)
    .bind(&job.final_response)
    .bind(&job.created_at)
    .bind(&job.completed_at)
    .execute(pool)
    .await;

    if let Err(e) = res {
        tracing::error!("Failed to save background job {} to DB: {}", job.id, e);
    }
}

// ═══════════════════════════════════════════════════
// API Request/Response Types
// ═══════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct SubmitJobRequest {
    pub message: String,
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default = "default_max_turns")]
    pub max_turns: u32,
}

fn default_mode() -> String {
    "dev".to_string()
}
fn default_max_turns() -> u32 {
    65
}

// ═══════════════════════════════════════════════════
// API Handlers
// ═══════════════════════════════════════════════════

/// POST /api/jobs — Submit a new background job
pub async fn submit_job(
    State(state): State<AppState>,
    Json(req): Json<SubmitJobRequest>,
) -> Json<serde_json::Value> {
    let job_id = format!("job-{}", chrono::Utc::now().timestamp_millis());
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let job = BackgroundJob {
        id: job_id.clone(),
        message: req.message.clone(),
        mode: req.mode.clone(),
        status: "running".to_string(),
        created_at: now,
        completed_at: None,
        turns_used: 0,
        tools_called: Vec::new(),
        output_path: None,
        log: vec![format!("Job submitted: {}", req.message)],
        final_response: None,
    };

    // Add to queue and save to DB
    state.job_queue.write().await.push(job.clone());
    save_job_to_db(&state.db_pool, &job).await;

    info!("🔧 Background job submitted: {} — \"{}\"", job_id, req.message);

    // Spawn the background agent
    let job_id_clone = job_id.clone();
    let state_clone = state.clone();
    let message = req.message.clone();
    let mode = req.mode.clone();
    let max_turns = req.max_turns;

    tokio::spawn(async move {
        run_background_job(state_clone, job_id_clone, message, mode, max_turns).await;
    });

    Json(serde_json::json!({
        "id": job_id,
        "status": "running",
        "message": "Job submitted. Check status at GET /api/jobs or GET /api/jobs/{id}"
    }))
}

/// GET /api/jobs — List all jobs
pub async fn list_jobs(State(state): State<AppState>) -> Json<serde_json::Value> {
    let jobs = state.job_queue.read().await;
    let summaries: Vec<serde_json::Value> = jobs
        .iter()
        .rev() // newest first
        .map(|j| {
            serde_json::json!({
                "id": j.id,
                "message": if j.message.len() > 80 { format!("{}...", &j.message[..80]) } else { j.message.clone() },
                "mode": j.mode,
                "status": j.status,
                "created_at": j.created_at,
                "completed_at": j.completed_at,
                "turns_used": j.turns_used,
                "tools_called": j.tools_called.len(),
                "output_path": j.output_path,
            })
        })
        .collect();

    Json(serde_json::json!({
        "jobs": summaries,
        "total": summaries.len(),
    }))
}

/// GET /api/jobs/:id — Get detailed job status
pub async fn job_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    let jobs = state.job_queue.read().await;
    if let Some(job) = jobs.iter().find(|j| j.id == id) {
        Json(serde_json::json!({
            "id": job.id,
            "message": job.message,
            "mode": job.mode,
            "status": job.status,
            "created_at": job.created_at,
            "completed_at": job.completed_at,
            "turns_used": job.turns_used,
            "tools_called": job.tools_called,
            "output_path": job.output_path,
            "log": job.log,
            "final_response": job.final_response,
        }))
    } else {
        Json(serde_json::json!({
            "error": format!("Job '{}' not found", id),
        }))
    }
}

/// DELETE /api/jobs/:id — Cancel a running job
pub async fn cancel_job(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    let mut jobs = state.job_queue.write().await;
    if let Some(job) = jobs.iter_mut().find(|j| j.id == id) {
        if job.status == "running" {
            job.status = "cancelled".to_string();
            job.completed_at = Some(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            job.log.push("🛑 Job cancelled by user.".to_string());
            save_job_to_db(&state.db_pool, job).await;
            info!("🛑 Background job {} cancelled", id);
            Json(serde_json::json!({
                "id": id,
                "status": "cancelled",
                "message": "Job cancelled. The agent loop will stop at the next tool boundary."
            }))
        } else {
            Json(serde_json::json!({
                "id": id,
                "status": job.status,
                "message": format!("Job is already '{}', cannot cancel.", job.status)
            }))
        }
    } else {
        Json(serde_json::json!({
            "error": format!("Job '{}' not found", id),
        }))
    }
}

// ═══════════════════════════════════════════════════
// Background Job Executor
// ═══════════════════════════════════════════════════

/// Run a background job by spawning the standard agent loop and consuming its output.
/// The agent loop is UNCHANGED — we just read from its channel and log to disk.
async fn run_background_job(
    state: AppState,
    job_id: String,
    message: String,
    mode: String,
    max_turns: u32,
) {
    info!("🏭 Background job {} starting: \"{}\"", job_id, message);

    // Create the same channel the SSE handler uses
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

    // Build the agent request (same struct the chat endpoint uses)
    let agent_request = AgentRequest {
        message: message.clone(),
        use_rag: true, // background jobs should use RAG context
        max_tokens: 32768,
        max_turns,
        sidecar_url: None,
        hardcore_mode: false,
        image_base64: None,
        history: Vec::new(),
        mode: mode.clone(),
    };

    // Spawn the agent loop (this is the same function the SSE endpoint calls internally)
    // We reuse the exact same code path — the agent doesn't know it's a background job.
    let llm_url = state.inference_router.read().await.active_url().to_string();
    let db_pool = state.db_pool.clone();
    let session_id = state.project.session_id.as_ref().clone();
    let game_state = state.project.game_state.clone();
    let character_sheet = state.player.character_sheet.clone();

    // Start the agent loop in its own task
    let agent_state = state.clone();
    tokio::spawn(async move {
        crate::agent::run_agent_loop(
            agent_state,
            tx,
            agent_request,
            llm_url,
            db_pool,
            session_id,
            game_state,
            character_sheet,
        )
        .await;
    });

    // === Consumer: read from the channel and build the job log ===
    let mut response_text = String::new();
    let mut turns: u32 = 0;
    let mut tools: Vec<String> = Vec::new();

    while let Some(msg) = rx.recv().await {
        // Check for cancellation
        {
            let jobs = state.job_queue.read().await;
            if let Some(job) = jobs.iter().find(|j| j.id == job_id) {
                if job.status == "cancelled" {
                    info!("🛑 Background job {} cancelled — stopping consumer", job_id);
                    break;
                }
            }
        }
        // Parse the SSE-formatted messages to extract useful info
        if msg.contains("\"status\":\"thinking\"") {
            if let Some(turn_str) = extract_json_field(&msg, "turn") {
                turns = turn_str.parse().unwrap_or(turns);
            }
            // Update job state
            update_job_turns(&state.job_queue, &state.db_pool, &job_id, turns).await;
        } else if msg.contains("\"status\":\"tool\"") {
            if let Some(tool_name) = extract_json_field(&msg, "tool") {
                tools.push(tool_name.clone());
                append_job_log(
                    &state.job_queue,
                    &state.db_pool,
                    &job_id,
                    format!("🔧 Tool: {}", tool_name),
                )
                .await;
            }
        } else if msg.starts_with("{\"content\":") || msg.starts_with("data: {\"content\":") {
            // Content token — accumulate the response
            let payload = msg.trim_start_matches("data: ");
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(payload) {
                if let Some(content) = json.get("content").and_then(|c| c.as_str()) {
                    response_text.push_str(content);
                }
            }
        } else if msg.contains("\"type\":\"tool_result\"") {
            // Tool result — log it
            let payload = msg.trim_start_matches("data: ");
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(payload) {
                if let Some(content) = json.get("content").and_then(|c| c.as_str()) {
                    let preview = if content.len() > 200 {
                        format!("{}...", &content[..200])
                    } else {
                        content.to_string()
                    };
                    append_job_log(
                        &state.job_queue,
                        &state.db_pool,
                        &job_id,
                        format!("📋 Result: {}", preview),
                    )
                    .await;
                }
            }
        } else if msg.contains("[DONE]") {
            break;
        }
    }

    // === Job Complete: Write report and update status ===
    let now = chrono::Local::now();
    let report_filename = format!(
        "{}_{}.md",
        now.format("%Y-%m-%d_%H%M"),
        job_id
    );
    let reports_dir =
        dirs::home_dir().unwrap_or_default().join("Workflow/trinity-reports");
    let _ = std::fs::create_dir_all(&reports_dir);
    let report_path = reports_dir.join(&report_filename);

    // Build the report
    let response_preview = if response_text.len() > 5000 {
        format!("{}...\n\n[Full response: {} chars]", &response_text[..5000], response_text.len())
    } else {
        response_text.clone()
    };

    let report = format!(
        "# Background Job Report — {}\n\n\
         **Job ID**: {}  \n\
         **Task**: {}  \n\
         **Mode**: {}  \n\
         **Status**: ✅ COMPLETE  \n\
         **Started**: {}  \n\
         **Completed**: {}  \n\
         **Turns Used**: {}  \n\
         **Tools Called**: {} ({})  \n\n\
         ---\n\n\
         ## Agent Response\n\n\
         {}\n\n\
         ---\n\n\
         *This report was generated automatically by Trinity Background Job Runner.*\n",
        now.format("%B %d, %Y %H:%M"),
        job_id,
        message,
        mode,
        // We'll read created_at from the job
        state.job_queue.read().await.iter().find(|j| j.id == job_id).map(|j| j.created_at.clone()).unwrap_or_default(),
        now.format("%Y-%m-%d %H:%M:%S"),
        turns,
        tools.len(),
        tools.join(", "),
        response_preview,
    );

    let _ = std::fs::write(&report_path, &report);
    info!(
        "📝 Background job {} complete — report: {}",
        job_id,
        report_path.display()
    );

    // Update job status to complete
    {
        let mut jobs = state.job_queue.write().await;
        if let Some(job) = jobs.iter_mut().find(|j| j.id == job_id) {
            job.status = "complete".to_string();
            job.completed_at = Some(now.format("%Y-%m-%d %H:%M:%S").to_string());
            job.turns_used = turns;
            job.tools_called = tools;
            job.output_path = Some(report_path.display().to_string());
            job.final_response = Some(if response_text.len() > 10000 {
                format!("{}...", &response_text[..10000])
            } else {
                response_text
            });
            job.log.push("✅ Job complete. Report written.".to_string());
            
            // Save final state to DB
            save_job_to_db(&state.db_pool, job).await;
        }
    }
}

// ═══════════════════════════════════════════════════
// Helper Functions
// ═══════════════════════════════════════════════════

/// Extract a JSON field value from an SSE message string
fn extract_json_field(msg: &str, field: &str) -> Option<String> {
    // Try to find and parse the JSON payload
    let payload = if msg.starts_with("event: ") {
        // Format: "event: type\ndata: {...}\n\n"
        msg.split("data: ").nth(1).unwrap_or(msg)
    } else {
        msg
    };

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(payload.trim()) {
        json.get(field).and_then(|v| match v {
            serde_json::Value::String(s) => Some(s.clone()),
            serde_json::Value::Number(n) => Some(n.to_string()),
            _ => Some(v.to_string()),
        })
    } else {
        None
    }
}

/// Update job turn count
async fn update_job_turns(queue: &JobQueue, pool: &sqlx::SqlitePool, job_id: &str, turns: u32) {
    let mut jobs = queue.write().await;
    if let Some(job) = jobs.iter_mut().find(|j| j.id == job_id) {
        job.turns_used = turns;
        save_job_to_db(pool, job).await;
    }
}

/// Append to job log
async fn append_job_log(queue: &JobQueue, pool: &sqlx::SqlitePool, job_id: &str, entry: String) {
    let mut jobs = queue.write().await;
    if let Some(job) = jobs.iter_mut().find(|j| j.id == job_id) {
        // Keep last 500 log entries
        if job.log.len() > 500 {
            job.log.drain(0..100);
        }
        job.log.push(entry);
        save_job_to_db(pool, job).await;
    }
}
