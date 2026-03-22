// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-iron-road/src/game_loop.rs
// ═══════════════════════════════════════════════════════════════════════════════
//
// PURPOSE:     Iron Road Game Loop — the backbone connecting all systems
//
// ARCHITECTURE:
//   This is the central integration point for the Iron Road game mechanics:
//
//   COLLECT  → scan user text for vocabulary, create Wild Creeps
//   CONSTRUCT → tame Creeps via multi-dimensional TamingProgress
//   QUEST    → fill Lesson MadLib slots with Tamed Creeps
//   BATTLE   → Creep vs Creep for contested MadLib slots
//   REWARDS  → Context Points, VaamProfile update, Book chapter generation
//
//   Each phase produces events that flow into the next. Completed lesson
//   plans fire RecyclerEvents that become Book of the Bible chapters.
//   VaamProfile is updated and persisted to CharacterSheet on every cycle.
//
// CHANGES:
//   2026-03-19  Cascade  Initial — connecting all 12 systems
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::great_recycler::RecyclerEvent;
use crate::vaam::madlibs::LessonMadlib;
use serde::{Deserialize, Serialize};
use trinity_protocol::semantic_creep::{CreepState, SemanticCreep};
use trinity_protocol::{Genre, VaamProfile};

/// The player's Creep collection — their vocabulary bestiary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreepBestiary {
    /// All Creeps the player has encountered
    pub creeps: Vec<SemanticCreep>,
    /// Total words scanned across all interactions
    pub words_scanned: u64,
    /// Total Creeps tamed (Rule of Three completed)
    pub creeps_tamed: u32,
    /// Total MadLib slots filled
    pub slots_filled: u32,
    /// Total battles won
    pub battles_won: u32,
}

impl CreepBestiary {
    pub fn new() -> Self {
        Self {
            creeps: Vec::new(),
            words_scanned: 0,
            creeps_tamed: 0,
            slots_filled: 0,
            battles_won: 0,
        }
    }

    /// Scan text for words and create/update Creeps.
    /// Accepts the current ADDIECRAPEYE phase index and Circuit quadrant
    /// so taming progress tracks multi-dimensional learning.
    /// Returns a list of events that occurred (new Creeps, tameable Creeps).
    pub fn scan_text(
        &mut self,
        text: &str,
        phase: Option<u8>,
        quadrant: Option<u8>,
        intent_match: f32,
    ) -> Vec<GameLoopEvent> {
        let mut events = Vec::new();
        let words: Vec<&str> = text
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|w| w.len() >= 4) // Only words with 4+ chars are interesting
            .collect();

        for word in &words {
            self.words_scanned += 1;
            let word_lower = word.to_lowercase();

            if let Some(creep) = self
                .creeps
                .iter_mut()
                .find(|c| c.word.to_lowercase() == word_lower)
            {
                // Existing Creep — record contextual usage
                if let Some(score) = creep.record_usage(phase, quadrant, false, intent_match) {
                    // Creep just became tameable — emit event for Scope Hope/Nope
                    events.push(GameLoopEvent::CreepTameable {
                        word: creep.word.clone(),
                        element: format!("{}", creep.element),
                        role: format!("{}", creep.role),
                        power: creep.power(),
                        taming_score: score,
                    });
                }
            } else {
                // New Wild Creep discovered
                let mut creep = SemanticCreep::from_word(&word_lower);
                creep.record_usage(phase, quadrant, false, intent_match);
                events.push(GameLoopEvent::CreepDiscovered {
                    word: creep.word.clone(),
                    element: format!("{}", creep.element),
                    role: format!("{}", creep.role),
                });
                self.creeps.push(creep);
            }
        }

        events
    }

    /// User chose Scope Hope for a word — tame it!
    pub fn scope_hope_creep(&mut self, word: &str) -> bool {
        if let Some(creep) = self.get_creep_mut(word) {
            if creep.scope_hope() {
                self.creeps_tamed += 1;
                return true;
            }
        }
        false
    }

    /// User chose Scope Nope for a word — leave it wild.
    pub fn scope_nope_creep(&mut self, word: &str) {
        if let Some(creep) = self.get_creep_mut(word) {
            creep.scope_nope();
        }
    }

    /// Get all usable (Tamed or Evolved) Creeps
    pub fn usable_creeps(&self) -> Vec<&SemanticCreep> {
        self.creeps.iter().filter(|c| c.is_usable()).collect()
    }

    /// Get all Wild Creeps (untamed vocabulary)
    pub fn wild_creeps(&self) -> Vec<&SemanticCreep> {
        self.creeps
            .iter()
            .filter(|c| c.state == CreepState::Wild)
            .collect()
    }

    /// Get a mutable reference to a Creep by word
    pub fn get_creep_mut(&mut self, word: &str) -> Option<&mut SemanticCreep> {
        let word_lower = word.to_lowercase();
        self.creeps
            .iter_mut()
            .find(|c| c.word.to_lowercase() == word_lower)
    }

    /// Summary for display
    pub fn summary(&self) -> String {
        format!(
            "Bestiary: {} total, {} tamed, {} wild | {} slots filled, {} battles won",
            self.creeps.len(),
            self.creeps.iter().filter(|c| c.is_usable()).count(),
            self.creeps
                .iter()
                .filter(|c| c.state == CreepState::Wild)
                .count(),
            self.slots_filled,
            self.battles_won,
        )
    }
}

impl Default for CreepBestiary {
    fn default() -> Self {
        Self::new()
    }
}

/// Events produced by the game loop — each can trigger UI updates, Book chapters, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameLoopEvent {
    /// A new Wild Creep was discovered in user text
    CreepDiscovered {
        word: String,
        element: String,
        role: String,
    },
    /// A Creep became tameable — ready for Scope Hope / Scope Nope decision
    CreepTameable {
        word: String,
        element: String,
        role: String,
        power: f32,
        taming_score: f32,
    },
    /// A MadLib slot was filled
    SlotFilled {
        lesson_id: String,
        slot_id: String,
        creep_word: String,
        cp_earned: u32,
    },
    /// A battle was resolved
    BattleResolved {
        lesson_id: String,
        slot_id: String,
        winner: String,
        loser: String,
        margin: f32,
    },
    /// A lesson plan was completed (all slots filled)
    LessonCompleted {
        lesson_id: String,
        title: String,
        completed_text: String,
        grade_level: String,
        subject: String,
    },
}

impl GameLoopEvent {
    /// Convert a lesson completion event into a RecyclerEvent for the Book system
    #[allow(clippy::too_many_arguments)] // Bridges game event to Book; genuinely needs full quest context
    pub fn to_recycler_event(
        &self,
        quest_id: &str,
        phase: &str,
        alias: &str,
        coal: f32,
        steam: u32,
        xp: u64,
        resonance_level: u32,
        genre: Genre,
    ) -> Option<RecyclerEvent> {
        match self {
            GameLoopEvent::LessonCompleted {
                title,
                completed_text,
                grade_level,
                subject,
                ..
            } => Some(RecyclerEvent {
                quest_id: quest_id.to_string(),
                phase: phase.to_string(),
                description: format!(
                    "Completed lesson plan '{}' for {} (Grade {}): {}",
                    title, subject, grade_level, completed_text
                ),
                alias: alias.to_string(),
                coal,
                steam,
                xp,
                resonance_level,
                genre,
            }),
            GameLoopEvent::CreepTameable {
                word,
                element,
                role,
                power,
                taming_score,
            } => Some(RecyclerEvent {
                quest_id: quest_id.to_string(),
                phase: phase.to_string(),
                description: format!(
                    "Scope Hope! Tamed Semantic Creep: '{}' ({}, {}, Power: {:.0}, Score: {:.2})",
                    word, element, role, power, taming_score
                ),
                alias: alias.to_string(),
                coal,
                steam,
                xp,
                resonance_level,
                genre,
            }),
            _ => None,
        }
    }
}

/// Process a completed lesson MadLib and generate game loop events.
/// This is the REWARDS phase — update profile, generate events for Book.
pub fn complete_lesson(lesson: &LessonMadlib, profile: &mut VaamProfile) -> Vec<GameLoopEvent> {
    let mut events = Vec::new();

    if !lesson.is_complete() {
        return events;
    }

    // Record all filled words in the profile
    for slot in &lesson.slots {
        if let Some(ref word) = slot.filled_by {
            profile.record_word_usage(word, true);
        }
    }

    // Generate the lesson completion event
    if let Some(ref text) = lesson.completed {
        events.push(GameLoopEvent::LessonCompleted {
            lesson_id: lesson.id.clone(),
            title: lesson.title.clone(),
            completed_text: text.clone(),
            grade_level: lesson.grade_level.clone(),
            subject: lesson.subject.clone(),
        });
    }

    events
}

/// Serialize a CharacterSheet + Bestiary to JSON for persistence.
/// This is the "save game" function — called after each game loop cycle.
pub fn save_state_json(
    bestiary: &CreepBestiary,
    profile: &VaamProfile,
) -> Result<String, serde_json::Error> {
    let state = serde_json::json!({
        "bestiary": bestiary,
        "vaam_profile": profile,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    serde_json::to_string_pretty(&state)
}

/// Load state from JSON
pub fn load_bestiary_json(json: &str) -> Result<CreepBestiary, serde_json::Error> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    if let Some(bestiary) = value.get("bestiary") {
        serde_json::from_value(bestiary.clone())
    } else {
        Ok(CreepBestiary::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_discovers_creeps() {
        let mut bestiary = CreepBestiary::new();
        let events = bestiary.scan_text(
            "The structure of geology reveals terrain patterns",
            None,
            None,
            0.5,
        );

        assert!(
            bestiary.creeps.len() >= 3,
            "Should discover 3+ words (4+ chars)"
        );
        assert!(events.iter().any(
            |e| matches!(e, GameLoopEvent::CreepDiscovered { word, .. } if word == "structure")
        ));
    }

    #[test]
    fn test_scan_repetition_does_not_auto_tame() {
        let mut bestiary = CreepBestiary::new();

        // Repeat in the same phase/quadrant — should NOT auto-tame
        bestiary.scan_text("The structure is important", Some(0), Some(0), 0.5);
        bestiary.scan_text("This structure holds up", Some(0), Some(0), 0.5);
        bestiary.scan_text("Good structure matters", Some(0), Some(0), 0.5);
        assert_eq!(
            bestiary.creeps_tamed, 0,
            "Repetition in same phase should not tame"
        );
    }

    #[test]
    fn test_scan_multi_phase_makes_tameable() {
        let mut bestiary = CreepBestiary::new();

        // Phase 1: passive discovery across different phases
        bestiary.scan_text("The structure is important", Some(0), Some(0), 0.8);
        bestiary.scan_text("This structure holds up", Some(1), Some(1), 0.9);
        bestiary.scan_text("Good structure matters", Some(2), Some(2), 1.0);

        // Passive alone shouldn't be enough (deliberation = 0)
        assert_eq!(bestiary.creeps_tamed, 0, "Should NOT be auto-tamed");

        // Phase 2: user deliberately uses the word twice (e.g., choosing it from suggestions)
        let creep = bestiary.get_creep_mut("structure").unwrap();
        creep.record_usage(Some(3), Some(0), true, 1.0);
        let result = creep.record_usage(Some(4), Some(0), true, 1.0);

        // Now it should be tameable
        assert!(
            result.is_some(),
            "Deliberate usage should push it past tameable threshold"
        );

        // User decides: Scope Hope!
        assert!(bestiary.scope_hope_creep("structure"));
        assert_eq!(bestiary.creeps_tamed, 1);
        assert_eq!(bestiary.usable_creeps().len(), 1);
    }

    #[test]
    fn test_usable_creeps_filters() {
        let mut bestiary = CreepBestiary::new();
        // Discover two words across phases
        bestiary.scan_text("structure terrain", Some(0), Some(0), 0.8);
        bestiary.scan_text("structure terrain", Some(1), Some(1), 0.9);
        bestiary.scan_text("structure terrain", Some(2), Some(2), 1.0);

        // Add deliberate usage for both to make them tameable (need 2 deliberate uses)
        for word in &["structure", "terrain"] {
            if let Some(creep) = bestiary.get_creep_mut(word) {
                creep.record_usage(Some(3), Some(0), true, 1.0);
                creep.record_usage(Some(4), Some(0), true, 1.0);
            }
        }

        // Tame structure, leave terrain wild
        bestiary.scope_hope_creep("structure");
        bestiary.scope_nope_creep("terrain");

        assert_eq!(bestiary.usable_creeps().len(), 1); // only structure
        assert_eq!(bestiary.wild_creeps().len(), 1); // terrain still wild
    }

    /// Helper: tame a creep through multi-dimensional progress + scope_hope
    fn tame_creep(creep: &mut SemanticCreep) {
        creep.record_usage(Some(0), Some(0), true, 0.9);
        creep.record_usage(Some(1), Some(1), true, 0.9);
        creep.record_usage(Some(2), Some(2), true, 1.0);
        creep.scope_hope();
    }

    #[test]
    fn test_lesson_completion_events() {
        let mut lesson = LessonMadlib::new(
            "test",
            "Test Lesson",
            "The {noun_topic} is {adjective_quality}.",
            "6-8",
            "Science",
        );

        let mut noun = SemanticCreep::from_word("geology");
        let mut adj = SemanticCreep::from_word("creative");
        tame_creep(&mut noun);
        tame_creep(&mut adj);

        lesson.fill_slot("noun_topic", &mut noun).unwrap();
        lesson.fill_slot("adjective_quality", &mut adj).unwrap();

        let mut profile = VaamProfile::new();
        let events = complete_lesson(&lesson, &mut profile);

        assert_eq!(events.len(), 1);
        assert!(
            matches!(&events[0], GameLoopEvent::LessonCompleted { title, .. } if title == "Test Lesson")
        );
    }

    #[test]
    fn test_event_to_recycler_event() {
        let event = GameLoopEvent::LessonCompleted {
            lesson_id: "test".to_string(),
            title: "Test Lesson".to_string(),
            completed_text: "The geology is creative.".to_string(),
            grade_level: "6-8".to_string(),
            subject: "Science".to_string(),
        };

        let recycler = event.to_recycler_event(
            "quest-001",
            "Implement",
            "Joshua",
            80.0,
            50,
            1000,
            3,
            Genre::Steampunk,
        );
        assert!(recycler.is_some());
        let r = recycler.unwrap();
        assert!(r.description.contains("Test Lesson"));
        assert_eq!(r.genre, Genre::Steampunk);
    }

    #[test]
    fn test_save_and_load_state() {
        let mut bestiary = CreepBestiary::new();
        bestiary.scan_text("structure terrain", Some(0), Some(0), 0.5);

        let profile = VaamProfile::new();
        let json = save_state_json(&bestiary, &profile).unwrap();

        let loaded = load_bestiary_json(&json).unwrap();
        assert_eq!(loaded.creeps.len(), bestiary.creeps.len());
        assert_eq!(loaded.creeps_tamed, bestiary.creeps_tamed);
    }

    #[test]
    fn test_bestiary_summary() {
        let mut bestiary = CreepBestiary::new();
        bestiary.scan_text("structure terrain", Some(0), Some(0), 0.8);
        bestiary.scan_text("structure terrain", Some(1), Some(1), 0.9);
        bestiary.scan_text("structure terrain", Some(2), Some(2), 1.0);
        // Add deliberate usage to make structure tameable
        if let Some(creep) = bestiary.get_creep_mut("structure") {
            creep.record_usage(Some(3), Some(0), true, 1.0);
            creep.record_usage(Some(4), Some(0), true, 1.0);
        }
        bestiary.scope_hope_creep("structure");

        let summary = bestiary.summary();
        assert!(summary.contains("2 total"));
        assert!(summary.contains("1 tamed"));
        assert!(summary.contains("1 wild"));
    }
}
