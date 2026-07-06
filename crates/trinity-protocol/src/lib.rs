//! ╔══════════════════════════════════════════════════════════════════════════════╗
//! ║                  THE TRINITY ID AI OS — "SIGN ON THE CAVE WALL"              ║
//! ╠══════════════════════════════════════════════════════════════════════════════╣
//! ║ ADDIECRAPEYE Phase Alignment:                                                ║
//! ║ 1. Analysis:   Socratic extraction (Ask Pete)            — The Eyes          ║
//! ║ 2. Design:     Bloom's Taxonomy & VAAM Definition         — The Brain         ║
//! ║ 3. Develop:    DAYDREAM Workshop (Code Generation)         — The Skeleton      ║
//! ║ 4. Implement:  Layer 3 Spatial Sandbox (The Iron Road)    — The Muscles       ║
//! ║ 5. Evaluate:   Quality Matters (QM) Alignment Check       — Nervous System    ║
//! ║ 6. Contrast:   Visual hierarchy & boundary design (CRAP)  — The Skin          ║
//! ║ 7. Repetition: Core loop solidification (CRAP)            — The Heart         ║
//! ║ 8. Alignment:  QM posture check & scope pruning (CRAP)    — The Spine         ║
//! ║ 9. Proximity:  UX optimization, Miller's Law (CRAP)       — The Hands         ║
//! ║ 10. Envision:  Meta-awareness, OS mindset (EYE)           — Third Eye         ║
//! ║ 11. Yoke:      Frontend-backend coupling (EYE)            — Connective Tissue ║
//! ║ 12. Evolve:    Deploy, autonomy, first breath (EYE)       — The Lungs         ║
//! ╚══════════════════════════════════════════════════════════════════════════════╝

// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-protocol
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        lib.rs
// PURPOSE:     Shared types and protocols — The Language of Trinity
//
// ARCHITECTURE:
//   • Core types: ChatMessage, Brain trait, Agent trait
//   • ADDIE phase definitions and state transitions
//   • CharacterSheet with IBSTPI competency domains
//   • MemoryBridge for context management
//   • QM Rubric for quality gates
//   • Stable API — don't break backward compatibility!
//
// DEPENDENCIES:
//   - serde — All types are serializable
//   - uuid — Unique identifiers
//   - chrono — Timestamps
//
// CHANGES:
//   2026-03-16  Cascade  Migrated to §17 comment standard
//
// ═══════════════════════════════════════════════════════════════════════════════

// Trinity AI Agent System
// Copyright (c) Joshua
// Shared under license for Ask_Pete (Purdue University)
//
// ═══════════════════════════════════════════════════════════════════════════════
// 📡 ZONE: PROTOCOL (Shared) | Context: /antigravity/CONTEXT.md
// ═══════════════════════════════════════════════════════════════════════════════
// VISION: Pure Rust • Type-Safe • Stable API (don't break compatibility!)
// Shared types used by Brain, Body, and all other zones.
// ═══════════════════════════════════════════════════════════════════════════════

//! # Trinity Protocol (The Language)
//!
//! ## Philosophy (Architectonics)
//! "Language is the substrate of thought. The Protocol defines the potential limits of
//!  communication between the Mind (Brain) and the Body. It must be expressive, type-safe,
//!  and future-proof."
//!
//! ## Instructions for Developers
//! 1. **Type Safety**: Use Rust enums to make illegal states unrepresentable (e.g., `AvatarState`).
//! 2. **Compatibility**: Changes here affect the whole ecosystem. Append, don't break.
//! 3. **Clarity**: Type names should be self-documenting (e.g., `ThinkResult`, `ActionRequest`).

pub mod artifact;
pub mod brain;
pub mod bridge;
pub mod diffusion;
pub mod id_contract;
pub mod memory;
pub mod ontology;
pub mod production;
pub mod sacred_circuitry;
pub mod sidecars;
pub mod state;
pub mod stream;
pub mod task;
pub mod pearl;
pub mod tutorial_events;
pub mod types;
pub mod vaam_profile;
pub mod vocabulary;

pub use artifact::{
    AgentMode, Artifact, GraphEdge, GraphNode, NodeStatus, PlanTask, StepItem, StepStatus,
};
pub use brain::BrainServiceClient;
pub use diffusion::*;
pub use id_contract::{ActionMap, ContractStatus, IdContract, LearningObjective, QuestMilestone};
pub use memory::MemoryServiceClient;
pub use ontology::MaterialIntegrity;
pub use pearl::{Pearl, PearlEvaluation, PearlMedium, PearlPhase};
pub use production::*;
pub use sacred_circuitry::{
    format_circuit_event, foundation_vocabulary, scan_ai_alignment, Circuit,
    CircuitAlignmentResult, CircuitQuadrant, CircuitryState,
};
pub use stream::{AgentConfig, AgentStatus, ModelTier, OrchestratorConfig, StreamEvent};
pub use task::*;
pub use types::*;
pub use vaam_profile::{Agreement, CommunicationStyle, VaamProfile, WordWeight};
pub use vocabulary::{
    MasteryUpdate, TierProgress, VocabularyDatabase, VocabularyMastery, VocabularyPack,
    VocabularySet, VocabularySuggestion, WordDetection,
};
