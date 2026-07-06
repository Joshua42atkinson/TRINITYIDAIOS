// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Protocol Layer
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:     types.rs
// PURPOSE:  Core shared types: ChatMessage, ToolCall, ToolResult, MemoryFact, InferenceRequest
// BIBLE:    Car 1 — ANALYSIS (System Overview, §1.3)
//
// ═══════════════════════════════════════════════════════════════════════════════

// Trinity AI Agent System
// Copyright (c) Joshua
// Shared under license for Ask_Pete (Purdue University)

use serde::{Deserialize, Serialize};

/// A single message in a chat conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Role of the speaker ("user", "assistant", "system")
    pub role: String,
    /// Text content of the message
    pub content: String,
    /// Creation timestamp (Unix epoch)
    pub timestamp: i64,
}

/// Audio data packet for voice transmission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoicePacket {
    /// Raw PCM audio data (usually 16-bit, mono)
    pub audio_data: Vec<u8>,
    /// Sample rate in Hz (e.g. 24000)
    pub sample_rate: u32,
}

/// Emotion values for voice synthesis (0.0 - 1.0 each).
/// Defines the emotional coloring of the generated speech.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmotionData {
    pub happiness: f32,
    pub anger: f32,
    pub sadness: f32,
    pub fear: f32,
    pub surprise: f32,
}

/// Response from chat with associated metadata (voice, emotion, state).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceResponse {
    /// Text content of the response.
    pub text: String,
    /// Synthesized audio (WAV format, 16-bit PCM), if requested.
    pub audio: Option<VoicePacket>,
    /// Emotion detected for this response.
    pub emotion: EmotionData,
    /// Avatar state hint (what animation to play).
    pub avatar_state: AvatarState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFact {
    pub id: String,
    pub content: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub quantization: String,
    pub context_size: u32,
}

/// Visual state for the 3D Avatar.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AvatarState {
    /// Neutral loop
    Idle,
    /// Processing/Reasoning animation
    Thinking,
    /// Typing logic/Coding animation
    Coding,
    /// Lip-sync speaking animation
    Speaking,
    /// Low-power mode / idle
    Sleeping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolError {
    pub code: u32,
    pub message: String,
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProtocolError {}: {}", self.code, self.message)
    }
}

impl std::error::Error for ProtocolError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareStats {
    pub memory_used_bytes: u64,
    pub memory_available_bytes: u64,
    pub memory_percent: f32,
    pub cpu_percent: f32,
    pub load_avg_1m: f32,
    pub gpu_available: bool,
}

// ============================================================================
// Image Generation Types
// ============================================================================

/// Request for image generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageRequest {
    /// Text prompt describing the image
    pub prompt: String,
    /// Negative prompt (things to avoid)
    pub negative_prompt: Option<String>,
    /// Output width (default: 1024)
    pub width: Option<u32>,
    /// Output height (default: 1024)
    pub height: Option<u32>,
    /// Number of inference steps (default: 30)
    pub steps: Option<u32>,
    /// Random seed (None = random)
    pub seed: Option<u64>,
}

impl ImageRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            negative_prompt: None,
            width: None,
            height: None,
            steps: None,
            seed: None,
        }
    }
}

/// Response from image generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResponse {
    /// PNG image data
    pub image_data: Vec<u8>,
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
    /// Prompt used
    pub prompt: String,
    /// Seed used
    pub seed: u64,
}

// ============================================================================
// Vision-to-Action Generation Types (NitroGen)
// ============================================================================

/// Request for Vision-to-Action (mapped directly to NitroGen expectations)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamepadActionRequest {
    /// 256x256 RGB image data (raw pixels or encoded PNG/JPEG)
    pub image_data: Vec<u8>,
    /// Optional structured task prompt or past action history
    pub task_context: Option<String>,
}

impl GamepadActionRequest {
    pub fn new(image_data: Vec<u8>) -> Self {
        Self {
            image_data,
            task_context: None,
        }
    }
}

/// Response generating standard gamepad controls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamepadActionResponse {
    /// Left Joystick X/Y (-1.0 to 1.0)
    pub left_stick: (f32, f32),
    /// Right Joystick X/Y (-1.0 to 1.0)
    pub right_stick: (f32, f32),

    // 17 Binary Gamepad Buttons
    pub button_a: bool,
    pub button_b: bool,
    pub button_x: bool,
    pub button_y: bool,
    pub dpad_up: bool,
    pub dpad_down: bool,
    pub dpad_left: bool,
    pub dpad_right: bool,
    pub bumper_left: bool,
    pub bumper_right: bool,
    pub trigger_left: bool,
    pub trigger_right: bool,
    pub thumb_left: bool,
    pub thumb_right: bool,
    pub start: bool,
    pub select: bool,
    pub guide: bool, // Xbox button / PS button
}

impl GamepadActionResponse {
    pub fn is_idle(&self) -> bool {
        self.left_stick.0.abs() < 0.1
            && self.left_stick.1.abs() < 0.1
            && self.right_stick.0.abs() < 0.1
            && self.right_stick.1.abs() < 0.1
            && !self.button_a
            && !self.button_b
            && !self.button_x
            && !self.button_y
            && !self.dpad_up
            && !self.dpad_down
            && !self.dpad_left
            && !self.dpad_right
    }
}

// ============================================================================
// Code Generation Types
// ============================================================================

/// Request for code generation (Coder skill)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeRequest {
    /// Description of the code to generate
    pub prompt: String,
    /// Language (e.g., "rust", "python", "typescript")
    pub language: String,
    /// Path to save the output code (if any)
    pub output_path: Option<String>,
    /// Whether to use grammar-constrained sampling
    pub use_grammar: bool,
}

impl CodeRequest {
    pub fn new(prompt: impl Into<String>, language: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            language: language.into(),
            output_path: None,
            use_grammar: true,
        }
    }
}

/// Response from code generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeResponse {
    /// The generated code
    pub code: String,
    /// Language used
    pub language: String,
    /// Path where code was saved (if any)
    pub saved_path: Option<String>,
    /// Whether syntax appears valid (basic check)
    pub syntax_valid: bool,
}

// ============================================================================
// Roles
// ============================================================================

/// Iron Road Crew subagent roles.
///
/// Each role maps to a specialized AI agent in the production pipeline
/// as defined in the Trinity Technical Bible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubagentRole {
    /// MiniMax-M2.1-REAP-50 — orchestrates the full pipeline.
    Conductor,
    /// Manages exports and package delivery.
    Dispatcher,
    /// Quality control reviewer.
    Yardmaster,
    /// Creates visual designs and schematics.
    Draftsman,
    /// Builds 3D models and technical assets.
    Engineer,
    /// Safety systems and constraint enforcement.
    Brakeman,
    /// Executes and drives the final output.
    Driver,
}

// ============================================================================
// Document Generation Types
// ============================================================================

/// Writing style for document generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WriteStyle {
    /// Technical documentation (API docs, READMEs)
    #[default]
    Technical,
    /// Blog post / article style
    BlogPost,
    /// Educational / tutorial
    Tutorial,
    /// Creative / storytelling
    Creative,
    /// Formal / business communication
    Formal,
    /// Casual / conversational
    Casual,
}

/// Request for document generation (Writer skill)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteRequest {
    /// Topic or subject matter
    pub topic: String,
    /// Style of writing
    pub style: WriteStyle,
    /// Target word count (approximate)
    pub target_words: u32,
    /// Path to save the output (if any)
    pub output_path: Option<String>,
}

impl WriteRequest {
    pub fn new(topic: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            style: WriteStyle::Technical,
            target_words: 500,
            output_path: None,
        }
    }
}

/// Response from document generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteResponse {
    /// The generated content
    pub content: String,
    /// Approximate word count
    pub word_count: u32,
    /// Path where saved (if any)
    pub saved_path: Option<String>,
}

// ============================================================================
// Assessment Generation Types (Educator Skill)
// ============================================================================

/// Type of assessment to generate
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AssessmentType {
    /// Multiple choice quiz
    Quiz,
    /// Hands-on lab project
    Lab,
    /// Coding challenge
    Challenge,
}

/// Difficulty level for assessments
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Request for assessment generation (Educator skill)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentRequest {
    /// Topic for the assessment
    pub topic: String,
    /// Type of assessment to generate
    pub assessment_type: AssessmentType,
    /// Difficulty level
    pub difficulty: Difficulty,
    /// Target audience description
    pub target_audience: String,
}

impl AssessmentRequest {
    pub fn new(topic: impl Into<String>, audience: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            assessment_type: AssessmentType::Quiz,
            difficulty: Difficulty::Intermediate,
            target_audience: audience.into(),
        }
    }

    pub fn with_type(mut self, assessment_type: AssessmentType) -> Self {
        self.assessment_type = assessment_type;
        self
    }

    pub fn with_difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = difficulty;
        self
    }
}

/// A quiz question with multiple choice options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer_idx: usize,
    pub explanation: String,
}

/// A hands-on lab project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabProject {
    pub title: String,
    pub objective: String,
    pub steps: Vec<String>,
    pub starter_code: Option<String>,
    pub solution: Option<String>,
}

/// Response from assessment generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssessmentResponse {
    /// A quiz with questions
    Quiz { questions: Vec<QuizQuestion> },
    /// A lab project
    Lab(LabProject),
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

/// The narrative genre for a project.
/// Genre determines vocabulary sets, narrative style, and visual theme.
/// Fixed at project creation - switching resets progress.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum Genre {
    /// Tech-noir, neon, gritty. Vocabulary: systems, networks, hacking.
    #[default]
    Cyberpunk,
    /// Victorian, industrial, brass. Vocabulary: mechanics, steam, engineering.
    Steampunk,
    /// Hopeful, organic, bright. Vocabulary: sustainability, ecology, growth.
    Solarpunk,
    /// Gothic, mysterious, shadow. Vocabulary: magic, runes, transformation.
    DarkFantasy,
}

impl Genre {
    pub fn display_name(&self) -> &'static str {
        match self {
            Genre::Cyberpunk => "Cyberpunk",
            Genre::Steampunk => "Steampunk",
            Genre::Solarpunk => "Solarpunk",
            Genre::DarkFantasy => "Dark Fantasy",
        }
    }

    pub fn narrative_style(&self) -> &'static str {
        match self {
            Genre::Cyberpunk => "Neon-lit corridors of data. The machine speaks in static.",
            Genre::Steampunk => "Brass gears turn with purpose. Steam carries ambition.",
            Genre::Solarpunk => "Green shoots through concrete. Hope grows in cracks.",
            Genre::DarkFantasy => "Shadows hold secrets. Runes glow with ancient intent.",
        }
    }

    pub fn vocab_path(&self) -> &'static str {
        match self {
            Genre::Cyberpunk => "data/vocab/cyberpunk",
            Genre::Steampunk => "data/vocab/steampunk",
            Genre::Solarpunk => "data/vocab/solarpunk",
            Genre::DarkFantasy => "data/vocab/dark_fantasy",
        }
    }
}

/// Vocabulary tier determines coal value and complexity.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum VocabularyTier {
    /// Common words, 1-5 Coal per correct usage
    #[default]
    Basic,
    /// Intermediate complexity, 5-10 Coal
    Intermediate,
    /// Advanced concepts, 10-20 Coal
    Advanced,
    /// Expert-level, 20-50 Coal
    Expert,
}

impl VocabularyTier {
    pub fn coal_range(&self) -> (u32, u32) {
        match self {
            VocabularyTier::Basic => (1, 5),
            VocabularyTier::Intermediate => (5, 10),
            VocabularyTier::Advanced => (10, 20),
            VocabularyTier::Expert => (20, 50),
        }
    }

    pub fn default_coal(&self) -> u32 {
        let (min, max) = self.coal_range();
        (min + max) / 2
    }
}

/// A single vocabulary word that can be detected and rewarded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyWord {
    /// The primary word/term
    pub word: String,
    /// Alternative forms that also count (e.g., "NPU" matches "neural processing unit")
    pub aliases: Vec<String>,
    /// Words that indicate correct usage context (e.g., ["thread", "parallel", "async"] for "NPU")
    pub context_clues: Vec<String>,
    /// Coal earned for correct usage
    pub coal_value: u32,
    /// Complexity tier
    pub tier: VocabularyTier,
    /// Cognitive level required
    pub bloom_level: BloomLevel,
    /// Optional definition for journal display
    pub definition: Option<String>,
    /// Tags for semantic grouping
    pub tags: Vec<String>,
}
