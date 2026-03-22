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
