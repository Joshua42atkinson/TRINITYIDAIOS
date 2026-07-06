use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use crate::AppState;
use crate::rag;

/// Get RAG knowledge base statistics
pub async fn rag_stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    rag::rag_stats(&state.db_pool)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Search RAG knowledge base
#[derive(Debug, Deserialize)]
pub struct RagSearchRequest {
    pub query: String,
}

pub async fn rag_search(
    State(state): State<AppState>,
    Json(request): Json<RagSearchRequest>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    rag::search_documents(&state.db_pool, &request.query)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
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

pub async fn ingest_document(
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

/// Fetch markdown text for the Four Chariots or other root documentation
pub async fn get_document(
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<String, (StatusCode, String)> {
    if doc_id.contains("..") || doc_id.contains('/') {
        return Err((StatusCode::BAD_REQUEST, "Invalid document ID".to_string()));
    }
    
    let mut filename = doc_id.clone();
    if !filename.ends_with(".md") {
        filename.push_str(".md");
    }
    
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(&filename);

    tokio::fs::read_to_string(&path).await.map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            format!("Document not found at {:?}: {}", path, e),
        )
    })
}

/// Ingest Trinity bible and active docs into RAG on startup
pub async fn auto_ingest_docs(pool: &sqlx::SqlitePool) {
    use tracing::{info, warn};
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

    // Phase 8: Autopoiesis Code Textbook Ingestion
    if let Err(e) = rag::auto_index_workspace(pool).await {
        warn!("⚠️ Failed to ingest Code Textbook into Vector DB: {}", e);
    }

    info!(
        "📚 Auto-ingest complete: {}/{} documents loaded into RAG",
        ingested,
        docs_to_ingest.len()
    );
}

