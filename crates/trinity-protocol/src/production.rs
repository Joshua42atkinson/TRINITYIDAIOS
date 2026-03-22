// Trinity Production Pipeline Types
// Shared types for NPC generation, Roblox export, and asset production.

use crate::SubagentRole;
use serde::{Deserialize, Serialize};

/// Current phase of the production pipeline
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProductionPhase {
    ConceptGeneration, // AI concept generation
    VisualDesign,      // Draftsman creates visual design
    AssetModeling,     // Engineer creates 3D models
    QualityReview,     // Yardmaster reviews quality
    FinalAssembly,     // Dispatcher assembles final product
    RobloxExport,      // Export to Roblox format
    Completed,         // Project finished
}

/// Project status for tracking production lifecycle
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProjectStatus {
    Draft,
    InProduction,
    Paused,
    Review,
    Approved,
    Exported,
    Published,
    Archived,
}

/// Result of a single production step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepResult {
    Pending,
    InProgress,
    Success(String),
    Warning(String),
    Error(String),
}

/// A step in the production history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionStep {
    pub phase: ProductionPhase,
    pub subagent: SubagentRole,
    pub result: StepResult,
}

/// A project for NPC generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcProject {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub production_history: Vec<ProductionStep>,
}

/// NPC Projects Collection
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Resource))]
pub struct NpcProjects {
    pub projects: Vec<NpcProject>,
    pub selected_project: Option<String>,
}

/// Request for NPC production
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcProductionRequest {
    pub id: String,
    pub user_description: String,
    pub target_audience: String,
    pub educational_context: Option<String>,
    pub visual_style: VisualStyle,
    pub personality_traits: Vec<String>,
    pub special_features: Vec<String>,
    pub priority: ProductionPriority,
    pub requested_by: String,
}

/// Priority for production tasks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProductionPriority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Visual style for generated NPCs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum VisualStyle {
    #[default]
    RobloxClassic,
    ModernSmooth,
    CartoonStylized,
    RealisticDetail,
    SciFiFuturistic,
    FantasyMedieval,
    CyberpunkNeon,
    SteampunkVictorian,
    MinimalistClean,
    ArtisticAbstract,
}
