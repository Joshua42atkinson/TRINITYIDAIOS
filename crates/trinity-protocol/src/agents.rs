// Trinity ID AI OS - Agent Protocol Definitions
// Copyright (c) Joshua
// Shared under license for Ask_Pete (Purdue University)

//! Agent Protocol Definitions
//!
//! Defines the standardized agent types, tasks, and communication protocols
//! used across Trinity's multi-agent system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Specialized Agent Types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentType {
    /// PETE - Main instructional design coordinator
    Pete,
    /// Conductor - Advanced workflow orchestration with REAP-97B
    Conductor,
    /// Blueprint Reviewer - Quality control and accountability partner
    BlueprintReviewer,
    /// Draftsman - Creates content and materials
    Draftsman,
    /// Engineer - Handles technical implementation
    Engineer,
    /// Dispatcher - SME Interviewer and Agile Router
    Dispatcher,
    /// Omni - Visual QA, Draftsman, Media Analyst
    Omni,
}

impl AgentType {
    /// Get agent display name
    pub fn display_name(&self) -> &'static str {
        match self {
            AgentType::Pete => "PETE",
            AgentType::Conductor => "Conductor",
            AgentType::BlueprintReviewer => "Blueprint Reviewer",
            AgentType::Draftsman => "Draftsman",
            AgentType::Engineer => "Engineer",
            AgentType::Dispatcher => "Dispatcher",
            AgentType::Omni => "Omni-Sense",
        }
    }

    /// Get agent description
    pub fn description(&self) -> &'static str {
        match self {
            AgentType::Pete => "Professional Educational & Training Expert - Your ID guide",
            AgentType::Conductor => "Advanced orchestration with emergent REAP-97B reasoning",
            AgentType::BlueprintReviewer => {
                "Quality control and accountability partner for blueprints"
            }
            AgentType::Draftsman => "Creates learning content and materials",
            AgentType::Engineer => "Implements technical solutions and platforms",
            AgentType::Dispatcher => "Conducts SME interviews and manages UI state",
            AgentType::Omni => "Visual QA, Draftsman, and Media Analyst unified",
        }
    }

    /// Get agent emoji
    pub fn emoji(&self) -> &'static str {
        match self {
            AgentType::Pete => "🤖",
            AgentType::Conductor => "🎯",
            AgentType::BlueprintReviewer => "�",
            AgentType::Draftsman => "📝",
            AgentType::Engineer => "⚙️",
            AgentType::Dispatcher => "🔄",
            AgentType::Omni => "👁️",
        }
    }

    /// Get associated model for this agent
    pub fn model_name(&self) -> &'static str {
        match self {
            AgentType::Pete => "MiniMax-REAP-50",
            AgentType::Conductor => "Qwen3.5-REAP-97B",
            AgentType::BlueprintReviewer => "MiniMax-REAP-50", // Shared with PETE
            AgentType::Draftsman => "Qwen3.5-35B",
            AgentType::Engineer => "Qwen3.5-35B",
            AgentType::Dispatcher => "Qwen3.5-35B",
            AgentType::Omni => "Qwen2-VL-7B",
        }
    }

    /// Get VRAM requirement in GB (VERIFIED on Strix Halo)
    pub fn vram_requirement_gb(&self) -> u32 {
        match self {
            AgentType::Pete => 42,             // MiniMax-REAP-50: 41.2GB measured
            AgentType::Conductor => 50,        // Qwen3.5-REAP-97B: 50GB estimated
            AgentType::BlueprintReviewer => 0, // Shared with PETE
            AgentType::Draftsman => 13,        // Qwen3.5-35B: 12.8GB calculated
            AgentType::Engineer => 13,         // Qwen3.5-35B: 12.8GB calculated
            AgentType::Dispatcher => 13,       // Qwen3.5-35B: 12.8GB calculated
            AgentType::Omni => 8,              // Qwen2-VL-7B: 7.4GB calculated
        }
    }
}

/// Agent Status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentMemoryStatus {
    Idle,
    Active,
    Waiting,
    Completed,
    Error(String),
    Loading,  // Agent model is loading
    Unloaded, // Agent model is unloaded from memory
}

/// Agent Task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    pub id: String,
    pub agent_type: AgentType,
    pub description: String,
    pub status: AgentMemoryStatus,
    pub created_at: std::time::SystemTime,
    pub completed_at: Option<std::time::SystemTime>,
    pub parameters: HashMap<String, String>,
}

/// Agent Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    pub agent_type: AgentType,
    pub tasks_completed: u32,
    pub average_task_time_ms: u64,
    pub memory_usage_mb: u32,
    pub last_activity: std::time::SystemTime,
    pub error_count: u32,
}

/// Agent Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemoryConfig {
    pub agent_type: AgentType,
    pub enabled: bool,
    pub auto_load: bool,
    pub priority: u8, // 0 = highest priority
    pub max_concurrent_tasks: u8,
}

impl Default for AgentMemoryConfig {
    fn default() -> Self {
        Self {
            agent_type: AgentType::Pete,
            enabled: true,
            auto_load: true,
            priority: 5,
            max_concurrent_tasks: 1,
        }
    }
}

/// Agent System Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSystemConfig {
    pub agents: HashMap<AgentType, AgentMemoryConfig>,
    pub max_total_memory_gb: u32,
    pub memory_management_mode: MemoryManagementMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryManagementMode {
    Manual,    // User controls what's loaded
    Automatic, // System loads/unloads based on usage
    Priority,  // Load based on priority and available memory
}

impl Default for AgentSystemConfig {
    fn default() -> Self {
        let mut agents = HashMap::new();

        // Default agent configurations
        agents.insert(
            AgentType::Pete,
            AgentMemoryConfig {
                agent_type: AgentType::Pete,
                enabled: true,
                auto_load: true,
                priority: 1,
                max_concurrent_tasks: 3,
            },
        );

        agents.insert(
            AgentType::Conductor,
            AgentMemoryConfig {
                agent_type: AgentType::Conductor,
                enabled: true,
                auto_load: false,
                priority: 2,
                max_concurrent_tasks: 1,
            },
        );

        agents.insert(
            AgentType::BlueprintReviewer,
            AgentMemoryConfig {
                agent_type: AgentType::BlueprintReviewer,
                enabled: true,
                auto_load: false,
                priority: 4,
                max_concurrent_tasks: 2,
            },
        );

        agents.insert(
            AgentType::Dispatcher,
            AgentMemoryConfig {
                agent_type: AgentType::Dispatcher,
                enabled: true,
                auto_load: true,
                priority: 0, // Always loaded for UI responsiveness
                max_concurrent_tasks: 5,
            },
        );

        Self {
            agents,
            max_total_memory_gb: 96, // Strix Halo allocatable VRAM
            memory_management_mode: MemoryManagementMode::Priority,
        }
    }
}
