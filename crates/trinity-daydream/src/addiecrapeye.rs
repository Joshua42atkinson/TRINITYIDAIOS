// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — ADDIECRAPEYE Phase Definitions
// ═══════════════════════════════════════════════════════════════════════════════
//
// SOURCE OF TRUTH: docs/active/ADDIECRAPEYE_CANONICAL.md
//
// 12 stations × 3 acts:
//   ACT I  (ADD)    — The Departure: Analyze, Design, Develop
//   ACT II (IECRAP) — The Initiation: Implement, Evaluate, Contrast, Repetition, Alignment, Proximity
//   ACT III (EYE)   — The Return: Envision, Yoke, Evolve
//
// ═══════════════════════════════════════════════════════════════════════════════

/// The 12 ADDIECRAPEYE phases — canonical names.
///
/// **ADDIE** = Purdue Instructional Design
/// **CRAP**  = Robin Williams' design philosophy (Contrast, Repetition, Alignment, Proximity)
/// **EYE**   = Meta-awareness (Envision, Yoke, Evolve)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Analyze,
    Design,
    Develop,
    Implement,
    Evaluate,
    Contrast,
    Repetition,
    Alignment,
    Proximity,
    Envision,
    Yoke,
    Evolve,
}

impl Phase {
    /// All 12 phases in order.
    pub const ALL: [Phase; 12] = [
        Phase::Analyze,
        Phase::Design,
        Phase::Develop,
        Phase::Implement,
        Phase::Evaluate,
        Phase::Contrast,
        Phase::Repetition,
        Phase::Alignment,
        Phase::Proximity,
        Phase::Envision,
        Phase::Yoke,
        Phase::Evolve,
    ];

    /// Which act this phase belongs to.
    pub fn act(self) -> Act {
        match self {
            Phase::Analyze | Phase::Design | Phase::Develop => Act::Departure,
            Phase::Implement
            | Phase::Evaluate
            | Phase::Contrast
            | Phase::Repetition
            | Phase::Alignment
            | Phase::Proximity => Act::Initiation,
            Phase::Envision | Phase::Yoke | Phase::Evolve => Act::Return,
        }
    }

    /// 1-indexed station number.
    pub fn station(self) -> u8 {
        match self {
            Phase::Analyze => 1,
            Phase::Design => 2,
            Phase::Develop => 3,
            Phase::Implement => 4,
            Phase::Evaluate => 5,
            Phase::Contrast => 6,
            Phase::Repetition => 7,
            Phase::Alignment => 8,
            Phase::Proximity => 9,
            Phase::Envision => 10,
            Phase::Yoke => 11,
            Phase::Evolve => 12,
        }
    }

    /// Short, human-readable name.
    pub fn label(self) -> &'static str {
        match self {
            Phase::Analyze => "Analyze",
            Phase::Design => "Design",
            Phase::Develop => "Develop",
            Phase::Implement => "Implement",
            Phase::Evaluate => "Evaluate",
            Phase::Contrast => "Contrast",
            Phase::Repetition => "Repetition",
            Phase::Alignment => "Alignment",
            Phase::Proximity => "Proximity",
            Phase::Envision => "Envision",
            Phase::Yoke => "Yoke",
            Phase::Evolve => "Evolve",
        }
    }

    /// Emoji icon for UI display.
    pub fn emoji(self) -> &'static str {
        match self {
            Phase::Analyze => "👀",
            Phase::Design => "🧠",
            Phase::Develop => "🦴",
            Phase::Implement => "🥩",
            Phase::Evaluate => "⚡",
            Phase::Contrast => "🧥",
            Phase::Repetition => "❤️",
            Phase::Alignment => "🦴",
            Phase::Proximity => "✋",
            Phase::Envision => "👁️",
            Phase::Yoke => "🔗",
            Phase::Evolve => "🫁",
        }
    }

    /// Hero's Journey stage name.
    pub fn hero_stage(self) -> &'static str {
        match self {
            Phase::Analyze => "The Ordinary World",
            Phase::Design => "The Call to Adventure",
            Phase::Develop => "Refusal of the Call",
            Phase::Implement => "Meeting the Mentor",
            Phase::Evaluate => "Crossing the Threshold",
            Phase::Contrast => "Tests, Allies, Enemies",
            Phase::Repetition => "Approach to Inmost Cave",
            Phase::Alignment => "The Ordeal",
            Phase::Proximity => "The Reward",
            Phase::Envision => "The Road Back",
            Phase::Yoke => "The Resurrection",
            Phase::Evolve => "Return with the Elixir",
        }
    }

    /// Body part for the Digital Golem.
    pub fn body_part(self) -> &'static str {
        match self {
            Phase::Analyze => "Eyes / Sensory Organs",
            Phase::Design => "The Brain",
            Phase::Develop => "The Skeleton",
            Phase::Implement => "The Muscles",
            Phase::Evaluate => "The Nervous System",
            Phase::Contrast => "The Skin / Hide",
            Phase::Repetition => "The Heart / Circulatory",
            Phase::Alignment => "The Spine",
            Phase::Proximity => "The Hands / Digits",
            Phase::Envision => "The Third Eye",
            Phase::Yoke => "Connective Tissue / Joints",
            Phase::Evolve => "Breath / Lungs",
        }
    }

    /// Location name in the Dumpster Universe.
    pub fn location(self) -> &'static str {
        match self {
            Phase::Analyze => "The Junkyard Peak",
            Phase::Design => "Blueprint Mesa",
            Phase::Develop => "The DAYDREAM Workshop",
            Phase::Implement => "The Proving Grounds",
            Phase::Evaluate => "The Friction Wastes",
            Phase::Contrast => "The Neon Chasm",
            Phase::Repetition => "The Loop Engine",
            Phase::Alignment => "The Great Chokepoint",
            Phase::Proximity => "The Optimization Yards",
            Phase::Envision => "The Overlook",
            Phase::Yoke => "The Grand Coupling",
            Phase::Evolve => "Conscious Framework Terminal",
        }
    }

    /// Party member(s) active at this phase.
    pub fn party_member(self) -> &'static str {
        match self {
            Phase::Analyze => "Ask Pete & The Evaluator",
            Phase::Design => "The Artist & The Evaluator",
            Phase::Develop => "The Engineer (Sword & Shield)",
            Phase::Implement => "The Engineer",
            Phase::Evaluate => "The Brakeman",
            Phase::Contrast => "The Visionary",
            Phase::Repetition => "The Engineer",
            Phase::Alignment => "The Evaluator",
            Phase::Proximity => "The Visionary & The Artist",
            Phase::Envision => "Ask Pete",
            Phase::Yoke => "The Entire Party",
            Phase::Evolve => "The Great Recycler",
        }
    }

    /// One-line description of what this phase does.
    pub fn description(self) -> &'static str {
        match self {
            Phase::Analyze => "Identify learning needs, gather SME input, define scope",
            Phase::Design => "Create learning objectives, structure, assessments",
            Phase::Develop => "Build content, materials, and learning resources",
            Phase::Implement => "Deploy, deliver, and test in the real world",
            Phase::Evaluate => "Measure success against defined metrics",
            Phase::Contrast => "Find what makes your design stand out from the forgettable",
            Phase::Repetition => "Identify the core concept that must be encountered multiple times",
            Phase::Alignment => "Verify hook → objective → assessment chain integrity",
            Phase::Proximity => "Cluster related content into coherent acts",
            Phase::Envision => "Write the PEARL vision: what the learner will feel",
            Phase::Yoke => "Connect learning objectives to real-world moments",
            Phase::Evolve => "Commit to the Book — the Iron Road continues",
        }
    }

    /// Try to parse a phase from a string (case-insensitive, handles server responses).
    pub fn from_str_loose(s: &str) -> Option<Phase> {
        match s.to_lowercase().trim() {
            "analyze" | "analysis" => Some(Phase::Analyze),
            "design" => Some(Phase::Design),
            "develop" | "development" => Some(Phase::Develop),
            "implement" | "implementation" => Some(Phase::Implement),
            "evaluate" | "evaluation" => Some(Phase::Evaluate),
            "contrast" | "correction" => Some(Phase::Contrast),
            "repetition" | "review" => Some(Phase::Repetition),
            "alignment" | "assessment" => Some(Phase::Alignment),
            "proximity" | "planning" => Some(Phase::Proximity),
            "envision" | "extension" => Some(Phase::Envision),
            "yoke" | "yield" => Some(Phase::Yoke),
            "evolve" | "evolution" | "execution" => Some(Phase::Evolve),
            _ => None,
        }
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.emoji(), self.label())
    }
}

/// The three acts of the ADDIECRAPEYE journey.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Act {
    /// ACT I: The Departure (ADD) — Building the Blueprint & Bones
    Departure,
    /// ACT II: The Initiation (IECRAP) — Fleshing out the World
    Initiation,
    /// ACT III: The Return (EYE) — Meta-Awareness & Release
    Return,
}

impl Act {
    pub fn label(self) -> &'static str {
        match self {
            Act::Departure => "ACT I: The Departure",
            Act::Initiation => "ACT II: The Initiation",
            Act::Return => "ACT III: The Return",
        }
    }

    pub fn acronym(self) -> &'static str {
        match self {
            Act::Departure => "ADD",
            Act::Initiation => "IECRAP",
            Act::Return => "EYE",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_phases_have_correct_count() {
        assert_eq!(Phase::ALL.len(), 12);
    }

    #[test]
    fn stations_are_sequential() {
        for (i, phase) in Phase::ALL.iter().enumerate() {
            assert_eq!(phase.station(), (i + 1) as u8);
        }
    }

    #[test]
    fn acts_partition_correctly() {
        let dep: Vec<_> = Phase::ALL.iter().filter(|p| p.act() == Act::Departure).collect();
        let ini: Vec<_> = Phase::ALL.iter().filter(|p| p.act() == Act::Initiation).collect();
        let ret: Vec<_> = Phase::ALL.iter().filter(|p| p.act() == Act::Return).collect();
        assert_eq!(dep.len(), 3);  // ADD
        assert_eq!(ini.len(), 6);  // IECRAP
        assert_eq!(ret.len(), 3);  // EYE
    }

    #[test]
    fn from_str_loose_handles_legacy_names() {
        // Correct names
        assert_eq!(Phase::from_str_loose("Analyze"), Some(Phase::Analyze));
        assert_eq!(Phase::from_str_loose("Contrast"), Some(Phase::Contrast));
        // Legacy wrong names should still parse
        assert_eq!(Phase::from_str_loose("Correction"), Some(Phase::Contrast));
        assert_eq!(Phase::from_str_loose("Review"), Some(Phase::Repetition));
        assert_eq!(Phase::from_str_loose("Assessment"), Some(Phase::Alignment));
        assert_eq!(Phase::from_str_loose("Planning"), Some(Phase::Proximity));
        assert_eq!(Phase::from_str_loose("Extension"), Some(Phase::Envision));
        assert_eq!(Phase::from_str_loose("Execution"), Some(Phase::Evolve));
    }

    #[test]
    fn display_includes_emoji() {
        let p = Phase::Analyze;
        let s = format!("{}", p);
        assert!(s.contains("Analyze"));
        assert!(s.contains("👀"));
    }
}
