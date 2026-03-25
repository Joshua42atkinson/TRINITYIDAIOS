// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-quest/src/hero.rs
// ═══════════════════════════════════════════════════════════════════════════════
//
// PURPOSE:     Hero's Journey stages and ADDIE phases
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// ADDIECRAPEYE station (phase)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Analysis,
    Design,
    Development,
    Implementation,
    Evaluation,
    Contrast,
    Repetition,
    Alignment,
    Proximity,
    Envision,
    Yoke,
    Evolve,
}

impl Phase {
    pub fn label(&self) -> &'static str {
        match self {
            Phase::Analysis => "Analysis",
            Phase::Design => "Design",
            Phase::Development => "Development",
            Phase::Implementation => "Implementation",
            Phase::Evaluation => "Evaluation",
            Phase::Contrast => "Contrast",
            Phase::Repetition => "Repetition",
            Phase::Alignment => "Alignment",
            Phase::Proximity => "Proximity",
            Phase::Envision => "Envision",
            Phase::Yoke => "Yoke",
            Phase::Evolve => "Evolve",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Phase::Analysis => "🔍",
            Phase::Design => "🎨",
            Phase::Development => "⚙️",
            Phase::Implementation => "🚀",
            Phase::Evaluation => "📊",
            Phase::Contrast => "🎨",
            Phase::Repetition => "🔄",
            Phase::Alignment => "📐",
            Phase::Proximity => "🤝",
            Phase::Envision => "👁️",
            Phase::Yoke => "🔗",
            Phase::Evolve => "🫁",
        }
    }

    pub fn next(&self) -> Option<Phase> {
        match self {
            Phase::Analysis => Some(Phase::Design),
            Phase::Design => Some(Phase::Development),
            Phase::Development => Some(Phase::Implementation),
            Phase::Implementation => Some(Phase::Evaluation),
            Phase::Evaluation => Some(Phase::Contrast),
            Phase::Contrast => Some(Phase::Repetition),
            Phase::Repetition => Some(Phase::Alignment),
            Phase::Alignment => Some(Phase::Proximity),
            Phase::Proximity => Some(Phase::Envision),
            Phase::Envision => Some(Phase::Yoke),
            Phase::Yoke => Some(Phase::Evolve),
            Phase::Evolve => None,
        }
    }

    pub fn dc(&self) -> u8 {
        match self {
            Phase::Analysis => 10,
            Phase::Design => 12,
            Phase::Development => 15,
            Phase::Implementation => 18,
            Phase::Evaluation => 20,
            Phase::Contrast => 22,
            Phase::Repetition => 25,
            Phase::Alignment => 28,
            Phase::Proximity => 30,
            Phase::Envision => 32,
            Phase::Yoke => 35,
            Phase::Evolve => 40,
        }
    }

    pub fn blooms(&self) -> &'static str {
        match self {
            Phase::Analysis => "Remember",
            Phase::Design => "Understand",
            Phase::Development => "Apply",
            Phase::Implementation => "Apply",
            Phase::Evaluation => "Analyze",
            Phase::Contrast => "Analyze",
            Phase::Repetition => "Evaluate",
            Phase::Alignment => "Evaluate",
            Phase::Proximity => "Create",
            Phase::Envision => "Create",
            Phase::Yoke => "Create",
            Phase::Evolve => "Create",
        }
    }

    pub fn agent(&self) -> &'static str {
        match self {
            Phase::Analysis => "P — Pete (Socratic Mirror)",
            Phase::Design => "A — Aesthetics (The Visionary)",
            Phase::Development => "T — Tempo (The Engineer)",
            Phase::Implementation => "T — Tempo (The Engineer)",
            Phase::Evaluation => "R — Research (The Brakeman)",
            Phase::Contrast => "A — Aesthetics (The Visionary)",
            Phase::Repetition => "T — Tempo (The Engineer)",
            Phase::Alignment => "R — Research (The Brakeman)",
            Phase::Proximity => "A — Aesthetics (The Visionary)",
            Phase::Envision => "P — Pete (Socratic Mirror)",
            Phase::Yoke => "P-ART Swarm (All Hands)",
            Phase::Evolve => "Y — Yardmaster (The User Ships)",
        }
    }

    /// Zero-indexed phase number (0=Analysis through 11=Evolve).
    /// WHY: scan_text() needs a u8 phase index so the Pythagorean taming system
    ///      can track encounter breadth across different ADDIECRAPEYE stations.
    pub fn phase_index(&self) -> u8 {
        match self {
            Phase::Analysis => 0,
            Phase::Design => 1,
            Phase::Development => 2,
            Phase::Implementation => 3,
            Phase::Evaluation => 4,
            Phase::Contrast => 5,
            Phase::Repetition => 6,
            Phase::Alignment => 7,
            Phase::Proximity => 8,
            Phase::Envision => 9,
            Phase::Yoke => 10,
            Phase::Evolve => 11,
        }
    }

    /// Sacred Circuitry quadrant for this phase.
    /// WHY: Context variety (25% of taming score) requires knowing which
    ///      quadrant of the instructional cycle a word was encountered in.
    ///   Scope (0) = ADDIE stations (Analysis–Evaluation)
    ///   Build (1) = CRAP stations (Contrast–Proximity)
    ///   Ship  (2) = EYE stations (Envision–Evolve)
    pub fn quadrant(&self) -> u8 {
        match self {
            Phase::Analysis
            | Phase::Design
            | Phase::Development
            | Phase::Implementation
            | Phase::Evaluation => 0, // Scope
            Phase::Contrast | Phase::Repetition | Phase::Alignment | Phase::Proximity => 1, // Build
            Phase::Envision | Phase::Yoke | Phase::Evolve => 2,                             // Ship
        }
    }

    /// Construct a Phase from its zero-based index (inverse of phase_index).
    pub fn from_index(index: u8) -> Option<Phase> {
        match index {
            0 => Some(Phase::Analysis),
            1 => Some(Phase::Design),
            2 => Some(Phase::Development),
            3 => Some(Phase::Implementation),
            4 => Some(Phase::Evaluation),
            5 => Some(Phase::Contrast),
            6 => Some(Phase::Repetition),
            7 => Some(Phase::Alignment),
            8 => Some(Phase::Proximity),
            9 => Some(Phase::Envision),
            10 => Some(Phase::Yoke),
            11 => Some(Phase::Evolve),
            _ => None,
        }
    }

    /// ADDIECRAPEYE group name: "ADDIE", "CRAP", or "EYE"
    pub fn group(&self) -> &'static str {
        match self {
            Phase::Analysis
            | Phase::Design
            | Phase::Development
            | Phase::Implementation
            | Phase::Evaluation => "ADDIE",
            Phase::Contrast | Phase::Repetition | Phase::Alignment | Phase::Proximity => "CRAP",
            Phase::Envision | Phase::Yoke | Phase::Evolve => "EYE",
        }
    }

    /// Sacred Circuitry quadrant name: "Scope", "Build", or "Ship"
    pub fn circuit_name(&self) -> &'static str {
        match self.quadrant() {
            0 => "Scope",
            1 => "Build",
            _ => "Ship",
        }
    }

    /// Minimum Steam required to advance past this phase.
    /// Steam is generated by productive tool calls (writing code, generating content).
    /// ADDIE stations = lower threshold (learning), CRAP stations = moderate (building),
    /// EYE stations = highest (shipping). You must DO WORK to advance.
    pub fn steam_required(&self) -> f32 {
        match self {
            // ADDIE — learning and scoping (lower barrier)
            Phase::Analysis => 10.0,
            Phase::Design => 15.0,
            Phase::Development => 25.0,
            Phase::Implementation => 30.0,
            Phase::Evaluation => 20.0,
            // CRAP — building and polishing (moderate barrier)
            Phase::Contrast => 25.0,
            Phase::Repetition => 30.0,
            Phase::Alignment => 35.0,
            Phase::Proximity => 40.0,
            // EYE — shipping and reflecting (highest barrier)
            Phase::Envision => 35.0,
            Phase::Yoke => 50.0,
            Phase::Evolve => 60.0,
        }
    }

    /// All 12 phases in order, for serializing to the frontend.
    pub fn all_phases() -> Vec<Phase> {
        (0..12).filter_map(Phase::from_index).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_index_roundtrip() {
        for i in 0..12u8 {
            let phase = Phase::from_index(i).expect(&format!("from_index({}) should work", i));
            assert_eq!(
                phase.phase_index(),
                i,
                "phase_index() roundtrip failed for {}",
                i
            );
        }
        assert!(Phase::from_index(12).is_none());
        assert!(Phase::from_index(255).is_none());
    }

    #[test]
    fn test_quadrant_boundaries() {
        // ADDIE = Scope (0)
        assert_eq!(Phase::Analysis.quadrant(), 0);
        assert_eq!(Phase::Evaluation.quadrant(), 0);
        // CRAP = Build (1)
        assert_eq!(Phase::Contrast.quadrant(), 1);
        assert_eq!(Phase::Proximity.quadrant(), 1);
        // EYE = Ship (2)
        assert_eq!(Phase::Envision.quadrant(), 2);
        assert_eq!(Phase::Evolve.quadrant(), 2);
    }
}

/// Campbell's 12 stages of the Hero's Journey
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeroStage {
    OrdinaryWorld,
    CallToAdventure,
    RefusalOfTheCall,
    MeetingTheMentor,
    CrossingTheThreshold,
    TestsAlliesEnemies,
    ApproachToInmostCave,
    TheOrdeal,
    TheReward,
    TheRoadBack,
    TheResurrection,
    ReturnWithElixir,
}

impl HeroStage {
    pub fn chapter(&self) -> u8 {
        match self {
            Self::OrdinaryWorld => 1,
            Self::CallToAdventure => 2,
            Self::RefusalOfTheCall => 3,
            Self::MeetingTheMentor => 4,
            Self::CrossingTheThreshold => 5,
            Self::TestsAlliesEnemies => 6,
            Self::ApproachToInmostCave => 7,
            Self::TheOrdeal => 8,
            Self::TheReward => 9,
            Self::TheRoadBack => 10,
            Self::TheResurrection => 11,
            Self::ReturnWithElixir => 12,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::OrdinaryWorld => "The Ordinary World",
            Self::CallToAdventure => "The Call to Adventure",
            Self::RefusalOfTheCall => "Refusal of the Call",
            Self::MeetingTheMentor => "Meeting the Mentor",
            Self::CrossingTheThreshold => "Crossing the Threshold",
            Self::TestsAlliesEnemies => "Tests, Allies, & Enemies",
            Self::ApproachToInmostCave => "Approach to the Inmost Cave",
            Self::TheOrdeal => "The Ordeal",
            Self::TheReward => "The Reward",
            Self::TheRoadBack => "The Road Back",
            Self::TheResurrection => "The Resurrection",
            Self::ReturnWithElixir => "Return with the Elixir",
        }
    }

    pub fn act(&self) -> &'static str {
        match self.chapter() {
            1..=5 => "Act I: Departure",
            6..=9 => "Act II: Initiation",
            10..=12 => "Act III: Return",
            _ => "Unknown",
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            Self::OrdinaryWorld => Some(Self::CallToAdventure),
            Self::CallToAdventure => Some(Self::RefusalOfTheCall),
            Self::RefusalOfTheCall => Some(Self::MeetingTheMentor),
            Self::MeetingTheMentor => Some(Self::CrossingTheThreshold),
            Self::CrossingTheThreshold => Some(Self::TestsAlliesEnemies),
            Self::TestsAlliesEnemies => Some(Self::ApproachToInmostCave),
            Self::ApproachToInmostCave => Some(Self::TheOrdeal),
            Self::TheOrdeal => Some(Self::TheReward),
            Self::TheReward => Some(Self::TheRoadBack),
            Self::TheRoadBack => Some(Self::TheResurrection),
            Self::TheResurrection => Some(Self::ReturnWithElixir),
            Self::ReturnWithElixir => None,
        }
    }
}
