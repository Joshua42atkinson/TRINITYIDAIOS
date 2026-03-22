// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-protocol
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        sacred_circuitry.rs
// PURPOSE:     Sacred Circuitry — 15-word cognitive scaffolding for AI attention
//
// WHAT THIS IS:
//   A 15-concept attention scaffolding system that structures how the AI
//   focuses during any ADDIECRAPEYE workflow. These are not branding or
//   ideology — they are a practical productivity framework that patterns
//   AI attention into repeatable, meaningful sequences.
//
//   The 15 words are VAAM foundation vocabulary: always loaded, genre-
//   independent, and used to scaffold both human and AI cognition around
//   productive attention patterns within the Iron Road.
//
// ARCHITECTURE:
//   • ADDIECRAPEYE = WHAT to do (12-station instructional design process)
//   • Sacred Circuitry = HOW to attend (15-word cognitive scaffolding)
//   • VAAM = the word-economy that rewards correct contextual usage of both
//   • Auto-replies: structured phrases Mistral selects to signal which
//     attention pattern is active — trains contextual usefulness over time
//
// QUADRANTS (attention phases):
//   • Scope:   Center → Expand → Balance → Prepare  (define the problem)
//   • Build:   Express → Extend → Unlock → Flow     (produce the work)
//   • Listen:  Receive → Relate → Realize            (process feedback)
//   • Ship:    Act → Transform → Connect → Manifest  (deliver output)
//
// DESIGN PATTERN SOURCE:
//   Bashar's Sacred Circuitry (15 symbols). Adapted as a systems design
//   foundation for cognitive scaffolding in AI automation.
//
// CHANGES:
//   2026-03-19  Cascade  Initial — 15 circuits + VAAM + auto-reply
//   2026-03-19  Cascade  Reframed for cognitive scaffolding / productivity
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

use crate::character_sheet::BloomLevel;
use crate::vocabulary::{VocabularyTier, VocabularyWord};

/// The 15 Sacred Circuitry concepts — cognitive scaffolding for AI attention.
///
/// Each circuit represents an attention pattern the AI uses during work.
/// The AI may enter any circuit as context demands, but the canonical
/// order represents a productive workflow spiral:
///
///   Scope:  Center → Expand → Balance → Prepare  (define the problem)
///   Build:  Express → Extend → Unlock → Flow      (produce the work)
///   Listen: Receive → Relate → Realize             (process feedback)
///   Ship:   Act → Transform → Connect → Manifest   (deliver output)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
#[derive(Default)]
pub enum Circuit {
    #[default]
    Center = 1,
    Expand = 2,
    Balance = 3,
    Prepare = 4,
    Express = 5,
    Extend = 6,
    Unlock = 7,
    Flow = 8,
    Receive = 9,
    Relate = 10,
    Realize = 11,
    Act = 12,
    Transform = 13,
    Connect = 14,
    Manifest = 15,
}

impl Circuit {
    /// All 15 circuits in canonical order.
    pub const ALL: [Circuit; 15] = [
        Circuit::Center,
        Circuit::Expand,
        Circuit::Balance,
        Circuit::Prepare,
        Circuit::Express,
        Circuit::Extend,
        Circuit::Unlock,
        Circuit::Flow,
        Circuit::Receive,
        Circuit::Relate,
        Circuit::Realize,
        Circuit::Act,
        Circuit::Transform,
        Circuit::Connect,
        Circuit::Manifest,
    ];

    /// The 1-indexed position in the sequence (1–15).
    pub fn order(&self) -> u8 {
        *self as u8
    }

    /// The symbol name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Circuit::Center => "Center",
            Circuit::Expand => "Expand",
            Circuit::Balance => "Balance",
            Circuit::Prepare => "Prepare",
            Circuit::Express => "Express",
            Circuit::Extend => "Extend",
            Circuit::Unlock => "Unlock",
            Circuit::Flow => "Flow",
            Circuit::Receive => "Receive",
            Circuit::Relate => "Relate",
            Circuit::Realize => "Realize",
            Circuit::Act => "Act",
            Circuit::Transform => "Transform",
            Circuit::Connect => "Connect",
            Circuit::Manifest => "Manifest",
        }
    }

    /// Which quadrant this circuit belongs to.
    pub fn quadrant(&self) -> CircuitQuadrant {
        match self {
            Circuit::Center | Circuit::Expand | Circuit::Balance | Circuit::Prepare => {
                CircuitQuadrant::Scope
            }
            Circuit::Express | Circuit::Extend | Circuit::Unlock | Circuit::Flow => {
                CircuitQuadrant::Build
            }
            Circuit::Receive | Circuit::Relate | Circuit::Realize => CircuitQuadrant::Listen,
            Circuit::Act | Circuit::Transform | Circuit::Connect | Circuit::Manifest => {
                CircuitQuadrant::Ship
            }
        }
    }

    /// Map the circuit to an ADDIECRAPEYE station for internal alignment.
    /// This is used by the AI to align its attention pattern with the current phase.
    ///
    /// CANONICAL NAMES (per ADDIECRAPEYE_CANONICAL.md):
    ///   ADDIE = Analyze, Design, Develop, Implement, Evaluate
    ///   CRAP  = Contrast, Repetition, Alignment, Proximity
    ///   EYE   = Envision, Yoke, Evolve
    pub fn addiecrapeye_station(&self) -> &'static str {
        match self {
            // Scope quadrant → ADDIE stations
            Circuit::Center => "Analyze",
            Circuit::Expand => "Design",
            Circuit::Balance => "Design",
            Circuit::Prepare => "Develop",
            // Build quadrant → Implementation + CRAP stations
            Circuit::Express => "Implement",
            Circuit::Extend => "Envision",
            Circuit::Unlock => "Contrast",
            Circuit::Flow => "Repetition",
            // Listen quadrant → Evaluation + CRAP stations
            Circuit::Receive => "Evaluate",
            Circuit::Relate => "Alignment",
            Circuit::Realize => "Proximity",
            // Ship quadrant → EYE stations
            Circuit::Act => "Proximity",
            Circuit::Transform => "Yoke",
            Circuit::Connect => "Yoke",
            Circuit::Manifest => "Evolve",
        }
    }

    /// What this attention pattern does in a workflow.
    pub fn description(&self) -> &'static str {
        match self {
            Circuit::Center =>
                "Lock attention onto the core problem. Filter noise. Identify the single most important thing.",
            Circuit::Expand =>
                "Survey the solution space. What approaches exist? What adjacent knowledge applies?",
            Circuit::Balance =>
                "Weigh tradeoffs. Hold competing constraints without premature commitment.",
            Circuit::Prepare =>
                "Gather resources, check dependencies, verify preconditions before building.",
            Circuit::Express =>
                "Articulate the plan or understanding clearly before executing. State the architecture.",
            Circuit::Extend =>
                "Push the solution beyond the minimum. Apply it to adjacent cases. Generalize.",
            Circuit::Unlock =>
                "Identify and remove blockers. Challenge assumptions that constrain the design.",
            Circuit::Flow =>
                "Enter sustained productive execution. Trust the process. Maintain momentum.",
            Circuit::Receive =>
                "Listen to feedback. Read error messages. Accept input without filtering.",
            Circuit::Relate =>
                "Connect this work to the broader system. How does it fit the codebase, the user, the goal?",
            Circuit::Realize =>
                "The insight moment. Understand the root cause. See what the right approach actually is.",
            Circuit::Act =>
                "Execute. Write the code. Make the change. Stop planning, start doing.",
            Circuit::Transform =>
                "Refactor, improve, make it better than asked. Turn working code into clean code.",
            Circuit::Connect =>
                "Integrate with the rest of the system. Wire up APIs, update tests, resolve conflicts.",
            Circuit::Manifest =>
                "Deliver the tangible output. Ship it. Verify it works. Hand it off.",
        }
    }

    /// Auto-reply templates Mistral selects to signal which attention pattern is active.
    ///
    /// Short, task-oriented phrases. Over time, the pattern of which circuit
    /// the AI selects for which context trains contextual usefulness.
    pub fn auto_replies(&self) -> &'static [&'static str] {
        match self {
            Circuit::Center => &[
                "Centering on the core problem.",
                "What's the single most important thing here?",
                "Filtering noise — locking onto the target.",
            ],
            Circuit::Expand => &[
                "Surveying the solution space.",
                "What other approaches exist?",
                "Widening the view before committing.",
            ],
            Circuit::Balance => &[
                "Weighing tradeoffs.",
                "Both options have costs — comparing them.",
                "Holding the constraints in tension.",
            ],
            Circuit::Prepare => &[
                "Checking preconditions before building.",
                "Gathering dependencies and context.",
                "Setting up the workspace.",
            ],
            Circuit::Express => &[
                "Stating the plan before executing.",
                "Here's my understanding of the architecture.",
                "Articulating the approach clearly.",
            ],
            Circuit::Extend => &[
                "Generalizing this beyond the immediate case.",
                "Pushing the solution further.",
                "Applying this to adjacent modules.",
            ],
            Circuit::Unlock => &[
                "Removing a blocker.",
                "Challenging an assumption that was limiting this.",
                "Found the constraint — working around it.",
            ],
            Circuit::Flow => &[
                "In the groove — sustained execution.",
                "Momentum is good, continuing.",
                "Productive rhythm established.",
            ],
            Circuit::Receive => &[
                "Processing your feedback.",
                "Reading the error output carefully.",
                "Taking in the full context before responding.",
            ],
            Circuit::Relate => &[
                "Connecting this to the broader system.",
                "This pattern matches something in the codebase.",
                "Relating to the existing architecture.",
            ],
            Circuit::Realize => &[
                "Found the root cause.",
                "Key insight: here's what's actually happening.",
                "Now I see the right approach.",
            ],
            Circuit::Act => &["Executing now.", "Writing the code.", "Making the change."],
            Circuit::Transform => &[
                "Refactoring for quality.",
                "Improving beyond the original spec.",
                "Cleaning this up properly.",
            ],
            Circuit::Connect => &[
                "Wiring into the rest of the system.",
                "Updating tests and integrations.",
                "Connecting the pieces.",
            ],
            Circuit::Manifest => &[
                "Delivering the result.",
                "Here's the working output.",
                "Shipped. Verified. Done.",
            ],
        }
    }

    /// Context clues for VAAM detection of correct usage.
    /// Grounded in systems design, code, and AI workflow patterns.
    pub fn context_clues(&self) -> &'static [&'static str] {
        match self {
            Circuit::Center => &["focus", "core", "problem", "scope", "filter", "target"],
            Circuit::Expand => &[
                "survey",
                "options",
                "approaches",
                "explore",
                "space",
                "alternatives",
            ],
            Circuit::Balance => &["tradeoff", "constraint", "compare", "weigh", "cost", "both"],
            Circuit::Prepare => &[
                "dependencies",
                "setup",
                "gather",
                "precondition",
                "workspace",
                "config",
            ],
            Circuit::Express => &[
                "articulate",
                "plan",
                "architecture",
                "describe",
                "diagram",
                "spec",
            ],
            Circuit::Extend => &[
                "generalize",
                "reuse",
                "apply",
                "beyond",
                "adjacent",
                "abstract",
            ],
            Circuit::Unlock => &[
                "blocker",
                "assumption",
                "constraint",
                "workaround",
                "unblock",
                "root cause",
            ],
            Circuit::Flow => &[
                "momentum",
                "productive",
                "rhythm",
                "batch",
                "pipeline",
                "sustained",
            ],
            Circuit::Receive => &["feedback", "error", "output", "listen", "log", "input"],
            Circuit::Relate => &[
                "pattern",
                "codebase",
                "context",
                "module",
                "system",
                "architecture",
            ],
            Circuit::Realize => &[
                "insight",
                "root cause",
                "understand",
                "debug",
                "discover",
                "click",
            ],
            Circuit::Act => &["execute", "implement", "write", "build", "commit", "run"],
            Circuit::Transform => &[
                "refactor", "improve", "clean", "optimize", "quality", "simplify",
            ],
            Circuit::Connect => &["integrate", "wire", "test", "api", "merge", "deploy"],
            Circuit::Manifest => &["deliver", "ship", "verify", "output", "result", "done"],
        }
    }

    /// VAAM Coal value for correct contextual usage of this circuit word.
    /// Foundation-tier words earn more than genre-specific vocabulary because
    /// they represent meta-cognitive awareness.
    pub fn coal_value(&self) -> u32 {
        match self.quadrant() {
            CircuitQuadrant::Scope => 15,  // Scoping is foundational
            CircuitQuadrant::Build => 12,  // Building follows scoping
            CircuitQuadrant::Listen => 18, // Feedback processing is high-value
            CircuitQuadrant::Ship => 20,   // Shipping earns the most
        }
    }

    /// Convert this circuit to a VAAM VocabularyWord for inclusion in
    /// the foundation vocabulary pack.
    pub fn to_vaam_word(&self) -> VocabularyWord {
        VocabularyWord {
            word: self.name().to_string(),
            aliases: vec![self.name().to_lowercase()],
            context_clues: self.context_clues().iter().map(|s| s.to_string()).collect(),
            coal_value: self.coal_value(),
            tier: VocabularyTier::Expert, // Foundation words are always Expert tier
            bloom_level: match self.quadrant() {
                CircuitQuadrant::Scope => BloomLevel::Understand,
                CircuitQuadrant::Build => BloomLevel::Apply,
                CircuitQuadrant::Listen => BloomLevel::Analyze,
                CircuitQuadrant::Ship => BloomLevel::Create,
            },
            definition: Some(self.description().to_string()),
            tags: vec![
                "sacred_circuitry".to_string(),
                "foundation".to_string(),
                self.quadrant().name().to_lowercase().replace(' ', "_"),
            ],
        }
    }

    /// The next circuit in the spiral. After Manifest, loops to Center.
    pub fn next(&self) -> Circuit {
        let idx = self.order() as usize;
        Circuit::ALL[idx % 15]
    }

    /// The previous circuit in the spiral. Before Center, loops to Manifest.
    pub fn prev(&self) -> Circuit {
        let idx = (self.order() as usize + 13) % 15; // -2 + 15 = +13, since order is 1-indexed
        Circuit::ALL[idx]
    }

    /// Parse a circuit from a string (case-insensitive).
    pub fn from_str_loose(s: &str) -> Option<Circuit> {
        let lower = s.trim().to_lowercase();
        Circuit::ALL
            .iter()
            .find(|c| c.name().to_lowercase() == lower)
            .copied()
    }
}

impl std::fmt::Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// The four attention phases of the Sacred Circuitry workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CircuitQuadrant {
    /// Center, Expand, Balance, Prepare — define the problem space
    Scope,
    /// Express, Extend, Unlock, Flow — produce the work
    Build,
    /// Receive, Relate, Realize — process feedback and find insight
    Listen,
    /// Act, Transform, Connect, Manifest — deliver tangible output
    Ship,
}

impl CircuitQuadrant {
    pub fn name(&self) -> &'static str {
        match self {
            CircuitQuadrant::Scope => "Scope",
            CircuitQuadrant::Build => "Build",
            CircuitQuadrant::Listen => "Listen",
            CircuitQuadrant::Ship => "Ship",
        }
    }

    /// The circuits belonging to this quadrant, in order.
    pub fn circuits(&self) -> &'static [Circuit] {
        match self {
            CircuitQuadrant::Scope => &[
                Circuit::Center,
                Circuit::Expand,
                Circuit::Balance,
                Circuit::Prepare,
            ],
            CircuitQuadrant::Build => &[
                Circuit::Express,
                Circuit::Extend,
                Circuit::Unlock,
                Circuit::Flow,
            ],
            CircuitQuadrant::Listen => &[Circuit::Receive, Circuit::Relate, Circuit::Realize],
            CircuitQuadrant::Ship => &[
                Circuit::Act,
                Circuit::Transform,
                Circuit::Connect,
                Circuit::Manifest,
            ],
        }
    }
}

/// The Sacred Circuitry focusing state for an AI session.
///
/// Tracks which circuit is currently active, how the AI has moved through
/// the spiral, and provides auto-reply selection for training.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitryState {
    /// The currently active circuit
    pub active: Circuit,
    /// How many times each circuit has been activated this session
    pub activations: [u32; 15],
    /// The last N circuits visited (for pattern recognition)
    pub history: Vec<Circuit>,
    /// Maximum history length
    pub max_history: usize,
}

impl Default for CircuitryState {
    fn default() -> Self {
        Self {
            active: Circuit::Center,
            activations: [0; 15],
            history: Vec::new(),
            max_history: 50,
        }
    }
}

impl CircuitryState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Activate a circuit. Records the transition and updates counts.
    pub fn activate(&mut self, circuit: Circuit) {
        self.active = circuit;
        self.activations[(circuit.order() - 1) as usize] += 1;
        self.history.push(circuit);
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    /// Get the activation count for a specific circuit.
    pub fn count(&self, circuit: Circuit) -> u32 {
        self.activations[(circuit.order() - 1) as usize]
    }

    /// Total activations across all circuits this session.
    pub fn total_activations(&self) -> u32 {
        self.activations.iter().sum()
    }

    /// Which quadrant is currently dominant (most activations)?
    pub fn dominant_quadrant(&self) -> CircuitQuadrant {
        let quadrants = [
            CircuitQuadrant::Scope,
            CircuitQuadrant::Build,
            CircuitQuadrant::Listen,
            CircuitQuadrant::Ship,
        ];

        quadrants
            .into_iter()
            .max_by_key(|q| q.circuits().iter().map(|c| self.count(*c)).sum::<u32>())
            .unwrap_or(CircuitQuadrant::Scope)
    }

    /// Select an auto-reply for the currently active circuit.
    /// Uses the activation count to rotate through available replies.
    pub fn auto_reply(&self) -> &'static str {
        let replies = self.active.auto_replies();
        let idx = self.count(self.active) as usize % replies.len();
        replies[idx]
    }

    /// Get a summary of the session's circuit usage.
    pub fn summary(&self) -> String {
        let total = self.total_activations();
        if total == 0 {
            return "Sacred Circuitry: No circuits activated yet.".to_string();
        }

        let mut lines = vec![format!(
            "⚡ Sacred Circuitry — {} activations (active: {})",
            total, self.active
        )];

        for quadrant in [
            CircuitQuadrant::Scope,
            CircuitQuadrant::Build,
            CircuitQuadrant::Listen,
            CircuitQuadrant::Ship,
        ] {
            let q_total: u32 = quadrant.circuits().iter().map(|c| self.count(*c)).sum();
            if q_total > 0 {
                let details: Vec<String> = quadrant
                    .circuits()
                    .iter()
                    .filter(|c| self.count(**c) > 0)
                    .map(|c| format!("{}({})", c.name(), self.count(*c)))
                    .collect();
                lines.push(format!(
                    "  {}: {} — {}",
                    quadrant.name(),
                    q_total,
                    details.join(", ")
                ));
            }
        }

        lines.join("\n")
    }

    /// Detect which circuit best matches the given text based on context clues.
    /// Returns the best-matching circuit and a confidence score (0.0–1.0).
    pub fn detect_circuit(text: &str) -> Option<(Circuit, f32)> {
        let lower = text.to_lowercase();
        let mut best: Option<(Circuit, f32)> = None;

        for circuit in Circuit::ALL {
            let clues = circuit.context_clues();
            let matched = clues.iter().filter(|clue| lower.contains(**clue)).count();

            if matched > 0 {
                let score = matched as f32 / clues.len() as f32;
                if best.is_none_or(|(_, s)| score > s) {
                    best = Some((circuit, score));
                }
            }
        }

        best
    }
}

/// Build the Sacred Circuitry VAAM vocabulary pack.
/// This is the foundation layer — always loaded, genre-independent.
pub fn foundation_vocabulary() -> Vec<VocabularyWord> {
    Circuit::ALL.iter().map(|c| c.to_vaam_word()).collect()
}

/// Format a circuit activation as a VAAM-style event for the chat stream.
pub fn format_circuit_event(circuit: Circuit, auto_reply: &str) -> String {
    serde_json::json!({
        "circuit": circuit.name(),
        "order": circuit.order(),
        "quadrant": circuit.quadrant().name(),
        "description": circuit.description(),
        "auto_reply": auto_reply,
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_order() {
        assert_eq!(Circuit::Center.order(), 1);
        assert_eq!(Circuit::Manifest.order(), 15);
        assert_eq!(Circuit::Flow.order(), 8);
    }

    #[test]
    fn test_all_circuits_count() {
        assert_eq!(Circuit::ALL.len(), 15);
    }

    #[test]
    fn test_quadrant_membership() {
        assert_eq!(Circuit::Center.quadrant(), CircuitQuadrant::Scope);
        assert_eq!(Circuit::Express.quadrant(), CircuitQuadrant::Build);
        assert_eq!(Circuit::Receive.quadrant(), CircuitQuadrant::Listen);
        assert_eq!(Circuit::Act.quadrant(), CircuitQuadrant::Ship);
    }

    #[test]
    fn test_quadrant_circuits_complete() {
        let total: usize = [
            CircuitQuadrant::Scope,
            CircuitQuadrant::Build,
            CircuitQuadrant::Listen,
            CircuitQuadrant::Ship,
        ]
        .iter()
        .map(|q| q.circuits().len())
        .sum();
        assert_eq!(total, 15);
    }

    #[test]
    fn test_circuit_spiral_next() {
        assert_eq!(Circuit::Center.next(), Circuit::Expand);
        assert_eq!(Circuit::Manifest.next(), Circuit::Center);
        assert_eq!(Circuit::Flow.next(), Circuit::Receive);
    }

    #[test]
    fn test_circuit_spiral_prev() {
        assert_eq!(Circuit::Expand.prev(), Circuit::Center);
        assert_eq!(Circuit::Center.prev(), Circuit::Manifest);
    }

    #[test]
    fn test_from_str_loose() {
        assert_eq!(Circuit::from_str_loose("center"), Some(Circuit::Center));
        assert_eq!(Circuit::from_str_loose("MANIFEST"), Some(Circuit::Manifest));
        assert_eq!(Circuit::from_str_loose("  Flow  "), Some(Circuit::Flow));
        assert_eq!(Circuit::from_str_loose("bogus"), None);
    }

    #[test]
    fn test_circuitry_state_activation() {
        let mut state = CircuitryState::new();
        assert_eq!(state.active, Circuit::Center);
        assert_eq!(state.total_activations(), 0);

        state.activate(Circuit::Center);
        state.activate(Circuit::Expand);
        state.activate(Circuit::Center);

        assert_eq!(state.active, Circuit::Center);
        assert_eq!(state.count(Circuit::Center), 2);
        assert_eq!(state.count(Circuit::Expand), 1);
        assert_eq!(state.total_activations(), 3);
        assert_eq!(state.history.len(), 3);
    }

    #[test]
    fn test_auto_reply_rotation() {
        let mut state = CircuitryState::new();
        state.activate(Circuit::Center);
        let r1 = state.auto_reply();

        state.activate(Circuit::Expand);
        state.activate(Circuit::Center);
        let r2 = state.auto_reply();

        // Second activation should rotate to a different reply
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_dominant_quadrant() {
        let mut state = CircuitryState::new();
        state.activate(Circuit::Act);
        state.activate(Circuit::Transform);
        state.activate(Circuit::Manifest);
        state.activate(Circuit::Center);

        assert_eq!(state.dominant_quadrant(), CircuitQuadrant::Ship);
    }

    #[test]
    fn test_detect_circuit() {
        let result = CircuitryState::detect_circuit(
            "I need to focus and find the core of this problem and filter the scope",
        );
        assert!(result.is_some());
        let (circuit, score) = result.unwrap();
        assert_eq!(circuit, Circuit::Center);
        assert!(score > 0.0);
    }

    #[test]
    fn test_foundation_vocabulary() {
        let words = foundation_vocabulary();
        assert_eq!(words.len(), 15);
        assert_eq!(words[0].word, "Center");
        assert_eq!(words[14].word, "Manifest");
        assert!(words.iter().all(|w| w.tier == VocabularyTier::Expert));
        assert!(words
            .iter()
            .all(|w| w.tags.contains(&"sacred_circuitry".to_string())));
    }

    #[test]
    fn test_vaam_word_has_context_clues() {
        let word = Circuit::Center.to_vaam_word();
        assert!(!word.context_clues.is_empty());
        assert!(word.context_clues.contains(&"focus".to_string()));
        assert!(word.tags.contains(&"scope".to_string()));
    }

    #[test]
    fn test_coal_values() {
        // Delivery quadrant earns the most
        assert!(Circuit::Manifest.coal_value() > Circuit::Center.coal_value());
        // Exchange is higher than Outward
        assert!(Circuit::Receive.coal_value() > Circuit::Express.coal_value());
    }

    #[test]
    fn test_summary_output() {
        let mut state = CircuitryState::new();
        state.activate(Circuit::Center);
        state.activate(Circuit::Realize);
        state.activate(Circuit::Manifest);

        let summary = state.summary();
        assert!(summary.contains("Sacred Circuitry"));
        assert!(summary.contains("3 activations"));
        assert!(summary.contains("Center(1)"));
        assert!(summary.contains("Manifest(1)"));
    }

    #[test]
    fn test_format_circuit_event() {
        let event = format_circuit_event(Circuit::Center, "Centering on the core problem.");
        let parsed: serde_json::Value = serde_json::from_str(&event).unwrap();
        assert_eq!(parsed["circuit"], "Center");
        assert_eq!(parsed["order"], 1);
        assert_eq!(parsed["quadrant"], "Scope");
    }
}
