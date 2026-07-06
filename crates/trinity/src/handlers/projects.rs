use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use crate::AppState;
use crate::persistence;

/// List game projects  
pub async fn list_projects(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<persistence::ProjectSummary>>, (StatusCode, String)> {
    let status_filter = params.get("status").map(|s| s.as_str());
    persistence::list_projects(&state.db_pool, status_filter, 50)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// ═══════════════════════════════════════════════════════════════════════
/// DEMO RESET — Clear chat history for prototype demonstrations
/// ═══════════════════════════════════════════════════════════════════════
pub async fn reset_demo_data(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let msgs = sqlx::query("DELETE FROM messages")
        .execute(&state.db_pool)
        .await
        .map(|r| r.rows_affected())
        .unwrap_or(0);
    let tools = sqlx::query("DELETE FROM tool_calls")
        .execute(&state.db_pool)
        .await
        .map(|r| r.rows_affected())
        .unwrap_or(0);
    tracing::info!("🔄 Demo reset: cleared {} messages, {} tool calls", msgs, tools);
    Ok(Json(serde_json::json!({
        "status": "reset_complete",
        "messages_cleared": msgs,
        "tool_calls_cleared": tools,
    })))
}

/// Archive a project to DAYDREAM
#[derive(Debug, Deserialize)]
pub struct ArchiveRequest {
    pub project_id: String,
    pub reason: String,
}

pub async fn archive_project(
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
pub struct RestoreRequest {
    pub project_id: String,
}

pub async fn restore_project_endpoint(
    State(state): State<AppState>,
    Json(request): Json<RestoreRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    persistence::restore_project(&state.db_pool, &request.project_id)
        .await
        .map(|_| Json(serde_json::json!({"status": "restored", "project_id": request.project_id})))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn api_community_templates(
    State(state): State<AppState>,
) -> impl axum::response::IntoResponse {
    match crate::persistence::list_community_templates(&state.db_pool).await {
        Ok(templates) => Json(serde_json::json!(templates)),
        Err(_) => Json(serde_json::json!([])),
    }
}
