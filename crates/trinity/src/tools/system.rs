use std::process::Stdio;
use tokio::process::Command;
use tracing::info;
use super::workspace_root;

pub fn human_size(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{} B", bytes);
    }
    if bytes < 1024 * 1024 {
        return format!("{:.1} KB", bytes as f64 / 1024.0);
    }
    if bytes < 1024 * 1024 * 1024 {
        return format!("{:.1} MB", bytes as f64 / 1_048_576.0);
    }
    format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
}

pub async fn tool_shell(params: &serde_json::Value) -> Result<String, String> {
    let command = params
        .get("command")
        .and_then(|c| c.as_str())
        .ok_or("Missing 'command' parameter")?;
    let cwd = params.get("cwd").and_then(|c| c.as_str()).unwrap_or(".");
    let dry_run = params.get("dry_run").and_then(|d| d.as_bool()).unwrap_or(false);

    // Simple shell sandboxing / validation
    let unsafe_keywords = [
        "rm -rf /", "sudo", "dd ", "mkfs", "chown", "chmod 777",
        ":(){:|:&};:", "forkbomb",
    ];
    for kw in &unsafe_keywords {
        if command.contains(kw) {
            return Err(format!(
                "🚨 Blocked command containing forbidden pattern '{}'",
                kw
            ));
        }
    }

    if dry_run {
        return Ok(format!(
            "🛠️ [DRY RUN] Would execute command '{}' in directory '{}'",
            command, cwd
        ));
    }

    info!("🐚 Executing command: '{}' in '{}'", command, cwd);

    // Set a 60s timeout for safety
    let run_cmd = match tokio::time::timeout(
        std::time::Duration::from_secs(60),
        Command::new("bash")
            .arg("-c")
            .arg(command)
            .current_dir(cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output(),
    )
    .await
    {
        Ok(Ok(output)) => output,
        Ok(Err(e)) => return Err(format!("Command failed to start: {}", e)),
        Err(_) => {
            return Err("🚨 Command timed out after 60 seconds (Ring 5 safety limit).".to_string())
        }
    };

    let stdout = String::from_utf8_lossy(&run_cmd.stdout);
    let stderr = String::from_utf8_lossy(&run_cmd.stderr);

    let mut result = String::new();
    if !stdout.is_empty() {
        result.push_str(&stdout);
    }
    if !stderr.is_empty() {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(&stderr);
    }

    if run_cmd.status.success() {
        Ok(result)
    } else {
        Err(format!(
            "Command failed (exit {}):\n{}",
            run_cmd.status.code().unwrap_or(-1),
            result
        ))
    }
}

pub async fn tool_system_info() -> Result<String, String> {
    let mut info_list = Vec::new();

    // Memory
    if let Ok(out) = Command::new("free")
        .arg("-h")
        .stdout(Stdio::piped())
        .output()
        .await
    {
        info_list.push("=== MEMORY ===".to_string());
        info_list.push(String::from_utf8_lossy(&out.stdout).to_string());
    }

    // Disk
    if let Ok(out) = Command::new("df")
        .args(["-h", "/home"])
        .stdout(Stdio::piped())
        .output()
        .await
    {
        info_list.push("=== DISK ===".to_string());
        info_list.push(String::from_utf8_lossy(&out.stdout).to_string());
    }

    // GPU
    if let Ok(out) = Command::new("bash")
        .arg("-c")
        .arg("cat /sys/class/drm/card*/device/gpu_busy_percent 2>/dev/null || echo 'N/A'")
        .stdout(Stdio::piped())
        .output()
        .await
    {
        info_list.push(format!(
            "=== GPU BUSY === {}%",
            String::from_utf8_lossy(&out.stdout).trim()
        ));
    }

    // Key services
    let services = [
        ("vllm", "Great Recycler (LLM brain)"),
        ("trinity", "SQLite (trinity_memory.db)"),
        ("trinity_voice", "Voice server (Kokoro TTS)"),
    ];
    info_list.push("=== SERVICES ===".to_string());
    for (proc_name, label) in &services {
        let running = Command::new("pgrep")
            .arg("-f")
            .arg(proc_name)
            .stdout(Stdio::piped())
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false);
        info_list.push(format!(
            "{}: {}",
            label,
            if running {
                "✅ running"
            } else {
                "❌ stopped"
            }
        ));
    }

    // Uptime + load
    if let Ok(out) = Command::new("uptime").stdout(Stdio::piped()).output().await {
        info_list.push(format!(
            "=== UPTIME === {}",
            String::from_utf8_lossy(&out.stdout).trim()
        ));
    }

    Ok(info_list.join("\n"))
}

pub async fn tool_sidecar_status() -> Result<String, String> {
    let mut status = Vec::new();

    let llm_ok = crate::inference::check_health("http://127.0.0.1:1234").await;
    status.push(format!(
        "Signal Tower / LM Studio (port 1234 — local inference): {}",
        if llm_ok { "✅ running" } else { "❌ stopped" }
    ));

    let arty_ok = crate::inference::check_health("http://127.0.0.1:8000").await;
    status.push(format!(
        "A.R.T.Y. Hub (port 8000 — vLLM reverse proxy): {}",
        if arty_ok { "✅ running" } else { "⬚ not started" }
    ));

    let embed_ok = crate::inference::check_health("http://127.0.0.1:8005").await;
    status.push(format!(
        "  R (Research): nomic-embed (port 8005 — embeddings for RAG): {}",
        if embed_ok { "✅ running" } else { "⬚ not started" }
    ));

    status.push(
        "Active Models: Gemma-4-E4B-AWQ (TRINITY/Recycler), nomic-embed-text-v1.5 (RAG)".to_string()
    );

    Ok(status.join("\n"))
}

pub async fn tool_cargo_check(params: &serde_json::Value) -> Result<String, String> {
    let crate_name = params
        .get("crate_name")
        .and_then(|c| c.as_str())
        .unwrap_or("trinity");

    // Pre-build zombie guard: kill orphan rustc/cc that hold cargo lock
    let zombies_killed = {
        let mut killed = 0u32;
        for pattern in &["rustc", "cc -cc1"] {
            if let Ok(output) = Command::new("pgrep")
                .args(["-f", pattern])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await
            {
                let pids = String::from_utf8_lossy(&output.stdout);
                for pid_str in pids.lines() {
                    if let Ok(_pid) = pid_str.trim().parse::<u32>() {
                        let _ = Command::new("kill")
                            .args(["-9", pid_str.trim()])
                            .output()
                            .await;
                        killed += 1;
                    }
                }
            }
        }
        killed
    };
    if zombies_killed > 0 {
        info!(
            "🧟 Pre-build guard: killed {} zombie process(es)",
            zombies_killed
        );
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    info!("🔨 Cargo check: -p {}", crate_name);

    let output = match tokio::time::timeout(
        std::time::Duration::from_secs(120),
        Command::new("cargo")
            .args(["check", "-p", crate_name, "--message-format=short"])
            .current_dir(workspace_root())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output(),
    )
    .await
    {
        Ok(Ok(output)) => output,
        Ok(Err(e)) => return Err(format!("cargo check failed to start: {}", e)),
        Err(_) => {
            return Err(format!(
                "🚨 cargo check timed out after 120s for crate '{}'",
                crate_name
            ))
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut result = String::new();
    if !stdout.is_empty() {
        result.push_str(&stdout);
    }
    if !stderr.is_empty() {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(&stderr);
    }

    // Truncate long output
    if result.len() > 8_000 {
        result = format!(
            "{}...\n\n[Truncated: {} bytes total]",
            &result[..8_000],
            result.len()
        );
    }

    if output.status.success() {
        Ok(format!(
            "✅ cargo check -p {} passed!\n\n{}",
            crate_name, result
        ))
    } else {
        Err(format!(
            "❌ cargo check -p {} FAILED (exit {}):\n\n{}",
            crate_name,
            output.status.code().unwrap_or(-1),
            result
        ))
    }
}
