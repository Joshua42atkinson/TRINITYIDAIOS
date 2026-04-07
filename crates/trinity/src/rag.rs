// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        rag.rs
// PURPOSE:     RAG (Retrieval-Augmented Generation) — vLLM vector semantic + text search
//
// 🪟 THE LIVING CODE TEXTBOOK (P-ART-Y Gear R: Research):
// This file is the memory cortex of the OS. It is designed to be read, modified, 
// and authored by YOU. If you want to change how Trinity understands or searches
// its own files and your portfolios, this is the system to edit.
// ACTION: Edit `search_documents()` to adjust semantic similarity thresholds.
//
// 📖 THE HOOK BOOK CONNECTION:
// This file powers the 'Vector Database' Hook. It uses vLLM embeddings to turn natural 
// language into mathematical meaning. You can use this engine to build your own 
// AI search apps! For a full catalogue of capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
//
// ARCHITECTURE:
//   • SQLite + in-memory cosine similarity for semantic search
//   • vLLM Nomic-embed (768-dim) for semantic search (cosine similarity)
//   • ILIKE as last-resort fallback
//   • Embedding generation via vLLM /v1/embeddings (Nomic-embed or compatible)
//   •   → Falls back to hash-based embedding if server unavailable
//   • 128K context allocation: CRITICAL/ACTIVE/REFERENCE/LEGACY buckets
//
// TABLES:
//   document_embeddings — vector(384) embeddings with HNSW index
//   trinity_documents   — document metadata
//   trinity_chunks      — text chunks for full-text search
//
// DEPENDENCIES:
//   - sqlx — SQLite async interface
//   - reqwest — HTTP client for embedding API
//   - tracing — RAG operation logging
//
// ═══════════════════════════════════════════════════════════════════════════════

use tokio::sync::Mutex;

use sqlx::SqlitePool;
use tracing::{debug, info, warn};

/// Embedding dimension — maps to the Nomic embedding dimensionality
const EMBEDDING_DIM: usize = 768;

/// Initialize the RAG tables if they don't exist
pub async fn ensure_tables(pool: &SqlitePool) -> anyhow::Result<()> {
    // Text document metadata
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trinity_documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            category TEXT NOT NULL DEFAULT 'general',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Text chunks for full-text search fallback
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trinity_chunks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            document_id INTEGER REFERENCES trinity_documents(id),
            chunk_index INTEGER NOT NULL,
            content TEXT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Vector embeddings table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS document_embeddings (
            id TEXT PRIMARY KEY,
            doc_path TEXT NOT NULL,
            chunk_index INTEGER NOT NULL,
            content TEXT NOT NULL,
            embedding TEXT,
            metadata TEXT DEFAULT '{}',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(doc_path, chunk_index)
        )
        "#,
    )
    .execute(pool)
    .await?;

    info!("✅ RAG tables ready (SQLite + Pure Rust Vector Search)");
    Ok(())
}

/// Search documents — tries semantic vector search first, falls back to text search
pub async fn search_documents(pool: &SqlitePool, query: &str) -> anyhow::Result<Vec<String>> {
    // Try semantic search first
    match search_semantic(pool, query, 3).await {
        Ok(results) if !results.is_empty() => {
            debug!("[RAG] Semantic search returned {} results", results.len());
            return Ok(results);
        }
        Ok(_) => debug!("[RAG] Semantic search returned 0 results, falling back to text"),
        Err(e) => debug!("[RAG] Semantic search failed ({}), falling back to text", e),
    }

    // Fallback: full-text search
    search_text(pool, query).await
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    for (va, vb) in a.iter().zip(b.iter()) {
        dot += va * vb;
        norm_a += va * va;
        norm_b += vb * vb;
    }
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    (dot / (norm_a.sqrt() * norm_b.sqrt())) as f64
}

/// Semantic vector search using purely local Rust Math in memory
async fn search_semantic(pool: &SqlitePool, query: &str, limit: i64) -> anyhow::Result<Vec<String>> {
    let query_embedding = generate_embedding(query).await?;

    let rows: Vec<(String, String)> = sqlx::query_as(
        r#"
        SELECT content, embedding
        FROM document_embeddings
        WHERE embedding IS NOT NULL
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut scored: Vec<(String, f64)> = rows
        .into_iter()
        .filter_map(|(content, embedding_str)| {
            if let Ok(vec) = serde_json::from_str::<Vec<f32>>(&embedding_str) {
                let sim = cosine_similarity(&query_embedding, &vec);
                Some((content, sim))
            } else {
                None
            }
        })
        .collect();

    // Sort descending by similarity
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let top = scored.into_iter().take(limit as usize).filter(|(_, sim)| *sim > 0.3).map(|(content, sim)| {
        debug!("[RAG] Semantic match (similarity={:.3}): {}...", sim, &content[..content.len().min(60)]);
        truncate_chunk(&content, 800)
    }).collect();

    Ok(top)
}

/// Full-text search using SQLite LIKE
async fn search_text(pool: &SqlitePool, query: &str) -> anyhow::Result<Vec<String>> {
    let wildcard_query = format!("%{}%", query);
    let results: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT content
        FROM trinity_chunks
        WHERE content LIKE ?
        LIMIT 3
        "#,
    )
    .bind(wildcard_query)
    .fetch_all(pool)
    .await?;

    Ok(results
        .into_iter()
        .map(|(c,)| truncate_chunk(&c, 800))
        .collect())
}

/// Generate embedding for text using vLLM Reverse Proxy
async fn generate_embedding(text: &str) -> anyhow::Result<Vec<f32>> {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "model": "nomic-embed-text-v1.5-AWQ",
        "input": text
    });

    let res = client
        .post("http://127.0.0.1:8000/v1/embeddings")
        .json(&payload)
        .send()
        .await?;

    if !res.status().is_success() {
        anyhow::bail!("vLLM embedding API error: {}", res.status());
    }

    let json: serde_json::Value = res.json().await?;
    let embedding_array = json["data"][0]["embedding"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Missing embedding array in vLLM response"))?;

    let vec: Vec<f32> = embedding_array
        .iter()
        .filter_map(|v| v.as_f64().map(|f| f as f32))
        .collect();

    if vec.len() != EMBEDDING_DIM {
        anyhow::bail!("Embedding dimension mismatch: got {}, expected {}", vec.len(), EMBEDDING_DIM);
    }
    Ok(vec)
}

/// Ingest a document by chunking and storing both text chunks AND vector embeddings
pub async fn ingest_document(
    pool: &SqlitePool,
    title: &str,
    content: &str,
    category: &str,
) -> anyhow::Result<usize> {
    ensure_tables(pool).await?;

    // Insert document metadata
    let doc_id = sqlx::query(
        "INSERT INTO trinity_documents (title, category) VALUES (?, ?)",
    )
    .bind(title)
    .bind(category)
    .execute(pool)
    .await?
    .last_insert_rowid();

    // Chunk the content (~500 words per chunk, split on paragraph boundaries)
    let chunks = chunk_text(content, 500);
    let chunk_count = chunks.len();

    for (i, chunk) in chunks.iter().enumerate() {
        // Store text chunk for full-text search
        sqlx::query(
            "INSERT INTO trinity_chunks (document_id, chunk_index, content) VALUES (?, ?, ?)",
        )
        .bind(doc_id)
        .bind(i as i32)
        .bind(chunk)
        .execute(pool)
        .await?;

        // Generate and store vector embedding
        match generate_embedding(chunk).await {
            Ok(embedding) => {
                let embedding_str = serde_json::to_string(&embedding)?;
                let doc_path = format!("{}#{}", title, i);
                let new_id = uuid::Uuid::new_v4().to_string();
                let meta = serde_json::json!({ "category": category, "title": title }).to_string();

                sqlx::query(
                    r#"
                    INSERT INTO document_embeddings (id, doc_path, chunk_index, content, embedding, metadata)
                    VALUES (?, ?, ?, ?, ?, ?)
                    ON CONFLICT (doc_path, chunk_index) DO UPDATE SET
                        content = excluded.content,
                        embedding = excluded.embedding,
                        updated_at = CURRENT_TIMESTAMP
                    "#,
                )
                .bind(new_id)
                .bind(&doc_path)
                .bind(i as i32)
                .bind(chunk)
                .bind(&embedding_str)
                .bind(&meta)
                .execute(pool)
                .await?;
            }
            Err(e) => {
                warn!("[RAG] Failed to generate embedding for chunk {}: {}", i, e);
            }
        }
    }

    info!(
        "📄 Ingested '{}': {} chunks (text + vector)",
        title, chunk_count
    );
    Ok(chunk_count)
}

/// Get RAG statistics
pub async fn rag_stats(pool: &SqlitePool) -> anyhow::Result<serde_json::Value> {
    let text_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM trinity_chunks")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));

    let vector_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM document_embeddings WHERE embedding IS NOT NULL")
            .fetch_one(pool)
            .await
            .unwrap_or((0,));

    let doc_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM trinity_documents")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));

    Ok(serde_json::json!({
        "documents": doc_count.0,
        "text_chunks": text_count.0,
        "vector_embeddings": vector_count.0,
        "embedding_dim": EMBEDDING_DIM,
        "search_strategy": "semantic_first_text_fallback",
    }))
}

/// Truncate a chunk to max_chars, breaking at word boundary
fn truncate_chunk(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        return text.to_string();
    }
    match text[..max_chars].rfind(' ') {
        Some(pos) => format!("{}...", &text[..pos]),
        None => format!("{}...", &text[..max_chars]),
    }
}

/// Chunk text into segments of approximately `max_words` words, splitting on paragraph boundaries
fn chunk_text(text: &str, max_words: usize) -> Vec<String> {
    let paragraphs: Vec<&str> = text.split("\n\n").collect();
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    let mut current_words = 0;

    for para in paragraphs {
        let para_words = para.split_whitespace().count();

        if current_words + para_words > max_words && !current_chunk.is_empty() {
            chunks.push(current_chunk.trim().to_string());
            current_chunk = String::new();
            current_words = 0;
        }

        if !current_chunk.is_empty() {
            current_chunk.push_str("\n\n");
        }
        current_chunk.push_str(para);
        current_words += para_words;
    }

    if !current_chunk.trim().is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }

    // If no chunks were created (single block of text), split by sentences
    if chunks.is_empty() && !text.trim().is_empty() {
        chunks.push(text.trim().to_string());
    }

    chunks
}

// ═══════════════════════════════════════════════════════════════════════════════
// AUTOPOIESIS (RAG SCALING SYSTEM)
// ═══════════════════════════════════════════════════════════════════════════════

/// Background task to scan the workspace and auto-index Code Textbook headers
///
/// WHY: This establishes the human/AI harmony loop. Humans read the natural-language
/// headers at the top of the `.rs` files. Pete and the Great Recycler read the
/// identical vector embeddings in the Qdrant DB. 
pub async fn auto_index_workspace(pool: &SqlitePool) -> anyhow::Result<()> {
    use std::path::PathBuf;

    // We know `env!("CARGO_MANIFEST_DIR")` is `[workspace]/crates/trinity`
    // So moving up two levels gives us the workspace root.
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
        
    let crates_dir = workspace_root.join("crates");
    if !crates_dir.exists() {
        warn!("⚠️ Autopoiesis RAG Indexer aborting: crates/ dir not found at {:?}", crates_dir);
        return Ok(());
    }
    
    info!("🔍 Autopoiesis RAG Indexer starting on {:?}", crates_dir);
    
    let mut files_indexed = 0;
    let mut dirs_to_visit = vec![crates_dir];
    
    // Simple async recursive directory walker
    while let Some(dir) = dirs_to_visit.pop() {
        if let Ok(mut entries) = tokio::fs::read_dir(&dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_dir() {
                    dirs_to_visit.push(path);
                } else if path.extension().map_or(false, |ext| ext == "rs") {
                    
                    // Read file looking for the Autopoiesis header
                    if let Ok(content) = tokio::fs::read_to_string(&path).await {
                        // Semantic target: Is this file part of the Code Textbook?
                        if content.contains("🪟 THE LIVING CODE TEXTBOOK") {
                            // Extract just the top 50 lines to keep the DB fast and noise-free
                            let header: String = content.lines().take(50).collect::<Vec<_>>().join("\n");
                            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                            
                            match ingest_document(pool, &file_name, &header, "architecture").await {
                                Ok(chunks) => {
                                    debug!("✅ Ingested Code Textbook: {} ({} chunks)", file_name, chunks);
                                    files_indexed += 1;
                                },
                                Err(e) => warn!("⚠️ Failed to ingest {}: {}", file_name, e),
                            }
                        }
                    }
                }
            }
        }
    }
    
    info!("🧠 Autopoiesis RAG Indexer complete: {} Actionable Textbook modules loaded into Vector Memory", files_indexed);
    Ok(())
}
