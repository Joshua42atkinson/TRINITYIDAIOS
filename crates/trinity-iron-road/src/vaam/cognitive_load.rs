// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Iron Road / VAAM Subsystem
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:         vaam/cognitive_load.rs
// BIBLE CAR:    Car 7 — REPETITION (Cognitive Load Theory in Code)
// HOOK SCHOOL:  🏫 Pedagogy — CLT Engine
// PURPOSE:      Readability scoring via Flesch-Kincaid Grade Level formula.
//               Measures intrinsic cognitive load of generated text against the
//               user's known VAAM vocabulary. Outputs a CognitiveLoadScore with
//               word complexity, grade level, and tier-match percentage — used
//               by Pete to calibrate response complexity to the learner's level.
//
// ARCHITECTURE:
//   • calculate_cognitive_load() takes text + user VAAM tier + known words
//   • Flesch-Kincaid: 0.39*(words/sentences) + 11.8*(syllables/words) - 15.59
//   • count_syllables() uses vowel-cluster heuristic with silent-e correction
//   • tier_match_percentage = (known complex words / total complex words) × 100
//   • Bible Car 7.3: Coal = intrinsic load, Steam = germane, Friction = extraneous
//
// DEPENDENCIES:
//   - trinity_protocol — VocabularyTier, VocabularyWord types
//
// CHANGES:
//   2026-03-16  Joshua Atkinson  Created for VAAM readability scoring
//   2026-03-26  Cascade          Added §17 header
//
// ═══════════════════════════════════════════════════════════════════════════════

use trinity_protocol::{VocabularyTier, VocabularyWord};

/// Represents the cognitive weight/load of a piece of generated text
#[derive(Debug, Clone)]
pub struct CognitiveLoadScore {
    pub total_words: usize,
    pub complex_words: usize,
    pub flesch_kincaid_grade: f32,
    pub tier_match_percentage: f32,
}

/// A basic heuristic readability calculator using Flesch-Kincaid formula
pub fn calculate_cognitive_load(
    text: &str,
    _user_tier: VocabularyTier,
    known_words: &[VocabularyWord],
) -> CognitiveLoadScore {
    let words: Vec<&str> = text.split_whitespace().collect();
    let total_words = words.len();
    let sentences = text.split(['.', '!', '?']).count().max(1);

    // Simple heuristic for syllables: count vowels
    let total_syllables: usize = words.iter().map(|w| count_syllables(w)).sum();

    // Flesch-Kincaid Grade Level Formula
    // 0.39 * (total_words / sentences) + 11.8 * (total_syllables / total_words) - 15.59
    let words_per_sentence = total_words as f32 / sentences as f32;
    let syllables_per_word = total_syllables as f32 / total_words.max(1) as f32;

    let flesch_kincaid_grade = (0.39 * words_per_sentence) + (11.8 * syllables_per_word) - 15.59;

    // Count complex words (>= 3 syllables)
    let complex_words = words.iter().filter(|w| count_syllables(w) >= 3).count();

    // Check how many of the complex words in the text match the user's known VAAM words
    let matched_known = words
        .iter()
        .filter(|w| {
            let clean = w
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            known_words.iter().any(|kw| kw.word.to_lowercase() == clean)
        })
        .count();

    let tier_match_percentage = if complex_words > 0 {
        (matched_known as f32 / complex_words as f32) * 100.0
    } else {
        100.0
    };

    CognitiveLoadScore {
        total_words,
        complex_words,
        flesch_kincaid_grade,
        tier_match_percentage,
    }
}

/// Very basic syllable counter (vowel heuristic)
fn count_syllables(word: &str) -> usize {
    let word = word
        .trim_matches(|c: char| !c.is_alphabetic())
        .to_lowercase();
    if word.is_empty() {
        return 0;
    }

    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let mut count = 0;
    let mut last_was_vowel = false;

    for ch in word.chars() {
        let is_vowel = vowels.contains(&ch);
        if is_vowel && !last_was_vowel {
            count += 1;
        }
        last_was_vowel = is_vowel;
    }

    // Subtract 1 for silent 'e' at the end (unless it's the only vowel)
    if word.ends_with('e') && count > 1 {
        count -= 1;
    }

    count.max(1)
}
