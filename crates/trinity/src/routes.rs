use std::path::PathBuf;
use axum::{Router, routing::{get, post}};
use tower_http::cors::CorsLayer;

use crate::AppState;
use crate::handlers;
use crate::auth;
use crate::agent;
use crate::tools;
use crate::jobs;
use crate::telephone;
use crate::health;


pub fn create_router(state: AppState) -> Router {
    let static_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("assets");

    // Serve Trinity static frontend files from static/ — nested under /trinity/
    let trinity_service = tower_http::services::ServeDir::new(&static_dir).fallback(
        tower_http::services::ServeFile::new(static_dir.join("index.html")),
    );

    let assets_service = tower_http::services::ServeDir::new(&assets_dir);

    // Portfolio static files (LDTAtkinson website — PRIMARY landing page at root /)
    let portfolio_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("LDTAtkinson/client/dist");
    let portfolio_service = tower_http::services::ServeDir::new(&portfolio_dir)
        .fallback(tower_http::services::ServeFile::new(
            portfolio_dir.join("index.html"),
        ));

    // ═══ API ROUTES (mounted at both /api/* and /trinity/api/*) ═══
    #[allow(unused_mut)]
    let mut api_routes = Router::new()
        .route("/api/health", get(health::health_check))
        .route("/api/hardware", get(handlers::system::get_hardware_status))
        .route("/api/v1/trinity", post(handlers::chat::trinity_chat).layer(axum::extract::DefaultBodyLimit::max(1024 * 1024)))
        .route("/api/chat", post(handlers::chat::chat).layer(axum::extract::DefaultBodyLimit::max(1024 * 1024)))
        .route("/api/chat/stream", post(handlers::chat::chat_stream).layer(axum::extract::DefaultBodyLimit::max(1024 * 1024)))
        .route("/api/chat/yardmaster", post(agent::agent_chat_stream).layer(axum::extract::DefaultBodyLimit::max(1024 * 1024)))
        .route("/api/chat/portfolio", post(handlers::chat::portfolio_chat_stream).layer(axum::extract::DefaultBodyLimit::max(1024 * 1024)))
        .route("/api/status", get(handlers::system::status))
        .route("/api/config/setup", post(handlers::system::setup_config))
        .route("/api/models", get(handlers::inference::list_models))
        .route("/api/models/active", get(handlers::inference::active_model))
        .route("/api/model/status", get(handlers::inference::model_status))
        .route("/api/models/switch", post(handlers::inference::switch_model))
        .route("/api/ingest", post(handlers::rag::ingest_document))
        .route("/api/tools", get(tools::list_tools))
        .route("/api/projects/community", get(handlers::projects::api_community_templates))
        .route("/api/tools/execute", post(tools::execute_tool))
        .route("/api/telemetry/stream", get(handlers::system::telemetry_stream))
        .route("/api/document/:id", get(handlers::rag::get_document))
        .route("/api/daydream/command", post(handlers::daydream::post_daydream_command))
        .route("/api/system/backend-start", post(handlers::system::backend_start))
        .route("/api/mcp", post(handlers::system::mcp_proxy))
        .route("/api/inference/status", get(handlers::inference::inference_status))
        .route("/api/inference/fleet", get(handlers::inference::fleet_status_endpoint))
        .route("/api/inference/resources", get(handlers::inference::inference_resources_endpoint))
        .route("/api/inference/switch", post(handlers::inference::inference_switch))
        .route("/api/inference/refresh", post(handlers::inference::inference_refresh))
        .route("/api/inference/start", post(handlers::inference::inference_start_endpoint))
        .route("/api/inference/stop", post(handlers::inference::inference_stop_endpoint))
        .route("/api/inference/hotel", get(handlers::inference::hotel_status_endpoint))
        .route("/api/inference/hotel/swap", post(handlers::inference::hotel_swap_endpoint))
        .route("/api/inference/hotel/studio", post(handlers::inference::hotel_studio_endpoint))
        .route("/api/inference/hotel/solo", post(handlers::inference::hotel_solo_endpoint))
        .route("/api/inference/hotel/close", post(handlers::inference::hotel_close_endpoint))
        .route("/api/inference/hotel/open", post(handlers::inference::hotel_open_endpoint))
        .route("/api/focus", get(handlers::focus::focus_status_endpoint))
        .route("/api/focus/creative", post(handlers::focus::focus_creative_endpoint))
        .route("/api/focus/code", post(handlers::focus::focus_code_endpoint))
        .route("/api/focus/night", post(handlers::focus::focus_night_endpoint))
        .route("/api/mode", get(handlers::system::get_app_mode).post(handlers::system::set_app_mode))
        .route("/api/stt/transcribe", post(handlers::voice::stt_transcribe).layer(axum::extract::DefaultBodyLimit::max(10 * 1024 * 1024)))
        .route("/api/stt/status", get(handlers::voice::stt_status))
        .route("/api/sessions", get(handlers::system::list_sessions))
        .route("/api/sessions/history", get(handlers::system::get_session_history))
        .route("/api/projects", get(handlers::projects::list_projects))
        .route("/api/projects/archive", post(handlers::projects::archive_project))
        .route("/api/projects/restore", post(handlers::projects::restore_project_endpoint))
        .route("/api/reset/demo", post(handlers::projects::reset_demo_data))
        .route("/api/rag/stats", get(handlers::rag::rag_stats))
        .route("/api/rag/search", post(handlers::rag::rag_search))
        .route("/api/memory", get(handlers::memory::memory_list).post(handlers::memory::memory_remember))
        .route("/api/memory/search", post(handlers::memory::memory_search))
        .route("/api/memory/:id", axum::routing::delete(handlers::memory::memory_forget))
        .route("/api/jobs", get(jobs::list_jobs).post(jobs::submit_job))
        .route("/api/jobs/chain", post(jobs::submit_chain))
        .route("/api/jobs/:id", get(jobs::job_status).delete(jobs::cancel_job))
        .route("/api/telephone", get(telephone::telephone_upgrade))
        .route("/docs/:filename", get(handlers::system::serve_chariot_doc))
        .route("/audiobook/:filename", get(handlers::system::serve_audiobook_audio))
        .route("/audiobook_art/:filename", get(handlers::system::serve_audiobook_art))
        .route("/phone", get(|| async {
            axum::response::Redirect::permanent("/trinity/phone.html")
        }));


    #[cfg(feature = "export")]
    {
        api_routes = api_routes
            .route("/api/eye/compile", post(handlers::export::eye_compile))
            .route("/api/eye/preview", get(handlers::export::eye_preview))
            .route("/api/eye/export", get(handlers::export::eye_export));
    }

        api_routes = api_routes
            .route("/api/journal/export/:id", get(handlers::journal::journal_export));
    }

    // ═══ MAIN APP: API routes + static file services ═══
    api_routes
        .nest_service("/trinity-assets", assets_service)
        .nest_service("/trinity", trinity_service)
        .fallback_service(portfolio_service)
        // ═══ SECURITY: API Authentication ═══
        .layer(axum::middleware::from_fn(auth::require_auth))
        // ═══ SECURITY: Rate Limiting ═══
        .layer(axum::middleware::from_fn(auth::rate_limit))
        // ═══ SECURITY: Restricted CORS ═══
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::AllowOrigin::predicate(|origin, _parts| {
                    let origin_bytes = origin.as_bytes();
                    if origin_bytes.starts_with(b"http://localhost:")
                        || origin_bytes.starts_with(b"http://127.0.0.1:")
                        || origin_bytes.starts_with(b"https://localhost:")
                        || origin_bytes.starts_with(b"https://127.0.0.1:")
                    {
                        return true;
                    }
                    if let Ok(ts_ip) = std::env::var("TRINITY_TAILSCALE_IP") {
                        if !ts_ip.is_empty() {
                            let ts_prefix = format!("http://{}", ts_ip);
                            if origin_bytes.starts_with(ts_prefix.as_bytes()) {
                                return true;
                            }
                        }
                    }
                    false
                }))
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                    axum::http::Method::OPTIONS,
                ])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ])
        )
        // ═══ SECURITY: Default Body Limit (2MB) ═══
        .layer(axum::extract::DefaultBodyLimit::max(2 * 1024 * 1024))
        .with_state(state)
}
