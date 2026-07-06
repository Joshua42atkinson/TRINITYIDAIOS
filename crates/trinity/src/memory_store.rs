use sqlx::SqlitePool;
use tracing::{debug, info};
use uuid::Uuid;

/// Create the memory table if it doesn't exist.
pub async fn init_memory_table(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trinity_memory (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            category TEXT DEFAULT 'general',
            importance INTEGER DEFAULT 5,
            created_at INTEGER NOT NULL,
            last_accessed INTEGER NOT NULL,
            access_count INTEGER DEFAULT 0
        )
        "#,
    )
    .execute(pool)
    .await?;
    debug!("[Memory] Memory table initialized");
    Ok(())
}

/// Store a fact in long-term memory.
/// Returns the generated ID.
pub async fn remember(pool: &SqlitePool, content: &str, category: &str) -> anyhow::Result<String> {
    let id = Uuid::new_v4().to_string();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    sqlx::query(
        "INSERT INTO trinity_memory (id, content, category, importance, created_at, last_accessed, access_count) VALUES (?, ?, ?, 5, ?, ?, 0)"
    )
    .bind(&id)
    .bind(content)
    .bind(category)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    info!("[Memory] Stored fact (id={}, category={})", &id[..8], category);
    Ok(id)
}

/// Recall relevant facts based on a text query.
/// Uses simple text matching (LIKE) for now — can be upgraded to semantic search later.
/// Returns up to `limit` facts, ordered by importance and recency.
pub async fn recall(pool: &SqlitePool, query: &str, limit: usize) -> anyhow::Result<Vec<MemoryFact>> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Simple keyword search — split query into words and match any
    let keywords: Vec<&str> = query.split_whitespace().filter(|w| w.len() > 2).collect();
    let limit = limit.min(20) as i64;

    let facts: Vec<MemoryFact> = if keywords.is_empty() {
        // No keywords — return most recent
        sqlx::query_as::<_, MemoryFact>(
            "SELECT id, content, category, importance, created_at, last_accessed, access_count FROM trinity_memory ORDER BY importance DESC, created_at DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(pool)
        .await?
    } else {
        // Build WHERE clause with OR conditions for each keyword
        let conditions: Vec<String> = keywords.iter()
            .map(|kw| format!("content LIKE '%{}%'", kw.replace('\'', "''")))
            .collect();
        let where_clause = conditions.join(" OR ");

        let sql = format!(
            "SELECT id, content, category, importance, created_at, last_accessed, access_count FROM trinity_memory WHERE {} ORDER BY importance DESC, created_at DESC LIMIT ?",
            where_clause
        );

        sqlx::query_as::<_, MemoryFact>(&sql)
        .bind(limit)
        .fetch_all(pool)
        .await?
    };

    // Update access timestamps
    for fact in &facts {
        let _ = sqlx::query("UPDATE trinity_memory SET last_accessed = ?, access_count = access_count + 1 WHERE id = ?")
            .bind(now)
            .bind(&fact.id)
            .execute(pool)
            .await;
    }

    debug!("[Memory] Recalled {} facts for query '{}'", facts.len(), &query[..query.len().min(50)]);
    Ok(facts)
}

/// Get all memory facts (for the memory view in the phone PWA).
pub async fn list_all(pool: &SqlitePool) -> anyhow::Result<Vec<MemoryFact>> {
    let facts: Vec<MemoryFact> = sqlx::query_as::<_, MemoryFact>(
        "SELECT id, content, category, importance, created_at, last_accessed, access_count FROM trinity_memory ORDER BY created_at DESC LIMIT 100"
    )
    .fetch_all(pool)
    .await?;

    Ok(facts)
}

/// Delete a memory fact by ID.
pub async fn forget(pool: &SqlitePool, id: &str) -> anyhow::Result<bool> {
    let result = sqlx::query("DELETE FROM trinity_memory WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

/// Build a context string from recalled facts for injection into the system prompt.
pub async fn recall_context(pool: &SqlitePool, query: &str) -> String {
    let facts = match recall(pool, query, 5).await {
        Ok(f) => f,
        Err(e) => {
            debug!("[Memory] Recall failed: {}", e);
            return String::new();
        }
    };

    if facts.is_empty() {
        return String::new();
    }

    let mut ctx = String::from("\n\n--- LONG-TERM MEMORY ---\n");
    for fact in &facts {
        ctx.push_str(&format!("- {}\n", fact.content));
    }
    ctx.push_str("--- END MEMORY ---\n");
    ctx
}

// ── Types ──

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct MemoryFact {
    pub id: String,
    pub content: String,
    pub category: String,
    pub importance: i32,
    pub created_at: i64,
    pub last_accessed: i64,
    pub access_count: i32,
}
