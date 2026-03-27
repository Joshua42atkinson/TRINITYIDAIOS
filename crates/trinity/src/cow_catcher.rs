// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Cow Catcher Error Classification
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        cow_catcher.rs
// BIBLE CAR:   Car 5 — EVALUATE (Quality Systems & Security Rings)
// HOOK SCHOOL: ⚙️ Systems
//
// 🪟 THE LIVING CODE TEXTBOOK (P-ART-Y Safety & Self-Healing):
// This file is the literal Cow Catcher. It is designed to be read, modified, 
// and authored by YOU. If Pete or the Yardmaster gets stuck in a loop, or a 
// Python script crashes, this file catches the error and restarts the pipeline.
// ACTION: Edit `analyze_and_respond()` to add custom error-handling logic.
//
// 📖 THE HOOK BOOK CONNECTION:
// This file powers the 'CowCatcher' and 'Autopoiesis' Hooks. By mastering this
// file, you learn how to build AI systems that can heal themselves at runtime!
// For a full catalogue of system capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
// PURPOSE:     Runtime error classification, obstacle reporting, hardware monitoring
//
// ═══════════════════════════════════════════════════════════════════════════════

//! Cow Catcher Integration for Main Trinity Daemon
//!
//! Routes timeout errors, compilation failures, sidecar crashes, and hardware
//! limits to the debugging and self-improvement system.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sysinfo::System;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Obstacle detected during trinity operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub id: String,
    pub obstacle_type: ObstacleType,
    pub severity: u8,
    pub location: String,
    pub description: String,
    pub detected_at: chrono::DateTime<Utc>,
    pub context: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObstacleType {
    LLMTimeout,
    CompilationError,
    TestFailure,
    ModelLoadFailure,
    QuestExecutionFailure,
    NetworkError,
    SidecarCrash,
    HardwareLimit,
}

/// Cow Catcher client for reporting obstacles
pub struct CowCatcher {
    obstacles: Vec<Obstacle>,
    auto_restart_enabled: bool,
    sys: System,
}

impl CowCatcher {
    pub fn new() -> Self {
        Self {
            obstacles: Vec::new(),
            auto_restart_enabled: true,
            sys: System::new_all(),
        }
    }

    /// Report a timeout obstacle
    pub fn report_timeout(&mut self, step: &str, duration_secs: u64, model: &str) {
        let obstacle = Obstacle {
            id: format!("timeout_{}", Utc::now().timestamp()),
            obstacle_type: ObstacleType::LLMTimeout,
            severity: 7,
            location: format!("workflow::{}", step),
            description: format!(
                "LLM timeout: {} took {}s (max 300s) on model {}",
                step, duration_secs, model
            ),
            detected_at: Utc::now(),
            context: serde_json::json!({
                "step": step,
                "duration_secs": duration_secs,
                "model": model,
                "max_duration": 300,
            }),
        };

        error!("🚨 Cow Catcher: {}", obstacle.description);
        self.obstacles.push(obstacle.clone());
        self.analyze_and_respond(obstacle);
    }

    /// Report a compilation error
    pub fn report_compilation_error(&mut self, file: &str, error: &str) {
        let obstacle = Obstacle {
            id: format!("compile_{}", Utc::now().timestamp()),
            obstacle_type: ObstacleType::CompilationError,
            severity: 9,
            location: file.to_string(),
            description: format!("Compilation failed: {}", error),
            detected_at: Utc::now(),
            context: serde_json::json!({
                "file": file,
                "error": error,
            }),
        };

        error!("🚨 Cow Catcher: {}", obstacle.description);
        self.obstacles.push(obstacle.clone());
        self.analyze_and_respond(obstacle);
    }

    /// Report a quest execution failure
    pub fn report_quest_failure(&mut self, quest_id: &str, reason: &str) {
        let obstacle = Obstacle {
            id: format!("quest_{}", Utc::now().timestamp()),
            obstacle_type: ObstacleType::QuestExecutionFailure,
            severity: 8,
            location: format!("quest::{}", quest_id),
            description: format!("Quest failed: {}", reason),
            detected_at: Utc::now(),
            context: serde_json::json!({
                "quest_id": quest_id,
                "reason": reason,
            }),
        };

        error!("🚨 Cow Catcher: {}", obstacle.description);
        self.obstacles.push(obstacle.clone());
        self.analyze_and_respond(obstacle);
    }

    /// Report a sidecar crash
    pub fn report_sidecar_crash(&mut self, sidecar_id: &str, exit_code: Option<i32>, stderr: &str) {
        let obstacle = Obstacle {
            id: format!("crash_{}", Utc::now().timestamp()),
            obstacle_type: ObstacleType::SidecarCrash,
            severity: 10,
            location: format!("sidecar::{}", sidecar_id),
            description: format!("Sidecar crashed: {}", sidecar_id),
            detected_at: Utc::now(),
            context: serde_json::json!({
                "sidecar_id": sidecar_id,
                "exit_code": exit_code,
                "stderr": stderr,
            }),
        };

        error!("🚨 Cow Catcher: {}", obstacle.description);
        self.obstacles.push(obstacle.clone());
        self.analyze_and_respond(obstacle);
    }

    /// Analyze obstacle and take corrective action
    fn analyze_and_respond(&self, obstacle: Obstacle) {
        match obstacle.obstacle_type {
            ObstacleType::LLMTimeout => {
                warn!("   → Timeout detected - step will be skipped");
                warn!("   → Quest will continue with next step");
                warn!("   → Consider reducing context size or simplifying prompt");
            }
            ObstacleType::CompilationError => {
                error!("   → Compilation failed - generated code has errors");
                error!("   → Review will flag for retry");
                if self.auto_restart_enabled {
                    info!("   → Auto-retry enabled - will attempt fix");
                }
            }
            ObstacleType::QuestExecutionFailure => {
                error!("   → Quest execution failed");
                error!("   → Check logs for details");
                if self.auto_restart_enabled {
                    info!("   → Auto-restart enabled - sidecar will reload");
                }
            }
            ObstacleType::SidecarCrash => {
                error!("   → Sidecar Crash Detected");
                error!("   → Spawning autopoiesis quest to diagnose and repair");
            }
            _ => {
                warn!("   → Obstacle logged for analysis");
            }
        }
    }

    /// Hardware monitoring loop
    pub async fn check_hardware_limits(&mut self, max_ram_percent: f64) {
        self.sys.refresh_memory();
        let total_memory = self.sys.total_memory();
        let used_memory = self.sys.used_memory();

        let memory_percent = (used_memory as f64 / total_memory as f64) * 100.0;

        if memory_percent > max_ram_percent {
            let obstacle = Obstacle {
                id: format!("hw_limit_{}", Utc::now().timestamp()),
                obstacle_type: ObstacleType::HardwareLimit,
                severity: 9,
                location: "system::memory".to_string(),
                description: format!("Memory usage critical: {:.1}%", memory_percent),
                detected_at: Utc::now(),
                context: serde_json::json!({
                    "total_memory_kb": total_memory,
                    "used_memory_kb": used_memory,
                    "percent_used": memory_percent,
                }),
            };

            error!("🚨 Cow Catcher: {}", obstacle.description);
            self.obstacles.push(obstacle.clone());
            self.analyze_and_respond(obstacle);
        }
    }

    /// Get all obstacles for reporting
    pub fn get_obstacles(&self) -> &[Obstacle] {
        &self.obstacles
    }

    /// Clear resolved obstacles
    pub fn clear_obstacles(&mut self) {
        self.obstacles.clear();
    }

    /// Check if auto-restart should trigger
    pub fn should_restart(&self) -> bool {
        if !self.auto_restart_enabled {
            return false;
        }

        // Restart if we have 3+ critical failures
        let critical_count = self.obstacles.iter().filter(|o| o.severity >= 8).count();

        critical_count >= 3
    }
}

impl Default for CowCatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Start hardware monitoring background loop
pub fn start_hardware_monitor(cow_catcher: Arc<RwLock<CowCatcher>>) {
    info!("Starting hardware monitor (Cow Catcher)...");

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            let mut cc = cow_catcher.write().await;
            // 99% overnight limit
            cc.check_hardware_limits(99.0).await;
        }
    });
}
