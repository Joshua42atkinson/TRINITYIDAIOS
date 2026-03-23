// Trinity Application States
// Definitions for the state machine used by all Bevy workspace nodes
//
// ═══════════════════════════════════════════════════════════════════════════════
// 🎮 THE AWAKENING: Pre-Journey Preparation
// ═══════════════════════════════════════════════════════════════════════════════
// The Awakening is the Isekai onboarding sequence that prepares the player
// for the Iron Road journey. These 8 phases occur BEFORE the player boards
// the train to the 12 Stations of Manifestation.
//
// The Iron Road itself is a separate state machine (IronRoadStation enum)
// representing the 12 train stops where the player constructs their Golem.
//
// AWAKENING PHASES → IRON ROAD STATIONS (thematic preparation):
//   SelectGenre → Station 1 (Analyze/Eyes) — Choosing the visual/cognitive lens
//   CoCreateVocab → Station 2 (Design/Brain) — Neural pathways for VAAM
//   ConfigureParty → Station 3 (Develop/Skeleton) — Structural bones of the party
//   QuestBriefing → [BOARD THE TRAIN] → Station 1 (Analyze)
//
// See TRINITY_TECHNICAL_BIBLE.md Section 14A for the full 12 Stations framework.

#[cfg(feature = "bevy")]
use bevy::prelude::*;
#[cfg(feature = "bevy")]
use std::hash::Hash;

// Workaround for Bevy 0.18 - States and SubStates not available in prelude
// Using custom derive macros instead
#[cfg(feature = "bevy")]
/// Top-level application state. This drives which Bevy systems and UI panels
/// are active. All rendering and input handling is scheduled per-state.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    /// First-run Isekai onboarding. The user has not yet set up their
    /// Character Sheet or connected to LM Studio. This is the complete
    /// "Phase 1 → Phase 4" Awakening sequence.
    #[default]
    Awakening,

    /// The main Iron Road loop. The HUD is active, quests are running,
    /// Pete provides contextual commentary. This is normal operation.
    IronRoad,

    /// The user is in the Trinity Workshop, designing and testing
    /// educational content. This is the creative/authoring mode.
    Workshop,

    /// The user is packaging a completed quest as a deployable Crate.
    /// Deferred to Sprint 3.
    CrateExport,
}

#[cfg(feature = "bevy")]
#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[source(AppState = AppState::Awakening)]
pub enum AwakeningPhase {
    /// Phase 1: The user lands in a half-built wireframe 3D world.
    /// Pete has not yet spawned. Ambient audio plays. The world is dark
    /// and atmospheric — deliberately incomplete.
    ///
    /// PRE-JOURNEY: The void before the awakening.
    #[default]
    TheVoid,

    /// Phase 2: Pete the Conductor (humanoid NPC) walks toward the camera.
    /// He speaks his opening lines via TTS. This phase ends when Pete's
    /// opening monologue is complete.
    ///
    /// PRE-JOURNEY: The call to adventure begins.
    PeteSpawns,

    /// Phase 3: Pete guides the user to install and connect LM Studio.
    /// An egui panel shows connection status. The world remains dim until
    /// the connection is established.
    ///
    /// PRE-JOURNEY: Establishing the neural link to the AI.
    ConnectLlm,

    /// Phase 4: The user creates their Character Sheet. This is the
    /// Isekai protagonist creation, with educational context.
    /// Hardware stats are scanned and applied.
    ///
    /// PRE-JOURNEY: Scanning the vessel (hardware) for the journey ahead.
    /// Maps to ADDIE Analysis — understanding the learner's context.
    CreateCharacter,

    /// Phase 5: User selects narrative genre (Cyberpunk, Steampunk, etc).
    /// Determines vocabulary themes and visual style.
    ///
    /// THEMATIC PREP for Station 1 (Analyze/Eyes):
    /// The genre is the lens through which the Golem will perceive the world.
    /// Choosing Cyberpunk = seeing the world through neon and circuits.
    /// Choosing Steampunk = seeing through brass and steam.
    /// This prepares the Eyes of the Golem before boarding the train.
    SelectGenre,

    /// Phase 6: User co-creates vocabulary with AI assistance.
    /// User describes goals, AI suggests words, user accepts/rejects.
    ///
    /// THEMATIC PREP for Station 2 (Design/Brain):
    /// Vocabulary = neural pathways for VAAM (Vocabulary Acquisition And Mastery).
    /// Each accepted word becomes a synapse in the Golem's Brain.
    /// Coal is earned by using vocabulary — this sets up the mining operation.
    CoCreateVocab,

    /// Phase 7: Configure AI party based on hardware.
    /// Auto-config with manual override option.
    ///
    /// THEMATIC PREP for Station 3 (Develop/Skeleton):
    /// Party roles = structural bones of the development process.
    /// Memory budget = structural integrity of the Skeleton.
    /// The Engineer (code), Artist (visuals), Evaluator (quality),
    /// Brakeman (safety), Visionary (vision) form the vertebral column.
    ConfigureParty,

    /// Phase 8: The world brightens fully. Pete gives the quest briefing.
    /// The user receives their first educational quest. The onboarding
    /// sequence is complete.
    ///
    /// TRANSITION: [BOARD THE TRAIN]
    /// The player is ready to begin the Iron Road journey.
    /// Next stop: Station 1 (Analyze) at Junkyard Peak.
    QuestBriefing,
}

#[cfg(feature = "bevy")]
#[derive(Resource, Debug, Default, Clone)]
pub struct ActiveCharacterSheet(pub Option<crate::character_sheet::CharacterSheet>);

/// Global resource holding a contract that is pending user review.
/// Set when Brain sends `BrainToBody::IdContractReady`.
/// Cleared when the user accepts or rejects the contract.
#[cfg(feature = "bevy")]
#[derive(Resource, Debug, Default)]
pub struct PendingContract(pub Option<crate::id_contract::IdContract>);

/// Iron Road HUD state — the cognitive-load metrics displayed during `AppState::IronRoad`.
///
/// Maps the LitRPG train metaphor: coal is mental fuel, steam is learning throughput,
/// cargo is accumulated knowledge, friction is cognitive load, resonance is mastery.
#[cfg(feature = "bevy")]
#[derive(Resource, Debug, Default)]
pub struct IronRoadState {
    /// Available cognitive fuel (0–100). Drains as quests are active.
    pub coal_reserves: f32,
    /// Active learning throughput in the current session.
    pub steam_production: f32,
    /// Accumulated knowledge weight from completed quests.
    pub cargo_weight: f32,
    /// Accumulated cognitive friction — too high triggers a flow-state warning.
    pub track_friction: f32,
    /// When true, new quests are blocked until friction drops below threshold.
    pub safety_lockout: bool,
    /// Overall mastery level (0–100), synced from CharacterSheet.
    pub resonance_level: f32,
    /// Engine tier (1–4), advances as resonance grows.
    pub engine_tier: u32,
}

/// Chat panel state. Named `ChatState` to distinguish from `AppState`.
#[cfg(feature = "bevy")]
#[derive(Resource)]
pub struct ChatState {
    pub messages: Vec<crate::types::ChatMessage>,
    pub input_text: String,
    pub avatar_state: crate::types::AvatarState,
    pub waiting_for_response: bool,
    pub voice_mode: bool,
    pub system_notifications: Vec<SystemNotification>,
}

#[cfg(feature = "bevy")]
#[derive(Clone, Debug)]
pub struct SystemNotification {
    pub message: String,
    pub urgency: NotificationUrgency,
    pub timestamp: f64,
}

#[cfg(feature = "bevy")]
#[derive(Clone, Debug, PartialEq)]
pub enum NotificationUrgency {
    Info,
    Warning,
    Critical,
}

#[cfg(feature = "bevy")]
impl Default for ChatState {
    fn default() -> Self {
        Self {
            messages: vec![crate::types::ChatMessage {
                role: "system".to_string(),
                content: "Welcome to Trinity Genesis. Press 'Connect' to link to the Brain node."
                    .to_string(),
                timestamp: chrono::Utc::now().timestamp(),
            }],
            input_text: String::new(),
            avatar_state: crate::types::AvatarState::Idle,
            waiting_for_response: false,
            voice_mode: false,
            system_notifications: Vec::new(),
        }
    }
}
