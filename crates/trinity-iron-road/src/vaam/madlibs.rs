// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Iron Road / VAAM Subsystem
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:         vaam/madlibs.rs
// BIBLE CAR:    Car 4 — IMPLEMENT (Iron Road Game Mechanics, §4.5)
// HOOK SCHOOL:  🏫 Pedagogy — Scope Creep Combat
// PURPOSE:      Lesson MadLibs quest mechanic — structured lesson plan templates
//               with typed gaps (noun_topic, adjective_quality, etc.) that the
//               user fills with tamed SemanticCreep vocabulary. When all slots
//               are filled, the lesson is complete and generates a
//               LessonCompleted event that flows into the Book system.
//
// ARCHITECTURE:
//   • MadlibPrompt — template with typed slots (required_parts_of_speech)
//   • LessonMadlib — multi-slot lesson with subject, target tier, and scoring
//   • CreepCard — display data for SemanticCreep creatures
//   • fill_slot() validates part-of-speech match, awards Context Points
//   • battle() resolves contested slots between two SemanticCreeps
//   • generate_lesson_madlibs() produces templates for a given subject/phase
//   • Bible Car 4.5: tamed Creeps fill MadLib slots → LessonCompleted event
//
// DEPENDENCIES:
//   - serde              — JSON serialization for frontend display
//   - trinity_protocol   — SemanticCreep, CreepState, battle() function
//
// CHANGES:
//   2026-03-16  Joshua Atkinson  Created for Lesson MadLib quest mechanic
//   2026-03-26  Cascade          Added §17 header
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use trinity_protocol::semantic_creep::{battle, CreepState, SemanticCreep};

/// Represents a dynamic "Madlibs" style creator prompt designed to extract
/// specific thematic data from the user based on their VAAM tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MadlibPrompt {
    pub id: String,
    pub title: String,
    pub template: String, // e.g., "The valiant {noun} raised their {weapon} against the {adjective} {monster}."
    pub required_parts_of_speech: Vec<String>, // ["noun", "weapon", "adjective", "monster"]
    pub completed_text: Option<String>,
}

impl MadlibPrompt {
    pub fn new(id: &str, title: &str, template: &str, required_parts: Vec<&str>) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            template: template.to_string(),
            required_parts_of_speech: required_parts.into_iter().map(|s| s.to_string()).collect(),
            completed_text: None,
        }
    }

    /// Fills the template with the provided words.
    /// Returns an error if the number of words doesn't match the required parts.
    pub fn fill(&mut self, words: &[String]) -> Result<String, String> {
        if words.len() != self.required_parts_of_speech.len() {
            return Err(format!(
                "Expected {} words, but got {}",
                self.required_parts_of_speech.len(),
                words.len()
            ));
        }

        let mut result = self.template.clone();
        for (i, part) in self.required_parts_of_speech.iter().enumerate() {
            let placeholder = format!("{{{}}}", part);
            // Replace only the first occurrence to handle duplicate parts sequentially
            result = result.replacen(&placeholder, &words[i], 1);
        }

        self.completed_text = Some(result.clone());
        Ok(result)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// LESSON MADLIBS — Creep-aware lesson plan templates
//
// This is the game mechanic: Tamed Creeps (mastered vocabulary) fill lesson
// plan template slots. Each slot has a part-of-speech requirement. The best
// Creep for each slot is determined by stats. Contested slots trigger battles.
// Used Creeps earn Context Points, making them stronger over time.
// ═══════════════════════════════════════════════════════════════════════════════

/// A slot in a lesson MadLib that a SemanticCreep can fill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreepSlot {
    /// Slot identifier within the template
    pub slot_id: String,
    /// Required part of speech (noun, verb, adjective, adverb)
    pub slot_type: String,
    /// The Creep currently filling this slot (if any)
    pub filled_by: Option<String>,
    /// Context Points awarded when this slot is filled
    pub cp_reward: u32,
}

/// Result of filling a slot with a Creep
#[derive(Debug, Clone)]
pub struct SlotFillResult {
    pub slot_id: String,
    pub creep_word: String,
    pub cp_earned: u32,
    pub was_contested: bool,
    pub battle_margin: f32,
}

/// A lesson plan MadLib template that uses SemanticCreeps.
///
/// This is the quest mechanic of the Iron Road game loop:
/// 1. AI generates a lesson plan template with placeholder slots
/// 2. Player selects Tamed Creeps from their collection to fill slots
/// 3. Stats determine fit quality; contested slots trigger battles
/// 4. Completed templates produce real lesson plan content
/// 5. Used Creeps earn Context Points (stronger over time)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonMadlib {
    /// Unique identifier
    pub id: String,
    /// Lesson plan title
    pub title: String,
    /// Template text with {slot_id} placeholders
    pub template: String,
    /// The slots that need filling
    pub slots: Vec<CreepSlot>,
    /// The completed lesson text (after all slots are filled)
    pub completed: Option<String>,
    /// Target audience grade level (e.g., "3-5", "6-8", "9-12")
    pub grade_level: String,
    /// Subject area
    pub subject: String,
}

impl LessonMadlib {
    /// Create a new lesson MadLib
    pub fn new(id: &str, title: &str, template: &str, grade_level: &str, subject: &str) -> Self {
        // Auto-detect slots from {placeholder} patterns in template
        let slots = extract_slots(template);

        Self {
            id: id.to_string(),
            title: title.to_string(),
            template: template.to_string(),
            slots,
            completed: None,
            grade_level: grade_level.to_string(),
            subject: subject.to_string(),
        }
    }

    /// Get unfilled slots
    pub fn open_slots(&self) -> Vec<&CreepSlot> {
        self.slots
            .iter()
            .filter(|s| s.filled_by.is_none())
            .collect()
    }

    /// Check if all slots are filled
    pub fn is_complete(&self) -> bool {
        self.slots.iter().all(|s| s.filled_by.is_some())
    }

    /// Suggest the best Creeps from a collection for each open slot.
    /// Only Tamed or Evolved Creeps can be suggested.
    /// Returns: Vec of (slot_id, Vec of (creep_word, fitness_score))
    pub fn suggest_creeps(&self, creeps: &[SemanticCreep]) -> Vec<(String, Vec<(String, f32)>)> {
        let usable: Vec<&SemanticCreep> = creeps.iter().filter(|c| c.is_usable()).collect();

        self.open_slots()
            .iter()
            .map(|slot| {
                let mut candidates: Vec<(String, f32)> = usable
                    .iter()
                    .filter(|c| slot_matches_creep(slot, c))
                    .map(|c| {
                        let fitness = slot_fitness(slot, c);
                        (c.word.clone(), fitness)
                    })
                    .collect();

                // Sort by fitness descending
                candidates
                    .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                candidates.truncate(5); // Top 5 suggestions

                (slot.slot_id.clone(), candidates)
            })
            .collect()
    }

    /// Fill a slot with a Creep. The Creep must be usable (Tamed or Evolved).
    /// Returns the fill result with CP earned.
    pub fn fill_slot(
        &mut self,
        slot_id: &str,
        creep: &mut SemanticCreep,
    ) -> Result<SlotFillResult, String> {
        if !creep.is_usable() {
            return Err(format!(
                "'{}' is {} — must be Tamed or Evolved to fill a slot",
                creep.word, creep.state
            ));
        }

        // Extract cp_reward and validate before mutating
        let cp_reward = {
            let slot = self
                .slots
                .iter()
                .find(|s| s.slot_id == slot_id)
                .ok_or_else(|| format!("Slot '{}' not found", slot_id))?;
            if slot.filled_by.is_some() {
                return Err(format!("Slot '{}' is already filled", slot_id));
            }
            slot.cp_reward
        };

        // Now mutate the slot
        let slot = self
            .slots
            .iter_mut()
            .find(|s| s.slot_id == slot_id)
            .unwrap();
        slot.filled_by = Some(creep.word.clone());

        // Award CP to the creep
        creep.add_context_points(cp_reward);

        // If all slots are now filled, generate the completed text
        if self.is_complete() {
            self.generate_completed_text();
        }

        Ok(SlotFillResult {
            slot_id: slot_id.to_string(),
            creep_word: creep.word.clone(),
            cp_earned: cp_reward,
            was_contested: false,
            battle_margin: 0.0,
        })
    }

    /// Fill a contested slot — two Creeps battle for it.
    /// The winner fills the slot and earns bonus CP.
    pub fn contest_slot(
        &mut self,
        slot_id: &str,
        challenger: &mut SemanticCreep,
        defender: &mut SemanticCreep,
    ) -> Result<SlotFillResult, String> {
        if !challenger.is_usable() || !defender.is_usable() {
            return Err("Both Creeps must be Tamed or Evolved to contest".to_string());
        }

        // Extract slot info before any mutable borrows
        let (slot_type, cp_reward) = {
            let slot = self
                .slots
                .iter()
                .find(|s| s.slot_id == slot_id)
                .ok_or_else(|| format!("Slot '{}' not found", slot_id))?;
            (slot.slot_type.clone(), slot.cp_reward)
        };

        // Battle!
        let (winner_idx, margin) = battle(challenger, defender, &slot_type);

        // Winner fills the slot with bonus CP
        let bonus_cp = (margin * 2.0) as u32;
        let total_cp = cp_reward + bonus_cp;

        // Determine winner/loser and apply CP
        let winner_word = if winner_idx == 0 {
            challenger.add_context_points(total_cp);
            defender.add_context_points(cp_reward / 4);
            challenger.word.clone()
        } else {
            defender.add_context_points(total_cp);
            challenger.add_context_points(cp_reward / 4);
            defender.word.clone()
        };

        // Update the slot
        let slot = self
            .slots
            .iter_mut()
            .find(|s| s.slot_id == slot_id)
            .unwrap();
        slot.filled_by = Some(winner_word.clone());

        if self.is_complete() {
            self.generate_completed_text();
        }

        Ok(SlotFillResult {
            slot_id: slot_id.to_string(),
            creep_word: winner_word,
            cp_earned: total_cp,
            was_contested: true,
            battle_margin: margin,
        })
    }

    /// Generate the completed text by replacing all slot placeholders
    fn generate_completed_text(&mut self) {
        let mut text = self.template.clone();
        for slot in &self.slots {
            if let Some(ref word) = slot.filled_by {
                let placeholder = format!("{{{}}}", slot.slot_id);
                text = text.replacen(&placeholder, word, 1);
            }
        }
        self.completed = Some(text);
    }
}

/// Extract slots from a template string. Placeholders are {slot_id} patterns.
/// Slot types are inferred from the slot_id name.
fn extract_slots(template: &str) -> Vec<CreepSlot> {
    let mut slots = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = template.chars().collect();

    while i < chars.len() {
        if chars[i] == '{' {
            let start = i + 1;
            while i < chars.len() && chars[i] != '}' {
                i += 1;
            }
            if i < chars.len() {
                let slot_id: String = chars[start..i].iter().collect();
                let slot_type = infer_slot_type(&slot_id);
                slots.push(CreepSlot {
                    slot_id,
                    slot_type,
                    filled_by: None,
                    cp_reward: 10,
                });
            }
        }
        i += 1;
    }

    slots
}

/// Infer the slot type (part of speech) from the slot ID name
fn infer_slot_type(slot_id: &str) -> String {
    let id = slot_id.to_lowercase();
    if id.contains("verb") || id.contains("action") || id.contains("process") {
        "verb".to_string()
    } else if id.contains("adj") || id.contains("descri") || id.contains("quality") {
        "adjective".to_string()
    } else if id.contains("adv") || id.contains("how") || id.contains("manner") {
        "adverb".to_string()
    } else {
        "noun".to_string()
    }
}

/// Check if a Creep's part of speech matches a slot requirement
fn slot_matches_creep(slot: &CreepSlot, creep: &SemanticCreep) -> bool {
    // Exact match
    if creep.part_of_speech == slot.slot_type {
        return true;
    }
    // Nouns can fill most slots (common in lesson plans)
    if creep.part_of_speech == "noun" && slot.slot_type != "verb" {
        return true;
    }
    false
}

/// Calculate how well a Creep fits a particular slot (higher = better)
fn slot_fitness(slot: &CreepSlot, creep: &SemanticCreep) -> f32 {
    let mut score = 0.0;

    // Exact POS match bonus
    if creep.part_of_speech == slot.slot_type {
        score += 20.0;
    }

    // Stats bonus based on slot type
    score += match slot.slot_type.as_str() {
        "noun" => creep.stats.ethos + creep.stats.pathos,
        "verb" => creep.stats.logos + creep.stats.speed,
        "adjective" => creep.stats.pathos + creep.stats.logos,
        "adverb" => creep.stats.speed + creep.stats.logos,
        _ => creep.stats.power() / 4.0,
    };

    // Context Points bonus (experienced words fit better)
    score += creep.context_points as f32 * 0.1;

    // Evolved Creeps get a bonus
    if creep.state == CreepState::Evolved {
        score *= 1.15;
    }

    score
}

/// Create sample lesson plan templates for common subjects.
/// These are the "quests" of the Iron Road — each one produces real educational content.
pub fn sample_lesson_templates() -> Vec<LessonMadlib> {
    vec![
        LessonMadlib::new(
            "lesson-science-001",
            "Introduction to the Scientific Method",
            "Students will learn to {action_verb} hypotheses about {noun_topic}. \
             They will {process_verb} data using {adjective_method} techniques \
             and {action_verb_2} their findings to the class.",
            "6-8",
            "Science",
        ),
        LessonMadlib::new(
            "lesson-ela-001",
            "Narrative Writing Workshop",
            "The {adjective_tone} story begins when a {noun_character} discovers \
             a {adjective_quality} {noun_object}. Students will {action_verb} \
             their own narratives using {noun_technique} and peer review.",
            "3-5",
            "English Language Arts",
        ),
        LessonMadlib::new(
            "lesson-math-001",
            "Problem Solving with Equations",
            "Students will {action_verb} equations to model {noun_scenario}. \
             Using {adjective_approach} reasoning, they will {process_verb} \
             solutions and verify with {noun_method}.",
            "9-12",
            "Mathematics",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: tame a creep through multi-dimensional progress + scope_hope
    fn tame_creep(creep: &mut SemanticCreep) {
        creep.record_usage(Some(0), Some(0), true, 0.9);
        creep.record_usage(Some(1), Some(1), true, 0.9);
        creep.record_usage(Some(2), Some(2), true, 1.0);
        creep.scope_hope();
    }

    #[test]
    fn test_basic_madlib_fill() {
        let mut prompt = MadlibPrompt::new(
            "test-001",
            "Test",
            "The {noun} is {adjective}.",
            vec!["noun", "adjective"],
        );
        let result = prompt.fill(&["dog".to_string(), "happy".to_string()]);
        assert_eq!(result.unwrap(), "The dog is happy.");
    }

    #[test]
    fn test_lesson_madlib_slot_extraction() {
        let lesson = LessonMadlib::new(
            "test",
            "Test Lesson",
            "Students will {action_verb} about {noun_topic} using {adjective_method} tools.",
            "6-8",
            "Science",
        );
        assert_eq!(lesson.slots.len(), 3);
        assert_eq!(lesson.slots[0].slot_type, "verb"); // action_verb → verb
        assert_eq!(lesson.slots[1].slot_type, "noun"); // noun_topic → noun
        assert_eq!(lesson.slots[2].slot_type, "adjective"); // adjective_method → adjective
    }

    #[test]
    fn test_fill_slot_with_tamed_creep() {
        let mut lesson = LessonMadlib::new(
            "test",
            "Test",
            "We will {action_verb} the {noun_topic}.",
            "6-8",
            "Test",
        );

        // Create and tame a creep
        let mut creep = SemanticCreep::from_word("structure");
        tame_creep(&mut creep);

        let result = lesson.fill_slot("noun_topic", &mut creep);
        assert!(result.is_ok());
        let fill = result.unwrap();
        assert_eq!(fill.creep_word, "structure");
        assert!(fill.cp_earned > 0);
        assert!(creep.context_points > 0);
    }

    #[test]
    fn test_wild_creep_rejected() {
        let mut lesson = LessonMadlib::new(
            "test",
            "Test",
            "The {noun_topic} is important.",
            "6-8",
            "Test",
        );

        let mut wild = SemanticCreep::from_word("terrain");
        assert_eq!(wild.state, CreepState::Wild);

        let result = lesson.fill_slot("noun_topic", &mut wild);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Wild"));
    }

    #[test]
    fn test_suggest_creeps() {
        let lesson = LessonMadlib::new(
            "test",
            "Test",
            "Students will {action_verb} the {noun_topic}.",
            "6-8",
            "Test",
        );

        // Create some tamed creeps
        let mut creeps: Vec<SemanticCreep> = vec![
            SemanticCreep::from_word("structure"),
            SemanticCreep::from_word("optimize"),
            SemanticCreep::from_word("terrain"),
        ];
        // Tame them all
        for c in &mut creeps {
            tame_creep(c);
        }

        let suggestions = lesson.suggest_creeps(&creeps);
        assert_eq!(suggestions.len(), 2); // 2 open slots
                                          // Each slot should have at least 1 suggestion
        for (slot_id, candidates) in &suggestions {
            assert!(
                !candidates.is_empty(),
                "Slot {} should have candidates",
                slot_id
            );
        }
    }

    #[test]
    fn test_contest_slot_battle() {
        let mut lesson = LessonMadlib::new("test", "Test", "We study {noun_topic}.", "6-8", "Test");

        let mut creep_a = SemanticCreep::from_word("structure");
        let mut creep_b = SemanticCreep::from_word("terrain");
        // Tame both
        tame_creep(&mut creep_a);
        tame_creep(&mut creep_b);

        let result = lesson.contest_slot("noun_topic", &mut creep_a, &mut creep_b);
        assert!(result.is_ok());
        let fill = result.unwrap();
        assert!(fill.was_contested);
        // Winner should have CP, loser should have some participation CP
        assert!(creep_a.context_points > 0 || creep_b.context_points > 0);
    }

    #[test]
    fn test_completed_text_generation() {
        let mut lesson = LessonMadlib::new(
            "test",
            "Test",
            "The {noun_topic} is {adjective_quality}.",
            "6-8",
            "Test",
        );

        let mut noun = SemanticCreep::from_word("geology");
        let mut adj = SemanticCreep::from_word("creative");
        tame_creep(&mut noun);
        tame_creep(&mut adj);

        lesson.fill_slot("noun_topic", &mut noun).unwrap();
        assert!(!lesson.is_complete());

        lesson.fill_slot("adjective_quality", &mut adj).unwrap();
        assert!(lesson.is_complete());
        assert_eq!(lesson.completed.unwrap(), "The geology is creative.");
    }

    #[test]
    fn test_sample_templates_valid() {
        let templates = sample_lesson_templates();
        assert_eq!(templates.len(), 3);
        for t in &templates {
            assert!(
                !t.slots.is_empty(),
                "Template '{}' should have slots",
                t.title
            );
            assert!(!t.grade_level.is_empty());
            assert!(!t.subject.is_empty());
        }
    }
}
