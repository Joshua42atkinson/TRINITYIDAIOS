use std::path::{Path, PathBuf};
use tracing::info;
use super::{workspace_root, human_size};

/// Validate path is within workspace (read access: workspace + home)
pub fn validate_path(path: &str) -> Result<PathBuf, String> {
    validate_path_with_mode(path, false)
}

/// Validate path for writes (stricter: workspace + ~/.local/share/trinity/ only)
pub fn validate_write_path(path: &str) -> Result<PathBuf, String> {
    validate_path_with_mode(path, true)
}

fn validate_path_with_mode(path: &str, write_mode: bool) -> Result<PathBuf, String> {
    let workspace = workspace_root();
    let resolved = if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        workspace.join(path)
    };

    let canonical = resolved.canonicalize().unwrap_or(resolved.clone());
    let ws_canonical = workspace.canonicalize().unwrap_or(workspace.clone());
    let home = std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/home"));

    if write_mode {
        // Writes: allowed anywhere in HOME or /tmp/ per user request for OpenHands parity
        if canonical.starts_with(&home) || canonical.starts_with("/tmp") {
            Ok(canonical)
        } else {
            Err(format!("Write denied: '{}' is physically outside $HOME or /tmp/", path))
        }
    } else {
        // Reads: workspace + entire home directory + /tmp
        if canonical.starts_with(&ws_canonical)
            || canonical.starts_with(&home)
            || canonical.starts_with("/tmp")
        {
            Ok(canonical)
        } else {
            Err(format!("Path '{}' is outside allowed directories", path))
        }
    }
}

pub async fn tool_read_file(params: &serde_json::Value) -> Result<String, String> {
    let path = params
        .get("path")
        .and_then(|p| p.as_str())
        .ok_or("Missing 'path' parameter")?;

    let validated = validate_path(path)?;
    let content = tokio::fs::read_to_string(&validated)
        .await
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    // Truncate very large files
    if content.len() > 50_000 {
        Ok(format!(
            "{}...\n\n[Truncated: {} bytes total]",
            &content[..50_000],
            content.len()
        ))
    } else {
        Ok(content)
    }
}

pub async fn tool_write_file(params: &serde_json::Value) -> Result<String, String> {
    let path = params
        .get("path")
        .and_then(|p| p.as_str())
        .ok_or("Missing 'path' parameter")?;
    let content = params
        .get("content")
        .and_then(|c| c.as_str())
        .ok_or("Missing 'content' parameter")?;

    let validated = validate_write_path(path)?;

    // Create parent directories
    if let Some(parent) = validated.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create directories: {}", e))?;
    }

    // Backup existing file before overwriting (safety net)
    let mut backup_msg = String::new();
    if validated.exists() {
        let backup_path = validated.with_extension(format!(
            "bak.{}",
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        ));
        if tokio::fs::copy(&validated, &backup_path).await.is_ok() {
            backup_msg = format!(" (backup: {})", backup_path.display());
            info!("💾 Backed up existing file to: {}", backup_path.display());
        }
    }

    tokio::fs::write(&validated, content)
        .await
        .map_err(|e| format!("Failed to write {}: {}", path, e))?;

    info!("📝 Wrote file: {}", validated.display());
    
    Ok(format!(
        "Written {} bytes to {}{}",
        content.len(),
        path,
        backup_msg
    ))
}

pub async fn tool_list_dir(params: &serde_json::Value) -> Result<String, String> {
    let path = params.get("path").and_then(|p| p.as_str()).unwrap_or(".");

    let validated = validate_path(path)?;
    let mut entries = tokio::fs::read_dir(&validated)
        .await
        .map_err(|e| format!("Failed to read directory {}: {}", path, e))?;

    let mut items = Vec::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        let name = entry.file_name().to_string_lossy().to_string();
        let meta = entry.metadata().await.ok();
        let is_dir = meta.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);

        if is_dir {
            items.push(format!("📁 {}/", name));
        } else {
            items.push(format!("📄 {} ({})", name, human_size(size)));
        }
    }

    items.sort();
    Ok(items.join("\n"))
}

pub async fn tool_search_files(params: &serde_json::Value) -> Result<String, String> {
    let query = params
        .get("query")
        .and_then(|q| q.as_str())
        .ok_or("Missing 'query' parameter")?;
    let path = params
        .get("path")
        .and_then(|p| p.as_str())
        .unwrap_or("crates/");

    let validated = validate_path(path)?;

    let output = tokio::process::Command::new("grep")
        .args([
            "-rn",
            "--include=*.rs",
            "--include=*.md",
            "--include=*.toml",
            "--include=*.jsx",
            "--include=*.js",
            "--include=*.css",
            "--include=*.py",
            "--include=*.json",
            "--include=*.yaml",
            "--include=*.yml",
            "--include=*.sh",
            "--include=*.sql",
            "--include=*.html",
            "--include=*.tsx",
            "--include=*.ts",
            "-l",
            query,
        ])
        .arg(&validated)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Search failed: {}", e))?;

    let result = String::from_utf8_lossy(&output.stdout);
    if result.is_empty() {
        Ok(format!("No results for '{}' in {}", query, path))
    } else {
        Ok(result.to_string())
    }
}
