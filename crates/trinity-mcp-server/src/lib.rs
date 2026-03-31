//! Trinity Memory MCP Server
//!
//! Provides intelligent documentation search and implementation status tracking
//! for Trinity development using semantic vector search.

use anyhow::Result;
use async_trait::async_trait;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use tracing::{debug, info};

pub mod embeddings;
pub mod pedagogical_schema;
pub mod quest_context;
pub mod self_work;
pub use embeddings::*;
pub use pedagogical_schema::*;
pub use quest_context::*;

/// Trinity Memory MCP Server
pub struct TrinityMcpServer {
    /// PostgreSQL connection pool
    pub db_pool: SqlitePool,
    /// Document cache
    document_cache: DashMap<String, CachedDocument>,
}

/// Re-export self_work types for use in handle_tool_call
pub use self_work::{SelfWorkResult, SelfWorkWorkflow, StepResult, WorkflowStep};

/// Cached document with metadata
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CachedDocument {
    path: String,
    content: String,
    last_modified: chrono::DateTime<chrono::Utc>,
    embedding_hash: u64,
}

/// MCP request/response types
#[derive(Debug, Deserialize)]
pub struct McpRequest {
    pub id: String,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct McpResponse {
    pub id: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<McpError>,
}

#[derive(Debug, Serialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
}

/// Tool definitions
#[derive(Debug, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

impl TrinityMcpServer {
    /// Create new Trinity MCP server from existing pool
    pub async fn new_from_pool(db_pool: SqlitePool) -> Result<Self> {
        info!("Initializing Trinity MCP Server from existing pool...");

        // Initialize database schema if needed
        Self::init_schema(&db_pool).await?;

        let server = Self {
            db_pool,
            document_cache: DashMap::new(),
        };

        // Load embedding model
        server.load_embedding_model().await?;

        info!("Trinity MCP Server initialized successfully");
        Ok(server)
    }

    /// Create new Trinity MCP server
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Initializing Trinity MCP Server...");

        // Connect to database
        let db_pool = SqlitePool::connect(database_url).await?;

        // Initialize database schema
        Self::init_schema(&db_pool).await?;

        let server = Self {
            db_pool,
            document_cache: DashMap::new(),
        };

        // Load embedding model
        server.load_embedding_model().await?;

        info!("Trinity MCP Server initialized successfully");
        Ok(server)
    }

    /// Initialize database schema
    async fn init_schema(db_pool: &SqlitePool) -> Result<()> {
        info!("Initializing database schema...");

        // Initialize base schema
        // Create vector extension table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS document_embeddings (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                doc_path TEXT NOT NULL,
                chunk_index INTEGER NOT NULL,
                content TEXT NOT NULL,
                embedding vector(384),
                metadata JSONB DEFAULT '{}',
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                UNIQUE(doc_path, chunk_index)
            )
        "#,
        )
        .execute(db_pool)
        .await?;

        // Note: HNSW index requires pgvector >= 0.5.0, skipping for now

        // Create implementation status table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS implementation_status (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                feature_name TEXT UNIQUE NOT NULL,
                status TEXT NOT NULL CHECK (status IN ('pending', 'in_progress', 'completed', 'blocked')),
                description TEXT,
                last_checked TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                evidence JSONB DEFAULT '{}',
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
        "#)
        .execute(db_pool)
        .await?;

        // Initialize pedagogical schema extensions
        let ped_schema = PedagogicalSchema::new(db_pool.clone());
        ped_schema.init().await?;

        info!("Database schema initialized");
        Ok(())
    }

    /// Load embedding model
    async fn load_embedding_model(&self) -> Result<()> {
        info!("Using hash-based embeddings for now");
        // For now, we'll use hash-based embeddings like in rag_search.rs
        // Can upgrade to real embeddings later
        Ok(())
    }

    /// Generate embedding for text
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Simple hash-based embedding (same as rag_search.rs)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();

        let mut embedding = vec![0.0f32; 384];
        for (i, val) in embedding.iter_mut().enumerate().take(384) {
            *val = ((hash >> (i % 64)) % 1000) as f32 / 1000.0;
        }

        // Normalize embedding
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        for v in &mut embedding {
            *v /= norm;
        }

        Ok(embedding)
    }

    /// Index a document
    pub async fn index_document(&self, path: &str, content: &str) -> Result<()> {
        info!("Indexing document: {}", path);

        // Split content into chunks
        let chunks = self.chunk_content(content, 500, 100);

        // Generate embeddings for each chunk
        for (index, chunk) in chunks.iter().enumerate() {
            let embedding = self.generate_embedding(chunk).await?;

            // Store in database
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
            .bind(path)
            .bind(index as i32)
            .bind(chunk)
            .bind(&serde_json::to_string(&embedding).unwrap_or_default())
            .execute(&self.db_pool)
            .await?;
        }

        // Cache the document
        self.document_cache.insert(
            path.to_string(),
            CachedDocument {
                path: path.to_string(),
                content: content.to_string(),
                last_modified: chrono::Utc::now(),
                embedding_hash: 0, // TODO: Calculate actual hash
            },
        );

        info!("Document indexed successfully: {} chunks", chunks.len());
        Ok(())
    }

    /// Split content into overlapping chunks
    fn chunk_content(&self, content: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let chars: Vec<char> = content.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let end = (i + chunk_size).min(chars.len());
            let chunk: String = chars[i..end].iter().collect();
            chunks.push(chunk);

            if end == chars.len() {
                break;
            }

            i = end - overlap;
        }

        chunks
    }

    /// Search for relevant documentation
    pub async fn search_documentation(&self, query: &str, limit: u32) -> Result<Vec<SearchResult>> {
        debug!("Searching documentation for: {}", query);

        // Generate query embedding
        let query_embedding = self.generate_embedding(query).await?;

        // Perform similarity search
        let rows = sqlx::query(
            r#"
            SELECT
                doc_path,
                chunk_index,
                content,
                1 - (embedding <=> $1) as similarity,
                metadata
            FROM document_embeddings
            WHERE embedding <=> $1 < 0.5
            ORDER BY embedding <=> $1
            LIMIT $2
        "#,
        )
        .bind(&serde_json::to_string(&query_embedding).unwrap_or_default())
        .bind(limit as i64)
        .fetch_all(&self.db_pool)
        .await?;

        let mut results = Vec::new();
        for row in rows {
            results.push(SearchResult {
                document_path: row.get("doc_path"),
                chunk_index: row.get("chunk_index"),
                content: row.get("content"),
                similarity: row.get("similarity"),
                metadata: row.get("metadata"),
            });
        }

        debug!("Found {} relevant documents", results.len());
        Ok(results)
    }

    /// Check implementation status
    pub async fn check_implementation_status(&self, feature: &str) -> Result<ImplementationStatus> {
        debug!("Checking implementation status for: {}", feature);

        // First try direct lookup
        let row = sqlx::query(
            r#"
            SELECT status, description, last_checked, evidence
            FROM implementation_status
            WHERE feature_name = $1
        "#,
        )
        .bind(feature)
        .fetch_optional(&self.db_pool)
        .await?;

        if let Some(row) = row {
            return Ok(ImplementationStatus {
                feature: feature.to_string(),
                status: row.get("status"),
                description: row.get("description"),
                last_checked: row.get("last_checked"),
                evidence: row.get("evidence"),
                found_in_docs: false,
            });
        }

        // If not found, search documentation
        let search_results = self
            .search_documentation(&format!("{} implemented completed status", feature), 5)
            .await?;

        if search_results.is_empty() {
            return Ok(ImplementationStatus {
                feature: feature.to_string(),
                status: "unknown".to_string(),
                description: "No information found about this feature".to_string(),
                last_checked: chrono::Utc::now(),
                evidence: serde_json::json!({}),
                found_in_docs: false,
            });
        }

        // Analyze search results to determine status
        let status = self.analyze_implementation_status(&search_results);

        Ok(ImplementationStatus {
            feature: feature.to_string(),
            status: status.clone(),
            description: format!(
                "Status inferred from documentation search: {} results found",
                search_results.len()
            ),
            last_checked: chrono::Utc::now(),
            evidence: serde_json::json!({
                "search_results": search_results.len(),
                "top_result": search_results.first().map(|r| r.content.clone())
            }),
            found_in_docs: true,
        })
    }

    /// Analyze implementation status from search results
    fn analyze_implementation_status(&self, results: &[SearchResult]) -> String {
        let content: String = results
            .iter()
            .map(|r| r.content.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ");

        // Simple keyword-based status detection
        if content.contains("✅") || content.contains("complete") || content.contains("implemented")
        {
            "completed".to_string()
        } else if content.contains("🔄")
            || content.contains("in progress")
            || content.contains("working")
        {
            "in_progress".to_string()
        } else if content.contains("⏳") || content.contains("pending") || content.contains("todo")
        {
            "pending".to_string()
        } else if content.contains("❌") || content.contains("blocked") || content.contains("error")
        {
            "blocked".to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Update implementation status
    pub async fn update_implementation_status(
        &self,
        feature: &str,
        status: &str,
        description: Option<&str>,
        evidence: serde_json::Value,
    ) -> Result<()> {
        info!("Updating implementation status for {}: {}", feature, status);

        sqlx::query(
            r#"
            INSERT INTO implementation_status (feature_name, status, description, evidence)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (feature_name) DO UPDATE SET
                status = EXCLUDED.status,
                description = EXCLUDED.description,
                evidence = EXCLUDED.evidence,
                last_checked = NOW()
        "#,
        )
        .bind(feature)
        .bind(status)
        .bind(description)
        .bind(evidence)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    /// Get available tools
    pub fn get_available_tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "check_implementation_status".to_string(),
                description: "Check if a feature has been implemented".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "feature": {
                            "type": "string",
                            "description": "The feature name to check"
                        }
                    },
                    "required": ["feature"]
                }),
            },
            Tool {
                name: "search_documentation".to_string(),
                description: "Search Trinity documentation".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "Search query"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Maximum number of results",
                            "default": 10
                        }
                    },
                    "required": ["query"]
                }),
            },
            Tool {
                name: "index_document".to_string(),
                description: "Index a document for search".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Document path"
                        },
                        "content": {
                            "type": "string",
                            "description": "Document content"
                        }
                    },
                    "required": ["path", "content"]
                }),
            },
        ]
    }
}

/// Search result
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub document_path: String,
    pub chunk_index: i32,
    pub content: String,
    pub similarity: f64,
    pub metadata: serde_json::Value,
}

/// Implementation status
#[derive(Debug, Clone, Serialize)]
pub struct ImplementationStatus {
    pub feature: String,
    pub status: String,
    pub description: String,
    pub last_checked: chrono::DateTime<chrono::Utc>,
    pub evidence: serde_json::Value,
    pub found_in_docs: bool,
}

#[async_trait]
pub trait McpHandler: Send + Sync {
    async fn handle_request(&self, request: McpRequest) -> Result<McpResponse>;
}

#[async_trait]
impl McpHandler for TrinityMcpServer {
    async fn handle_request(&self, request: McpRequest) -> Result<McpResponse> {
        let result = match request.method.as_str() {
            "tools/list" => serde_json::to_value(self.get_available_tools())?,
            "tools/call" => self.handle_tool_call(&request.params).await?,
            _ => {
                return Ok(McpResponse {
                    id: request.id,
                    result: None,
                    error: Some(McpError {
                        code: -32601,
                        message: format!("Method not found: {}", request.method),
                    }),
                })
            }
        };

        Ok(McpResponse {
            id: request.id,
            result: Some(result),
            error: None,
        })
    }
}

impl TrinityMcpServer {
    async fn handle_tool_call(&self, params: &serde_json::Value) -> Result<serde_json::Value> {
        let tool_name = params["name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing tool name"))?;

        let arguments = &params["arguments"];

        match tool_name {
            "execute_self_work" => {
                let workflow: SelfWorkWorkflow =
                    serde_json::from_value(arguments["workflow"].clone())?;

                let result: SelfWorkResult = self.execute_self_work(&workflow).await?;
                Ok(serde_json::to_value(result)?)
            }
            "check_implementation_status" => {
                let feature = arguments["feature"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing feature parameter"))?;

                let status = self.check_implementation_status(feature).await?;
                Ok(serde_json::to_value(status)?)
            }
            "search_documentation" => {
                let query = arguments["query"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing query parameter"))?;

                let limit = arguments["limit"].as_u64().unwrap_or(10) as u32;

                let results = self.search_documentation(query, limit).await?;
                Ok(serde_json::to_value(results)?)
            }
            "index_document" => {
                let path = arguments["path"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing path parameter"))?;

                let content = arguments["content"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing content parameter"))?;

                self.index_document(path, content).await?;

                Ok(serde_json::json!({"status": "success", "message": "Document indexed"}))
            }
            "quest_start" => {
                let quest_json = arguments["quest"].clone();
                let quest: QuestContext = serde_json::from_value(quest_json)?;

                let workspace_root =
                    std::env::var("TRINITY_WORKSPACE_ROOT").unwrap_or_else(|_| ".".to_string());
                let loader = QuestContextLoader::new(
                    self.db_pool.clone(),
                    std::path::PathBuf::from(workspace_root),
                );

                let result = loader.load_quest_context(&quest).await?;
                Ok(serde_json::to_value(result)?)
            }
            "quest_context" => {
                let query = arguments["query"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing query parameter"))?;
                let limit = arguments["limit"].as_u64().unwrap_or(5) as usize;

                let workspace_root =
                    std::env::var("TRINITY_WORKSPACE_ROOT").unwrap_or_else(|_| ".".to_string());
                let loader = QuestContextLoader::new(
                    self.db_pool.clone(),
                    std::path::PathBuf::from(workspace_root),
                );

                let contexts = loader.get_relevant_context(query, limit).await?;
                Ok(serde_json::json!({
                    "contexts": contexts,
                    "count": contexts.len()
                }))
            }
            "quest_verify" => {
                // Verify commands are run by the sidecar, not MCP
                // This just returns the commands for the caller to execute
                let quest_json = arguments["quest"].clone();
                let quest: QuestContext = serde_json::from_value(quest_json)?;

                Ok(serde_json::json!({
                    "verify_commands": quest.verify_commands,
                    "count": quest.verify_commands.len()
                }))
            }
            _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_name)),
        }
    }
}
