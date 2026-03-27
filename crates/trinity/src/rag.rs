// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        rag.rs
// PURPOSE:     RAG (Retrieval-Augmented Generation) — pgvector semantic + text search
//
// 🪟 THE LIVING CODE TEXTBOOK (P-ART-Y Gear R: Research):
// This file is the memory cortex of the OS. It is designed to be read, modified, 
// and authored by YOU. If you want to change how Trinity understands or searches
// its own files and your portfolios, this is the system to edit.
// ACTION: Edit `search_documents()` to adjust semantic similarity thresholds.
//
// 📖 THE HOOK BOOK CONNECTION:
// This file powers the 'Vector Database' Hook. It uses pgvector to turn natural 
// language into mathematical meaning. You can use this engine to build your own 
// AI search apps! For a full catalogue of capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
//
// ARCHITECTURE:
//   • pgvector for semantic search (cosine similarity via HNSW index)
//   • Full-text search (PostgreSQL ts_rank) as fallback
//   • ILIKE as last-resort fallback
//   • Embedding generation via llama-server /v1/embeddings
//   •   → Falls back to hash-based embedding if server unavailable
//   • 128K context allocation: CRITICAL/ACTIVE/REFERENCE/LEGACY buckets
//
// TABLES:
//   document_embeddings — vector(384) embeddings with HNSW index
//   trinity_documents   — document metadata
//   trinity_chunks      — text chunks for full-text search
//
// DEPENDENCIES:
//   - sqlx — PostgreSQL async interface
//   - reqwest — HTTP client for embedding API
//   - tracing — RAG operation logging
//
// ═══════════════════════════════════════════════════════════════════════════════

use sqlx::PgPool;
use tracing::{debug, info, warn};

/// Embedding dimension — matches the existing document_embeddings table schema
const EMBEDDING_DIM: usize = 384;

/// Initialize the RAG tables if they don't exist
pub async fn ensure_tables(pool: &PgPool) -> anyhow::Result<()> {
    // Ensure pgvector extension is available
    sqlx::query("CREATE EXTENSION IF NOT EXISTS vector")
        .execute(pool)
        .await
        .ok(); // Silently fail if pgvector not installed

    // Text document metadata
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trinity_documents (
            id SERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            category TEXT NOT NULL DEFAULT 'general',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Text chunks for full-text search fallback
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trinity_chunks (
            id SERIAL PRIMARY KEY,
            document_id INTEGER REFERENCES trinity_documents(id),
            chunk_index INTEGER NOT NULL,
            content TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Trigram index for text search
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_chunks_content_trgm
        ON trinity_chunks USING gin (content gin_trgm_ops)
        "#,
    )
    .execute(pool)
    .await
    .ok(); // Silently fail if pg_trgm not installed

    // Vector embeddings table (may already exist)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS document_embeddings (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            doc_path TEXT NOT NULL,
            chunk_index INTEGER NOT NULL,
            content TEXT NOT NULL,
            embedding vector(384),
            metadata JSONB DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW(),
            UNIQUE(doc_path, chunk_index)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // HNSW index for fast approximate nearest neighbor search
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_embeddings_hnsw 
        ON document_embeddings USING hnsw (embedding vector_cosine_ops)
        "#,
    )
    .execute(pool)
    .await
    .ok(); // Ok if already exists

    info!("✅ RAG tables ready (pgvector + text search)");
    Ok(())
}

/// Search documents — tries semantic vector search first, falls back to text search
pub async fn search_documents(pool: &PgPool, query: &str) -> anyhow::Result<Vec<String>> {
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

/// Semantic vector search using pgvector cosine similarity
async fn search_semantic(pool: &PgPool, query: &str, limit: i64) -> anyhow::Result<Vec<String>> {
    let query_embedding = generate_embedding(query).await?;

    // Format embedding as PostgreSQL vector literal: '[0.1,0.2,...]'
    let embedding_str = format!(
        "[{}]",
        query_embedding
            .iter()
            .map(|v| format!("{:.6}", v))
            .collect::<Vec<_>>()
            .join(",")
    );

    let results: Vec<(String, f64)> = sqlx::query_as(
        r#"
        SELECT content, 1 - (embedding <=> $1::vector) as similarity
        FROM document_embeddings
        WHERE embedding IS NOT NULL
        ORDER BY embedding <=> $1::vector
        LIMIT $2
        "#,
    )
    .bind(&embedding_str)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(results
        .into_iter()
        .filter(|(_, sim)| *sim > 0.3) // Minimum similarity threshold
        .map(|(content, sim)| {
            debug!(
                "[RAG] Semantic match (similarity={:.3}): {}...",
                sim,
                &content[..content.len().min(60)]
            );
            truncate_chunk(&content, 800)
        })
        .collect())
}

/// Full-text search using PostgreSQL ts_rank
async fn search_text(pool: &PgPool, query: &str) -> anyhow::Result<Vec<String>> {
    // Try full-text search first
    let results: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT content
        FROM trinity_chunks
        WHERE to_tsvector('english', content) @@ plainto_tsquery('english', $1)
        ORDER BY ts_rank(to_tsvector('english', content), plainto_tsquery('english', $1)) DESC
        LIMIT 3
        "#,
    )
    .bind(query)
    .fetch_all(pool)
    .await?;

    if !results.is_empty() {
        return Ok(results
            .into_iter()
            .map(|(c,)| truncate_chunk(&c, 800))
            .collect());
    }

    // Fallback: simple ILIKE search
    let results: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT content
        FROM trinity_chunks
        WHERE content ILIKE '%' || $1 || '%'
        LIMIT 3
        "#,
    )
    .bind(query)
    .fetch_all(pool)
    .await?;

    Ok(results
        .into_iter()
        .map(|(c,)| truncate_chunk(&c, 800))
        .collect())
}

/// Generate embedding for text using llama-server's /v1/embeddings endpoint
/// Falls back to deterministic hash-based embedding if server is unavailable
async fn generate_embedding(text: &str) -> anyhow::Result<Vec<f32>> {
    // Try llama-server embeddings first
    let llm_base = std::env::var("LLM_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    let url = format!("{}/v1/embeddings", llm_base.trim_end_matches('/'));

    let client = &*crate::http::QUICK;

    let body = serde_json::json!({
        "input": text,
        "model": "mistral"
    });

    match client.post(&url).json(&body).send().await {
        Ok(resp) if resp.status().is_success() => {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(embedding) = json["data"][0]["embedding"].as_array() {
                    let vec: Vec<f32> = embedding
                        .iter()
                        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                        .collect();
                    // Truncate or pad to EMBEDDING_DIM
                    let mut result = vec![0.0f32; EMBEDDING_DIM];
                    for (i, v) in vec.iter().enumerate().take(EMBEDDING_DIM) {
                        result[i] = *v;
                    }
                    return Ok(result);
                }
            }
        }
        Ok(resp) => {
            debug!(
                "[RAG] Embedding API returned {}, using hash fallback",
                resp.status()
            );
        }
        Err(e) => {
            debug!(
                "[RAG] Embedding API unreachable ({}), using hash fallback",
                e
            );
        }
    }

    // Fallback: deterministic hash-based embedding
    // This provides consistent but non-semantic similarity
    Ok(hash_embedding(text))
}

/// Deterministic hash-based embedding fallback
/// Provides consistent embeddings based on word frequencies
fn hash_embedding(text: &str) -> Vec<f32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut embedding = vec![0.0f32; EMBEDDING_DIM];
    let words: Vec<&str> = text.split_whitespace().collect();

    // Hash each word and distribute across dimensions
    for (i, word) in words.iter().enumerate() {
        let mut hasher = DefaultHasher::new();
        word.to_lowercase().hash(&mut hasher);
        let hash = hasher.finish();

        // Spread the hash across multiple dimensions
        for j in 0..8 {
            let dim = ((hash >> (j * 8)) as usize + i * 3) % EMBEDDING_DIM;
            embedding[dim] += 1.0;
        }
    }

    // L2 normalize
    let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for v in &mut embedding {
            *v /= norm;
        }
    }

    embedding
}

/// Ingest a document by chunking and storing both text chunks AND vector embeddings
pub async fn ingest_document(
    pool: &PgPool,
    title: &str,
    content: &str,
    category: &str,
) -> anyhow::Result<usize> {
    ensure_tables(pool).await?;

    // Insert document metadata
    let doc_id: (i32,) = sqlx::query_as(
        "INSERT INTO trinity_documents (title, category) VALUES ($1, $2) RETURNING id",
    )
    .bind(title)
    .bind(category)
    .fetch_one(pool)
    .await?;

    // Chunk the content (~500 words per chunk, split on paragraph boundaries)
    let chunks = chunk_text(content, 500);
    let chunk_count = chunks.len();

    for (i, chunk) in chunks.iter().enumerate() {
        // Store text chunk for full-text search
        sqlx::query(
            "INSERT INTO trinity_chunks (document_id, chunk_index, content) VALUES ($1, $2, $3)",
        )
        .bind(doc_id.0)
        .bind(i as i32)
        .bind(chunk)
        .execute(pool)
        .await?;

        // Generate and store vector embedding
        match generate_embedding(chunk).await {
            Ok(embedding) => {
                let embedding_str = format!(
                    "[{}]",
                    embedding
                        .iter()
                        .map(|v| format!("{:.6}", v))
                        .collect::<Vec<_>>()
                        .join(",")
                );

                let doc_path = format!("{}#{}", title, i);
                sqlx::query(
                    r#"
                    INSERT INTO document_embeddings (doc_path, chunk_index, content, embedding, metadata)
                    VALUES ($1, $2, $3, $4::vector, $5)
                    ON CONFLICT (doc_path, chunk_index) DO UPDATE SET
                        content = EXCLUDED.content,
                        embedding = EXCLUDED.embedding,
                        updated_at = NOW()
                    "#,
                )
                .bind(&doc_path)
                .bind(i as i32)
                .bind(chunk)
                .bind(&embedding_str)
                .bind(serde_json::json!({ "category": category, "title": title }))
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
pub async fn rag_stats(pool: &PgPool) -> anyhow::Result<serde_json::Value> {
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
pub async fn auto_index_workspace(pool: &PgPool) -> anyhow::Result<()> {
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
