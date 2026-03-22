// Trinity AI Agent System
// Copyright (c) Joshua
//
// ═══════════════════════════════════════════════════════════════════════════════
// 📡 ZONE: PROTOCOL | Module: Character Sheet
// ═══════════════════════════════════════════════════════════════════════════════
// The user's persistent identity in Trinity. The CharacterSheet drives:
//   1. The Awakening class-selection UI (UserClass)
//   2. The RAG system prompt (Pete adjusts tone based on class + skill levels)
//   3. The ID Contract scope (Pete counters with a milestone count appropriate
//      to the user's current resonance level)

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::vocabulary::Genre;

// ============================================================================
// INTENT ENGINEERING (The Digital Quarry)
// ============================================================================
// The CharacterSheet IS the intent model. Intent posture captures not
// "what the user wants to do" but "how they want to grow while doing it."
//
// This works bidirectionally:
//   USER → AI: "I'm here to learn, not just get output" (Mastery)
//   AI → USER: "I'm uncertain about this — let me show my reasoning" (Vulnerability)
//
// Brené Brown: "Vulnerability is the birthplace of innovation, creativity,
// and change." A system that can't admit uncertainty can't teach.
// Pythagoras: "Educate the children and it won't be necessary to punish the men."
// ============================================================================

/// Intent posture — HOW the user wants to engage with this session.
/// Neither choice is wrong. The pedagogical value is in the awareness.
/// When Trinity works autonomously (no user present), Mastery means
/// "prioritize quality and learning" while Efficiency means "ship it."
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum IntentPosture {
    /// "I want to learn through struggle." Build internal skill.
    /// The AI scaffolds — asks questions, presents options, waits.
    /// Coal cost is higher but XP yield is doubled.
    #[default]
    Mastery,
    /// "I want to get the task done." Prioritize output.
    /// The AI acts more directly — suggests solutions, automates.
    /// Coal cost is lower but XP yield is halved.
    Efficiency,
}

impl IntentPosture {
    pub fn display_name(&self) -> &'static str {
        match self {
            IntentPosture::Mastery => "Mastery — learn through the journey",
            IntentPosture::Efficiency => "Efficiency — focus on the destination",
        }
    }

    /// Coal cost multiplier for this posture.
    /// Mastery costs more attention but yields more growth.
    pub fn coal_multiplier(&self) -> f32 {
        match self {
            IntentPosture::Mastery => 1.5,
            IntentPosture::Efficiency => 0.75,
        }
    }

    /// XP multiplier for this posture.
    /// Mastery doubles learning because the user did the work.
    pub fn xp_multiplier(&self) -> f32 {
        match self {
            IntentPosture::Mastery => 2.0,
            IntentPosture::Efficiency => 1.0,
        }
    }
}

fn default_vulnerability() -> f32 {
    0.5 // Start balanced — willing to be surprised but not directionless
}

/// The user's full identity and progression record in Trinity.
/// Persisted to `~/.local/share/trinity/character_sheet.json`.
/// User preferences for the audio pipeline and flow state
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioPreferences {
    /// Preferred genre for the voice output and background music (e.g. "Steampunk", "Cyberpunk")
    pub genre: Option<String>,
    /// Preferred voice ID for TTS
    pub voice_id: Option<String>,
    /// Whether background music should play during flow states
    pub music_flow_enabled: bool,
    /// Specific genre for background music if different from overall narrative genre
    pub bg_music_genre: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSheet {
    /// Unique user identifier (stable across sessions).
    pub user_id: Uuid,
    /// The conductor's chosen name (shown in the UI).
    pub alias: String,
    /// The role the user selected during The Awakening.
    pub user_class: UserClass,
    /// Overall level (increases with XP from completed contracts).
    pub resonance_level: u32,
    /// Lifetime XP earned across all accepted and completed contracts.
    pub total_xp: u64,
    /// Current attention reserve (0.0 = empty, 100.0 = fully charged).
    pub current_coal: f32,

    // --- THE AWAKENING: HARDWARE STATS ("EQUIPMENT") ---
    /// The user's VRAM in GB. Determines how many heavy models can be loaded.
    pub mana_pool_vram: u32,
    /// The user's System RAM in GB. Determines the maximum party size.
    pub stamina_ram: u32,
    /// The user's Compute/NPU capability. Determines generation speed.
    pub agility_compute: u32,
    /// The resulting agent orchestration strategy based on hardware.
    pub concurrency_mode: ConcurrencyMode,

    // --- THE AWAKENING: PROJECT CONFIGURATION ---
    /// The narrative genre selected during character creation.
    /// Determines vocabulary themes, visual style, and narrative tone.
    #[serde(default)]
    pub genre: Genre,
    /// ID of the user's custom vocabulary pack (co-created with AI).
    /// If None, uses the default vocabulary for the selected genre.
    #[serde(default)]
    pub vocabulary_pack_id: Option<Uuid>,
    /// The AI party configuration (which models fill which roles).
    #[serde(default)]
    pub party_config: PartyConfig,
    /// Creative sidecar settings (ComfyUI + MusicGPT) - the "VIBE contract".
    #[serde(default)]
    pub creative_config: CreativeConfig,

    /// Audio pipeline preferences
    #[serde(default)]
    pub audio_preferences: AudioPreferences,

    /// Per-skill proficiency scores (0.0 – 100.0 each).
    pub skills: HashMap<SkillType, f32>,
    /// IDs of completed ID Contracts (used by RAG to avoid duplicate lessons).
    pub completed_contracts: Vec<Uuid>,

    /// VAAM preference profile — word weights, attention patterns, style,
    /// and agreements between user and AI about what matters.
    #[serde(default)]
    pub vaam_profile: crate::vaam_profile::VaamProfile,

    // --- INTENT ENGINEERING (The Digital Quarry) ---
    // The CharacterSheet IS the intent model. These fields capture not just
    // WHO the user is, but WHY they're here and HOW they want to grow.
    /// Current intent posture — Mastery (learn through struggle) or
    /// Efficiency (get the task done). Neither is wrong; awareness is the point.
    /// Brené Brown: "Vulnerability is the birthplace of innovation."
    #[serde(default)]
    pub intent_posture: IntentPosture,

    /// The user's stated purpose for this session — one clear sentence.
    /// Updated at session start and revisited at Envision phase.
    /// "What is the core purpose of the task you are about to undertake?"
    #[serde(default)]
    pub session_intent: Option<String>,

    /// Vulnerability level — willingness to be uncertain, to not know,
    /// to let the process reveal what needs revealing.
    /// 0.0 = I want certainty and control. 1.0 = I'm open to surprises.
    /// This goes both ways — when the AI works autonomously, it also needs
    /// to be "vulnerable" (transparent about uncertainty, not overclaiming).
    #[serde(default = "default_vulnerability")]
    pub vulnerability: f32,

    /// Whether the I AM grounding ritual has been completed this session.
    /// "I Am Here. I Am Enough. I Choose."
    #[serde(default)]
    pub grounding_complete: bool,

    // --- SESSION ZERO (Pete's Character Creation) ---
    // These fields are populated by Pete's 3 Socratic questions at the start.
    // They feed into every system prompt so Pete references them naturally.
    /// Teaching experience level — "What's your teaching experience level?"
    #[serde(default)]
    pub experience: Option<String>,

    /// Target audience — "Who are your students? (age, context)"
    #[serde(default)]
    pub audience: Option<String>,

    /// Success vision — "What does success look like for this lesson?"
    #[serde(default)]
    pub success_vision: Option<String>,
}

impl CharacterSheet {
    pub fn new(alias: impl Into<String>, user_class: UserClass) -> Self {
        Self {
            user_id: Uuid::new_v4(),
            alias: alias.into(),
            user_class,
            resonance_level: 1,
            total_xp: 0,
            current_coal: 100.0,

            // Default hardware stats (will be updated by the Hardware Scanner)
            mana_pool_vram: 0,
            stamina_ram: 0,
            agility_compute: 0,
            concurrency_mode: ConcurrencyMode::LoneWolf,

            // Project configuration (set during Awakening)
            genre: Genre::default(),
            vocabulary_pack_id: None,
            party_config: PartyConfig::default(),
            creative_config: CreativeConfig::default(),
            audio_preferences: AudioPreferences::default(),

            skills: HashMap::new(),
            completed_contracts: Vec::new(),
            vaam_profile: crate::vaam_profile::VaamProfile::default(),

            // Intent Engineering — start grounded
            intent_posture: IntentPosture::default(),
            session_intent: None,
            vulnerability: default_vulnerability(),
            grounding_complete: false,

            // Session Zero — populated by Pete's character creation questions
            experience: None,
            audience: None,
            success_vision: None,
        }
    }

    /// Add XP and recalculate resonance level.
    /// Resonance level = floor(sqrt(total_xp / 100)) + 1, capped at 100.
    pub fn award_xp(&mut self, xp: u64) {
        self.total_xp += xp;
        let new_level = ((self.total_xp / 100) as f64).sqrt() as u32 + 1;
        self.resonance_level = new_level.min(100);
    }

    /// Consume coal (attention). Returns the amount actually consumed
    /// (may be less than requested if reserves are low).
    pub fn consume_coal(&mut self, amount: f32) -> f32 {
        let consumed = amount.min(self.current_coal);
        self.current_coal -= consumed;
        consumed
    }

    /// Restore coal (e.g., after a Pomodoro break).
    pub fn restore_coal(&mut self, amount: f32) {
        self.current_coal = (self.current_coal + amount).min(100.0);
    }

    // --- Intent Engineering Methods ---

    /// Complete the I AM grounding ritual.
    /// "I Am Here. I Am Enough. I Choose."
    /// Call this at session start before any quest interaction.
    pub fn ground(&mut self) {
        self.grounding_complete = true;
    }

    /// Set the session intent — one clear sentence about WHY.
    /// "What is the core purpose of the task you are about to undertake?"
    pub fn set_intent(&mut self, intent: impl Into<String>, posture: IntentPosture) {
        self.session_intent = Some(intent.into());
        self.intent_posture = posture;
    }

    /// Generate a compact intent summary for conductor system prompts.
    /// This is how the CharacterSheet's intent feeds into Pete's awareness.
    pub fn intent_summary(&self) -> String {
        let mut parts = Vec::new();

        // Grounding status
        if self.grounding_complete {
            parts.push("Grounded ✓".to_string());
        } else {
            parts.push("⚠ NOT GROUNDED — user has not completed I AM ritual".to_string());
        }

        // Posture
        parts.push(format!("Posture: {}", self.intent_posture.display_name()));

        // Session intent
        if let Some(ref intent) = self.session_intent {
            parts.push(format!("Purpose: {}", intent));
        } else {
            parts.push("⚠ No session intent stated — ask the user WHY they're here".to_string());
        }

        // Vulnerability
        let v_label = if self.vulnerability > 0.7 {
            "open to discovery"
        } else if self.vulnerability < 0.3 {
            "wants certainty"
        } else {
            "balanced"
        };
        parts.push(format!(
            "Vulnerability: {} ({})",
            self.vulnerability, v_label
        ));

        // Session Zero context
        if let Some(ref exp) = self.experience {
            parts.push(format!("Experience: {}", exp));
        }
        if let Some(ref aud) = self.audience {
            parts.push(format!("Audience: {}", aud));
        }
        if let Some(ref vis) = self.success_vision {
            parts.push(format!("Success Vision: {}", vis));
        }

        parts.join(" | ")
    }
}

impl Default for CharacterSheet {
    fn default() -> Self {
        Self::new("Conductor", UserClass::SubjectMatterExpert)
    }
}

/// The hardware-determined capability for agent concurrency.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum ConcurrencyMode {
    /// 24GB VRAM or less: Only one agent can be loaded at a time.
    /// The user must swap agents manually ("Forming a Party" means choosing one companion).
    #[default]
    LoneWolf,
    /// 32GB-64GB VRAM: Can load a small party (e.g., text agent + vision agent).
    SmallSquad,
    /// 128GB+ VRAM (Strix Halo/Server): Can run all agents concurrently as a guild.
    Guild,
}

/// The four roles a Trinity user can play, selected during The Awakening.
/// Trinity autonomously fills the skills of roles the user is NOT playing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UserClass {
    /// "I know what needs to be taught." — Drives content selection and accuracy.
    SubjectMatterExpert,
    /// "I know how to scaffold the learning." — Drives ADDIE structure.
    InstructionalDesigner,
    /// "I know what success looks like." — Drives evaluation criteria.
    Stakeholder,
    /// "I experience what gets built." — Drives from the learner's perspective.
    Player,
}

impl UserClass {
    pub fn display_name(&self) -> &'static str {
        match self {
            UserClass::SubjectMatterExpert => "Subject Matter Expert",
            UserClass::InstructionalDesigner => "Instructional Designer",
            UserClass::Stakeholder => "Stakeholder",
            UserClass::Player => "Player",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            UserClass::SubjectMatterExpert => "🧑‍🏫",
            UserClass::InstructionalDesigner => "🎓",
            UserClass::Stakeholder => "📊",
            UserClass::Player => "🎮",
        }
    }

    pub fn tagline(&self) -> &'static str {
        match self {
            UserClass::SubjectMatterExpert => "I know what needs to be taught.",
            UserClass::InstructionalDesigner => "I know how to scaffold the learning.",
            UserClass::Stakeholder => "I know what success looks like.",
            UserClass::Player => "I experience what gets built.",
        }
    }
}

/// The instructional design competency areas tracked per-user.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SkillType {
    /// Ability to define learning goals and scope a lesson plan.
    #[serde(alias = "InstructionalDesign")]
    CurriculumDesign,
    /// Ability to find and select appropriate learning materials.
    #[serde(alias = "DocumentManagement")]
    ContentCuration,
    /// Ability to create formative and summative assessments.
    AssessmentDesign,
    /// Ability to encode learning into game mechanics (the Trinity core skill).
    #[serde(alias = "BlueprintSynthesis")]
    GamificationDesign,
    /// Ability to build engaging narratives that carry cognitive load.
    #[serde(alias = "ModelDelegation")]
    NarrativeDesign,
}

/// Bloom's Taxonomy cognitive level. Used to calibrate the complexity of
/// ID Contract milestones relative to the user's resonance level.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BloomLevel {
    /// Recall facts (lowest cognitive demand).
    Remember,
    /// Explain in own words.
    Understand,
    /// Use knowledge in new situations.
    Apply,
    /// Break down information into component parts.
    Analyze,
    /// Make judgments based on criteria.
    Evaluate,
    /// Produce new or original work (highest cognitive demand).
    Create,
}

impl BloomLevel {
    pub fn display_name(&self) -> &'static str {
        match self {
            BloomLevel::Remember => "Remember",
            BloomLevel::Understand => "Understand",
            BloomLevel::Apply => "Apply",
            BloomLevel::Analyze => "Analyze",
            BloomLevel::Evaluate => "Evaluate",
            BloomLevel::Create => "Create",
        }
    }

    /// Suggested resonance level required to target this Bloom level.
    pub fn minimum_resonance(&self) -> u32 {
        match self {
            BloomLevel::Remember => 1,
            BloomLevel::Understand => 3,
            BloomLevel::Apply => 8,
            BloomLevel::Analyze => 15,
            BloomLevel::Evaluate => 25,
            BloomLevel::Create => 40,
        }
    }
}

// ============================================================================
// PARTY CONFIGURATION
// ============================================================================

/// AI Party configuration for the user's project.
/// Determines which models fill which roles based on hardware.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyConfig {
    /// Which roles are filled by which models
    pub roles: HashMap<PartyRole, ModelAssignment>,
    /// Total memory budget in GB
    pub memory_budget_gb: u32,
    /// Whether user has overridden auto-config
    pub is_customized: bool,
}

impl PartyConfig {
    /// Create auto-configured party based on hardware
    pub fn auto_configure(vram_gb: u32, _ram_gb: u32) -> Self {
        let (roles, budget) = if vram_gb < 24 {
            // LoneWolf: Single model, swap as needed
            let mut roles = HashMap::new();
            roles.insert(PartyRole::Conductor, ModelAssignment::mistral_small_4());
            (roles, 12)
        } else if vram_gb < 64 {
            // SmallSquad: Conductor + one specialist
            let mut roles = HashMap::new();
            roles.insert(PartyRole::Conductor, ModelAssignment::mistral_small_4());
            roles.insert(PartyRole::Engineer, ModelAssignment::reap_25b());
            (roles, 36)
        } else {
            // Guild: Full party
            let mut roles = HashMap::new();
            roles.insert(PartyRole::Conductor, ModelAssignment::mistral_small_4());
            roles.insert(PartyRole::Engineer, ModelAssignment::reap_25b());
            roles.insert(PartyRole::Evaluator, ModelAssignment::opus_27b());
            roles.insert(PartyRole::Artist, ModelAssignment::opus_27b());
            roles.insert(PartyRole::Brakeman, ModelAssignment::reap_25b());
            roles.insert(PartyRole::Visionary, ModelAssignment::qwen_35b());
            (roles, 128)
        };

        Self {
            roles,
            memory_budget_gb: budget,
            is_customized: false,
        }
    }

    /// Get total memory used by assigned models
    pub fn total_memory_used(&self) -> u32 {
        self.roles.values().map(|m| m.memory_gb).sum()
    }

    /// Check if a model can be added within budget
    pub fn can_add_model(&self, memory_gb: u32) -> bool {
        self.total_memory_used() + memory_gb <= self.memory_budget_gb
    }
}

/// Roles in the AI party (companion system)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PartyRole {
    /// Main orchestrator - guides conversation and delegates
    Conductor,
    /// Code generation specialist
    Engineer,
    /// Quality review and evaluation
    Evaluator,
    /// Creative and design work
    Artist,
    /// Safety and testing
    Brakeman,
    /// Visual analysis (screenshots, UI)
    Visionary,
}

impl PartyRole {
    pub fn display_name(&self) -> &'static str {
        match self {
            PartyRole::Conductor => "Conductor",
            PartyRole::Engineer => "Engineer",
            PartyRole::Evaluator => "Evaluator",
            PartyRole::Artist => "Artist",
            PartyRole::Brakeman => "Brakeman",
            PartyRole::Visionary => "Visionary",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            PartyRole::Conductor => "🎭",
            PartyRole::Engineer => "⚙️",
            PartyRole::Evaluator => "📊",
            PartyRole::Artist => "🎨",
            PartyRole::Brakeman => "🛡️",
            PartyRole::Visionary => "👁️",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            PartyRole::Conductor => "Guides conversation and delegates tasks",
            PartyRole::Engineer => "Generates and modifies code",
            PartyRole::Evaluator => "Reviews quality against rubrics",
            PartyRole::Artist => "Creates designs and narratives",
            PartyRole::Brakeman => "Tests and validates safety",
            PartyRole::Visionary => "Analyzes visuals and UI",
        }
    }
}

/// A model assigned to a party role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelAssignment {
    /// Model identifier (e.g., "gpt-oss-20b")
    pub model_id: String,
    /// Display name (e.g., "GPT-OSS 20B")
    pub display_name: String,
    /// Path to model files
    pub model_path: String,
    /// Memory requirement in GB
    pub memory_gb: u32,
    /// Whether model is currently loaded
    pub is_loaded: bool,
}

impl ModelAssignment {
    /// P — Conductor (Pete): Mistral Small 4 119B MoE
    /// Split GGUF: 37GB + 31GB = 68GB total, ~6.5B active params, 40+ tok/s
    /// 256k context with Q4 KV cache quantization, vision capable
    /// Served via llama-server on port 8080
    pub fn mistral_small_4() -> Self {
        Self {
            model_id: "mistral-small-4-119b".to_string(),
            display_name: "Mistral Small 4 119B MoE (256k Q4 KV + Vision)".to_string(),
            model_path: "trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf"
                .to_string(),
            memory_gb: 68,
            is_loaded: false,
        }
    }

    /// Y — Yardmaster: Ming-flash-omni-2.0
    /// ~195GB safetensors (42 shards), true omni-modal (text+vision+audio)
    /// BailingMoeV2 backbone: 256 experts, 8 active per token
    /// MUST be served via vLLM (custom /generate protocol, NOT OpenAI-compat)
    pub fn ming_omni() -> Self {
        Self {
            model_id: "ming-flash-omni-2.0".to_string(),
            display_name: "Ming-flash-omni-2.0 (Truly OMNI — text+vision+audio)".to_string(),
            model_path: "trinity-models/safetensors/Ming-flash-omni-2.0".to_string(),
            memory_gb: 195,
            is_loaded: false,
        }
    }

    /// A-R-T (R — Research): Qwen3-Coder-REAP-25B MoE
    /// 15GB GGUF, 3B active params, Rust-specialized code generation
    pub fn reap_25b() -> Self {
        Self {
            model_id: "reap-25b-a3b".to_string(),
            display_name: "REAP 25B MoE (Rust Code Gen)".to_string(),
            model_path: "trinity-models/gguf/Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf".to_string(),
            memory_gb: 15,
            is_loaded: false,
        }
    }

    /// A-R-T (R — Research): Crow 9B
    /// 5.3GB GGUF, fast research/reasoning agent
    pub fn crow_9b() -> Self {
        Self {
            model_id: "crow-9b".to_string(),
            display_name: "Crow 9B (Research Agent)".to_string(),
            model_path:
                "trinity-models/gguf/Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf"
                    .to_string(),
            memory_gb: 5,
            is_loaded: false,
        }
    }

    /// A-R-T (T — Tempo): OmniCoder 9B
    /// 5.4GB GGUF, code generation for tempo/music pipeline
    pub fn omnicoder_9b() -> Self {
        Self {
            model_id: "omnicoder-9b".to_string(),
            display_name: "OmniCoder 9B (Tempo/Code)".to_string(),
            model_path: "trinity-models/gguf/OmniCoder-9B-Q4_K_M.gguf".to_string(),
            memory_gb: 5,
            is_loaded: false,
        }
    }

    /// Reserve: Qwen3.5-27B Claude Opus Reasoning Distilled
    /// 21GB GGUF, advanced reasoning for evaluation
    pub fn opus_27b() -> Self {
        Self {
            model_id: "opus-27b".to_string(),
            display_name: "Qwen3.5-27B Claude Opus (Evaluator)".to_string(),
            model_path:
                "trinity-models/gguf/Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled.i1-Q6_K.gguf"
                    .to_string(),
            memory_gb: 21,
            is_loaded: false,
        }
    }

    /// Reserve: Qwen3.5-35B-A3B (Visionary with vision projector)
    /// 20GB GGUF, MoE with 3B active
    pub fn qwen_35b() -> Self {
        Self {
            model_id: "qwen-35b-a3b".to_string(),
            display_name: "Qwen3.5-35B-A3B (Visionary)".to_string(),
            model_path: "trinity-models/gguf/Qwen3.5-35B-A3B-Q4_K_M.gguf".to_string(),
            memory_gb: 20,
            is_loaded: false,
        }
    }
}

impl Default for ModelAssignment {
    fn default() -> Self {
        Self::mistral_small_4()
    }
}

// ============================================================================
// CREATIVE CONFIGURATION - VIBE CONTRACT
// ============================================================================

/// Creative sidecar settings - the "VIBE contract" between Trinity and user.
/// Controls visual style (ComfyUI) and music style (MusicGPT) for the experience.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreativeConfig {
    /// Visual style for image generation
    pub visual_style: VisualStyle,
    /// Music style for background audio
    pub music_style: MusicStyle,
    /// Image generation settings
    #[serde(default)]
    pub image_settings: ImageSettings,
    /// Audio generation settings
    #[serde(default)]
    pub audio_settings: AudioSettings,
    /// Whether creative sidecars are enabled
    #[serde(default = "default_creative_enabled")]
    pub creative_enabled: bool,
}

fn default_creative_enabled() -> bool {
    true
}

impl Default for CreativeConfig {
    fn default() -> Self {
        Self {
            visual_style: VisualStyle::Steampunk,
            music_style: MusicStyle::Orchestral,
            image_settings: ImageSettings::default(),
            audio_settings: AudioSettings::default(),
            creative_enabled: true,
        }
    }
}

impl CreativeConfig {
    /// Create creative config from genre - maps narrative genre to visual/music style
    pub fn from_genre(genre: &crate::vocabulary::Genre) -> Self {
        use crate::vocabulary::Genre;

        let (visual, music) = match genre {
            Genre::Steampunk => (VisualStyle::Steampunk, MusicStyle::Orchestral),
            Genre::Cyberpunk => (VisualStyle::Cyberpunk, MusicStyle::Electronic),
            Genre::Solarpunk => (VisualStyle::Minimalist, MusicStyle::Ambient),
            Genre::DarkFantasy => (VisualStyle::Fantasy, MusicStyle::Orchestral),
        };

        Self {
            visual_style: visual,
            music_style: music,
            image_settings: ImageSettings::default(),
            audio_settings: AudioSettings::default(),
            creative_enabled: true,
        }
    }
}

/// Visual style for ComfyUI image generation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum VisualStyle {
    #[default]
    Steampunk, // Iron Road default - brass, gears, steam
    Cyberpunk,  // Neon, chrome, holograms
    Fantasy,    // Magic, medieval, ethereal
    Minimalist, // Clean, modern, simple
    Retro,      // 8-bit, pixel art, nostalgic
    Noir,       // Dark, moody, detective
}

impl VisualStyle {
    pub fn display_name(&self) -> &'static str {
        match self {
            VisualStyle::Steampunk => "Steampunk",
            VisualStyle::Cyberpunk => "Cyberpunk",
            VisualStyle::Fantasy => "Fantasy",
            VisualStyle::Minimalist => "Minimalist",
            VisualStyle::Retro => "Retro",
            VisualStyle::Noir => "Noir",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            VisualStyle::Steampunk => "⚙️",
            VisualStyle::Cyberpunk => "🌃",
            VisualStyle::Fantasy => "🐉",
            VisualStyle::Minimalist => "⬜",
            VisualStyle::Retro => "👾",
            VisualStyle::Noir => "🎩",
        }
    }

    /// Get style prompt suffix for ComfyUI
    pub fn prompt_suffix(&self) -> &'static str {
        match self {
            VisualStyle::Steampunk => "steampunk aesthetic, brass gears, steam pipes, Victorian industrial, warm amber lighting",
            VisualStyle::Cyberpunk => "cyberpunk aesthetic, neon lights, holographic displays, futuristic cityscape, blue and pink lighting",
            VisualStyle::Fantasy => "fantasy aesthetic, magical atmosphere, ethereal glow, medieval architecture, mystical elements",
            VisualStyle::Minimalist => "minimalist aesthetic, clean lines, simple shapes, neutral colors, modern design",
            VisualStyle::Retro => "retro pixel art style, 8-bit graphics, nostalgic gaming aesthetic, limited color palette",
            VisualStyle::Noir => "noir aesthetic, dramatic shadows, black and white, film noir lighting, mysterious atmosphere",
        }
    }
}

/// Music style for MusicGPT background audio
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MusicStyle {
    #[default]
    Orchestral, // Epic adventure - Iron Road default
    Lofi,       // Chill focus beats
    Electronic, // Synthwave, ambient electronic
    Jazz,       // Noir detective vibes
    Ambient,    // Atmospheric, minimal
    Classical,  // Traditional orchestral
}

impl MusicStyle {
    pub fn display_name(&self) -> &'static str {
        match self {
            MusicStyle::Orchestral => "Orchestral",
            MusicStyle::Lofi => "Lo-fi",
            MusicStyle::Electronic => "Electronic",
            MusicStyle::Jazz => "Jazz",
            MusicStyle::Ambient => "Ambient",
            MusicStyle::Classical => "Classical",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            MusicStyle::Orchestral => "🎻",
            MusicStyle::Lofi => "🎧",
            MusicStyle::Electronic => "🎹",
            MusicStyle::Jazz => "🎷",
            MusicStyle::Ambient => "🌌",
            MusicStyle::Classical => "🎼",
        }
    }

    /// Get style prompt for MusicGPT
    pub fn prompt(&self) -> &'static str {
        match self {
            MusicStyle::Orchestral => {
                "epic orchestral background music, cinematic, adventure theme, strings and brass"
            }
            MusicStyle::Lofi => "lofi hip hop beats, chill focus music, relaxed, study beats",
            MusicStyle::Electronic => {
                "synthwave electronic music, ambient synthesizer, futuristic atmosphere"
            }
            MusicStyle::Jazz => {
                "smooth jazz background music, noir detective vibes, saxophone and piano"
            }
            MusicStyle::Ambient => "ambient atmospheric music, minimal, spacey, meditation",
            MusicStyle::Classical => "classical orchestral music, baroque style, elegant, timeless",
        }
    }
}

/// Image generation settings for ComfyUI
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageSettings {
    /// Image width in pixels
    pub width: u32,
    /// Image height in pixels
    pub height: u32,
    /// Number of inference steps (quality vs speed)
    pub steps: u32,
    /// CFG scale (prompt adherence)
    pub cfg_scale: f32,
    /// Seed for reproducibility (-1 for random)
    pub seed: i64,
}

impl Default for ImageSettings {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 1024,
            steps: 4,       // SDXL Turbo is fast
            cfg_scale: 1.0, // Turbo default
            seed: -1,       // Random
        }
    }
}

/// Audio generation settings for MusicGPT
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioSettings {
    /// Duration in seconds
    pub duration_secs: u32,
    /// Sample rate
    pub sample_rate: u32,
    /// Temperature (creativity)
    pub temperature: f32,
    /// Whether to loop continuously
    pub loop_playback: bool,
    /// Volume (0.0 - 1.0)
    pub volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            duration_secs: 60,
            sample_rate: 44100,
            temperature: 0.8,
            loop_playback: true,
            volume: 0.3, // Background music level
        }
    }
}
