use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use crate::AppState;
use crate::memory_store;
use crate::handlers::rag::RagSearchRequest;

/// GET /api/memory — list all memory facts
pub async fn memory_list(
    State(state): State<AppState>,
) -> Result<Json<Vec<memory_store::MemoryFact>>, (StatusCode, String)> {
    memory_store::list_all(&state.db_pool)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// POST /api/memory — store a new fact
#[derive(Debug, Deserialize)]
pub struct MemoryRememberRequest {
    pub content: String,
    #[serde(default)]
    pub category: String,
}

pub async fn memory_remember(
    State(state): State<AppState>,
    Json(request): Json<MemoryRememberRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let category = if request.category.is_empty() { "general" } else { &request.category };
    let id = memory_store::remember(&state.db_pool, &request.content, category)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!({ "id": id, "status": "remembered" })))
}

/// POST /api/memory/search — recall relevant facts
pub async fn memory_search(
    State(state): State<AppState>,
    Json(request): Json<RagSearchRequest>,
) -> Result<Json<Vec<memory_store::MemoryFact>>, (StatusCode, String)> {
    memory_store::recall(&state.db_pool, &request.query, 10)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// DELETE /api/memory/:id — forget a fact
pub async fn memory_forget(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let deleted = memory_store::forget(&state.db_pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!({ "deleted": deleted, "id": id })))
}
