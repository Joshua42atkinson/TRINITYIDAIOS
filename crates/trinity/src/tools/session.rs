use std::path::PathBuf;
use tracing::info;
use super::home_dir;

pub async fn tool_quest_status() -> Result<String, String> {
    Ok("⚠️ quest_status is only available through the agent chat loop (Yardmaster tab). Use POST /api/quest for the REST API.".to_string())
}

pub async fn tool_quest_advance(params: &serde_json::Value) -> Result<String, String> {
    let _direction = params
        .get("direction")
        .and_then(|d| d.as_str())
        .unwrap_or("next");
    Ok("⚠️ quest_advance is only available through the agent chat loop (Yardmaster tab). Use POST /api/quest/advance for the REST API.".to_string())
}

pub async fn tool_work_log(params: &serde_json::Value) -> Result<String, String> {
    let title = params
        .get("title")
        .and_then(|t| t.as_str())
        .ok_or("Missing 'title' parameter")?;
    let content = params
        .get("content")
        .and_then(|c| c.as_str())
        .ok_or("Missing 'content' parameter")?;
    let status = params
        .get("status")
        .and_then(|s| s.as_str())
        .unwrap_or("complete");

    info!("📝 Recording work log: '{}' (status: {})", title, status);

    let client = &*crate::http::LONG;
    let payload = serde_json::json!({
        "title": title,
        "content": content,
        "status": status,
    });

    let response = client
        .post("http://127.0.0.1:3000/api/journal")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to record work log: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to save work log: {}",
            response.text().await.unwrap_or_default()
        ));
    }

    Ok(format!(
        "✅ Work log successfully recorded in journal: '{}' [{}]",
        title, status
    ))
}

pub async fn tool_task_queue(params: &serde_json::Value) -> Result<String, String> {
    let action = params
        .get("action")
        .and_then(|a| a.as_str())
        .ok_or("Missing 'action' parameter ('read', 'add', 'complete', 'next')")?;

    let task = params.get("task").and_then(|t| t.as_str()).unwrap_or("");
    let index = params.get("index").and_then(|i| i.as_u64()).unwrap_or(0) as usize;

    let queue_path = home_dir().join(".local/share/trinity/task_queue.md");

    // Ensure directory exists
    if let Some(parent) = queue_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create task queue directory: {}", e))?;
    }

    match action {
        "read" => {
            if !queue_path.exists() {
                return Ok("📋 Task queue is empty. Use action='add' to add tasks.".to_string());
            }
            let content = std::fs::read_to_string(&queue_path)
                .map_err(|e| format!("Failed to read task queue: {}", e))?;
            if content.trim().is_empty() {
                Ok("📋 Task queue is empty.".to_string())
            } else {
                Ok(format!("📋 CURRENT TASK QUEUE:\n\n{}", content))
            }
        }
        "add" => {
            if task.is_empty() {
                return Err("Missing 'task' description for action='add'".to_string());
            }

            let mut current = if queue_path.exists() {
                std::fs::read_to_string(&queue_path).unwrap_or_default()
            } else {
                String::new()
            };

            if !current.ends_with('\n') && !current.is_empty() {
                current.push('\n');
            }
            current.push_str(&format!("- [ ] {}\n", task));

            std::fs::write(&queue_path, &current)
                .map_err(|e| format!("Failed to update task queue: {}", e))?;

            info!("✅ Added task to queue: {}", task);
            Ok(format!("✅ Added task: '{}' to queue.", task))
        }
        "complete" => {
            if index == 0 {
                return Err("Must provide a 1-based 'index' to complete a task.".to_string());
            }

            if !queue_path.exists() {
                return Err("No task queue exists to complete from.".to_string());
            }

            let content = std::fs::read_to_string(&queue_path)
                .map_err(|e| format!("Failed to read task queue: {}", e))?;

            let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
            let mut todo_count = 0;
            let mut found = false;

            for line in lines.iter_mut() {
                if line.starts_with("- [ ] ") {
                    todo_count += 1;
                    if todo_count == index {
                        *line = line.replace("- [ ] ", "- [x] ");
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                return Err(format!("Task #{} not found or already complete.", index));
            }

            std::fs::write(&queue_path, lines.join("\n") + "\n")
                .map_err(|e| format!("Failed to update task queue: {}", e))?;

            info!("✅ Task #{} marked complete", index);
            Ok(format!("✅ Task #{} marked complete.", index))
        }
        "next" => {
            if !queue_path.exists() {
                return Ok("📋 No task queue exists. Nothing to do.".to_string());
            }

            let content = std::fs::read_to_string(&queue_path)
                .map_err(|e| format!("Failed to read task queue: {}", e))?;

            let mut task_num = 0;
            for line in content.lines() {
                if line.starts_with("- [ ] ") {
                    task_num += 1;
                    let task_text = line.trim_start_matches("- [ ] ");
                    return Ok(format!(
                        "📋 Next task (#{}):\n{}\n\nUse task_queue(action='complete', index={}) when done.",
                        task_num, task_text, task_num
                    ));
                }
            }

            Ok("🎉 All tasks complete! Use task_queue(action='add') to add more, or work_log() to write a session report.".to_string())
        }
        _ => Err(format!(
            "Unknown task_queue action: '{}'. Use 'read', 'add', 'complete', or 'next'.",
            action
        )),
    }
}

pub async fn tool_analyze_document(params: &serde_json::Value) -> Result<String, String> {
    let image_path = params["image_path"]
        .as_str()
        .ok_or("Missing 'image_path' parameter")?;
    let question = params["question"]
        .as_str()
        .unwrap_or("Parse this document to Markdown.");

    let image_bytes = tokio::fs::read(image_path)
        .await
        .map_err(|e| format!("Failed to read image {}: {}", image_path, e))?;

    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&image_bytes);

    let ext = std::path::Path::new(image_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png")
        .to_lowercase();
    let mime = match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "pdf" => "application/pdf",
        _ => "image/png",
    };

    let researcher_url =
        std::env::var("RESEARCHER_URL").unwrap_or_else(|_| "http://127.0.0.1:8001".to_string());

    let client = &*crate::http::LONG;

    let payload = serde_json::json!({
        "model": "qianfan-ocr",
        "messages": [{
            "role": "user",
            "content": [
                {
                    "type": "image_url",
                    "image_url": {
                        "url": format!("data:{};base64,{}", mime, b64)
                    }
                },
                {
                    "type": "text",
                    "text": question
                }
            ]
        }],
        "max_tokens": 16384,
        "temperature": 0.1
    });

    info!(
        "🔬 Researcher analyzing document: {} (question: {})",
        image_path, question
    );

    let response = client
        .post(format!("{}/v1/chat/completions", researcher_url))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Researcher sub-agent not responding on {}: {}. Start TRINITY on port 8001", researcher_url, e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Researcher returned {}: {}", status, body));
    }

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse researcher response: {}", e))?;

    let content = result["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No output from researcher");

    Ok(format!(
        "🔬 RESEARCHER (Qianfan-OCR) Analysis:\n\n{}",
        content
    ))
}

fn session_dir() -> PathBuf {
    let home = home_dir();
    home.join(".local/share/trinity/sessions")
}

pub async fn tool_save_session_summary(params: &serde_json::Value) -> Result<String, String> {
    let title = params["title"].as_str().ok_or("Missing 'title' parameter")?;
    let summary = params["summary"].as_str().ok_or("Missing 'summary' parameter")?;
    let next_steps = params["next_steps"].as_str().ok_or("Missing 'next_steps' parameter")?;
    let files_changed = params["files_changed"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    let sdir = session_dir();
    tokio::fs::create_dir_all(&sdir)
        .await
        .map_err(|e| format!("Failed to create session dir: {}", e))?;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("session_{}.json", timestamp);
    let path = sdir.join(&filename);

    let session_data = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "title": title,
        "summary": summary,
        "next_steps": next_steps,
        "files_changed": files_changed,
    });

    let json_str = serde_json::to_string_pretty(&session_data)
        .map_err(|e| format!("Serialization error: {}", e))?;

    tokio::fs::write(&path, json_str)
        .await
        .map_err(|e| format!("Failed to write session file {}: {}", path.display(), e))?;

    // Maintain a symlink/copy at "latest.json"
    let latest_path = sdir.join("latest.json");
    let _ = tokio::fs::remove_file(&latest_path).await;
    let _ = tokio::fs::copy(&path, &latest_path).await;

    info!("💾 Saved session summary: {}", path.display());
    Ok(format!(
        "✅ Session summary saved successfully.\nFile: {}\nLatest shortcut updated.",
        filename
    ))
}

pub async fn tool_load_session_context(_params: &serde_json::Value) -> Result<String, String> {
    let sdir = session_dir();
    let latest_path = sdir.join("latest.json");

    if !latest_path.exists() {
        return Ok("ℹ️ No previous session context found. This is a clean slate.".to_string());
    }

    let content = tokio::fs::read_to_string(&latest_path)
        .await
        .map_err(|e| format!("Failed to read latest session: {}", e))?;

    let parsed: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse session JSON: {}", e))?;

    let title = parsed["title"].as_str().unwrap_or("Untitled Session");
    let summary = parsed["summary"].as_str().unwrap_or("No summary provided.");
    let next_steps = parsed["next_steps"].as_str().unwrap_or("None.");
    let timestamp = parsed["timestamp"].as_str().unwrap_or("Unknown time");
    let files = parsed["files_changed"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        })
        .unwrap_or_else(|| "None".to_string());

    Ok(format!(
        "🔄 BOOTSTRAPPED CONTEXT FROM PREVIOUS SESSION ({})\n\
         Title: {}\n\
         Summary: {}\n\
         Next Steps: {}\n\
         Files Changed: {}",
        timestamp, title, summary, next_steps, files
    ))
}
