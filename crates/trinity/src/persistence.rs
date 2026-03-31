// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        persistence.rs
// PURPOSE:     Conversation & project persistence — nothing is ever lost
//
// ARCHITECTURE:
//   • PostgreSQL tables for sessions, messages, and projects
//   • Every user/assistant message is saved to DB immediately
//   • Sessions restore conversation history across server restarts
//   • DAYDREAM archive: scope creeps become scope hopes via recycling
//
// TABLES:
//   trinity_sessions   — session metadata (id, alias, timestamps)
//   trinity_messages   — every message in every session (role, content, metadata)
//   trinity_projects   — game projects with GDD JSON and archive status
//
// DEPENDENCIES:
//   - sqlx — PostgreSQL async operations
//   - serde — JSON serialization
//   - chrono — Timestamps
//   - uuid — Session/project IDs
//   - tracing — Operation logging
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::{info, warn};

/// Run all SQL migration files from the migrations/ directory.
///
/// PostgreSQL does not allow multiple commands in a single prepared statement.
/// This function reads each `.sql` file, splits on `;` boundaries (respecting
/// dollar-quoted function bodies like `$$ ... $$`), and executes each statement
/// individually. Files are sorted by name so numbered migrations run in order.
pub async fn run_all_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    let migrations_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("migrations"))
        .unwrap_or_else(|| std::path::PathBuf::from("migrations"));

    if !migrations_dir.exists() {
        info!("📂 No migrations/ directory found — skipping SQL migrations");
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(&migrations_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "sql")
                .unwrap_or(false)
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in &entries {
        let path = entry.path();
        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        let sql = std::fs::read_to_string(&path)?;

        let statements = split_sql_statements(&sql);
        let mut executed = 0;

        for stmt in &statements {
            let trimmed = stmt.trim();
            if trimmed.is_empty() || trimmed.starts_with("--") {
                continue;
            }
            match sqlx::query(trimmed).execute(pool).await {
                Ok(_) => executed += 1,
                Err(e) => {
                    // Log but don't fail — many statements use IF NOT EXISTS
                    // and some might conflict with existing schema
                    warn!(
                        "⚠️ Migration {} statement failed (continuing): {}",
                        filename,
                        e.to_string().lines().next().unwrap_or("unknown")
                    );
                }
            }
        }
        info!(
            "📜 Migration {}: {}/{} statements executed",
            filename,
            executed,
            statements.len()
        );
    }

    info!("✅ All SQL migrations processed");
    Ok(())
}

/// Split a SQL file into individual statements, respecting dollar-quoted
/// function bodies (`$$ ... $$`) used in CREATE FUNCTION / CREATE TRIGGER.
///
/// Simple `;` splitting would break inside PL/pgSQL function bodies.
fn split_sql_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut in_dollar_quote = false;
    let mut chars = sql.chars().peekable();

    while let Some(ch) = chars.next() {
        current.push(ch);

        // Track $$ dollar-quoting (used in PL/pgSQL functions)
        if ch == '$' {
            if let Some(&next) = chars.peek() {
                if next == '$' {
                    current.push(chars.next().unwrap());
                    in_dollar_quote = !in_dollar_quote;
                    continue;
                }
            }
        }

        // Split on `;` only when not inside a dollar-quoted block
        if ch == ';' && !in_dollar_quote {
            let stmt = current.trim().to_string();
            if !stmt.is_empty() && stmt != ";" {
                statements.push(stmt);
            }
            current.clear();
        }
    }

    // Handle trailing statement without semicolon
    let remaining = current.trim().to_string();
    if !remaining.is_empty() && remaining != ";" && !remaining.starts_with("--") {
        statements.push(remaining);
    }

    statements
}

/// Session summary for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub id: String,
    pub alias: String,
    pub message_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// Project summary for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub id: String,
    pub session_id: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
    pub archived_at: Option<String>,
    pub archive_reason: Option<String>,
}

/// Ensure all persistence tables exist
pub async fn ensure_persistence_tables(pool: &SqlitePool) -> anyhow::Result<()> {
    // Sessions table — one per conversation/workflow
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trinity_sessions (
            id TEXT PRIMARY KEY,
            alias TEXT NOT NULL DEFAULT '',
            mode TEXT NOT NULL DEFAULT 'dev',
            metadata TEXT NOT NULL DEFAULT '{}',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Messages table — every message in every session
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trinity_messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id TEXT NOT NULL REFERENCES trinity_sessions(id) ON DELETE CASCADE,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            image_base64 TEXT,
            metadata TEXT NOT NULL DEFAULT '{}',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Index for fast session message retrieval
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_messages_session_id 
        ON trinity_messages (session_id, created_at)
        "#,
    )
    .execute(pool)
    .await?;

    // Projects table — game/learning projects with GDD
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trinity_projects (
            id TEXT PRIMARY KEY,
            session_id TEXT REFERENCES trinity_sessions(id),
            name TEXT NOT NULL,
            gdd_json TEXT NOT NULL DEFAULT '{}',
            status TEXT NOT NULL DEFAULT 'active',
            workspace_path TEXT,
            archive_reason TEXT,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            archived_at DATETIME
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Tool calls table — logs every agent tool invocation for analytics
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trinity_tool_calls (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id TEXT NOT NULL,
            tool_name TEXT NOT NULL,
            params TEXT NOT NULL DEFAULT '{}',
            result_status TEXT NOT NULL DEFAULT 'ok',
            result_preview TEXT,
            latency_ms INTEGER,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Index for tool call analytics
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_tool_calls_session 
        ON trinity_tool_calls (session_id, created_at)
        "#,
    )
    .execute(pool)
    .await?;

    // Background Jobs table — autonomous agent work
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trinity_background_jobs (
            id TEXT PRIMARY KEY,
            message TEXT NOT NULL,
            mode TEXT NOT NULL,
            status TEXT NOT NULL,
            turns_used INTEGER NOT NULL DEFAULT 0,
            tools_called TEXT NOT NULL DEFAULT '[]',
            output_path TEXT,
            log TEXT NOT NULL DEFAULT '[]',
            final_response TEXT,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            completed_at DATETIME
        )
        "#,
    )
    .execute(pool)
    .await?;

    info!("✅ Persistence tables ready");
    Ok(())
}

/// Log a tool call for analytics and debugging
pub async fn save_tool_call(
    pool: &SqlitePool,
    session_id: &str,
    tool_name: &str,
    params: &serde_json::Value,
    result_status: &str,
    result_preview: Option<&str>,
    latency_ms: i32,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO trinity_tool_calls (session_id, tool_name, params, result_status, result_preview, latency_ms) 
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(session_id)
    .bind(tool_name)
    .bind(params)
    .bind(result_status)
    .bind(result_preview)
    .bind(latency_ms)
    .execute(pool)
    .await?;

    Ok(())
}

/// Create or get a session
pub async fn ensure_session(pool: &SqlitePool, session_id: &str, mode: &str) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO trinity_sessions (id, mode) 
        VALUES ($1, $2) 
        ON CONFLICT (id) DO UPDATE SET updated_at = NOW()
        "#,
    )
    .bind(session_id)
    .bind(mode)
    .execute(pool)
    .await?;

    Ok(())
}

/// Save a message to the database
pub async fn save_message(
    pool: &SqlitePool,
    session_id: &str,
    role: &str,
    content: &str,
    image_base64: Option<&str>,
    metadata: Option<&serde_json::Value>,
) -> anyhow::Result<()> {
    let default_meta = serde_json::json!({});
    let meta = metadata.unwrap_or(&default_meta);

    sqlx::query(
        r#"
        INSERT INTO trinity_messages (session_id, role, content, image_base64, metadata) 
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(session_id)
    .bind(role)
    .bind(content)
    .bind(image_base64)
    .bind(meta)
    .execute(pool)
    .await?;

    // Touch session updated_at
    sqlx::query("UPDATE trinity_sessions SET updated_at = NOW() WHERE id = $1")
        .bind(session_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Load conversation history for a session (most recent N messages)
pub async fn load_session_history(
    pool: &SqlitePool,
    session_id: &str,
    limit: i64,
) -> anyhow::Result<Vec<crate::ChatMessage>> {
    let rows: Vec<(String, String, Option<String>)> = sqlx::query_as(
        r#"
        SELECT role, content, image_base64 
        FROM trinity_messages 
        WHERE session_id = $1 
        ORDER BY created_at DESC 
        LIMIT $2
        "#,
    )
    .bind(session_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    // Reverse to get chronological order
    let messages: Vec<crate::ChatMessage> = rows
        .into_iter()
        .rev()
        .map(|(role, content, image_base64)| crate::ChatMessage {
            role,
            content,
            timestamp: None,
            image_base64,
        })
        .collect();

    Ok(messages)
}

/// List all sessions (most recent first)
pub async fn list_sessions(pool: &SqlitePool, limit: i64) -> anyhow::Result<Vec<SessionSummary>> {
    let rows: Vec<(String, String, i64, String, String)> = sqlx::query_as(
        r#"
        SELECT 
            s.id, 
            s.alias,
            COALESCE((SELECT COUNT(*) FROM trinity_messages m WHERE m.session_id = s.id), 0) as msg_count,
            s.created_at::TEXT,
            s.updated_at::TEXT
        FROM trinity_sessions s
        ORDER BY s.updated_at DESC
        LIMIT $1
        "#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(
            |(id, alias, message_count, created_at, updated_at)| SessionSummary {
                id,
                alias,
                message_count,
                created_at,
                updated_at,
            },
        )
        .collect())
}

/// Create a new project linked to a session
pub async fn create_project(
    pool: &SqlitePool,
    project_id: &str,
    session_id: &str,
    name: &str,
    workspace_path: Option<&str>,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO trinity_projects (id, session_id, name, workspace_path) 
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (id) DO UPDATE SET 
            name = EXCLUDED.name,
            workspace_path = EXCLUDED.workspace_path
        "#,
    )
    .bind(project_id)
    .bind(session_id)
    .bind(name)
    .bind(workspace_path)
    .execute(pool)
    .await?;

    info!("📦 Project created: {} ({})", name, project_id);
    Ok(())
}

/// Save GDD JSON to a project
pub async fn save_project_gdd(
    pool: &SqlitePool,
    project_id: &str,
    gdd: &serde_json::Value,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE trinity_projects SET gdd_json = $1 WHERE id = $2
        "#,
    )
    .bind(gdd)
    .bind(project_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Archive a project (DAYDREAM — scope creep to scope hope)
pub async fn archive_project(pool: &SqlitePool, project_id: &str, reason: &str) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE trinity_projects 
        SET status = 'archived', 
            archive_reason = $1, 
            archived_at = NOW() 
        WHERE id = $2
        "#,
    )
    .bind(reason)
    .bind(project_id)
    .execute(pool)
    .await?;

    info!(
        "🌙 Project archived to DAYDREAM: {} (reason: {})",
        project_id, reason
    );
    Ok(())
}

/// Restore a project from DAYDREAM archive
pub async fn restore_project(pool: &SqlitePool, project_id: &str) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE trinity_projects 
        SET status = 'active', 
            archive_reason = NULL, 
            archived_at = NULL 
        WHERE id = $1
        "#,
    )
    .bind(project_id)
    .execute(pool)
    .await?;

    info!("☀️ Project restored from DAYDREAM: {}", project_id);
    Ok(())
}

/// List projects (active or archived)
pub async fn list_projects(
    pool: &SqlitePool,
    status_filter: Option<&str>,
    limit: i64,
) -> anyhow::Result<Vec<ProjectSummary>> {
    type ProjectRow = (
        String,
        String,
        String,
        String,
        String,
        Option<String>,
        Option<String>,
    );
    let rows: Vec<ProjectRow> = if let Some(status) = status_filter {
        sqlx::query_as(
            r#"
            SELECT id, session_id, name, status, created_at::TEXT, archived_at::TEXT, archive_reason
            FROM trinity_projects 
            WHERE status = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(status)
        .bind(limit)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as(
            r#"
            SELECT id, session_id, name, status, created_at::TEXT, archived_at::TEXT, archive_reason
            FROM trinity_projects 
            ORDER BY created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(pool)
        .await?
    };

    Ok(rows
        .into_iter()
        .map(
            |(id, session_id, name, status, created_at, archived_at, archive_reason)| {
                ProjectSummary {
                    id,
                    session_id,
                    name,
                    status,
                    created_at,
                    archived_at,
                    archive_reason,
                }
            },
        )
        .collect())
}

/// Get the total message count across all sessions
pub async fn total_message_count(pool: &SqlitePool) -> anyhow::Result<i64> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM trinity_messages")
        .fetch_one(pool)
        .await?;
    Ok(count)
}

/// Get the total tool call count across all sessions
pub async fn total_tool_call_count(pool: &SqlitePool) -> anyhow::Result<i64> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM trinity_tool_calls")
        .fetch_one(pool)
        .await?;
    Ok(count)
}

/// Returns recent archived templates for the community feature
pub async fn list_community_templates(pool: &SqlitePool) -> anyhow::Result<Vec<ProjectSummary>> {
    let mut conn = pool.acquire().await?;
    let records = sqlx::query_as::<_, (i64, String, String, String, String, Option<String>, Option<String>)>(
        "SELECT id, session_id, name, status, created_at, archived_at, archive_reason FROM trinity_projects WHERE status = 'archived' ORDER BY id DESC LIMIT 50"
    )
    .fetch_all(&mut *conn)
    .await?;

    Ok(records
        .into_iter()
        .map(
            |(id, session_id, name, status, created_at, archived_at, archive_reason)| {
                ProjectSummary {
                    id: id.to_string(),
                    session_id,
                    name,
                    status,
                    created_at,
                    archived_at,
                    archive_reason,
                }
            },
        )
        .collect())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_simple_statements() {
        let sql = "CREATE TABLE foo (id INT); CREATE INDEX idx ON foo(id);";
        let stmts = split_sql_statements(sql);
        assert_eq!(stmts.len(), 2);
        assert!(stmts[0].starts_with("CREATE TABLE"));
        assert!(stmts[1].starts_with("CREATE INDEX"));
    }

    #[test]
    fn test_split_dollar_quoted_function() {
        let sql = r#"
CREATE TABLE test (id INT);

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER test_trigger
    BEFORE UPDATE ON test
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();
"#;
        let stmts = split_sql_statements(sql);
        assert_eq!(
            stmts.len(),
            3,
            "Should find CREATE TABLE, CREATE FUNCTION, CREATE TRIGGER"
        );
        // The function body should be kept intact (not split on internal `;`)
        assert!(
            stmts[1].contains("BEGIN"),
            "Function body should be preserved"
        );
        assert!(
            stmts[1].contains("RETURN NEW"),
            "Function body should include RETURN"
        );
    }

    #[test]
    fn test_split_with_comments() {
        let sql = "-- This is a comment\nCREATE TABLE t (id INT);\n-- Another comment\nINSERT INTO t VALUES (1);";
        let stmts = split_sql_statements(sql);
        assert_eq!(stmts.len(), 2);
    }

    #[test]
    fn test_split_empty_input() {
        let stmts = split_sql_statements("");
        assert!(stmts.is_empty());
    }

    #[test]
    fn test_split_migration_004_pattern() {
        // This is the pattern that caused the "cannot insert multiple commands" error
        let sql = r#"
CREATE TABLE IF NOT EXISTS quest_state (
    id SERIAL PRIMARY KEY,
    player_id TEXT NOT NULL DEFAULT 'default',
    UNIQUE(player_id)
);

CREATE INDEX IF NOT EXISTS idx_quest_state_player ON quest_state(player_id);

INSERT INTO quest_state (player_id) VALUES ('default') ON CONFLICT DO NOTHING;
"#;
        let stmts = split_sql_statements(sql);
        assert_eq!(
            stmts.len(),
            3,
            "Should split into CREATE TABLE, CREATE INDEX, INSERT"
        );
    }
}
