//! Trinity Documentation Ingestion Binary

use anyhow::Result;
use ignore::Walk;
use sqlx::SqlitePool;
use std::env;
use tracing::info;
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    fmt::init();

    // Connect to database
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://trinity:trinity6226@localhost:5432/trinity".to_string());

    let db_pool = SqlitePool::connect(&database_url).await?;
    info!("Connected to database");

    // Initialize MCP server for indexing
    let server = trinity_mcp_server::TrinityMcpServer::new(&database_url).await?;

    // Walk through documentation files
    let mut indexed_count = 0;
    let walker = Walk::new("./").filter(|entry| {
        entry
            .as_ref()
            .ok()
            .map(|e| {
                let path = e.path();
                let is_md = path.extension().map(|ext| ext == "md").unwrap_or(false);
                let not_target = !path.components().any(|c| c.as_os_str() == "target");
                let not_git = !path.components().any(|c| c.as_os_str() == ".git");
                is_md && not_target && not_git
            })
            .unwrap_or(false)
    });

    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        info!("Indexing: {}", path.display());

        // Read file content
        let content = match tokio::fs::read_to_string(path).await {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read {}: {}", path.display(), e);
                continue;
            }
        };

        // Index document
        if let Err(e) = server
            .index_document(&path.to_string_lossy(), &content)
            .await
        {
            eprintln!("Failed to index {}: {}", path.display(), e);
        } else {
            indexed_count += 1;
        }
    }

    info!("Successfully indexed {} documents", indexed_count);

    // Update implementation status for known features
    update_known_features(&db_pool).await?;

    info!("Documentation ingestion complete!");
    Ok(())
}

async fn update_known_features(db_pool: &SqlitePool) -> Result<()> {
    info!("Updating known implementation status...");

    let features = vec![
        (
            "97B Model Integration",
            "completed",
            "Qwen3.5-REAP-97B operational at 5.0 tokens/sec",
        ),
        (
            "Asset Generation Pipeline",
            "completed",
            "Complete pipeline from behavior to content (Mar 6)",
        ),
        (
            "QM Rubric Integration",
            "completed",
            "Quality Matters validation implemented",
        ),
        (
            "ADDIE Workflow",
            "completed",
            "Full instructional design workflow",
        ),
        (
            "MCP Integration",
            "in_progress",
            "Building MCP server for documentation search",
        ),
        (
            "Vector Database",
            "completed",
            "PostgreSQL with pgvector setup complete",
        ),
        (
            "Real Embeddings",
            "pending",
            "Replace hash-based embeddings with proper model",
        ),
    ];

    for (feature, status, description) in features {
        sqlx::query(
            r#"
            INSERT INTO implementation_status (feature_name, status, description)
            VALUES ($1, $2, $3)
            ON CONFLICT (feature_name) DO UPDATE SET
                status = EXCLUDED.status,
                description = EXCLUDED.description,
                last_checked = NOW()
        "#,
        )
        .bind(feature)
        .bind(status)
        .bind(description)
        .execute(db_pool)
        .await?;
    }

    info!("Implementation status updated");
    Ok(())
}
