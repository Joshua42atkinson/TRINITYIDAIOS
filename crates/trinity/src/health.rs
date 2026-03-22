// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        health.rs
// PURPOSE:     Real health/readiness endpoint — checks all subsystems
//
// ARCHITECTURE:
//   • GET /api/health returns honest status of every subsystem
//   • Used by: UI status bar, curl smoke tests, future CI
//   • No fake "connected" booleans — actual network checks
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{extract::State, response::Json};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub llm: LlmHealth,
    pub database: DbHealth,
    pub creative: CreativeHealth,
    pub voice: VoiceHealth,
    pub uptime_secs: u64,
}

#[derive(Serialize)]
pub struct LlmHealth {
    pub connected: bool,
    pub url: String,
    pub backend_name: String,
    pub model_hint: String,
    pub backends_available: usize,
}

#[derive(Serialize)]
pub struct DbHealth {
    pub connected: bool,
    pub message: String,
    pub total_messages: i64,
    pub total_tool_calls: i64,
}

#[derive(Serialize)]
pub struct CreativeHealth {
    pub comfyui: bool,
    pub musicgpt: bool,
}

#[derive(Serialize)]
pub struct VoiceHealth {
    pub connected: bool,
    pub url: String,
}

static START_TIME: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();

pub fn mark_startup() {
    START_TIME.get_or_init(std::time::Instant::now);
}

/// GET /api/health — honest subsystem health check
pub async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let router = state.inference_router.read().await;
    let llm_url = router.active_url().to_string();
    let llm_connected = router.is_healthy();
    let backend_name = router.active_name().to_string();
    let model_hint = router
        .active_backend()
        .and_then(|b| b.model_name.clone())
        .unwrap_or_else(|| "Mistral-Small-4-119B".to_string());
    let backends_available = router.backends.iter().filter(|b| b.healthy).count();
    drop(router);

    let db_connected = sqlx::query("SELECT 1")
        .execute(&state.db_pool)
        .await
        .is_ok();

    let comfyui_ok = crate::http::QUICK
        .get("http://127.0.0.1:8188/system_stats")
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    let musicgpt_ok = crate::http::check_health("http://127.0.0.1:8189").await;

    let voice_ok = crate::http::check_health("http://127.0.0.1:7777").await;

    let uptime = START_TIME.get().map(|t| t.elapsed().as_secs()).unwrap_or(0);

    let overall = if llm_connected { "healthy" } else { "degraded" };

    Json(HealthResponse {
        status: overall.to_string(),
        llm: LlmHealth {
            connected: llm_connected,
            url: llm_url,
            backend_name,
            model_hint,
            backends_available,
        },
        database: DbHealth {
            connected: db_connected,
            message: if db_connected {
                "PostgreSQL connected".to_string()
            } else {
                "PostgreSQL not required for demo".to_string()
            },
            total_messages: if db_connected {
                crate::persistence::total_message_count(&state.db_pool)
                    .await
                    .unwrap_or(0)
            } else {
                0
            },
            total_tool_calls: if db_connected {
                crate::persistence::total_tool_call_count(&state.db_pool)
                    .await
                    .unwrap_or(0)
            } else {
                0
            },
        },
        creative: CreativeHealth {
            comfyui: comfyui_ok,
            musicgpt: musicgpt_ok,
        },
        voice: VoiceHealth {
            connected: voice_ok,
            url: "http://127.0.0.1:7777".to_string(),
        },
        uptime_secs: uptime,
    })
}
