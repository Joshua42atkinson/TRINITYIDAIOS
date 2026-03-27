// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Iron Road / VAAM Subsystem
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:         vaam/litrpg.rs
// BIBLE CAR:    Car 4 — IMPLEMENT (Iron Road Game Mechanics)
// HOOK SCHOOL:  🎭 Identity — Character Sheet
// PURPOSE:      Generates LitRPG "Player Handbook" sections from the user's
//               mastered VAAM vocabulary. Each mastered word becomes a stylized
//               skill entry with tier and definition. The handbook is the
//               narrative reflection of vocabulary mastery — making abstract
//               learning progress feel like character progression.
//
// ARCHITECTURE:
//   • generate_handbook_section() takes player name + quest + mastered words
//   • Outputs formatted markdown with skill entries per word
//   • Fed into the Book of the Bible narrative and CharacterSheet display
//   • Bible Car 4.4: The Bestiary is the Pokédex; this is the skill tree
//
// DEPENDENCIES:
//   - trinity_protocol — VocabularyWord type (word, tier, definition)
//
// CHANGES:
//   2026-03-16  Joshua Atkinson  Created for LitRPG handbook generation
//   2026-03-26  Cascade          Added §17 header
//
// ═══════════════════════════════════════════════════════════════════════════════

use trinity_protocol::VocabularyWord;

/// Generates a "LitRPG / Player Handbook" style block using the user's mastered VAAM words
pub fn generate_handbook_section(
    player_name: &str,
    quest_name: &str,
    mastered_words: &[VocabularyWord],
) -> String {
    let mut handbook = format!("# Player Handbook: The {} Chronicles\n\n", player_name);

    handbook.push_str(&format!("## Quest: {}\n\n", quest_name));

    handbook.push_str("### Unlocked Skills & Mastered Syntax\n");
    if mastered_words.is_empty() {
        handbook.push_str("*No advanced syntax mastered yet. Return to the Iron Road to train.*");
    } else {
        for word in mastered_words {
            // Stylized LitRPG entry for each mastered word. definition is an Option<String> in trinity_protocol
            let def = word
                .definition
                .as_deref()
                .unwrap_or("A mysterious concept yet to be fully defined.");
            handbook.push_str(&format!(
                "- **Skill [{}]** (Tier: {:?}): {}\n",
                word.word, word.tier, def
            ));
        }
    }

    handbook.push_str("\n### System Prompt: Synthesis\n");
    handbook.push_str("The NPU Great Recycler has integrated these syntax blocks into your core cognitive matrix. Your semantic mass has decreased, allowing for faster processing loops in future quests.\n");

    handbook
}
