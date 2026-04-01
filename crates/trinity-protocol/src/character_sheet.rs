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

    // --- LORE & ROLEPLAY (The DnD Layer) ---
    /// The user's visual appearance and avatar description in the Iron Road.
    #[serde(default)]
    pub appearance: Option<String>,

    /// The character's origin story, background lore, or how they arrived at the Iron Road.
    #[serde(default)]
    pub backstory: Option<String>,
    
    /// Narrative alignment or pedagogical stance (e.g., "Chaotic Constructor", "Lawful Guide").
    #[serde(default)]
    pub alignment: Option<String>,

    /// Current short-term personal or narrative goal (distinct from the heavy academic goals).
    #[serde(default)]
    pub current_quest_flavor: Option<String>,

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

    /// Shadow status — tracks the "Ghost Train" (Imposter Syndrome/Anxiety).
    /// The Shadow isn't the enemy — it's unprocessed telemetry.
    #[serde(default)]
    pub shadow_status: ShadowStatus,

    // --- COGNITIVE LOGISTICS (The Engine Vitals) ---
    /// Current Steam — germane cognitive load / momentum.
    /// Spent to power ART sidecars and creative generation.
    /// Earned by completing lessons and taming vocabulary.
    #[serde(default = "default_steam")]
    pub current_steam: f32,

    /// Track Friction — extraneous cognitive load penalty.
    /// Mitigated by the Gilbreth Protocol (minimize friction).
    /// High friction = slower XP, harder generation.
    #[serde(default)]
    pub track_friction: f32,

    /// Cargo Slots — working memory capacity (Miller's Law: 7 ± 2).
    /// Limits how many active concepts the user can juggle at once.
    #[serde(default = "default_cargo_slots")]
    pub cargo_slots: u8,

    /// Locomotive Profile — the user's cognitive processing archetype.
    /// Determines the narrative style of Pete's scaffolding.
    #[serde(default)]
    pub locomotive_profile: LocomotiveProfile,

    // --- PURDUE LDT PORTFOLIO (The Graduation Track) ---
    /// The LDT Portfolio is the isomorphic layer that maps academic
    /// requirements (IBSTPI, ATD, AECT, QM) to game progression.
    /// 12 completed artifacts = graduation. The game IS the portfolio.
    #[serde(default)]
    pub ldt_portfolio: LdtPortfolio,

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

    /// Consecutive negative RLHF feedback count — tracks Shadow escalation.
    /// 3+ negatives in a row = Shadow becomes Active.
    #[serde(default)]
    pub consecutive_negatives: u8,
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
            creative_config: CreativeConfig::default(),
            audio_preferences: AudioPreferences::default(),

            skills: HashMap::new(),
            completed_contracts: Vec::new(),
            vaam_profile: crate::vaam_profile::VaamProfile::default(),

            // Lore & Roleplay defaults
            appearance: None,
            backstory: None,
            alignment: None,
            current_quest_flavor: None,

            // Intent Engineering — start grounded
            intent_posture: IntentPosture::default(),
            session_intent: None,
            vulnerability: default_vulnerability(),
            grounding_complete: false,
            shadow_status: ShadowStatus::default(),

            // Cognitive Logistics
            current_steam: default_steam(),
            track_friction: 0.0,
            cargo_slots: default_cargo_slots(),
            locomotive_profile: LocomotiveProfile::default(),

            // LDT Portfolio — empty until user starts building
            ldt_portfolio: LdtPortfolio::default(),

            // Session Zero — populated by Pete's character creation questions
            experience: None,
            audience: None,
            success_vision: None,

            // RLHF tracking
            consecutive_negatives: 0,
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

    /// Recalculate vulnerability from Shadow + Friction compound.
    /// Called after any shadow_status or track_friction mutation.
    /// This is the physics engine that makes Pete's tone respond to the user's state.
    pub fn recalculate_vulnerability(&mut self) {
        let shadow_weight = match self.shadow_status {
            ShadowStatus::Clear => 0.0,
            ShadowStatus::Stirring => 0.15,
            ShadowStatus::Active => 0.35,
            ShadowStatus::Processed => -0.1, // processed = more resilient
        };
        let friction_weight = (self.track_friction / 100.0) * 0.3;
        self.vulnerability = (0.5 + shadow_weight + friction_weight).clamp(0.0, 1.0);
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

// ============================================================================
// COGNITIVE LOGISTICS — The Engine Vitals
// ============================================================================

fn default_steam() -> f32 {
    0.0 // Start cold — Steam is earned, not given
}

fn default_cargo_slots() -> u8 {
    7 // Miller's Law: 7 ± 2
}

/// Shadow Status — tracks the "Ghost Train" (Imposter Syndrome/Anxiety).
/// The Shadow isn't the enemy — it's unprocessed telemetry.
/// Brené Brown: "Owning our story can be hard but not nearly as
/// difficult as spending our lives running from it."
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ShadowStatus {
    /// No shadow detected — the user is grounded.
    #[default]
    Clear,
    /// Shadow stirring — user showed signs of avoidance or frustration.
    /// Pete adjusts scaffolding: more encouragement, fewer challenges.
    Stirring,
    /// Shadow active — user explicitly flagged anxiety or imposter syndrome.
    /// Pete enters maintenance mode: reflection prompts, not task prompts.
    Active,
    /// Shadow processed — user completed a reflection journal.
    /// Heavilon Event survived. "One brick higher, Operator."
    Processed,
}

/// Locomotive Profile — the user's cognitive processing archetype.
/// Determines narrative style and pacing of Pete's scaffolding.
/// Named after railroad locomotive classes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LocomotiveProfile {
    /// Fast processor, impatient with scaffolding. Wants efficiency.
    /// Pete: shorter prompts, faster pacing, more autonomy.
    InterceptorExpress,
    /// Methodical, analytical, wants to understand WHY before acting.
    /// Pete: deeper explanations, more Socratic questioning.
    #[default]
    AnalyzerClass,
    /// Versatile, adapts to context. Comfortable switching modes.
    /// Pete: balanced scaffolding, reads the room.
    AllTerrainSwitcher,
    /// Cautious, prefers safety. Wants clear guardrails and validation.
    /// Pete: more encouragement, explicit checkpoints, gentle pacing.
    ArmoredSupplyTrain,
}

impl LocomotiveProfile {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::InterceptorExpress => "Interceptor Express",
            Self::AnalyzerClass => "Analyzer Class",
            Self::AllTerrainSwitcher => "All-Terrain Switcher",
            Self::ArmoredSupplyTrain => "Armored Supply Train",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::InterceptorExpress => "🚄",
            Self::AnalyzerClass => "🔬",
            Self::AllTerrainSwitcher => "🔀",
            Self::ArmoredSupplyTrain => "🛡️",
        }
    }
}

// ============================================================================
// PURDUE LDT PORTFOLIO — The Graduation Track
// ============================================================================
//
// Isomorphic mapping: academic rubrics → game physics.
// 12 artifacts = graduation. The game IS the portfolio.
//
// The LDT Portfolio answers the question the original CharacterSheet couldn't:
//   "WHAT has the user DONE, and WHERE are they on the path to graduation?"
//
// Standards mapped:
//   • IBSTPI — Instructional Design Competencies (foundational domains)
//   • ATD — Association for Talent Development (capability model)
//   • AECT — Association for Educational Communications & Technology (ethics)
//   • QM — Quality Matters (Higher Ed Rubric for course design)
// ============================================================================

/// The full LDT Portfolio — tracks graduation progress.
/// Flat scores for practical serialization to React frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdtPortfolio {
    /// How many of the 12 required challenges are complete.
    pub completed_challenges: u8,
    /// Gate review status string: "Locked", "Active", "InReview", "Graduated".
    pub gate_review_status: String,

    /// The Ledger of Created Work (subconscious inventory).
    pub artifact_vault: Vec<PortfolioArtifact>,

    /// The user's acquired Spells (Tools/Hooks) functioning as Trading Cards.
    /// This allows Hooks to level up (Maturity) as they tame Scope Creep.
    #[serde(default)]
    pub hook_deck: HashMap<String, HookCard>,

    /// Declarative competency scores (0.0–100.0 each).
    /// IBSTPI — Instructional Design Competencies.
    pub ibstpi_score: f32,
    /// ATD — Association for Talent Development capability.
    pub atd_score: f32,
    /// AECT — Association for Educational Communications & Technology.
    pub aect_score: f32,

    /// Rolling average of Quality Matters alignment scores (0.0–100.0).
    pub qm_alignment_score: f32,

    /// Count of catastrophic failures rebuilt "one brick higher".
    pub heavilon_events_survived: u32,

    /// Deep reflection journals completed after burnout (Max 17 steps).
    /// Maps to the Purdue Memorial Union's 17 steps.
    pub memorial_steps_climbed: u32,
}

/// A dynamic Trading Card representing a Hook Book capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookCard {
    pub id: String,
    pub title: String,
    pub school: String,
    pub level: u8,
    pub xp: u32,
    pub creeps_tamed: u32,
}

impl Default for LdtPortfolio {
    fn default() -> Self {
        let mut default_deck = HashMap::new();
        default_deck.insert("Pearl".to_string(), HookCard {
            id: "Pearl".to_string(),
            title: "The Pearl".to_string(),
            school: "School of Identity".to_string(),
            level: 1,
            xp: 0,
            creeps_tamed: 0,
        });
        default_deck.insert("Coal".to_string(), HookCard {
            id: "Coal".to_string(),
            title: "The Coal".to_string(),
            school: "School of Friction".to_string(),
            level: 1,
            xp: 0,
            creeps_tamed: 0,
        });
        default_deck.insert("Steam".to_string(), HookCard {
            id: "Steam".to_string(),
            title: "The Steam".to_string(),
            school: "School of Momentum".to_string(),
            level: 1,
            xp: 0,
            creeps_tamed: 0,
        });
        default_deck.insert("Hook".to_string(), HookCard {
            id: "Hook".to_string(),
            title: "The Hook".to_string(),
            school: "School of Engagement".to_string(),
            level: 1,
            xp: 0,
            creeps_tamed: 0,
        });
        default_deck.insert("Mirror".to_string(), HookCard {
            id: "Mirror".to_string(),
            title: "The Mirror".to_string(),
            school: "School of Reflection".to_string(),
            level: 1,
            xp: 0,
            creeps_tamed: 0,
        });
        default_deck.insert("Compass".to_string(), HookCard {
            id: "Compass".to_string(),
            title: "The Compass".to_string(),
            school: "School of Scaffolding".to_string(),
            level: 1,
            xp: 0,
            creeps_tamed: 0,
        });

        Self {
            completed_challenges: 0,
            gate_review_status: "Locked".to_string(),
            qm_alignment_score: 0.0,
            ibstpi_score: 10.0,
            atd_score: 10.0,
            aect_score: 10.0,
            artifact_vault: Vec::new(),
            heavilon_events_survived: 0,
            memorial_steps_climbed: 0,
            hook_deck: default_deck,
        }
    }
}

impl LdtPortfolio {
    /// Check if the user has met all graduation requirements.
    pub fn is_graduation_ready(&self) -> bool {
        self.completed_challenges >= 12
            && self.artifact_vault.len() >= 12
            && self.qm_alignment_score >= 85.0
    }

    /// Recalculate portfolio metrics after an artifact is added.
    pub fn recalculate(&mut self) {
        self.completed_challenges = self.artifact_vault.len() as u8;
        if !self.artifact_vault.is_empty() {
            let total_qm: f32 = self.artifact_vault.iter().map(|a| a.qm_score).sum();
            self.qm_alignment_score = total_qm / self.artifact_vault.len() as f32;
        }
        // Gate review auto-upgrade
        if self.completed_challenges >= 12 && self.qm_alignment_score >= 85.0 {
            self.gate_review_status = "Ready For Graduation".to_string();
        } else if self.completed_challenges > 0 {
            self.gate_review_status = "Active".to_string();
        }
    }
}

// --- Portfolio Artifacts ---

/// A completed work product in the user's portfolio.
/// Each artifact maps to standards and includes a reflection journal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioArtifact {
    pub artifact_id: Uuid,
    /// Human-readable title (e.g., "The Edutainment Gap White Paper")
    pub title: String,
    /// Which ADDIECRAPEYE phase generated this artifact.
    pub addiecrapeye_phase: String,
    /// What kind of artifact this is (e.g., "Screencast", "XR Module", "Lesson Plan").
    pub artifact_type: String,
    /// THE CRUCIAL ACADEMIC REQUIREMENT: The Reflection.
    /// "How I acquired this technology and applied it to ID practice..."
    pub reflection_journal: String,
    /// Which supra-badge domain this artifact demonstrates
    /// (e.g., "Professional Foundations", "Design & Development").
    pub aligned_supra_badge: String,
    /// QM score from the Evaluator Agent (0.0–100.0).
    pub qm_score: f32,
    /// Whether the AECT ethics standard was validated.
    /// (Privacy Moat: was PII protected during creation?)
    pub aect_ethics_cleared: bool,
    /// The Hook Book spells used to forge this artifact (e.g. "Socratic Interview", "Express Wizard").
    #[serde(default)]
    pub hooks_cast: Vec<String>,
}

impl PortfolioArtifact {
    pub fn new(
        title: impl Into<String>,
        phase: impl Into<String>,
        artifact_type: impl Into<String>,
        supra_badge: impl Into<String>,
    ) -> Self {
        Self {
            artifact_id: Uuid::new_v4(),
            title: title.into(),
            addiecrapeye_phase: phase.into(),
            artifact_type: artifact_type.into(),
            reflection_journal: String::new(),
            aligned_supra_badge: supra_badge.into(),
            qm_score: 0.0,
            aect_ethics_cleared: false,
            hooks_cast: Vec::new(),
        }
    }

    /// Is this artifact complete enough for the Gate Review?
    pub fn is_review_ready(&self) -> bool {
        !self.reflection_journal.is_empty()
            && self.qm_score >= 70.0
    }
}
