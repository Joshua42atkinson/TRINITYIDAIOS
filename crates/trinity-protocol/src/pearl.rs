// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-protocol/src/pearl.rs
// ═══════════════════════════════════════════════════════════════════════════════
//
// PURPOSE:     The PEARL — Perspective Engineering Aesthetic Research Layout
//
// The PEARL is the focusing agent for each project. It captures:
//   Subject — What the SME knows (the pearl of wisdom)
//   Medium  — How it should be delivered (game, storyboard, simulation...)
//   Vision  — What the user expects to FEEL at the end (the vibe)
//
// The PEARL sits between CharacterSheet (WHO) and QuestState (WHERE):
//   CharacterSheet = persistent identity across projects
//   PEARL          = per-project focus (WHAT + WHY + HOW)
//   QuestState     = progress through the PEARL via ADDIECRAPEYE
//
// Every ADDIECRAPEYE phase checks alignment against the PEARL:
//   ADDIE (1-5):  Extract the PEARL — who is the user, what do they teach?
//   CRAP  (6-9):  Place the PEARL — design the experience around the wisdom
//   EYE  (10-12): Refine the PEARL — reflect, iterate, evolve
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

// ============================================================================
// PEARL MEDIUM — How the wisdom should be delivered
// ============================================================================

/// The delivery format for the SME's wisdom.
/// Determines which ART sidecar tools are most relevant and
/// what kind of output ADDIECRAPEYE should produce.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum PearlMedium {
    /// Interactive Bevy game — full scaffold, ECS systems, game loop
    #[default]
    Game,
    /// Visual storyboard — sequential frames, branching narrative
    Storyboard,
    /// Interactive simulation — physics, cause-and-effect sandbox
    Simulation,
    /// Structured lesson plan — ADDIE-aligned document
    LessonPlan,
    /// Assessment instrument — rubrics, quizzes, formative checks
    Assessment,
    /// Narrative book — chapters, illustrations, reflections
    Book,
    /// Custom medium specified by the user
    Other(String),
}

impl PearlMedium {
    pub fn display_name(&self) -> &str {
        match self {
            PearlMedium::Game => "Game",
            PearlMedium::Storyboard => "Storyboard",
            PearlMedium::Simulation => "Simulation",
            PearlMedium::LessonPlan => "Lesson Plan",
            PearlMedium::Assessment => "Assessment",
            PearlMedium::Book => "Book",
            PearlMedium::Other(s) => s.as_str(),
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            PearlMedium::Game => "🎮",
            PearlMedium::Storyboard => "🎬",
            PearlMedium::Simulation => "🔬",
            PearlMedium::LessonPlan => "📋",
            PearlMedium::Assessment => "📝",
            PearlMedium::Book => "📖",
            PearlMedium::Other(_) => "✨",
        }
    }

    /// Which ART tools are most relevant for this medium.
    /// Used by the conductor to suggest party assignments.
    pub fn suggested_tools(&self) -> &[&str] {
        match self {
            PearlMedium::Game => &["scaffold_bevy_game", "comfyui_image", "musicgpt"],
            PearlMedium::Storyboard => &["comfyui_image", "narrative_engine"],
            PearlMedium::Simulation => &["scaffold_bevy_game", "comfyui_image"],
            PearlMedium::LessonPlan => &["narrative_engine", "gdd_compile"],
            PearlMedium::Assessment => &["gdd_compile", "qm_rubric"],
            PearlMedium::Book => &["narrative_engine", "comfyui_image"],
            PearlMedium::Other(_) => &["narrative_engine"],
        }
    }
}

// ============================================================================
// PEARL PHASE — Where in Extract → Place → Refine
// ============================================================================

/// The cognitive lifecycle phase of the PEARL.
/// Maps to the three groups of ADDIECRAPEYE:
///   Extracting = ADDIE (stations 1-5) — pull wisdom out of the SME
///   Placing    = CRAP  (stations 6-9) — design the experience around it
///   Refining   = EYE   (stations 10-12) — reflect, iterate, ship
///   Polished   = complete — the PEARL is ready for the Book of the Bible
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum PearlPhase {
    /// ADDIE (1-5): Extract the pearl of wisdom from the SME.
    /// Bloom's: Remember → Understand → Apply → Apply → Evaluate
    #[default]
    Extracting,
    /// CRAP (6-9): Place the wisdom into concrete design artifacts.
    /// Bloom's: Analyze → Apply → Evaluate → Analyze
    Placing,
    /// EYE (10-12): Refine through reflection and iteration.
    /// Bloom's: Evaluate → Create → Create
    Refining,
    /// The PEARL is complete — all 12 stations passed alignment check.
    Polished,
}

impl PearlPhase {
    pub fn display_name(&self) -> &str {
        match self {
            PearlPhase::Extracting => "Extracting",
            PearlPhase::Placing => "Placing",
            PearlPhase::Refining => "Refining",
            PearlPhase::Polished => "Polished",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            PearlPhase::Extracting => "🦪",
            PearlPhase::Placing => "💎",
            PearlPhase::Refining => "✨",
            PearlPhase::Polished => "🌟",
        }
    }

    /// Advance to the next phase. Returns None if already Polished.
    pub fn next(&self) -> Option<PearlPhase> {
        match self {
            PearlPhase::Extracting => Some(PearlPhase::Placing),
            PearlPhase::Placing => Some(PearlPhase::Refining),
            PearlPhase::Refining => Some(PearlPhase::Polished),
            PearlPhase::Polished => None,
        }
    }

    /// Map an ADDIECRAPEYE station index (1-12) to the corresponding PearlPhase.
    pub fn from_station(station: u8) -> PearlPhase {
        match station {
            1..=5 => PearlPhase::Extracting,
            6..=9 => PearlPhase::Placing,
            10..=12 => PearlPhase::Refining,
            _ => PearlPhase::Extracting, // fallback
        }
    }
}

// ============================================================================
// PEARL EVALUATION — Alignment scores per ADDIECRAPEYE group
// ============================================================================

/// Alignment scores measuring how well the work product matches the PEARL.
/// Each score is 0.0–1.0. The overall alignment is the weighted average.
///
/// Scoring:
///   0.0 = no alignment (work product drifted completely from PEARL)
///   0.5 = partial alignment (some objectives met, scope drifting)
///   0.8 = strong alignment (PEARL-aligned, minor refinements needed)
///   1.0 = perfect alignment (work product IS the PEARL made manifest)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PearlEvaluation {
    /// ADDIE score (stations 1-5): Did we extract the right wisdom?
    /// Weight: 40% — the foundation must be solid.
    pub addie_score: f32,

    /// CRAP score (stations 6-9): Is the design faithful to the wisdom?
    /// Weight: 35% — the design must serve the pedagogy.
    pub crap_score: f32,

    /// EYE score (stations 10-12): Does the output match the vision?
    /// Weight: 25% — the final polish reflects the user's intent.
    pub eye_score: f32,
}

impl Default for PearlEvaluation {
    fn default() -> Self {
        Self {
            addie_score: 0.0,
            crap_score: 0.0,
            eye_score: 0.0,
        }
    }
}

impl PearlEvaluation {
    /// Weighted overall alignment score.
    /// ADDIE 40%, CRAP 35%, EYE 25%.
    pub fn overall_alignment(&self) -> f32 {
        let raw = (self.addie_score * 0.40) + (self.crap_score * 0.35) + (self.eye_score * 0.25);
        raw.clamp(0.0, 1.0)
    }

    /// Whether the PEARL is aligned enough for the user to advance.
    /// Threshold: 0.6 — you don't need perfection, but you need intent alignment.
    pub fn is_aligned(&self) -> bool {
        self.overall_alignment() >= 0.6
    }

    /// Update the score for the current PEARL phase group.
    pub fn update_score(&mut self, phase: PearlPhase, score: f32) {
        let clamped = score.clamp(0.0, 1.0);
        match phase {
            PearlPhase::Extracting => self.addie_score = clamped,
            PearlPhase::Placing => self.crap_score = clamped,
            PearlPhase::Refining => self.eye_score = clamped,
            PearlPhase::Polished => {} // Already complete
        }
    }

    /// Compact display for UI badges.
    pub fn grade(&self) -> &str {
        let score = self.overall_alignment();
        if score >= 0.9 {
            "A+"
        } else if score >= 0.8 {
            "A"
        } else if score >= 0.7 {
            "B+"
        } else if score >= 0.6 {
            "B"
        } else if score >= 0.5 {
            "C"
        } else if score >= 0.3 {
            "D"
        } else {
            "F"
        }
    }
}

// ============================================================================
// THE PEARL — The focusing agent
// ============================================================================

/// The PEARL — per-project alignment document.
///
/// Created when the user selects a subject and begins a quest.
/// Persisted alongside QuestState. Evaluated at every phase transition.
///
/// The PEARL is to the quest what the CharacterSheet is to the user:
///   CharacterSheet = WHO (persistent identity)
///   PEARL          = WHAT (per-project focus)
///   QuestState     = WHERE (progress through ADDIECRAPEYE)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pearl {
    /// The SME's domain of expertise.
    /// "What pearl of wisdom do they carry?"
    pub subject: String,

    /// How the wisdom should be delivered.
    /// "A game? A storyboard? A simulation? A book?"
    pub medium: PearlMedium,

    /// What the user expects to FEEL at the end.
    /// One sentence capturing the vibe, the intent, the desired emotional outcome.
    /// "Students feel like they discovered Newton's laws themselves."
    pub vision: String,

    /// Current cognitive lifecycle phase.
    pub phase: PearlPhase,

    /// Alignment scores per ADDIECRAPEYE group.
    pub evaluation: PearlEvaluation,

    /// ISO timestamp when the PEARL was created.
    pub created_at: String,

    /// How many times the user has consciously refined the PEARL.
    /// Each refinement signals deepening understanding — the autopoiesis loop.
    pub refined_count: u32,
}

impl Pearl {
    /// Create a new PEARL for a subject. Medium and vision can be refined later.
    pub fn new(subject: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            medium: PearlMedium::default(),
            vision: String::new(),
            phase: PearlPhase::default(),
            evaluation: PearlEvaluation::default(),
            created_at: chrono::Utc::now().to_rfc3339(),
            refined_count: 0,
        }
    }

    /// Create a fully specified PEARL — subject, medium, and vision all set.
    pub fn with_vision(
        subject: impl Into<String>,
        medium: PearlMedium,
        vision: impl Into<String>,
    ) -> Self {
        Self {
            subject: subject.into(),
            medium,
            vision: vision.into(),
            phase: PearlPhase::default(),
            evaluation: PearlEvaluation::default(),
            created_at: chrono::Utc::now().to_rfc3339(),
            refined_count: 0,
        }
    }

    /// Refine the PEARL — user has consciously updated their vision or medium.
    /// This is the autopoiesis loop: the PEARL deepens as the user learns.
    pub fn refine(&mut self, vision: Option<String>, medium: Option<PearlMedium>) {
        if let Some(v) = vision {
            self.vision = v;
        }
        if let Some(m) = medium {
            self.medium = m;
        }
        self.refined_count += 1;
    }

    /// Update the PEARL phase based on the current ADDIECRAPEYE station (1-12).
    pub fn sync_phase_from_station(&mut self, station: u8) {
        self.phase = PearlPhase::from_station(station);
    }

    /// Check if the PEARL is aligned enough to advance to the next phase group.
    pub fn alignment_check(&self) -> bool {
        self.evaluation.is_aligned()
    }

    /// Generate a compact summary for injection into conductor system prompts.
    /// This tells Pete: what the user is building, in what format, and why.
    pub fn prompt_summary(&self) -> String {
        let mut parts = vec![
            format!(
                "PEARL: {} via {} {}",
                self.subject,
                self.medium.icon(),
                self.medium.display_name()
            ),
            format!("Phase: {} {}", self.phase.icon(), self.phase.display_name()),
        ];

        if !self.vision.is_empty() {
            parts.push(format!("Vision: \"{}\"", self.vision));
        } else {
            parts.push(
                "⚠ No vision set — ask the user: 'What do you want to FEEL when this is done?'"
                    .to_string(),
            );
        }

        let alignment = self.evaluation.overall_alignment();
        if alignment > 0.0 {
            parts.push(format!(
                "Alignment: {:.0}% ({}) | ADDIE:{:.0}% CRAP:{:.0}% EYE:{:.0}%",
                alignment * 100.0,
                self.evaluation.grade(),
                self.evaluation.addie_score * 100.0,
                self.evaluation.crap_score * 100.0,
                self.evaluation.eye_score * 100.0,
            ));
        }

        if self.refined_count > 0 {
            parts.push(format!("Refined {}×", self.refined_count));
        }

        parts.join(" | ")
    }

    /// Mark the PEARL as polished (all 12 stations complete and aligned).
    pub fn polish(&mut self) {
        self.phase = PearlPhase::Polished;
    }

    /// Whether the PEARL has a vision set.
    pub fn has_vision(&self) -> bool {
        !self.vision.is_empty()
    }
}

impl Default for Pearl {
    fn default() -> Self {
        Self::new("Unspecified")
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pearl_creation_defaults() {
        let pearl = Pearl::new("Ecosystems");
        assert_eq!(pearl.subject, "Ecosystems");
        assert_eq!(pearl.medium, PearlMedium::Game);
        assert!(pearl.vision.is_empty());
        assert_eq!(pearl.phase, PearlPhase::Extracting);
        assert_eq!(pearl.evaluation.overall_alignment(), 0.0);
        assert_eq!(pearl.refined_count, 0);
    }

    #[test]
    fn test_pearl_with_vision() {
        let pearl = Pearl::with_vision(
            "Physics",
            PearlMedium::Simulation,
            "Students feel like they discovered Newton's laws themselves",
        );
        assert_eq!(pearl.subject, "Physics");
        assert_eq!(pearl.medium, PearlMedium::Simulation);
        assert!(pearl.has_vision());
        assert_eq!(pearl.phase, PearlPhase::Extracting);
    }

    #[test]
    fn test_pearl_phase_transitions() {
        assert_eq!(PearlPhase::Extracting.next(), Some(PearlPhase::Placing));
        assert_eq!(PearlPhase::Placing.next(), Some(PearlPhase::Refining));
        assert_eq!(PearlPhase::Refining.next(), Some(PearlPhase::Polished));
        assert_eq!(PearlPhase::Polished.next(), None);
    }

    #[test]
    fn test_pearl_phase_from_station() {
        // ADDIE stations 1-5
        for s in 1..=5 {
            assert_eq!(PearlPhase::from_station(s), PearlPhase::Extracting);
        }
        // CRAP stations 6-9
        for s in 6..=9 {
            assert_eq!(PearlPhase::from_station(s), PearlPhase::Placing);
        }
        // EYE stations 10-12
        for s in 10..=12 {
            assert_eq!(PearlPhase::from_station(s), PearlPhase::Refining);
        }
    }

    #[test]
    fn test_evaluation_scoring_and_alignment() {
        let mut eval = PearlEvaluation::default();
        assert_eq!(eval.overall_alignment(), 0.0);
        assert!(!eval.is_aligned());

        // Set ADDIE score high — not enough alone (40% weight)
        eval.addie_score = 1.0;
        assert!(!eval.is_aligned()); // 0.40 < 0.60

        // Add CRAP score — now aligned (40% + 35% = 75%)
        eval.crap_score = 1.0;
        assert!(eval.is_aligned()); // 0.75 >= 0.60

        // Full scores
        eval.eye_score = 1.0;
        assert!((eval.overall_alignment() - 1.0).abs() < 0.01);
        assert_eq!(eval.grade(), "A+");
    }

    #[test]
    fn test_evaluation_update_by_phase() {
        let mut eval = PearlEvaluation::default();
        eval.update_score(PearlPhase::Extracting, 0.8);
        assert!((eval.addie_score - 0.8).abs() < 0.01);

        eval.update_score(PearlPhase::Placing, 0.7);
        assert!((eval.crap_score - 0.7).abs() < 0.01);

        eval.update_score(PearlPhase::Refining, 0.9);
        assert!((eval.eye_score - 0.9).abs() < 0.01);

        // Polished phase doesn't change anything
        eval.update_score(PearlPhase::Polished, 0.5);
        assert!((eval.eye_score - 0.9).abs() < 0.01);
    }

    #[test]
    fn test_evaluation_clamping() {
        let mut eval = PearlEvaluation::default();
        eval.update_score(PearlPhase::Extracting, 1.5);
        assert!((eval.addie_score - 1.0).abs() < 0.01);

        eval.update_score(PearlPhase::Placing, -0.5);
        assert!((eval.crap_score - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_pearl_refine() {
        let mut pearl = Pearl::new("History");
        assert_eq!(pearl.refined_count, 0);

        pearl.refine(
            Some("Students time-travel through primary sources".to_string()),
            Some(PearlMedium::Storyboard),
        );
        assert_eq!(pearl.refined_count, 1);
        assert_eq!(pearl.medium, PearlMedium::Storyboard);
        assert!(pearl.has_vision());

        // Partial refine — only update vision
        pearl.refine(
            Some("Students argue historical cases like lawyers".to_string()),
            None,
        );
        assert_eq!(pearl.refined_count, 2);
        assert_eq!(pearl.medium, PearlMedium::Storyboard); // unchanged
    }

    #[test]
    fn test_pearl_sync_and_polish() {
        let mut pearl = Pearl::new("Math");

        pearl.sync_phase_from_station(3);
        assert_eq!(pearl.phase, PearlPhase::Extracting);

        pearl.sync_phase_from_station(7);
        assert_eq!(pearl.phase, PearlPhase::Placing);

        pearl.sync_phase_from_station(11);
        assert_eq!(pearl.phase, PearlPhase::Refining);

        pearl.polish();
        assert_eq!(pearl.phase, PearlPhase::Polished);
    }

    #[test]
    fn test_pearl_prompt_summary() {
        let mut pearl = Pearl::with_vision(
            "Physics",
            PearlMedium::Game,
            "Students discover gravity through play",
        );
        pearl.evaluation.addie_score = 0.85;
        pearl.evaluation.crap_score = 0.70;
        pearl.refined_count = 2;

        let summary = pearl.prompt_summary();
        assert!(summary.contains("Physics"));
        assert!(summary.contains("🎮"));
        assert!(summary.contains("Game"));
        assert!(summary.contains("discover gravity"));
        assert!(summary.contains("Refined 2×"));
        assert!(summary.contains("ADDIE:85%"));
    }

    #[test]
    fn test_pearl_serialization_roundtrip() {
        let pearl = Pearl::with_vision(
            "Chemistry",
            PearlMedium::Simulation,
            "Students feel like alchemists mixing potions",
        );

        let json = serde_json::to_string(&pearl).expect("serialize");
        let restored: Pearl = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(restored.subject, "Chemistry");
        assert_eq!(restored.medium, PearlMedium::Simulation);
        assert_eq!(
            restored.vision,
            "Students feel like alchemists mixing potions"
        );
        assert_eq!(restored.phase, PearlPhase::Extracting);
    }

    #[test]
    fn test_pearl_medium_properties() {
        assert_eq!(PearlMedium::Game.display_name(), "Game");
        assert_eq!(PearlMedium::Game.icon(), "🎮");
        assert!(!PearlMedium::Game.suggested_tools().is_empty());

        let custom = PearlMedium::Other("VR Experience".to_string());
        assert_eq!(custom.display_name(), "VR Experience");
        assert_eq!(custom.icon(), "✨");
    }

    #[test]
    fn test_evaluation_grades() {
        let mut eval = PearlEvaluation::default();

        // F grade
        assert_eq!(eval.grade(), "F");

        // D grade
        eval.addie_score = 0.5;
        eval.crap_score = 0.3;
        assert_eq!(eval.grade(), "D");

        // B grade
        eval.addie_score = 0.8;
        eval.crap_score = 0.8;
        eval.eye_score = 0.0;
        assert_eq!(eval.grade(), "B");

        // A+ grade
        eval.addie_score = 1.0;
        eval.crap_score = 1.0;
        eval.eye_score = 1.0;
        assert_eq!(eval.grade(), "A+");
    }
}
