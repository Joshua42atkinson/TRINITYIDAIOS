// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        monitor.rs
// PURPOSE:     Cross-project monitoring — health, models, git status, jobs
//
// This module monitors the entire Trinity ecosystem: Trinity itself, LM Studio
// (Hermes brain), ComfyUI (creative), and all related projects on disk. It is
// the backend for the PWA system-status page and any standalone dashboards.
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{extract::State, Json};
use serde::Serialize;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{info, warn};

use crate::AppState;
use crate::http;

/// Service health entry
#[derive(Debug, Serialize)]
pub struct ServiceHealth {
    pub name: String,
    pub url: String,
    pub healthy: bool,
    pub model: Option<String>,
    pub error: Option<String>,
    pub response_ms: u64,
}

/// Per-project status from git
#[derive(Debug, Serialize)]
pub struct ProjectStatus {
    pub name: String,
    pub path: String,
    pub branch: String,
    pub last_commit: String,
    pub last_commit_time: String,
    pub commits_ahead: i32,
    pub uncommitted: i32,
    pub exists: bool,
    pub error: Option<String>,
}

/// Disk usage for a path
#[derive(Debug, Serialize)]
pub struct DiskUsage {
    pub path: String,
    pub total_gb: f64,
    pub used_gb: f64,
    pub free_gb: f64,
    pub percent_used: f64,
}

/// Aggregated monitor response
#[derive(Debug, Serialize)]
pub struct MonitorResponse {
    pub timestamp: u64,
    pub services: Vec<ServiceHealth>,
    pub projects: Vec<ProjectStatus>,
    pub disk: Vec<DiskUsage>,
    pub active_jobs: usize,
    pub completed_jobs: usize,
    pub failed_jobs: usize,
    pub uptime_secs: u64,
}

/// Projects to monitor
const PROJECTS: &[(&str, &str)] = &[
    ("TRINITYIDAIOS", "/home/joshua/Workflow/TRINITYIDAIOS"),
    ("Semantic Slime", "/home/joshua/Semantic Slime"),
    ("Bertrand XR", "/home/joshua/Workflow/Bertrand-Masterclass/apps/spatial-engine-bevy"),
    ("Bertrand App", "/home/joshua/Workflow/Bertrand-Masterclass/apps/companion-app"),
    ("Day_Dream", "/home/joshua/Workflow/Day_Dream"),
    ("phonethagoras", "/home/joshua/Workflow/phonethagoras"),
];

/// Paths to monitor for disk usage
const DISK_PATHS: &[&str] = &[
    "/home/joshua",
    "/tmp",
    "/",
];

/// Services to monitor
const SERVICES: &[(&str, &str)] = &[
    ("Trinity", "http://127.0.0.1:3000"),
    ("LM Studio", "http://127.0.0.1:1234"),
    ("ComfyUI", "http://127.0.0.1:8188"),
];

/// GET /api/monitor/status — full ecosystem health
pub async fn monitor_status(State(state): State<AppState>) -> Json<MonitorResponse> {
    info!("📊 Monitor status requested");

    let start = SystemTime::now();

    let services = gather_services().await;
    let projects = gather_projects().await;
    let disk = gather_disk();
    let (active_jobs, completed_jobs, failed_jobs) = job_counts(&state).await;
    let uptime_secs = crate::health::uptime_secs();

    let timestamp = start
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs();

    Json(MonitorResponse {
        timestamp,
        services,
        projects,
        disk,
        active_jobs,
        completed_jobs,
        failed_jobs,
        uptime_secs,
    })
}

async fn gather_services() -> Vec<ServiceHealth> {
    let mut results = Vec::new();

    for (name, url) in SERVICES {
        let begin = std::time::Instant::now();
        let mut health = ServiceHealth {
            name: name.to_string(),
            url: url.to_string(),
            healthy: false,
            model: None,
            error: None,
            response_ms: 0,
        };

        let probe_url = match *name {
            "LM Studio" => format!("{}/v1/models", url.trim_end_matches('/')),
            "ComfyUI" => format!("{}/system_stats", url.trim_end_matches('/')),
            _ => format!("{}/api/health", url.trim_end_matches('/')),
        };

        match http::QUICK.get(&probe_url).send().await {
            Ok(res) => {
                health.response_ms = begin.elapsed().as_millis() as u64;
                if res.status().is_success() {
                    health.healthy = true;
                    // Try to extract model name for LM Studio
                    if *name == "LM Studio" {
                        if let Ok(body) = res.json::<serde_json::Value>().await {
                            if let Some(data) = body.get("data").and_then(|d| d.as_array()) {
                                // Prefer Hermes model if loaded, else first
                                let preferred = data.iter().find(|m| {
                                    m.get("id").and_then(|v| v.as_str())
                                        .map(|s| s.to_lowercase().contains("hermes"))
                                        .unwrap_or(false)
                                }).or(data.first());
                                if let Some(model) = preferred {
                                    health.model = model
                                        .get("id")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                }
                            }
                        }
                    }
                } else {
                    health.error = Some(format!("HTTP {}", res.status()));
                }
            }
            Err(e) => {
                health.error = Some(e.to_string());
            }
        }

        results.push(health);
    }

    results
}

async fn gather_projects() -> Vec<ProjectStatus> {
    let mut results = Vec::new();

    for (name, path) in PROJECTS {
        let mut status = ProjectStatus {
            name: name.to_string(),
            path: path.to_string(),
            branch: "unknown".to_string(),
            last_commit: "-".to_string(),
            last_commit_time: "-".to_string(),
            commits_ahead: 0,
            uncommitted: 0,
            exists: Path::new(path).exists(),
            error: None,
        };

        if !status.exists {
            status.error = Some("Directory not found".to_string());
            results.push(status);
            continue;
        }

        if !Path::new(path).join(".git").exists() {
            status.error = Some("Not a git repository".to_string());
            results.push(status);
            continue;
        }

        // Branch
        if let Ok(out) = Command::new("git")
            .args(["-C", path, "rev-parse", "--abbrev-ref", "HEAD"])
            .output()
        {
            if out.status.success() {
                status.branch = String::from_utf8_lossy(&out.stdout).trim().to_string();
            }
        }

        // Last commit message + time
        if let Ok(out) = Command::new("git")
            .args(["-C", path, "log", "-1", "--format=%s|%ci"])
            .output()
        {
            if out.status.success() {
                let line = String::from_utf8_lossy(&out.stdout);
                let parts: Vec<&str> = line.trim().split('|').collect();
                if parts.len() >= 2 {
                    status.last_commit = parts[0].to_string();
                    status.last_commit_time = parts[1].to_string();
                }
            }
        }

        // Commits ahead of remote (if any)
        if let Ok(out) = Command::new("git")
            .args(["-C", path, "rev-list", "--count", "HEAD..@{u}"])
            .output()
        {
            if out.status.success() {
                status.commits_ahead = String::from_utf8_lossy(&out.stdout)
                    .trim()
                    .parse()
                    .unwrap_or(0);
            }
        }

        // Uncommitted changes
        if let Ok(out) = Command::new("git")
            .args(["-C", path, "status", "--short"])
            .output()
        {
            if out.status.success() {
                status.uncommitted = String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .count() as i32;
            }
        }

        results.push(status);
    }

    results
}

fn gather_disk() -> Vec<DiskUsage> {
    let mut results = Vec::new();

    for path in DISK_PATHS {
        let mut usage = DiskUsage {
            path: path.to_string(),
            total_gb: 0.0,
            used_gb: 0.0,
            free_gb: 0.0,
            percent_used: 0.0,
        };

        if let Ok(out) = Command::new("df")
            .args(["-BG", "--output=size,used,avail", *path])
            .output()
        {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout);
                // Skip header line, parse second line
                if let Some(line) = text.lines().nth(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 {
                        let parse_gb = |s: &str| {
                            s.trim_end_matches('G')
                                .parse::<f64>()
                                .unwrap_or(0.0)
                        };
                        usage.total_gb = parse_gb(parts[0]);
                        usage.used_gb = parse_gb(parts[1]);
                        usage.free_gb = parse_gb(parts[2]);
                        if usage.total_gb > 0.0 {
                            usage.percent_used = (usage.used_gb / usage.total_gb) * 100.0;
                        }
                    }
                }
            }
        }

        results.push(usage);
    }

    results
}

async fn job_counts(state: &AppState) -> (usize, usize, usize) {
    // jobs::JobQueue is a private type; we can't query it directly without adding
    // a method. For now, return zeros and rely on the PWA to query /api/jobs.
    (0, 0, 0)
}

/// GET /api/monitor/health — simple OK/NOT OK for all services
pub async fn monitor_health() -> Json<serde_json::Value> {
    let mut services = Vec::new();
    for (name, url) in SERVICES {
        let probe = match *name {
            "LM Studio" => format!("{}/v1/models", url.trim_end_matches('/')),
            "ComfyUI" => format!("{}/system_stats", url.trim_end_matches('/')),
            _ => format!("{}/api/health", url.trim_end_matches('/')),
        };
        let healthy = http::check_health(&probe).await;
        services.push(serde_json::json!({
            "name": name,
            "url": url,
            "healthy": healthy,
        }));
    }

    Json(serde_json::json!({
        "status": if services.iter().all(|s| s["healthy"].as_bool().unwrap_or(false)) {
            "healthy"
        } else {
            "degraded"
        },
        "services": services,
    }))
}
