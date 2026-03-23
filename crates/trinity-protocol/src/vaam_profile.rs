// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-protocol
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        vaam_profile.rs
// PURPOSE:     VAAM Profile — the bridge between user preference and AI attention
//
// WHAT THIS IS:
//   VAAM (Vocabulary Acquisition And Mastery) is the meaning-making machine. It maps
//   through every level of the system isomorphically: Sacred Circuitry →
//   VAAM → ADDIECRAPEYE → CharacterSheet.
//
//   This module tracks HOW the user communicates — their word preferences,
//   attention patterns, and communication style. Just as the AI uses weights
//   for words, so do people. We build preference into the engineering,
//   recognizing it as both bias and a way to build the user's POV as SME.
//
//   The VaamProfile is the agreement bridge: how the user and Trinity
//   negotiate what words matter, what attention patterns to prioritize,
//   and what style of interaction to use.
//
// ARCHITECTURE:
//   • Sacred Circuitry = AI's foundation vocabulary (15 words, 4 quadrants)
//   • VaamProfile = user's vocabulary fingerprint (preferences, style, agreements)
//   • Together they form the negotiated attention space between human and AI
//   • CharacterSheet holds the profile; ADDIECRAPEYE workflows consume it
//
// ISOMORPHIC MAPPING:
//   Sacred Circuitry (HOW to attend) ──┐
//   VaamProfile (WHAT user prefers)  ──┼── CharacterSheet (WHO the user is)
//   ADDIECRAPEYE (WHAT to do)       ──┘         │
//                                        MadLibs / Quests / Voice
//
// CHANGES:
//   2026-03-19  Cascade  Initial — preference tracking, style, agreements
//
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::sacred_circuitry::{Circuit, CircuitQuadrant};

/// The user's VAAM profile — their word preferences, attention patterns,
/// and communication style. This is the bridge between how the user
/// naturally communicates and how the AI structures its responses.
///
/// Updated continuously as the user interacts with Trinity. Persisted
/// as part of the CharacterSheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaamProfile {
    /// Which Sacred Circuitry quadrants the user naturally gravitates toward.
    /// [Scope, Build, Listen, Ship] — normalized weights summing to ~1.0.
    pub circuit_affinity: [f32; 4],

    /// Per-circuit activation count from the user's natural language.
    /// Index 0 = Center, 1 = Expand, ..., 14 = Manifest.
    pub circuit_usage: [u32; 15],

    /// Words the user consistently prefers (their vocabulary fingerprint).
    /// Key is the lowercase word. Only tracked for words that appear in
    /// VAAM vocabulary or Sacred Circuitry.
    pub word_weights: HashMap<String, WordWeight>,

    /// Communication style derived from interaction patterns.
    pub style: CommunicationStyle,

    /// Explicit agreements between user and AI about what matters.
    /// Built up over time through conversation and negotiation.
    pub agreements: Vec<Agreement>,

    /// Total interactions analyzed to build this profile.
    pub interactions_analyzed: u64,
}

impl Default for VaamProfile {
    fn default() -> Self {
        Self {
            // Start with equal affinity across all quadrants
            circuit_affinity: [0.25, 0.25, 0.25, 0.25],
            circuit_usage: [0; 15],
            word_weights: HashMap::new(),
            style: CommunicationStyle::default(),
            agreements: Vec::new(),
            interactions_analyzed: 0,
        }
    }
}

impl VaamProfile {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record that the user used a Sacred Circuitry word in context.
    /// Updates circuit_usage and recalculates circuit_affinity.
    pub fn record_circuit_usage(&mut self, circuit: Circuit) {
        let idx = (circuit.order() - 1) as usize;
        self.circuit_usage[idx] += 1;
        self.recalculate_affinity();
    }

    /// Record a word the user chose to use. If `alternatives_available` is true,
    /// this was a deliberate choice (higher affinity signal).
    pub fn record_word_usage(&mut self, word: &str, alternatives_available: bool) {
        let key = word.to_lowercase();
        let weight = self.word_weights.entry(key).or_default();
        weight.times_chosen += 1;
        weight.times_available += 1;
        if alternatives_available {
            // Deliberate choice when alternatives existed — boost the signal
            weight.deliberate_choices += 1;
        }
        weight.recalculate();
    }

    /// Record a full interaction turn for style analysis.
    /// `word_count` = number of words in the user's message.
    /// `question_count` = number of questions asked.
    /// `statement_count` = number of declarative statements.
    pub fn record_interaction(
        &mut self,
        word_count: usize,
        question_count: usize,
        statement_count: usize,
    ) {
        self.interactions_analyzed += 1;
        self.style.update(
            word_count,
            question_count,
            statement_count,
            self.interactions_analyzed,
        );
    }

    /// Add an explicit agreement between user and AI.
    pub fn add_agreement(&mut self, topic: String, circuit: Circuit, weight: f32) {
        // Check if an agreement on this topic already exists
        if let Some(existing) = self.agreements.iter_mut().find(|a| a.topic == topic) {
            existing.circuit = circuit;
            existing.weight = weight.clamp(0.0, 1.0);
            existing.updated_at = Utc::now();
        } else {
            self.agreements.push(Agreement {
                topic,
                circuit,
                weight: weight.clamp(0.0, 1.0),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        }
    }

    /// Remove an agreement by topic.
    pub fn remove_agreement(&mut self, topic: &str) {
        self.agreements.retain(|a| a.topic != topic);
    }

    /// Get the user's dominant quadrant (where they spend the most attention).
    pub fn dominant_quadrant(&self) -> CircuitQuadrant {
        let quadrants = [
            CircuitQuadrant::Scope,
            CircuitQuadrant::Build,
            CircuitQuadrant::Listen,
            CircuitQuadrant::Ship,
        ];
        let max_idx = self
            .circuit_affinity
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);
        quadrants[max_idx]
    }

    /// Get the user's top N preferred words by affinity.
    pub fn top_words(&self, n: usize) -> Vec<(&str, f32)> {
        let mut words: Vec<_> = self
            .word_weights
            .iter()
            .map(|(w, wt)| (w.as_str(), wt.affinity))
            .collect();
        words.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        words.truncate(n);
        words
    }

    /// Get active agreements (non-zero weight), sorted by weight descending.
    pub fn active_agreements(&self) -> Vec<&Agreement> {
        let mut active: Vec<_> = self.agreements.iter().filter(|a| a.weight > 0.0).collect();
        active.sort_by(|a, b| {
            b.weight
                .partial_cmp(&a.weight)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        active
    }

    /// Generate a compact summary for inclusion in AI system prompts.
    /// This is how VAAM preference feeds into the AI's attention.
    pub fn prompt_summary(&self) -> String {
        let mut parts = Vec::new();

        // Dominant attention pattern
        let dom = self.dominant_quadrant();
        parts.push(format!("User leans {}", dom.name()));

        // Style
        let style = &self.style;
        let brevity = if style.brevity > 0.6 {
            "terse"
        } else if style.brevity < 0.4 {
            "verbose"
        } else {
            "balanced"
        };
        let directness = if style.directness > 0.6 {
            "direct"
        } else if style.directness < 0.4 {
            "exploratory"
        } else {
            "mixed"
        };
        parts.push(format!("Style: {} + {}", brevity, directness));

        // Top words
        let top = self.top_words(5);
        if !top.is_empty() {
            let word_list: Vec<_> = top.iter().map(|(w, _)| *w).collect();
            parts.push(format!("Key words: {}", word_list.join(", ")));
        }

        // Active agreements
        let agreements = self.active_agreements();
        if !agreements.is_empty() {
            let agreement_list: Vec<_> = agreements
                .iter()
                .take(3)
                .map(|a| format!("{}({})", a.topic, a.circuit))
                .collect();
            parts.push(format!("Agreed: {}", agreement_list.join(", ")));
        }

        parts.join(" | ")
    }

    /// Recalculate quadrant affinity from circuit usage counts.
    fn recalculate_affinity(&mut self) {
        let quadrant_totals: [f32; 4] = [
            // Scope: Center(0) + Expand(1) + Balance(2) + Prepare(3)
            self.circuit_usage[0..4].iter().sum::<u32>() as f32,
            // Build: Express(4) + Extend(5) + Unlock(6) + Flow(7)
            self.circuit_usage[4..8].iter().sum::<u32>() as f32,
            // Listen: Receive(8) + Relate(9) + Realize(10)
            self.circuit_usage[8..11].iter().sum::<u32>() as f32,
            // Ship: Act(11) + Transform(12) + Connect(13) + Manifest(14)
            self.circuit_usage[11..15].iter().sum::<u32>() as f32,
        ];

        let total: f32 = quadrant_totals.iter().sum();
        if total > 0.0 {
            for (i, qt) in quadrant_totals.iter().enumerate() {
                self.circuit_affinity[i] = qt / total;
            }
        }
    }
}

/// How much the user gravitates toward a specific word.
/// Tracks both raw usage and deliberate choice (when alternatives existed).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordWeight {
    /// How often the user has used this word
    pub times_chosen: u32,
    /// How often this word was encountered (used or available)
    pub times_available: u32,
    /// How often the user picked this word when alternatives existed
    pub deliberate_choices: u32,
    /// Computed affinity — frequency × deliberateness.
    /// Range 0.0–1.0 where 1.0 = frequently used AND deliberately chosen.
    pub affinity: f32,
}

impl Default for WordWeight {
    fn default() -> Self {
        Self {
            times_chosen: 0,
            times_available: 0,
            deliberate_choices: 0,
            affinity: 0.0,
        }
    }
}

impl WordWeight {
    fn recalculate(&mut self) {
        let frequency = self.times_chosen as f32 / self.times_available.max(1) as f32;
        // Boost affinity when user deliberately chose this word over alternatives
        let deliberate_ratio = if self.times_chosen > 0 {
            0.5 + 0.5 * (self.deliberate_choices as f32 / self.times_chosen as f32)
        } else {
            0.5
        };
        self.affinity = (frequency * deliberate_ratio).min(1.0);
    }
}

/// Communication style derived from interaction patterns.
/// All values are 0.0–1.0, computed as running averages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStyle {
    /// 0.0 = verbose (many words per turn), 1.0 = terse (few words)
    pub brevity: f32,
    /// 0.0 = exploratory (mostly questions), 1.0 = direct (mostly statements)
    pub directness: f32,
}

impl Default for CommunicationStyle {
    fn default() -> Self {
        Self {
            brevity: 0.5,
            directness: 0.5,
        }
    }
}

impl CommunicationStyle {
    /// Update style from a new interaction. Uses exponential moving average
    /// so recent interactions have more weight.
    fn update(
        &mut self,
        word_count: usize,
        question_count: usize,
        statement_count: usize,
        total_interactions: u64,
    ) {
        // Smoothing factor — heavier weight on recent interactions
        let alpha = if total_interactions < 10 { 0.3 } else { 0.1 };

        // Brevity: normalize word count to 0–1 range (50 words = verbose, 5 = terse)
        let brevity_sample = 1.0 - (word_count as f32 / 50.0).min(1.0);
        self.brevity = self.brevity * (1.0 - alpha) + brevity_sample * alpha;

        // Directness: ratio of statements to total utterances
        let total_utterances = (question_count + statement_count).max(1);
        let directness_sample = statement_count as f32 / total_utterances as f32;
        self.directness = self.directness * (1.0 - alpha) + directness_sample * alpha;
    }
}

/// An explicit agreement between user and AI about what matters.
/// Built through conversation: "This is important" → Agreement recorded.
/// The AI references active agreements in its system prompt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agreement {
    /// What was agreed on (e.g., "vocabulary is the core mechanic")
    pub topic: String,
    /// Which Sacred Circuitry pattern this relates to
    pub circuit: Circuit,
    /// How important (0.0 = deprioritized, 1.0 = critical)
    pub weight: f32,
    /// When this agreement was first made
    pub created_at: DateTime<Utc>,
    /// When this agreement was last updated
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_profile() {
        let profile = VaamProfile::new();
        assert_eq!(profile.circuit_affinity, [0.25, 0.25, 0.25, 0.25]);
        assert_eq!(profile.interactions_analyzed, 0);
        assert!(profile.word_weights.is_empty());
    }

    #[test]
    fn test_circuit_usage_updates_affinity() {
        let mut profile = VaamProfile::new();

        // Heavy Ship usage
        profile.record_circuit_usage(Circuit::Act);
        profile.record_circuit_usage(Circuit::Act);
        profile.record_circuit_usage(Circuit::Transform);
        profile.record_circuit_usage(Circuit::Manifest);

        // Light Scope usage
        profile.record_circuit_usage(Circuit::Center);

        assert_eq!(profile.dominant_quadrant(), CircuitQuadrant::Ship);
        assert!(profile.circuit_affinity[3] > profile.circuit_affinity[0]); // Ship > Scope
    }

    #[test]
    fn test_word_weight_tracking() {
        let mut profile = VaamProfile::new();

        // "center" used 3 times, all deliberate
        profile.record_word_usage("center", true);
        profile.record_word_usage("center", true);
        profile.record_word_usage("center", true);
        // "expand" used once, not deliberate (no alternatives)
        profile.record_word_usage("expand", false);

        let top = profile.top_words(5);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, "center");
        // center: freq=1.0, deliberate=100% → affinity=1.0
        // expand: freq=1.0, deliberate=0%   → affinity=0.5
        assert!(
            top[0].1 > top[1].1,
            "center={} expand={}",
            top[0].1,
            top[1].1
        );
    }

    #[test]
    fn test_communication_style() {
        let mut profile = VaamProfile::new();

        // Simulate terse, direct user (few words, mostly statements)
        for i in 1..=10 {
            profile.record_interaction(8, 0, 2);
            assert_eq!(profile.interactions_analyzed, i);
        }

        assert!(
            profile.style.brevity > 0.6,
            "brevity={}",
            profile.style.brevity
        );
        assert!(
            profile.style.directness > 0.6,
            "directness={}",
            profile.style.directness
        );
    }

    #[test]
    fn test_agreement_lifecycle() {
        let mut profile = VaamProfile::new();

        profile.add_agreement("vocabulary is core".to_string(), Circuit::Center, 0.9);
        profile.add_agreement("ship working code".to_string(), Circuit::Manifest, 0.8);

        assert_eq!(profile.active_agreements().len(), 2);

        // Update existing agreement
        profile.add_agreement("vocabulary is core".to_string(), Circuit::Center, 1.0);
        assert_eq!(profile.agreements.len(), 2); // no duplicate

        // Remove
        profile.remove_agreement("ship working code");
        assert_eq!(profile.active_agreements().len(), 1);
    }

    #[test]
    fn test_prompt_summary() {
        let mut profile = VaamProfile::new();

        profile.record_circuit_usage(Circuit::Act);
        profile.record_circuit_usage(Circuit::Manifest);
        profile.record_word_usage("execute", true);
        profile.add_agreement("productivity".to_string(), Circuit::Flow, 0.9);

        // Simulate terse style
        for _ in 0..5 {
            profile.record_interaction(6, 0, 2);
        }

        let summary = profile.prompt_summary();
        assert!(summary.contains("Ship"), "summary={}", summary);
        assert!(summary.contains("execute"), "summary={}", summary);
        assert!(summary.contains("productivity"), "summary={}", summary);
    }

    #[test]
    fn test_affinity_normalization() {
        let mut profile = VaamProfile::new();

        // All usage in one quadrant
        for _ in 0..10 {
            profile.record_circuit_usage(Circuit::Center);
        }

        // Affinity should sum to ~1.0
        let sum: f32 = profile.circuit_affinity.iter().sum();
        assert!((sum - 1.0).abs() < 0.01, "sum={}", sum);

        // Scope should dominate
        assert!(profile.circuit_affinity[0] > 0.9);
    }

    #[test]
    fn test_word_affinity_caps_at_one() {
        let mut profile = VaamProfile::new();

        // Use word 5 times but only 2 alternatives existed
        profile.record_word_usage("focus", false);
        profile.record_word_usage("focus", false);
        profile.record_word_usage("focus", true);
        profile.record_word_usage("focus", true);
        profile.record_word_usage("focus", false);

        let weight = profile.word_weights.get("focus").unwrap();
        assert!(weight.affinity <= 1.0);
    }
}
