use axum::Json;
use serde_json::Value;

/// GET /api/focus — Current focus mode status
pub async fn focus_status_endpoint() -> Json<Value> {
    use sysinfo::System;
    let mut sys = System::new_all();
    sys.refresh_processes();

    let windsurf_running = sys.processes().values().any(|p| {
        p.name().to_lowercase().contains("windsurf")
    });
    let antigravity_running = sys.processes().values().any(|p| {
        p.name().to_lowercase().contains("antigravity")
    });
    let comfyui_running = sys.processes().values().any(|p| {
        let cmd = p.cmd().iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
        cmd.contains("ComfyUI") && cmd.contains("main.py")
    });
    let trinity_running = sys.processes().values().any(|p| {
        let cmd = p.cmd().iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
        cmd.contains("trinity") && cmd.contains("release")
    });

    let p_online = reqwest::get("http://localhost:8000/v1/models").await.is_ok();
    let a_online = reqwest::get("http://localhost:8188/system_stats").await.is_ok();

    let mode = if !windsurf_running && !antigravity_running && (p_online || a_online) {
        "creative"
    } else if (windsurf_running || antigravity_running) && !p_online && !a_online {
        "code"
    } else if !windsurf_running && !antigravity_running && !p_online && !a_online {
        "night"
    } else {
        "mixed"
    };

    Json(serde_json::json!({
        "mode": mode,
        "processes": {
            "windsurf": windsurf_running,
            "antigravity": antigravity_running,
            "comfyui": comfyui_running,
            "trinity": trinity_running,
            "p_diffusiongemma": p_online,
            "a_comfyui": a_online,
        },
        "message": match mode {
            "creative" => "Creative Focus — Trinity models running, IDEs off",
            "code" => "Code Focus — IDEs running, Trinity models off",
            "night" => "Night Shift — all heavy processes off",
            _ => "Mixed mode — some processes from multiple modes running",
        }
    }))
}

/// POST /api/focus/creative — Kill IDEs, ensure Trinity models are running
pub async fn focus_creative_endpoint() -> Json<Value> {
    use tokio::process::Command;
    let mut killed = Vec::new();

    // Kill IDEs
    for (name, pattern) in [("windsurf", "windsurf"), ("antigravity", "antigravity"), ("language_server", "language_server_linux")] {
        let result = Command::new("pkill")
            .args(["-f", pattern])
            .output()
            .await;
        if let Ok(out) = result {
            if out.status.success() {
                killed.push(format!("killed_{}", name));
            }
        }
    }

    // Ensure hotel is in studio mode
    let hotel_result = crate::hotel_manager::hotel_studio_start().await;
    let hotel_ok = hotel_result.iter().filter(|r| r.success).count();

    Json(serde_json::json!({
        "status": "creative",
        "killed": killed,
        "hotel_residents_online": hotel_ok,
        "message": "Creative Focus — IDEs killed, Trinity P+A running. Full VRAM for creative pipeline."
    }))
}

/// POST /api/focus/code — Kill Trinity models, free VRAM for IDE agents
pub async fn focus_code_endpoint() -> Json<Value> {
    use tokio::process::Command;
    let mut killed = Vec::new();

    // Kill ComfyUI
    let result = Command::new("pkill")
        .args(["-f", "ComfyUI/venv/bin/python main.py"])
        .output()
        .await;
    if let Ok(out) = result {
        if out.status.success() {
            killed.push("comfyui");
        }
    }

    // Kill DiffusionGemma via podman
    let result = Command::new("podman")
        .args(["stop", "-t", "5"])
        .output()
        .await;
    if let Ok(out) = result {
        if out.status.success() {
            killed.push("diffusiongemma_container");
        }
    }

    // Kill any vllm processes
    let result = Command::new("pkill")
        .args(["-f", "vllm"])
        .output()
        .await;
    if let Ok(out) = result {
        if out.status.success() {
            killed.push("vllm");
        }
    }

    // Close hotel
    crate::hotel_manager::hotel_close_all().await;

    Json(serde_json::json!({
        "status": "code",
        "killed": killed,
        "message": "Code Focus — Trinity models killed, VRAM freed for IDE agents."
    }))
}

/// POST /api/focus/night — Kill everything heavy
pub async fn focus_night_endpoint() -> Json<Value> {
    use tokio::process::Command;
    let mut killed = Vec::new();

    // Kill IDEs
    for (name, pattern) in [("windsurf", "windsurf"), ("antigravity", "antigravity"), ("language_server", "language_server_linux"), ("blender", "blender"), ("godot", "godot")] {
        let result = Command::new("pkill")
            .args(["-f", pattern])
            .output()
            .await;
        if let Ok(out) = result {
            if out.status.success() {
                killed.push(name);
            }
        }
    }

    // Kill ComfyUI
    let result = Command::new("pkill")
        .args(["-f", "ComfyUI/venv/bin/python main.py"])
        .output()
        .await;
    if let Ok(out) = result {
        if out.status.success() {
            killed.push("comfyui");
        }
    }

    // Kill DiffusionGemma
    let result = Command::new("podman")
        .args(["stop", "-t", "5"])
        .output()
        .await;
    if let Ok(out) = result {
        if out.status.success() {
            killed.push("diffusiongemma");
        }
    }

    let result = Command::new("pkill")
        .args(["-f", "vllm"])
        .output()
        .await;
    if let Ok(out) = result {
        if out.status.success() {
            killed.push("vllm");
        }
    }

    // Close hotel
    crate::hotel_manager::hotel_close_all().await;

    Json(serde_json::json!({
        "status": "night",
        "killed": killed,
        "message": "Night Shift — all heavy processes killed. System ready for maintenance or rest."
    }))
}
