#![cfg(feature = "export")]

use axum::{extract::State, Json, http::StatusCode};
use crate::AppState;
use crate::{export, eye_container};
use tracing::info;

/// Compile an EYE container from the current quest state.
/// Returns the container as JSON — useful for preview before export.
pub async fn eye_compile(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let game = state.project.game_state.read().await;
    let vocab_db = state.vaam_bridge.vaam.database.read().await;
    let all_vocab: Vec<trinity_protocol::VocabularyWord> = vocab_db.all_words().into_iter().cloned().collect();
    drop(vocab_db);
    let container = eye_container::compile_container(&game, &all_vocab);
    let json = serde_json::to_value(&container).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("EYE compile failed: {}", e),
        )
    })?;
    info!(
        "👁️ EYE container compiled: {} objectives, {} vocab, {} assets",
        container.objectives.len(),
        container.vocabulary.len(),
        container.assets.len()
    );
    Ok(Json(serde_json::json!({
        "status": "ok",
        "container": json,
    })))
}

/// Preview the compiled EYE container as JSON.
pub async fn eye_preview(State(state): State<AppState>) -> Json<serde_json::Value> {
    let game = state.project.game_state.read().await;
    let vocab_db = state.vaam_bridge.vaam.database.read().await;
    let all_vocab: Vec<trinity_protocol::VocabularyWord> = vocab_db.all_words().into_iter().cloned().collect();
    drop(vocab_db);
    let container = eye_container::compile_container(&game, &all_vocab);
    Json(serde_json::to_value(&container).unwrap_or_default())
}

/// Export the EYE container as a downloadable HTML5 file.
/// Query params: ?format=html5_quiz | html5_adventure | raw_json | docx_portfolio | zip_portfolio
pub async fn eye_export(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<axum::response::Response, (StatusCode, String)> {
    let game = state.project.game_state.read().await;
    let vocab_db = state.vaam_bridge.vaam.database.read().await;
    let all_vocab: Vec<trinity_protocol::VocabularyWord> = vocab_db.all_words().into_iter().cloned().collect();
    drop(vocab_db);
    let container = eye_container::compile_container(&game, &all_vocab);

    let format = params
        .get("format")
        .and_then(|f| {
            serde_json::from_value::<eye_container::ExportFormat>(serde_json::Value::String(
                f.clone(),
            ))
            .ok()
        })
        .unwrap_or_default();

    let format_for_log = format.clone();
    let (filename, bytes, content_type) = tokio::task::spawn_blocking(move || {
        export::export(&container, &format)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Export task failed: {}", e)))?;

    info!(
        "📦 EYE export: {} ({} bytes, format: {:?})",
        filename,
        bytes.len(),
        format_for_log
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
