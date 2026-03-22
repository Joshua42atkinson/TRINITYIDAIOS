//! Cow Catcher Integration for Sidecar
//!
//! Routes timeout errors, compilation failures, and other obstacles
//! to the debugging and self-improvement system.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

/// Obstacle detected during sidecar operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarObstacle {
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
}

/// Cow Catcher client for reporting obstacles
pub struct CowCatcher {
    obstacles: Vec<SidecarObstacle>,
    auto_restart_enabled: bool,
}

impl CowCatcher {
    pub fn new() -> Self {
        Self {
            obstacles: Vec::new(),
            auto_restart_enabled: true,
        }
    }

    /// Report a timeout obstacle
    pub fn report_timeout(&mut self, step: &str, duration_secs: u64, model: &str) {
        let obstacle = SidecarObstacle {
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
        let obstacle = SidecarObstacle {
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
        let obstacle = SidecarObstacle {
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

    /// Analyze obstacle and take corrective action
    fn analyze_and_respond(&self, obstacle: SidecarObstacle) {
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
            _ => {
                warn!("   → Obstacle logged for analysis");
            }
        }
    }

    /// Get all obstacles for reporting
    #[allow(dead_code)] // Exposed via /api/health diagnostics in future
    pub fn get_obstacles(&self) -> &[SidecarObstacle] {
        &self.obstacles
    }

    /// Clear resolved obstacles
    #[allow(dead_code)] // Called when obstacles are resolved via UI
    pub fn clear_obstacles(&mut self) {
        self.obstacles.clear();
    }

    /// Check if auto-restart should trigger
    #[allow(dead_code)] // Used by auto-restart policy when critical threshold exceeded
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
