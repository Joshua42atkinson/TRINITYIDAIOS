// standards.rs — Academic standards database (NGSS, Common Core)
// Provides search and alignment for instructional design.

use sqlx::SqlitePool;
use tracing::{info, debug};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Standard {
    pub id: String,
    pub framework: String,
    pub subject: String,
    pub grade_band: String,
    pub category: String,
    pub code: String,
    pub description: String,
}

/// Ensure the standards table exists (migration 007 also handles this)
pub async fn ensure_standards_table(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trinity_standards (
            id TEXT PRIMARY KEY,
            framework TEXT NOT NULL,
            subject TEXT NOT NULL,
            grade_band TEXT NOT NULL,
            category TEXT NOT NULL DEFAULT '',
            code TEXT NOT NULL,
            description TEXT NOT NULL,
            performance_expectations TEXT DEFAULT '[]',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"CREATE INDEX IF NOT EXISTS idx_standards_framework
           ON trinity_standards (framework, subject, grade_band)"#,
    )
    .execute(pool)
    .await?;

    info!("✅ Standards table ready");
    Ok(())
}

/// Search standards by keyword (ILIKE on code + description)
pub async fn search_standards(pool: &SqlitePool, query: &str, limit: i64) -> anyhow::Result<Vec<Standard>> {
    let pattern = format!("%{}%", query);
    let rows: Vec<(String, String, String, String, String, String, String)> = sqlx::query_as(
        r#"
        SELECT id, framework, subject, grade_band, category, code, description
        FROM trinity_standards
        WHERE code ILIKE $1 OR description ILIKE $1 OR category ILIKE $1
        ORDER BY framework, grade_band, code
        LIMIT $2
        "#,
    )
    .bind(&pattern)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    let results: Vec<Standard> = rows.into_iter().map(|(id, framework, subject, grade_band, category, code, description)| Standard {
        id, framework, subject, grade_band, category, code, description,
    }).collect();

    debug!("[Standards] Search '{}' returned {} results", query, results.len());
    Ok(results)
}

/// List standards by framework and optional subject/grade filter
pub async fn list_standards(
    pool: &SqlitePool,
    framework: Option<&str>,
    subject: Option<&str>,
    grade_band: Option<&str>,
    limit: i64,
) -> anyhow::Result<Vec<Standard>> {
    let mut sql = String::from(
        "SELECT id, framework, subject, grade_band, category, code, description FROM trinity_standards WHERE 1=1"
    );
    let mut binds: Vec<String> = Vec::new();

    if let Some(fw) = framework {
        binds.push(fw.to_string());
        sql.push_str(&format!(" AND framework = ${}", binds.len()));
    }
    if let Some(sub) = subject {
        binds.push(sub.to_string());
        sql.push_str(&format!(" AND subject = ${}", binds.len()));
    }
    if let Some(gb) = grade_band {
        binds.push(gb.to_string());
        sql.push_str(&format!(" AND grade_band = ${}", binds.len()));
    }

    binds.push(limit.to_string());
    sql.push_str(&format!(" ORDER BY framework, grade_band, code LIMIT ${}", binds.len()));

    let mut q = sqlx::query_as::<_, (String, String, String, String, String, String, String)>(&sql);
    for bind in &binds {
        q = q.bind(bind);
    }

    let rows = q.fetch_all(pool).await?;

    let results: Vec<Standard> = rows.into_iter().map(|(id, framework, subject, grade_band, category, code, description)| Standard {
        id, framework, subject, grade_band, category, code, description,
    }).collect();

    Ok(results)
}

/// Align lesson content to standards using LLM semantic matching.
/// Takes lesson content + target framework/grade, returns matched standards with reasoning.
pub async fn align_standards_llm(
    pool: &SqlitePool,
    content: &str,
    framework: &str,
    grade_band: &str,
) -> anyhow::Result<Vec<(Standard, String)>> {
    // First, get candidate standards from DB
    let candidates = list_standards(pool, Some(framework), None, Some(grade_band), 50).await?;

    if candidates.is_empty() {
        return Ok(vec![]);
    }

    // Build a prompt for the LLM to match
    let standards_list: String = candidates.iter().enumerate().map(|(i, s)| {
        format!("{}. [{}] {}: {}", i + 1, s.code, s.category, s.description)
    }).collect::<Vec<_>>().join("\n");

    let system_prompt = format!(
        r#"You are an instructional design standards alignment expert.
Given lesson content and a list of {} standards for grade band {}, identify which standards are MOST relevant to the lesson content.

For each relevant standard, provide:
1. The standard code (exact match from the list)
2. A brief reasoning (1-2 sentences) explaining why it aligns

Only include standards that genuinely align. Be selective — quality over quantity.
Respond in JSON format:
{{"alignments": [{{"code": "STANDARD_CODE", "reasoning": "why it fits"}}]}}"#,
        framework, grade_band
    );

    let user_prompt = format!("LESSON CONTENT:\n{}\n\nAVAILABLE STANDARDS:\n{}", content, standards_list);

    let client = &*crate::http::LONG;
    let body = serde_json::json!({
        "model": "default",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt}
        ],
        "temperature": 0.1,
        "max_tokens": 1024
    });

    let response = client
        .post("http://127.0.0.1:1234/v1/chat/completions")
        .json(&body)
        .timeout(std::time::Duration::from_secs(60))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Standards alignment LLM request failed: {}", e))?;

    let result: serde_json::Value = response.json().await.map_err(|e| anyhow::anyhow!("LLM JSON parse: {}", e))?;
    let llm_text = result["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("");

    // Parse LLM response — try JSON first, fall back to text parsing
    let alignments = parse_alignment_response(llm_text);

    // Match LLM codes to Standard structs
    let mut matched: Vec<(Standard, String)> = Vec::new();
    for (code, reasoning) in &alignments {
        if let Some(std) = candidates.iter().find(|s| s.code == *code) {
            matched.push((std.clone(), reasoning.clone()));
        }
    }

    info!("[Standards] Aligned {} standards for {} {}", matched.len(), framework, grade_band);
    Ok(matched)
}

/// Parse the LLM alignment response — handles JSON or plain text
fn parse_alignment_response(text: &str) -> Vec<(String, String)> {
    // Try JSON parse first
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(text) {
        if let Some(arr) = parsed["alignments"].as_array() {
            return arr.iter().filter_map(|a| {
                let code = a["code"].as_str()?.to_string();
                let reasoning = a["reasoning"].as_str().unwrap_or("").to_string();
                Some((code, reasoning))
            }).collect();
        }
    }

    // Fallback: try to find JSON embedded in text
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text[start..=end]) {
                if let Some(arr) = parsed["alignments"].as_array() {
                    return arr.iter().filter_map(|a| {
                        let code = a["code"].as_str()?.to_string();
                        let reasoning = a["reasoning"].as_str().unwrap_or("").to_string();
                        Some((code, reasoning))
                    }).collect();
                }
            }
        }
    }

    // Last resort: return empty — caller will handle
    Vec::new()
}
