// Trinity AI Agent System
// Copyright (c) Joshua
//
// ═══════════════════════════════════════════════════════════════════════════════
// 📡 ZONE: PROTOCOL | Module: Vocabulary (VAAM)
// ═══════════════════════════════════════════════════════════════════════════════
// VAAM = Vocabulary As A Mechanism
//
// A game mechanic that rewards players for using domain-specific vocabulary
// correctly in context. Words detected in player input earn Coal (potential energy)
// which can be burned to generate Steam (actual work output).
//
// This transforms learning into a tangible game economy:
// - Player says: "I want to use the NPU for multi-threading"
// - VAAM detects: [NPU] [multi-threading] in correct context
// - System rewards: +15 Coal
// - Player delegates to AI: Burns Coal → generates Steam

use crate::character_sheet::BloomLevel;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The narrative genre for a project.
/// Genre determines vocabulary sets, narrative style, and visual theme.
/// Fixed at project creation - switching resets progress.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum Genre {
    /// Tech-noir, neon, gritty. Vocabulary: systems, networks, hacking.
    #[default]
    Cyberpunk,
    /// Victorian, industrial, brass. Vocabulary: mechanics, steam, engineering.
    Steampunk,
    /// Hopeful, organic, bright. Vocabulary: sustainability, ecology, growth.
    Solarpunk,
    /// Gothic, mysterious, shadow. Vocabulary: magic, runes, transformation.
    DarkFantasy,
}

impl Genre {
    pub fn display_name(&self) -> &'static str {
        match self {
            Genre::Cyberpunk => "Cyberpunk",
            Genre::Steampunk => "Steampunk",
            Genre::Solarpunk => "Solarpunk",
            Genre::DarkFantasy => "Dark Fantasy",
        }
    }

    pub fn narrative_style(&self) -> &'static str {
        match self {
            Genre::Cyberpunk => "Neon-lit corridors of data. The machine speaks in static.",
            Genre::Steampunk => "Brass gears turn with purpose. Steam carries ambition.",
            Genre::Solarpunk => "Green shoots through concrete. Hope grows in cracks.",
            Genre::DarkFantasy => "Shadows hold secrets. Runes glow with ancient intent.",
        }
    }

    pub fn vocab_path(&self) -> &'static str {
        match self {
            Genre::Cyberpunk => "data/vocab/cyberpunk",
            Genre::Steampunk => "data/vocab/steampunk",
            Genre::Solarpunk => "data/vocab/solarpunk",
            Genre::DarkFantasy => "data/vocab/dark_fantasy",
        }
    }
}

/// Vocabulary tier determines coal value and complexity.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum VocabularyTier {
    /// Common words, 1-5 Coal per correct usage
    #[default]
    Basic,
    /// Intermediate complexity, 5-10 Coal
    Intermediate,
    /// Advanced concepts, 10-20 Coal
    Advanced,
    /// Expert-level, 20-50 Coal
    Expert,
}

impl VocabularyTier {
    pub fn coal_range(&self) -> (u32, u32) {
        match self {
            VocabularyTier::Basic => (1, 5),
            VocabularyTier::Intermediate => (5, 10),
            VocabularyTier::Advanced => (10, 20),
            VocabularyTier::Expert => (20, 50),
        }
    }

    pub fn default_coal(&self) -> u32 {
        let (min, max) = self.coal_range();
        (min + max) / 2
    }
}

/// A single vocabulary word that can be detected and rewarded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyWord {
    /// The primary word/term
    pub word: String,
    /// Alternative forms that also count (e.g., "NPU" matches "neural processing unit")
    pub aliases: Vec<String>,
    /// Words that indicate correct usage context (e.g., ["thread", "parallel", "async"] for "NPU")
    pub context_clues: Vec<String>,
    /// Coal earned for correct usage
    pub coal_value: u32,
    /// Complexity tier
    pub tier: VocabularyTier,
    /// Cognitive level required
    pub bloom_level: BloomLevel,
    /// Optional definition for journal display
    pub definition: Option<String>,
    /// Tags for semantic grouping
    pub tags: Vec<String>,
}

/// A vocabulary pack created by the user with AI assistance.
/// Stored in PostgreSQL and JSON for persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyPack {
    /// Unique identifier for this pack
    pub id: uuid::Uuid,
    /// The genre this pack is associated with
    pub genre: Genre,
    /// User-visible name for the pack
    pub name: String,
    /// Description of the pack's focus
    pub description: String,
    /// The vocabulary words in this pack
    pub words: Vec<VocabularyWord>,
    /// When this pack was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The user who created this pack
    pub user_id: uuid::Uuid,
}

impl VocabularyPack {
    /// Create a new empty pack
    pub fn new(genre: Genre, name: String, description: String, user_id: uuid::Uuid) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            genre,
            name,
            description,
            words: Vec::new(),
            created_at: chrono::Utc::now(),
            user_id,
        }
    }

    /// Save pack to JSON file
    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load pack from JSON file
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let pack: VocabularyPack = serde_json::from_str(&content)?;
        Ok(pack)
    }

    /// Convert to a VocabularyDatabase for use in VAAM
    pub fn to_database(&self) -> VocabularyDatabase {
        let mut db = VocabularyDatabase::new(self.genre);
        for word in &self.words {
            db.add_word(word.clone());
        }
        db
    }
}

/// A vocabulary suggestion from AI for user review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularySuggestion {
    /// The suggested word
    pub word: String,
    /// Definition of the word
    pub definition: String,
    /// Suggested tier
    pub tier: VocabularyTier,
    /// Suggested coal value
    pub coal_value: u32,
    /// Context clues for detection
    pub context_clues: Vec<String>,
    /// Aliases for the word
    pub aliases: Vec<String>,
    /// Tags for grouping
    pub tags: Vec<String>,
}

impl VocabularySuggestion {
    /// Convert to a VocabularyWord
    pub fn to_word(&self) -> VocabularyWord {
        VocabularyWord {
            word: self.word.clone(),
            definition: Some(self.definition.clone()),
            tier: self.tier,
            coal_value: self.coal_value,
            bloom_level: BloomLevel::Understand,
            aliases: self.aliases.clone(),
            context_clues: self.context_clues.clone(),
            tags: self.tags.clone(),
        }
    }
}

impl VocabularyWord {
    /// Check if this word matches the given input (case-insensitive)
    pub fn matches(&self, input: &str) -> bool {
        let input_lower = input.to_lowercase();

        // Check primary word
        if input_lower.contains(&self.word.to_lowercase()) {
            return true;
        }

        // Check aliases
        for alias in &self.aliases {
            if input_lower.contains(&alias.to_lowercase()) {
                return true;
            }
        }

        false
    }

    /// Verify the word is used in correct context
    pub fn verify_context(&self, context: &str) -> bool {
        let context_lower = context.to_lowercase();

        // If no context clues defined, accept any usage
        if self.context_clues.is_empty() {
            return true;
        }

        // Check if at least one context clue appears nearby
        self.context_clues
            .iter()
            .any(|clue| context_lower.contains(&clue.to_lowercase()))
    }

    /// Calculate coal earned based on context correctness
    pub fn calculate_coal(&self, context: &str) -> u32 {
        if self.verify_context(context) {
            self.coal_value
        } else {
            // Incorrect usage earns 0 coal but still counts as discovery
            0
        }
    }
}

/// A set of vocabulary words for a specific genre and tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularySet {
    /// The genre this set belongs to
    pub genre: Genre,
    /// The tier level
    pub tier: VocabularyTier,
    /// The words in this set
    pub words: Vec<VocabularyWord>,
}

impl VocabularySet {
    /// Load vocabulary set from JSON file
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let set: VocabularySet = serde_json::from_str(&content)?;
        Ok(set)
    }

    /// Save vocabulary set to JSON file
    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Complete vocabulary database for a project.
/// Contains all tiers for the project's genre.
#[derive(Debug, Clone, Default)]
pub struct VocabularyDatabase {
    /// The genre for this database
    pub genre: Genre,
    /// All vocabulary words organized by tier
    pub by_tier: HashMap<VocabularyTier, Vec<VocabularyWord>>,
    /// Quick lookup by word string
    pub word_index: HashMap<String, usize>,
}

impl VocabularyDatabase {
    /// Create a new empty database for the given genre
    pub fn new(genre: Genre) -> Self {
        Self {
            genre,
            by_tier: HashMap::new(),
            word_index: HashMap::new(),
        }
    }

    /// Load all vocabulary sets for a genre from disk
    pub fn load_genre(genre: Genre) -> anyhow::Result<Self> {
        let mut db = Self::new(genre);
        let base_path = std::path::Path::new(genre.vocab_path());

        for tier in [
            VocabularyTier::Basic,
            VocabularyTier::Intermediate,
            VocabularyTier::Advanced,
            VocabularyTier::Expert,
        ] {
            let filename = format!("{:?}.json", tier).to_lowercase();
            let path = base_path.join(&filename);

            if path.exists() {
                let set = VocabularySet::load(&path)?;
                db.add_set(set);
            }
        }

        Ok(db)
    }

    /// Add a vocabulary set to the database
    pub fn add_set(&mut self, set: VocabularySet) {
        let tier = set.tier;
        let words = set.words;

        for word in &words {
            self.word_index
                .insert(word.word.to_lowercase(), words.len() - 1);
            for alias in &word.aliases {
                self.word_index
                    .insert(alias.to_lowercase(), words.len() - 1);
            }
        }

        self.by_tier.insert(tier, words);
    }

    /// Add a single word to the database
    pub fn add_word(&mut self, word: VocabularyWord) {
        let tier = word.tier;

        // Add to word index
        self.word_index.insert(word.word.to_lowercase(), 0);
        for alias in &word.aliases {
            self.word_index.insert(alias.to_lowercase(), 0);
        }

        // Add to tier
        self.by_tier.entry(tier).or_default().push(word);
    }

    /// Get all words across all tiers
    pub fn all_words(&self) -> Vec<&VocabularyWord> {
        self.by_tier.values().flat_map(|v| v.iter()).collect()
    }

    /// Get words for a specific tier
    pub fn words_for_tier(&self, tier: VocabularyTier) -> Option<&Vec<VocabularyWord>> {
        self.by_tier.get(&tier)
    }

    /// Scan input text for vocabulary words
    pub fn scan(&self, input: &str) -> Vec<WordDetection> {
        let mut detections = Vec::new();

        for word in self.all_words() {
            if word.matches(input) {
                let coal = word.calculate_coal(input);

                detections.push(WordDetection {
                    word: word.word.clone(),
                    tier: word.tier,
                    bloom_level: word.bloom_level,
                    coal_earned: coal,
                    is_correct_usage: coal > 0,
                    context: extract_context(input, &word.word),
                });
            }
        }

        detections
    }
}

/// A detected vocabulary word in player input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDetection {
    /// The word that was detected
    pub word: String,
    /// The tier of the word
    pub tier: VocabularyTier,
    /// The cognitive level
    pub bloom_level: BloomLevel,
    /// Coal earned (0 if used incorrectly)
    pub coal_earned: u32,
    /// Whether the word was used in correct context
    pub is_correct_usage: bool,
    /// The surrounding context (sentence)
    pub context: String,
}

/// Extract context around a word (the sentence containing it)
fn extract_context(input: &str, word: &str) -> String {
    let word_lower = word.to_lowercase();
    let input_lower = input.to_lowercase();

    if let Some(pos) = input_lower.find(&word_lower) {
        // Find sentence boundaries
        let before: String = input[..pos]
            .chars()
            .rev()
            .take_while(|&c| c != '.' && c != '!' && c != '?')
            .collect::<String>()
            .chars()
            .rev()
            .collect();
        let after: String = input[pos..]
            .chars()
            .take_while(|&c| c != '.' && c != '!' && c != '?')
            .collect();

        format!("{}{}", before, after)
    } else {
        input.to_string()
    }
}

/// Player's vocabulary mastery state for a project.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VocabularyMastery {
    /// Words discovered (used at least once)
    pub discovered: HashMap<String, u32>, // word -> times used
    /// Words mastered (used correctly 3 times)
    pub mastered: Vec<String>,
    /// Total coal earned from vocabulary
    pub total_coal_earned: u64,
}

impl VocabularyMastery {
    /// Record a word detection and update mastery
    pub fn record_detection(&mut self, detection: &WordDetection) -> MasteryUpdate {
        let count = self.discovered.entry(detection.word.clone()).or_insert(0);
        *count += 1;

        self.total_coal_earned += detection.coal_earned as u64;

        // Check for mastery (Rule of Three)
        let newly_mastered =
            *count >= 3 && !self.mastered.contains(&detection.word) && detection.is_correct_usage;

        if newly_mastered {
            self.mastered.push(detection.word.clone());
        }

        MasteryUpdate {
            word: detection.word.clone(),
            times_used: *count,
            coal_earned: detection.coal_earned,
            newly_mastered,
            total_mastered: self.mastered.len(),
        }
    }

    /// Check if a word is mastered
    pub fn is_mastered(&self, word: &str) -> bool {
        self.mastered.contains(&word.to_string())
    }

    /// Get mastery progress for a tier
    pub fn tier_progress(&self, db: &VocabularyDatabase, tier: VocabularyTier) -> TierProgress {
        let total = db.words_for_tier(tier).map(|v| v.len()).unwrap_or(0);
        let mastered = self
            .mastered
            .iter()
            .filter(|w| {
                db.words_for_tier(tier)
                    .map(|words| words.iter().any(|vw| &vw.word == *w))
                    .unwrap_or(false)
            })
            .count();

        TierProgress {
            tier,
            total_words: total,
            mastered_words: mastered,
            percentage: if total > 0 {
                (mastered as f32 / total as f32) * 100.0
            } else {
                0.0
            },
        }
    }
}

/// Result of recording a word detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryUpdate {
    pub word: String,
    pub times_used: u32,
    pub coal_earned: u32,
    pub newly_mastered: bool,
    pub total_mastered: usize,
}

/// Progress for a vocabulary tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierProgress {
    pub tier: VocabularyTier,
    pub total_words: usize,
    pub mastered_words: usize,
    pub percentage: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_matching() {
        let word = VocabularyWord {
            word: "async".to_string(),
            aliases: vec!["asynchronous".to_string()],
            context_clues: vec!["await".to_string()],
            coal_value: 3,
            tier: VocabularyTier::Basic,
            bloom_level: BloomLevel::Apply,
            definition: Some("Async pattern".to_string()),
            tags: vec![],
        };

        assert!(word.matches("async"));
        assert!(word.matches("asynchronous"));
        assert!(!word.matches("sync"));
    }

    #[test]
    fn test_context_verification() {
        let word = VocabularyWord {
            word: "struct".to_string(),
            aliases: vec![],
            context_clues: vec!["impl".to_string(), "field".to_string()],
            coal_value: 2,
            tier: VocabularyTier::Basic,
            bloom_level: BloomLevel::Remember,
            definition: Some("Data structure".to_string()),
            tags: vec![],
        };

        assert!(word.verify_context("I need to impl a struct with fields"));
        assert!(!word.verify_context("The struct of the building is old"));
    }

    #[test]
    fn test_coal_calculation() {
        let mut db = VocabularyDatabase::new(Genre::Cyberpunk);

        db.add_word(VocabularyWord {
            word: "async".to_string(),
            aliases: vec![],
            context_clues: vec!["await".to_string()],
            coal_value: 3,
            tier: VocabularyTier::Basic,
            bloom_level: BloomLevel::Apply,
            definition: Some("Async".to_string()),
            tags: vec![],
        });

        db.add_word(VocabularyWord {
            word: "NPU".to_string(),
            aliases: vec![],
            context_clues: vec!["inference".to_string()],
            coal_value: 8,
            tier: VocabularyTier::Intermediate,
            bloom_level: BloomLevel::Understand,
            definition: Some("Neural Processing Unit".to_string()),
            tags: vec![],
        });

        let detections = db.scan("I want to use async await and NPU inference");

        let total_coal: u32 = detections.iter().map(|d| d.coal_earned).sum();
        assert_eq!(total_coal, 11, "Should earn 3 + 8 = 11 coal");
    }

    #[test]
    fn test_vocabulary_mastery_rule_of_three() {
        let mut mastery = VocabularyMastery::default();

        let detection = WordDetection {
            word: "async".to_string(),
            tier: VocabularyTier::Basic,
            bloom_level: BloomLevel::Apply,
            coal_earned: 3,
            is_correct_usage: true,
            context: "async await".to_string(),
        };

        // First use - discovery (times_used = 1)
        let u1 = mastery.record_detection(&detection);
        assert_eq!(u1.times_used, 1, "First use should have times_used = 1");

        // Second use
        let u2 = mastery.record_detection(&detection);
        assert_eq!(u2.times_used, 2);

        // Third use - MASTERY!
        let u3 = mastery.record_detection(&detection);
        assert!(u3.newly_mastered, "Third use should trigger mastery");
        assert!(mastery.mastered.contains(&"async".to_string()));
    }

    #[test]
    fn test_tier_progress() {
        let mut mastery = VocabularyMastery::default();
        let mut db = VocabularyDatabase::new(Genre::Cyberpunk);

        // Add 5 basic words
        for i in 0..5 {
            db.add_word(VocabularyWord {
                word: format!("word{}", i),
                aliases: vec![],
                context_clues: vec![],
                coal_value: 1,
                tier: VocabularyTier::Basic,
                bloom_level: BloomLevel::Remember,
                definition: Some(format!("Word {}", i)),
                tags: vec![],
            });
        }

        // Master 2 of them
        for word in &["word0", "word1"] {
            let detection = WordDetection {
                word: word.to_string(),
                tier: VocabularyTier::Basic,
                bloom_level: BloomLevel::Remember,
                coal_earned: 1,
                is_correct_usage: true,
                context: word.to_string(),
            };
            for _ in 0..3 {
                mastery.record_detection(&detection);
            }
        }

        let progress = mastery.tier_progress(&db, VocabularyTier::Basic);

        assert_eq!(progress.total_words, 5);
        assert_eq!(progress.mastered_words, 2);
        assert_eq!(progress.percentage, 40.0);
    }

    #[test]
    fn test_alias_detection() {
        let mut db = VocabularyDatabase::new(Genre::Cyberpunk);

        db.add_word(VocabularyWord {
            word: "NPU".to_string(),
            aliases: vec!["neural processing unit".to_string()],
            context_clues: vec!["inference".to_string()],
            coal_value: 8,
            tier: VocabularyTier::Intermediate,
            bloom_level: BloomLevel::Understand,
            definition: Some("Neural Processing Unit".to_string()),
            tags: vec![],
        });

        let detections = db.scan("The neural processing unit handles inference");

        assert!(!detections.is_empty(), "Should detect alias");
        assert_eq!(detections[0].word, "NPU");
    }
}
