// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        health.rs
// PURPOSE:     Real health/readiness endpoint — checks all subsystems
//
// 🪟 THE LIVING CODE TEXTBOOK (P-ART-Y Infrastructure):
// This file is the pulse monitor of the entire Trinity system. It is designed to 
// be read, modified, and authored by YOU. If you add a new sidecar or AI model, 
// you must add its health check here to ensure the Yardmaster can monitor it.
// ACTION: Add a new struct to `HealthResponse` to monitor your own custom models.
//
// 📖 THE HOOK BOOK CONNECTION:
// This file manages the system stability Hooks. Before Pete or the ART agent 
// execute complex workflows, they check these vital signs to ensure survival.
// For a full catalogue of system capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
//
// ARCHITECTURE:
//   • GET /api/health returns honest status of every subsystem
//   • Used by: UI status bar, curl smoke tests, future CI
//   • No fake "connected" booleans — actual network checks
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{extract::State, response::IntoResponse};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub llm: LlmHealth,
    pub database: DbHealth,
    pub creative: CreativeHealth,
    pub voice: VoiceHealth,
    pub cow_catcher: CowCatcherHealth,
    pub uptime_secs: u64,
}

#[derive(Serialize)]
pub struct CowCatcherHealth {
    pub obstacle_count: usize,
    pub critical_count: usize,
    pub should_restart: bool,
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
    pub tempo_online: bool,
    pub programming_online: bool,
    pub reasoning_online: bool,
    pub aesthetics_online: bool,
    /// Legacy alias for frontend compatibility
    #[serde(rename = "pete_online")]
    pub _pete_online: bool,
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
pub async fn health_check(
    State(state): State<AppState>,
    req: axum::extract::Request,
) -> axum::response::Response {
    let is_tunnel = req.headers().contains_key("cf-connecting-ip")
        || req.headers().contains_key("cf-ray");

    let router = state.inference_router.read().await;
    let llm_connected = router.is_healthy();
    drop(router);

    // Read cow_catcher diagnostics
    let cc = state.cow_catcher.read().await;
    let critical_count = cc.get_obstacles().iter().filter(|o| o.severity >= 8).count();
    drop(cc);

    let overall = if !llm_connected {
        "degraded"
    } else if critical_count > 0 {
        "warning"
    } else {
        "healthy"
    };

    // ═══ SECURITY: H5 Telemetry Leak Protection ═══
    // If request comes from the internet (tunnel), return ONLY the status.
    // Do not reveal internal API urls, database stats, or model variants.
    if is_tunnel {
        return axum::Json(serde_json::json!({ "status": overall })).into_response();
    }

    // For local IDE/UI usage, gather full diagnostics
    let router = state.inference_router.read().await;
    let llm_url = router.active_url().to_string();
    let backend_name = router.active_name().to_string();
    let model_hint = router
        .active_backend()
        .and_then(|b| b.model_name.clone())
        .unwrap_or_else(|| "Great_Recycler".to_string());
    let backends_available = router.backends.iter().filter(|b| b.healthy).count();
    drop(router);

    let db_connected = sqlx::query("SELECT 1")
        .execute(&state.db_pool)
        .await
        .is_ok();

    let tempo_ok = crate::http::QUICK
        .get("http://127.0.0.1:8001/health")
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    let programming_ok = crate::http::check_health("http://127.0.0.1:8000/health").await;
    let reasoning_ok = crate::http::check_health("http://127.0.0.1:8002/health").await;
    let aesthetics_ok = crate::http::check_health("http://127.0.0.1:8003/health").await;
    let voice_ok = false; // TTS is embedded via Kokoro ORT — no external sidecar

    let cc = state.cow_catcher.read().await;
    let obstacle_count = cc.get_obstacles().len();
    let should_restart = cc.should_restart();
    drop(cc);

    let uptime = START_TIME.get().map(|t| t.elapsed().as_secs()).unwrap_or(0);

    axum::Json(HealthResponse {
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
                "SQLite connected".to_string()
            } else {
                "SQLite not available".to_string()
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
            tempo_online: tempo_ok,
            programming_online: programming_ok,
            reasoning_online: reasoning_ok,
            aesthetics_online: aesthetics_ok,
            _pete_online: tempo_ok,
        },
        voice: VoiceHealth {
            connected: voice_ok,
            url: "embedded://ort/kokoro".to_string(),
        },
        cow_catcher: CowCatcherHealth {
            obstacle_count,
            critical_count,
            should_restart,
        },
        uptime_secs: uptime,
    })
    .into_response()
}
