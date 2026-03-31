//! Quest Context Loader
//!
//! Loads quest context_files into MCP for semantic search.
//! This enables the AI to have relevant context when executing quests.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use std::path::Path;
use tokio::fs;
use tracing::{info, warn};

/// Quest definition (minimal for context loading)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestContext {
    pub id: String,
    pub title: String,
    pub context_files: Vec<String>,
    pub target_files: Vec<String>,
    pub verify_commands: Vec<String>,
}

/// Result of loading quest context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedContext {
    pub quest_id: String,
    pub files_loaded: usize,
    pub total_chunks: usize,
    pub errors: Vec<String>,
}

/// Quest context loader
pub struct QuestContextLoader {
    db_pool: SqlitePool,
    workspace_root: std::path::PathBuf,
}

impl QuestContextLoader {
    /// Create new quest context loader
    pub fn new(db_pool: SqlitePool, workspace_root: std::path::PathBuf) -> Self {
        Self {
            db_pool,
            workspace_root,
        }
    }

    /// Load quest context_files into MCP for semantic search
    pub async fn load_quest_context(&self, quest: &QuestContext) -> Result<LoadedContext> {
        info!("Loading context for quest: {}", quest.id);

        let mut files_loaded = 0;
        let mut total_chunks = 0;
        let mut errors = Vec::new();

        for context_file in &quest.context_files {
            let file_path = self.workspace_root.join(context_file);

            match self.load_file(&file_path).await {
                Ok(chunks) => {
                    files_loaded += 1;
                    total_chunks += chunks;
                    info!("Loaded {} chunks from {}", chunks, context_file);
                }
                Err(e) => {
                    warn!("Failed to load {}: {}", context_file, e);
                    errors.push(format!("{}: {}", context_file, e));
                }
            }
        }

        Ok(LoadedContext {
            quest_id: quest.id.clone(),
            files_loaded,
            total_chunks,
            errors,
        })
    }

    /// Load a single file into the embedding index
    async fn load_file(&self, path: &Path) -> Result<usize> {
        let content = fs::read_to_string(path).await?;

        // Chunk the content
        let chunks = self.chunk_content(&content, 500, 100);

        // Get relative path for storage
        let relative_path = path
            .strip_prefix(&self.workspace_root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        // Index each chunk
        for (index, chunk) in chunks.iter().enumerate() {
            let embedding = self.generate_embedding(chunk).await?;

            sqlx::query(
                r#"
                INSERT INTO document_embeddings (doc_path, chunk_index, content, embedding)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (doc_path, chunk_index) DO UPDATE SET
                    content = EXCLUDED.content,
                    embedding = EXCLUDED.embedding,
                    updated_at = NOW()
            "#,
            )
            .bind(&relative_path)
            .bind(index as i32)
            .bind(chunk)
            .bind(&serde_json::to_string(&embedding).unwrap_or_default())
            .execute(&self.db_pool)
            .await?;
        }

        Ok(chunks.len())
    }

    /// Get relevant context for a quest step or query
    pub async fn get_relevant_context(&self, query: &str, limit: usize) -> Result<Vec<String>> {
        let query_embedding = self.generate_embedding(query).await?;

        // Cosine similarity search via pgvector
        let rows = sqlx::query(
            r#"
            SELECT content, doc_path
            FROM document_embeddings
            ORDER BY embedding <=> $1
            LIMIT $2
        "#,
        )
        .bind(&serde_json::to_string(&query_embedding).unwrap_or_default())
        .bind(limit as i32)
        .fetch_all(&self.db_pool)
        .await?;

        let contexts: Vec<String> = rows
            .iter()
            .map(|row| {
                let content: String = row.try_get("content").unwrap_or_default();
                let doc_path: String = row.try_get("doc_path").unwrap_or_default();
                format!("[{}]: {}", doc_path, content)
            })
            .collect();

        Ok(contexts)
    }

    /// Chunk content into overlapping segments
    fn chunk_content(&self, content: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut chunks = Vec::new();

        let mut start = 0;
        while start < words.len() {
            let end = (start + chunk_size).min(words.len());
            let chunk = words[start..end].join(" ");
            chunks.push(chunk);

            start += chunk_size - overlap;
            if start >= words.len() {
                break;
            }
        }

        chunks
    }

    /// Generate embedding for text (hash-based for now)
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();

        let mut embedding = vec![0.0f32; 384];
        for (i, val) in embedding.iter_mut().enumerate().take(384) {
            *val = ((hash.wrapping_add(i as u64)) % 1000) as f32 / 1000.0;
        }

        // Normalize
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for v in &mut embedding {
                *v /= norm;
            }
        }

        Ok(embedding)
    }

    /// Clear context for a specific quest (optional cleanup)
    pub async fn clear_quest_context(&self, _quest_id: &str) -> Result<()> {
        // Note: This would require tracking which files belong to which quest
        // For now, we keep all indexed content for potential reuse
        info!("Context clearing not implemented - keeping indexed content for reuse");
        Ok(())
    }
}

/// Load quest from JSON file
pub async fn load_quest_from_file(path: &Path) -> Result<QuestContext> {
    let content = fs::read_to_string(path).await?;
    let quest: QuestContext = serde_json::from_str(&content)?;
    Ok(quest)
}
