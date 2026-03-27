// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Protocol Layer
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:     stream.rs
// PURPOSE:  Streaming protocol types for agent SSE events and real-time chat delivery
// BIBLE:    Car 9 — PROXIMITY (UI Experience, §9.1)
//
// ═══════════════════════════════════════════════════════════════════════════════

// Trinity AI Agent System
// Copyright (c) Joshua
// Shared under license for Ask_Pete (Purdue University)

//! Streaming Protocol - Agent Event Types for RPC
//!
//! These types are used to stream agent events from Brain to Body
//! for the Antigravity Window visualization.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "bevy")]
use bevy::prelude::*;

use crate::artifact::Artifact;

/// Model tier for agent task routing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ModelTier {
    /// Fastest tier - small models for quick tasks (Gemma 2B, etc)
    Fast,
    /// Standard tier - balanced models for routine work (Llama 8B, etc)
    #[default]
    Standard,
    /// Powerful tier - large models for complex reasoning (Llama 70B, etc)
    Powerful,
    /// Reflection tier - the big model for deep thinking (Llama 4 Scout, Qwen 235B)
    Reflection,
}

/// Agent status for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatus {
    pub id: String,
    pub name: String,
    pub model_tier: ModelTier,
    pub is_busy: bool,
    pub current_task: Option<String>,
}

/// Events streamed from Brain to Body for Antigravity Window
#[cfg_attr(feature = "bevy", derive(Message))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamEvent {
    /// Agent started working on a task
    TaskStarted {
        agent_id: String,
        task_id: Uuid,
        task_name: String,
    },
    /// Agent is thinking/reasoning (stream thought tokens)
    Thinking { agent_id: String, thought: String },
    /// Agent completed a task
    TaskCompleted {
        agent_id: String,
        task_id: Uuid,
        result: String,
        duration_ms: u64,
    },
    /// Agent encountered an error
    TaskFailed {
        agent_id: String,
        task_id: Uuid,
        error: String,
    },
    /// Global orchestration status update
    OrchestrationStatus {
        active_agents: usize,
        queued_tasks: usize,
        completed_tasks: usize,
    },
    /// New artifact was created or updated
    ArtifactUpdate { artifact: Artifact },
    /// RAG hit information
    RagHit {
        title: String,
        content: String,
        similarity: f32,
        source: String,
    },
    /// Code generation update
    CodeGenerated {
        agent_id: String,
        file_path: String,
        code_snippet: String,
        line_count: usize,
    },
    /// Command execution started
    CommandRunning { agent_id: String, command: String },
    /// Command execution output
    CommandOutput {
        agent_id: String,
        stdout: String,
        stderr: String,
    },
    /// Agent status update
    AgentStatusUpdate { agents: Vec<AgentStatus> },
    /// Artifact generated
    ArtifactGenerated {
        agent_id: String,
        artifact: Artifact,
    },
    /// Mode changed
    ModeChanged {
        agent_id: String,
        mode: crate::artifact::AgentMode,
        reason: Option<String>,
    },
    /// VAAM vocabulary detection event
    VaamEvent {
        /// Detected words with coal earned
        detections: Vec<VaamDetection>,
        /// Total coal from this message
        total_coal: u32,
        /// Words that became mastered (Rule of Three)
        newly_mastered: Vec<String>,
        /// Session total coal
        session_total: u64,
    },
}

/// A detected vocabulary word from VAAM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaamDetection {
    /// The word detected
    pub word: String,
    /// Coal earned (0 if incorrect context)
    pub coal: u32,
    /// Whether context was correct
    pub correct: bool,
    /// Tier of the word
    pub tier: String,
}

/// Configuration for the orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    pub max_parallel_tasks: usize,
    pub default_model_tier: ModelTier,
    pub enabled_agents: Vec<String>,
}

/// Configuration for an individual agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: String,
    pub name: String,
    pub model_tier: ModelTier,
    pub system_prompt: String,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            max_parallel_tasks: 4,
            default_model_tier: ModelTier::Standard,
            enabled_agents: vec![
                "pete".to_string(),
                "aesthetics".to_string(),
                "research".to_string(),
                "tempo".to_string(),
            ],
        }
    }
}
