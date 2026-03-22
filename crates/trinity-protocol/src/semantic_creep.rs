// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-protocol
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        semantic_creep.rs
// PURPOSE:     Semantic Creep — word-creatures for the Iron Road game loop
//
// WHAT THIS IS:
//   Words made flesh. Every vocabulary word in VAAM can become a SemanticCreep —
//   a creature with stats, element, role, and state. Wild Creeps are untamed
//   vocabulary (Scope Nope). Tamed Creeps are mastered words (Scope Hope).
//
//   The Creep system is the game layer on top of VAAM. It turns vocabulary
//   mastery into a collectible creature mechanic:
//
//   - COLLECT vocabulary → detect words in user text (VaamState)
//   - CONSTRUCT Creeps → Etymology determines Element + Role + Stats
//   - QUEST → Tamed Creeps fill MadLib lesson plan slots
//   - BATTLE → Creep vs Creep for contested MadLib slots
//   - REWARDS → Context Points make Creeps stronger over time
//
// ARCHITECTURE:
//   Etymology root → Element (Fire/Water/Earth/Air/Shadow/Light)
//   Morphological suffix → Role (Tank/Striker/Support)
//   Stats: Logos (logic/attack), Pathos (emotion/HP), Ethos (trust/defense), Speed
//   State: Wild → Tamed (Rule of Three) → Evolved (suffixes/prefixes)
//
// DESIGN SOURCE:
//   archive/003_logos_engine.md — Etymon system
//   archive/005_integrated_game_loop.md — Game loop spec
//   Renamed from "Semantic Slime" to "Semantic Creep" (Scope Creep monsters)
//
// CHANGES:
//   2026-03-19  Cascade  Initial — creatures for the Iron Road
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::fmt;

/// Elemental affinity derived from a word's etymological root.
/// Root determines the "soul" of the Creep — its primary power source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreepElement {
    /// Ignis/Pyr roots — high attack (Logos)
    Fire,
    /// Aqua/Hydr roots — high health (Pathos)
    Water,
    /// Terra/Geo roots — high defense (Ethos)
    Earth,
    /// Aer/Pneu roots — high speed
    Air,
    /// Umbra/Scot roots — debuffs, stealth
    Shadow,
    /// Lux/Phot roots — buffs, reveal
    Light,
    /// No recognized etymology — neutral stats
    Neutral,
}

impl CreepElement {
    /// Get the stat bonus multiplier for this element
    pub fn stat_bonus(&self) -> CreepStats {
        match self {
            Self::Fire => CreepStats {
                logos: 1.5,
                pathos: 0.8,
                ethos: 0.8,
                speed: 1.0,
            },
            Self::Water => CreepStats {
                logos: 0.8,
                pathos: 1.5,
                ethos: 1.0,
                speed: 0.8,
            },
            Self::Earth => CreepStats {
                logos: 0.8,
                pathos: 1.0,
                ethos: 1.5,
                speed: 0.8,
            },
            Self::Air => CreepStats {
                logos: 1.0,
                pathos: 0.8,
                ethos: 0.8,
                speed: 1.5,
            },
            Self::Shadow => CreepStats {
                logos: 1.2,
                pathos: 0.9,
                ethos: 0.9,
                speed: 1.2,
            },
            Self::Light => CreepStats {
                logos: 0.9,
                pathos: 1.2,
                ethos: 1.2,
                speed: 0.9,
            },
            Self::Neutral => CreepStats {
                logos: 1.0,
                pathos: 1.0,
                ethos: 1.0,
                speed: 1.0,
            },
        }
    }

    /// Detect element from a word's etymology or spelling patterns
    pub fn from_word(word: &str) -> Self {
        let w = word.to_lowercase();

        // Latin/Greek root detection
        if w.contains("ign")
            || w.contains("pyr")
            || w.contains("therm")
            || w.contains("cand")
            || w.contains("flam")
            || w.contains("combust")
        {
            return Self::Fire;
        }
        if w.contains("aqua")
            || w.contains("hydr")
            || w.contains("mar")
            || w.contains("naut")
            || w.contains("flu")
            || w.contains("liq")
        {
            return Self::Water;
        }
        if w.contains("terra")
            || w.contains("geo")
            || w.contains("lith")
            || w.contains("petr")
            || w.contains("fund")
            || w.contains("struct")
        {
            return Self::Earth;
        }
        if w.contains("aer")
            || w.contains("pneu")
            || w.contains("vent")
            || w.contains("spir")
            || w.contains("atmo")
            || w.contains("cycl")
        {
            return Self::Air;
        }
        if w.contains("umbr")
            || w.contains("scot")
            || w.contains("noct")
            || w.contains("crypt")
            || w.contains("occult")
            || w.contains("shadow")
        {
            return Self::Shadow;
        }
        if w.contains("luc")
            || w.contains("lum")
            || w.contains("phot")
            || w.contains("radi")
            || w.contains("clar")
            || w.contains("splend")
        {
            return Self::Light;
        }

        Self::Neutral
    }
}

impl fmt::Display for CreepElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fire => write!(f, "🔥 Fire"),
            Self::Water => write!(f, "💧 Water"),
            Self::Earth => write!(f, "🪨 Earth"),
            Self::Air => write!(f, "💨 Air"),
            Self::Shadow => write!(f, "🌑 Shadow"),
            Self::Light => write!(f, "✨ Light"),
            Self::Neutral => write!(f, "⚪ Neutral"),
        }
    }
}

/// Combat role derived from a word's morphological suffix.
/// Suffix determines the "body" of the Creep — how it fights.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreepRole {
    /// Noun suffixes (-tion, -ity, -ment, -ness, -ance) — high defense
    Tank,
    /// Verb suffixes (-ize, -ate, -fy, -en) — high attack
    Striker,
    /// Adjective suffixes (-ous, -al, -ive, -ful, -able) — balanced/utility
    Support,
}

impl CreepRole {
    /// Get the stat multiplier for this role
    pub fn stat_bonus(&self) -> CreepStats {
        match self {
            Self::Tank => CreepStats {
                logos: 0.9,
                pathos: 1.3,
                ethos: 1.3,
                speed: 0.7,
            },
            Self::Striker => CreepStats {
                logos: 1.4,
                pathos: 0.8,
                ethos: 0.8,
                speed: 1.2,
            },
            Self::Support => CreepStats {
                logos: 1.0,
                pathos: 1.1,
                ethos: 1.1,
                speed: 1.0,
            },
        }
    }

    /// Detect role from a word's suffix
    pub fn from_word(word: &str) -> Self {
        let w = word.to_lowercase();

        // Verb suffixes → Striker
        if w.ends_with("ize")
            || w.ends_with("ate")
            || w.ends_with("fy")
            || w.ends_with("en")
            || w.ends_with("ify")
        {
            return Self::Striker;
        }

        // Adjective suffixes → Support
        if w.ends_with("ous")
            || w.ends_with("ive")
            || w.ends_with("ful")
            || w.ends_with("able")
            || w.ends_with("ible")
            || w.ends_with("al")
            || w.ends_with("ent")
            || w.ends_with("ant")
            || w.ends_with("ic")
        {
            return Self::Support;
        }

        // Noun suffixes → Tank (default — most words are nouns)
        Self::Tank
    }
}

impl fmt::Display for CreepRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tank => write!(f, "🛡️ Tank"),
            Self::Striker => write!(f, "⚔️ Striker"),
            Self::Support => write!(f, "💚 Support"),
        }
    }
}

/// Creep combat stats. All values are multiplied together:
/// final_stat = base * element_bonus * role_bonus * context_bonus
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CreepStats {
    /// Logic / Attack power
    pub logos: f32,
    /// Emotion / Health points
    pub pathos: f32,
    /// Trust / Defense
    pub ethos: f32,
    /// Initiative / Agility
    pub speed: f32,
}

impl CreepStats {
    /// Base stats for a newly created Creep (before element/role modifiers)
    pub fn base() -> Self {
        Self {
            logos: 10.0,
            pathos: 10.0,
            ethos: 10.0,
            speed: 10.0,
        }
    }

    /// Apply element and role multipliers to base stats
    pub fn with_modifiers(element: CreepElement, role: CreepRole) -> Self {
        let base = Self::base();
        let elem = element.stat_bonus();
        let role_b = role.stat_bonus();

        Self {
            logos: (base.logos * elem.logos * role_b.logos).round(),
            pathos: (base.pathos * elem.pathos * role_b.pathos).round(),
            ethos: (base.ethos * elem.ethos * role_b.ethos).round(),
            speed: (base.speed * elem.speed * role_b.speed).round(),
        }
    }

    /// Total power rating (sum of all stats)
    pub fn power(&self) -> f32 {
        self.logos + self.pathos + self.ethos + self.speed
    }

    /// Apply Context Points bonus: +1% per 10 CP to all stats
    pub fn with_context_bonus(mut self, context_points: u32) -> Self {
        let bonus = 1.0 + (context_points as f32 / 1000.0);
        self.logos *= bonus;
        self.pathos *= bonus;
        self.ethos *= bonus;
        self.speed *= bonus;
        self
    }
}

impl Default for CreepStats {
    fn default() -> Self {
        Self::base()
    }
}

/// The lifecycle state of a Creep.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreepState {
    /// Untamed — detected but not yet mastered
    Wild,
    /// Tamed via Scope Hope decision after reaching taming threshold
    Tamed,
    /// Enhanced with morphological modifiers (suffixes/prefixes)
    Evolved,
}

impl fmt::Display for CreepState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wild => write!(f, "Wild"),
            Self::Tamed => write!(f, "Tamed"),
            Self::Evolved => write!(f, "Evolved"),
        }
    }
}

/// How a Semantic Creep is tamed — not by repetition alone, but by
/// multi-dimensional learning that mirrors how humans actually master vocabulary.
///
/// Pythagoras taught that understanding requires three things:
///   Arithmos (quantity), Harmonia (relationship), and Logos (meaning).
///
/// TamingProgress maps these to measurable dimensions:
///   - Encounter (Arithmos) — breadth: how many different phases/contexts?
///   - Context (Harmonia) — variety: used across different Circuit quadrants?
///   - Deliberation (Logos) — choice: did the user pick this word consciously?
///   - Resonance — intent: does the word align with the session's purpose?
///
/// The Creep becomes TAMEABLE (not auto-tamed) when combined score ≥ 1.0.
/// The user must then make a Scope Hope / Scope Nope decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TamingProgress {
    /// Total encounters with this word
    pub encounter_count: u32,

    /// Set of distinct ADDIECRAPEYE phase indices where this word appeared (0–11)
    /// More phases = deeper understanding across the learning journey
    pub phases_encountered: Vec<u8>,

    /// Set of distinct Circuit quadrant indices where this word was used (0–3)
    /// More quadrants = broader contextual understanding
    pub quadrants_encountered: Vec<u8>,

    /// Number of times the user deliberately chose this word over alternatives
    pub deliberate_uses: u32,

    /// How well this word aligns with the user's stated session intent (0.0–1.0)
    /// Updated each time the word is used, based on conductor context
    pub resonance: f32,
}

impl Default for TamingProgress {
    fn default() -> Self {
        Self {
            encounter_count: 0,
            phases_encountered: Vec::new(),
            quadrants_encountered: Vec::new(),
            deliberate_uses: 0,
            resonance: 0.0,
        }
    }
}

impl TamingProgress {
    /// Record an encounter with this word.
    /// `phase`: the ADDIECRAPEYE phase index (0–11)
    /// `quadrant`: the Sacred Circuitry quadrant index (0–3)
    /// `deliberate`: whether the user chose this word when alternatives existed
    /// `intent_match`: how well this usage aligns with session intent (0.0–1.0)
    pub fn record(
        &mut self,
        phase: Option<u8>,
        quadrant: Option<u8>,
        deliberate: bool,
        intent_match: f32,
    ) {
        self.encounter_count += 1;

        if let Some(p) = phase {
            if !self.phases_encountered.contains(&p) {
                self.phases_encountered.push(p);
            }
        }

        if let Some(q) = quadrant {
            if !self.quadrants_encountered.contains(&q) {
                self.quadrants_encountered.push(q);
            }
        }

        if deliberate {
            self.deliberate_uses += 1;
        }

        // Exponential moving average for resonance
        let alpha = if self.encounter_count < 5 { 0.4 } else { 0.2 };
        self.resonance = self.resonance * (1.0 - alpha) + intent_match * alpha;
    }

    /// Calculate the taming score (0.0 to ~2.0+).
    /// Creep is TAMEABLE when score ≥ 1.0.
    ///
    /// Formula (Pythagorean weights):
    ///   Encounter breadth (Arithmos): 30% — must see it in ≥ 3 phases
    ///   Context variety (Harmonia):   25% — must use across ≥ 2 quadrants
    ///   Deliberation (Logos):         25% — conscious choice matters
    ///   Resonance (Intent):           20% — must serve the purpose
    pub fn taming_score(&self) -> f32 {
        // Encounter breadth: 3+ distinct phases = full credit
        let encounter = (self.phases_encountered.len() as f32 / 3.0).min(1.0);

        // Context variety: 2+ distinct quadrants = full credit
        let context = (self.quadrants_encountered.len() as f32 / 2.0).min(1.0);

        // Deliberation: at least 2 deliberate uses = full credit
        let deliberation = (self.deliberate_uses as f32 / 2.0).min(1.0);

        // Resonance: direct from intent alignment
        let resonance = self.resonance.min(1.0);

        encounter * 0.30 + context * 0.25 + deliberation * 0.25 + resonance * 0.20
    }

    /// Is this Creep ready for a Scope Hope / Scope Nope decision?
    /// Threshold is 0.85 — you don't need perfection to start taming.
    /// The remaining growth happens through Context Points after taming.
    pub fn is_tameable(&self) -> bool {
        self.taming_score() >= 0.85
    }

    /// Display a progress bar for the user
    pub fn progress_display(&self) -> String {
        let score = self.taming_score();
        let filled = (score * 10.0).round() as usize;
        let empty = 10_usize.saturating_sub(filled);
        format!(
            "[{}{}] {:.0}% | Phases: {} | Quadrants: {} | Deliberate: {} | Resonance: {:.0}%",
            "█".repeat(filled),
            "░".repeat(empty),
            score * 100.0,
            self.phases_encountered.len(),
            self.quadrants_encountered.len(),
            self.deliberate_uses,
            self.resonance * 100.0,
        )
    }
}

/// A Semantic Creep — a word made into a creature.
///
/// Every vocabulary word in VAAM can become a SemanticCreep. Wild Creeps
/// lurk in user text waiting to be tamed. Tamed Creeps fill MadLib slots
/// and battle for lesson plan territory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticCreep {
    /// The word this Creep represents
    pub word: String,
    /// Optional definition
    pub definition: Option<String>,
    /// Elemental affinity (from etymology)
    pub element: CreepElement,
    /// Combat role (from morphology)
    pub role: CreepRole,
    /// Current combat stats
    pub stats: CreepStats,
    /// Wild, Tamed, or Evolved
    pub state: CreepState,
    /// Multi-dimensional taming progress (replaces flat usage_count)
    pub taming: TamingProgress,
    /// Context Points — earned through quest usage, makes the Creep stronger
    pub context_points: u32,
    /// Part of speech (noun, verb, adjective, adverb)
    pub part_of_speech: String,
    /// Vocabulary tier from VAAM
    pub tier: String,
}

impl SemanticCreep {
    /// Create a new Wild Creep from a word
    pub fn from_word(word: &str) -> Self {
        let element = CreepElement::from_word(word);
        let role = CreepRole::from_word(word);
        let stats = CreepStats::with_modifiers(element, role);

        Self {
            word: word.to_string(),
            definition: None,
            element,
            role,
            stats,
            state: CreepState::Wild,
            taming: TamingProgress::default(),
            context_points: 0,
            part_of_speech: detect_part_of_speech(word),
            tier: "Basic".to_string(),
        }
    }

    /// Create from a word with a known definition and tier
    pub fn from_word_full(word: &str, definition: &str, tier: &str) -> Self {
        let mut creep = Self::from_word(word);
        creep.definition = Some(definition.to_string());
        creep.tier = tier.to_string();
        creep
    }

    /// Record a usage of this word in context.
    /// Returns `Some(score)` if the Creep just became tameable (Scope Hope/Nope time).
    /// Returns `None` if still wild and not yet tameable.
    pub fn record_usage(
        &mut self,
        phase: Option<u8>,
        quadrant: Option<u8>,
        deliberate: bool,
        intent_match: f32,
    ) -> Option<f32> {
        self.taming
            .record(phase, quadrant, deliberate, intent_match);

        // Check if this encounter just pushed the Creep past the tameable threshold
        if self.state == CreepState::Wild && self.taming.is_tameable() {
            Some(self.taming.taming_score())
        } else {
            None
        }
    }

    /// User chose Scope Hope — tame the Creep!
    /// This is a conscious decision, not an automatic transition.
    pub fn scope_hope(&mut self) -> bool {
        if self.state == CreepState::Wild && self.taming.is_tameable() {
            self.state = CreepState::Tamed;
            true
        } else {
            false
        }
    }

    /// User chose Scope Nope — leave it wild, but acknowledge the encounter.
    /// The Creep stays in the bestiary but doesn't join the active vocabulary.
    /// It can still become tameable again later.
    pub fn scope_nope(&mut self) {
        // Reset resonance slightly — the user said "not now"
        self.taming.resonance *= 0.5;
    }

    /// Add Context Points (earned from quest usage)
    pub fn add_context_points(&mut self, points: u32) {
        self.context_points += points;
        // Recalculate stats with CP bonus
        let base = CreepStats::with_modifiers(self.element, self.role);
        self.stats = base.with_context_bonus(self.context_points);
    }

    /// Evolve with a morphological modifier (suffix/prefix)
    pub fn evolve(&mut self, modifier: &str) -> String {
        let new_word = format!("{}{}", self.word, modifier);
        self.state = CreepState::Evolved;
        // Recalculate role based on new suffix
        self.role = CreepRole::from_word(&new_word);
        let base = CreepStats::with_modifiers(self.element, self.role);
        self.stats = base.with_context_bonus(self.context_points);
        // Bonus for evolution
        self.stats.logos *= 1.1;
        self.stats.pathos *= 1.1;
        self.stats.ethos *= 1.1;
        self.stats.speed *= 1.1;
        self.word = new_word.clone();
        new_word
    }

    /// Can this Creep be used in MadLibs? (Must be Tamed or Evolved)
    pub fn is_usable(&self) -> bool {
        self.state == CreepState::Tamed || self.state == CreepState::Evolved
    }

    /// Combat power rating
    pub fn power(&self) -> f32 {
        self.stats.power()
    }

    /// Format as a card display
    pub fn card(&self) -> String {
        format!(
            "[{}] {} — {} | {} | Power: {:.0}\n  Taming: {}\n  Stats: L:{:.0} P:{:.0} E:{:.0} S:{:.0} | CP: {}",
            self.state, self.word, self.element, self.role, self.power(),
            self.taming.progress_display(),
            self.stats.logos, self.stats.pathos, self.stats.ethos, self.stats.speed,
            self.context_points,
        )
    }
}

impl fmt::Display for SemanticCreep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}, {}, {})",
            self.word, self.state, self.element, self.role
        )
    }
}

/// Simple part-of-speech detection from suffixes
fn detect_part_of_speech(word: &str) -> String {
    let w = word.to_lowercase();
    if w.ends_with("ize")
        || w.ends_with("ate")
        || w.ends_with("fy")
        || w.ends_with("en")
        || w.ends_with("ify")
    {
        "verb".to_string()
    } else if w.ends_with("ous")
        || w.ends_with("ive")
        || w.ends_with("ful")
        || w.ends_with("able")
        || w.ends_with("ible")
        || w.ends_with("al")
        || w.ends_with("ic")
        || w.ends_with("ant")
        || w.ends_with("ent")
    {
        "adjective".to_string()
    } else if w.ends_with("ly") {
        "adverb".to_string()
    } else {
        "noun".to_string()
    }
}

/// Resolve a MadLib battle between two Creeps competing for the same slot.
/// Returns the index of the winner (0 or 1) and the margin of victory.
pub fn battle(a: &SemanticCreep, b: &SemanticCreep, slot_type: &str) -> (usize, f32) {
    // The relevant stat depends on the MadLib slot type
    let (a_score, b_score) = match slot_type.to_lowercase().as_str() {
        "noun" => (
            a.stats.ethos + a.stats.pathos,
            b.stats.ethos + b.stats.pathos,
        ),
        "verb" => (a.stats.logos + a.stats.speed, b.stats.logos + b.stats.speed),
        "adjective" => (
            a.stats.pathos + a.stats.logos,
            b.stats.pathos + b.stats.logos,
        ),
        "adverb" => (a.stats.speed + a.stats.logos, b.stats.speed + b.stats.logos),
        _ => (a.power(), b.power()),
    };

    if a_score >= b_score {
        (0, a_score - b_score)
    } else {
        (1, b_score - a_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_from_fire_word() {
        assert_eq!(CreepElement::from_word("ignition"), CreepElement::Fire);
        assert_eq!(CreepElement::from_word("pyrotechnic"), CreepElement::Fire);
        assert_eq!(CreepElement::from_word("combustion"), CreepElement::Fire);
    }

    #[test]
    fn test_element_from_water_word() {
        assert_eq!(CreepElement::from_word("aquatic"), CreepElement::Water);
        assert_eq!(CreepElement::from_word("hydraulic"), CreepElement::Water);
    }

    #[test]
    fn test_element_from_earth_word() {
        assert_eq!(CreepElement::from_word("terrain"), CreepElement::Earth);
        assert_eq!(CreepElement::from_word("geology"), CreepElement::Earth);
        assert_eq!(CreepElement::from_word("structure"), CreepElement::Earth);
    }

    #[test]
    fn test_element_neutral_for_unknown() {
        assert_eq!(CreepElement::from_word("happy"), CreepElement::Neutral);
        assert_eq!(CreepElement::from_word("dog"), CreepElement::Neutral);
    }

    #[test]
    fn test_role_from_suffix() {
        assert_eq!(CreepRole::from_word("optimize"), CreepRole::Striker);
        assert_eq!(CreepRole::from_word("generate"), CreepRole::Striker);
        assert_eq!(CreepRole::from_word("creative"), CreepRole::Support);
        assert_eq!(CreepRole::from_word("dangerous"), CreepRole::Support);
        assert_eq!(CreepRole::from_word("ignition"), CreepRole::Tank);
    }

    #[test]
    fn test_stats_with_modifiers() {
        let stats = CreepStats::with_modifiers(CreepElement::Fire, CreepRole::Striker);
        // Fire gives high logos, Striker gives high logos — should compound
        assert!(
            stats.logos > 15.0,
            "Fire+Striker should have logos > 15, got {}",
            stats.logos
        );
        assert!(
            stats.speed > stats.ethos,
            "Striker should be faster than tough"
        );
    }

    #[test]
    fn test_creep_from_word() {
        let creep = SemanticCreep::from_word("ignition");
        assert_eq!(creep.element, CreepElement::Fire);
        assert_eq!(creep.role, CreepRole::Tank); // -tion suffix = noun = Tank
        assert_eq!(creep.state, CreepState::Wild);
        assert_eq!(creep.taming.encounter_count, 0);
        assert_eq!(creep.part_of_speech, "noun");
    }

    #[test]
    fn test_taming_progress_dimensions() {
        let mut prog = TamingProgress::default();

        // Single encounter — low score
        prog.record(Some(0), Some(0), false, 0.0);
        assert!(
            prog.taming_score() < 0.3,
            "Single encounter should score low, got {}",
            prog.taming_score()
        );

        // Add more phases and quadrants
        prog.record(Some(1), Some(1), true, 0.5);
        prog.record(Some(2), Some(0), true, 0.8);
        let score = prog.taming_score();
        assert!(
            score > 0.5,
            "Multi-dimensional progress should score > 0.5, got {}",
            score
        );
    }

    #[test]
    fn test_taming_requires_multi_dimensional_progress() {
        let mut creep = SemanticCreep::from_word("structure");
        assert_eq!(creep.state, CreepState::Wild);

        // Just repeating in the same phase/quadrant should NOT tame
        for _ in 0..10 {
            creep.record_usage(Some(0), Some(0), false, 0.0);
        }
        assert_eq!(
            creep.state,
            CreepState::Wild,
            "Flat repetition should not tame"
        );
        assert!(
            !creep.taming.is_tameable(),
            "Should not be tameable from repetition alone"
        );

        // Now add breadth: different phases, quadrants, deliberation, resonance
        creep.record_usage(Some(1), Some(1), true, 0.8);
        creep.record_usage(Some(2), Some(2), true, 0.9);
        creep.record_usage(Some(3), Some(0), true, 1.0);

        // Now it should be tameable (but NOT auto-tamed)
        assert!(
            creep.taming.is_tameable(),
            "Multi-dimensional progress should make it tameable"
        );
        assert_eq!(creep.state, CreepState::Wild, "Should NOT be auto-tamed");
    }

    #[test]
    fn test_scope_hope_tames_creep() {
        let mut creep = SemanticCreep::from_word("terrain");
        // Build up taming progress
        creep.record_usage(Some(0), Some(0), true, 0.8);
        creep.record_usage(Some(1), Some(1), true, 0.9);
        creep.record_usage(Some(2), Some(2), true, 1.0);

        assert!(creep.taming.is_tameable());
        assert!(creep.scope_hope(), "Scope Hope should succeed");
        assert_eq!(creep.state, CreepState::Tamed);
        assert!(creep.is_usable());
    }

    #[test]
    fn test_scope_nope_keeps_wild() {
        let mut creep = SemanticCreep::from_word("terrain");
        creep.record_usage(Some(0), Some(0), true, 0.8);
        creep.record_usage(Some(1), Some(1), true, 0.9);
        creep.record_usage(Some(2), Some(2), true, 1.0);

        assert!(creep.taming.is_tameable());
        let resonance_before = creep.taming.resonance;
        creep.scope_nope();
        assert_eq!(
            creep.state,
            CreepState::Wild,
            "Scope Nope should keep it wild"
        );
        assert!(
            creep.taming.resonance < resonance_before,
            "Resonance should decrease after Nope"
        );
    }

    #[test]
    fn test_evolution() {
        let mut creep = SemanticCreep::from_word("optimize");
        // Tame it first
        creep.record_usage(Some(0), Some(0), true, 0.9);
        creep.record_usage(Some(1), Some(1), true, 0.9);
        creep.record_usage(Some(2), Some(2), true, 1.0);
        creep.scope_hope();
        assert_eq!(creep.state, CreepState::Tamed);

        let new_word = creep.evolve("d");
        assert_eq!(new_word, "optimized");
        assert_eq!(creep.state, CreepState::Evolved);
    }

    #[test]
    fn test_context_points_boost() {
        let mut creep = SemanticCreep::from_word("terrain");
        let initial_power = creep.power();

        creep.add_context_points(100);
        assert!(creep.power() > initial_power, "CP should increase power");
    }

    #[test]
    fn test_battle_resolution() {
        let fire_striker = SemanticCreep::from_word("optimize"); // Neutral+Striker
        let earth_tank = SemanticCreep::from_word("structure"); // Earth+Tank

        // For a noun slot, Tank (defense+health) should win
        let (winner, margin) = battle(&fire_striker, &earth_tank, "noun");
        assert_eq!(
            winner, 1,
            "Earth Tank should win noun slot, margin: {}",
            margin
        );

        // For a verb slot, Striker (attack+speed) should win
        let (winner, _) = battle(&fire_striker, &earth_tank, "verb");
        assert_eq!(winner, 0, "Striker should win verb slot");
    }

    #[test]
    fn test_card_display() {
        let creep = SemanticCreep::from_word("illumination");
        let card = creep.card();
        assert!(card.contains("Wild"));
        assert!(card.contains("illumination"));
        assert!(card.contains("Light"));
        assert!(card.contains("Taming:"));
    }

    #[test]
    fn test_wild_creep_not_usable() {
        let creep = SemanticCreep::from_word("terrain");
        assert!(!creep.is_usable());
    }

    #[test]
    fn test_part_of_speech_detection() {
        assert_eq!(detect_part_of_speech("optimize"), "verb");
        assert_eq!(detect_part_of_speech("creative"), "adjective");
        assert_eq!(detect_part_of_speech("quickly"), "adverb");
        assert_eq!(detect_part_of_speech("ignition"), "noun");
    }

    #[test]
    fn test_taming_progress_display() {
        let mut prog = TamingProgress::default();
        prog.record(Some(0), Some(0), true, 0.5);
        prog.record(Some(1), Some(1), false, 0.8);
        let display = prog.progress_display();
        assert!(display.contains("Phases: 2"));
        assert!(display.contains("Quadrants: 2"));
        assert!(display.contains("Deliberate: 1"));
    }

    #[test]
    fn test_record_usage_returns_score_when_tameable() {
        let mut creep = SemanticCreep::from_word("aquatic");
        // Build up but not quite enough
        assert!(creep.record_usage(Some(0), Some(0), true, 0.8).is_none());
        assert!(creep.record_usage(Some(1), Some(1), true, 0.9).is_none());
        // This should push it over the edge
        let result = creep.record_usage(Some(2), Some(2), true, 1.0);
        assert!(result.is_some(), "Should return taming score when tameable");
        let score = result.unwrap();
        assert!(
            score >= 0.85,
            "Score should be >= 0.85 (tameable threshold), got {}",
            score
        );
    }
}
