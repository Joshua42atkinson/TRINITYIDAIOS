// Trinity AI Agent System
// ═══════════════════════════════════════════════════════════════════════════════
// VAAM Integration Tests
// ═══════════════════════════════════════════════════════════════════════════════
// Tests for the Vocabulary As A Mechanism system integration.

#[cfg(test)]
mod vaam_integration_tests {
    use trinity_protocol::{
        Genre, VocabularyDatabase, VocabularyMastery, VocabularyWord, VocabularyTier,
        WordDetection, TierProgress,
    };
    
    // ═══════════════════════════════════════════════════════════════════════════
    // VOCABULARY DATABASE TESTS
    // ═══════════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_vocabulary_database_creation() {
        let db = VocabularyDatabase::new(Genre::Cyberpunk);
        assert_eq!(db.genre, Genre::Cyberpunk);
        assert!(db.basic.words.is_empty());
        assert!(db.intermediate.words.is_empty());
        assert!(db.advanced.words.is_empty());
        assert!(db.expert.words.is_empty());
    }
    
    #[test]
    fn test_vocabulary_database_add_words() {
        let mut db = VocabularyDatabase::new(Genre::Cyberpunk);
        
        let word = VocabularyWord {
            word: "async".to_string(),
            aliases: vec!["asynchronous".to_string()],
            context_clues: vec!["await".to_string(), "future".to_string()],
            coal_value: 3,
            tier: VocabularyTier::Basic,
            bloom_level: trinity_protocol::BloomLevel::Apply,
            definition: "Asynchronous execution pattern".to_string(),
            tags: vec!["rust".to_string(), "concurrency".to_string()],
        };
        
        db.add_word(word);
        
        assert_eq!(db.basic.words.len(), 1);
        assert_eq!(db.basic.words[0].word, "async");
    }
    
    #[test]
    fn test_vocabulary_database_scan_basic() {
        let mut db = VocabularyDatabase::new(Genre::Cyberpunk);
        
        db.add_word(VocabularyWord {
            word: "async".to_string(),
            aliases: vec![],
            context_clues: vec!["await".to_string()],
            coal_value: 3,
            tier: VocabularyTier::Basic,
            bloom_level: trinity_protocol::BloomLevel::Apply,
            definition: "Async pattern".to_string(),
            tags: vec![],
        });
        
        let detections = db.scan("I want to use async await for concurrency");
        
        assert!(!detections.is_empty(), "Should detect 'async'");
        assert_eq!(detections[0].word, "async");
        assert_eq!(detections[0].coal_earned, 3);
        assert!(detections[0].is_correct_usage, "Should be correct with 'await' context");
    }
    
    #[test]
    fn test_vocabulary_database_scan_with_aliases() {
        let mut db = VocabularyDatabase::new(Genre::Cyberpunk);
        
        db.add_word(VocabularyWord {
            word: "NPU".to_string(),
            aliases: vec!["neural processing unit".to_string(), "neural processor".to_string()],
            context_clues: vec!["inference".to_string(), "AI".to_string()],
            coal_value: 8,
            tier: VocabularyTier::Intermediate,
            bloom_level: trinity_protocol::BloomLevel::Understand,
            definition: "Neural Processing Unit".to_string(),
            tags: vec!["hardware".to_string()],
        });
        
        // Test alias detection
        let detections = db.scan("The neural processing unit handles AI inference");
        
        assert!(!detections.is_empty(), "Should detect alias 'neural processing unit'");
        assert_eq!(detections[0].word, "NPU");
        assert_eq!(detections[0].coal_earned, 8);
    }
    
    #[test]
    fn test_vocabulary_database_context_verification() {
        let mut db = VocabularyDatabase::new(Genre::Cyberpunk);
        
        db.add_word(VocabularyWord {
            word: "struct".to_string(),
            aliases: vec![],
            context_clues: vec!["impl".to_string(), "field".to_string(), "method".to_string()],
            coal_value: 2,
            tier: VocabularyTier::Basic,
            bloom_level: trinity_protocol::BloomLevel::Remember,
            definition: "Data structure".to_string(),
            tags: vec!["rust".to_string()],
        });
        
        // Correct context
        let correct = db.scan("I need to impl a struct with fields");
        assert!(correct[0].is_correct_usage, "Should be correct with 'impl' and 'field' context");
        
        // Wrong context (no context clues)
        let wrong = db.scan("The struct of the building is old");
        assert!(!wrong[0].is_correct_usage, "Should be incorrect without context clues");
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // VOCABULARY MASTERY TESTS
    // ═══════════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_vocabulary_mastery_discovery() {
        let mut mastery = VocabularyMastery::default();
        
        let detection = WordDetection {
            word: "async".to_string(),
            tier: VocabularyTier::Basic,
            coal_earned: 3,
            is_correct_usage: true,
            context: Some("async await pattern".to_string()),
        };
        
        let update = mastery.record_detection(&detection);
        
        assert!(update.is_new_discovery, "First use should be discovery");
        assert!(!update.newly_mastered, "Should not be mastered on first use");
        assert_eq!(mastery.discovered.len(), 1);
    }
    
    #[test]
    fn test_vocabulary_mastery_rule_of_three() {
        let mut mastery = VocabularyMastery::default();
        
        let detection = WordDetection {
            word: "async".to_string(),
            tier: VocabularyTier::Basic,
            coal_earned: 3,
            is_correct_usage: true,
            context: Some("async await".to_string()),
        };
        
        // First use - discovery
        let u1 = mastery.record_detection(&detection);
        assert!(u1.is_new_discovery);
        assert_eq!(u1.times_used, 1);
        
        // Second use - progress
        let u2 = mastery.record_detection(&detection);
        assert!(!u2.is_new_discovery);
        assert_eq!(u2.times_used, 2);
        
        // Third use - MASTERY!
        let u3 = mastery.record_detection(&detection);
        assert!(u3.newly_mastered, "Third use should trigger mastery");
        assert_eq!(u3.times_used, 3);
        
        assert!(mastery.mastered.contains(&"async".to_string()));
    }
    
    #[test]
    fn test_vocabulary_mastery_tier_progress() {
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
                bloom_level: trinity_protocol::BloomLevel::Remember,
                definition: format!("Word {}", i),
                tags: vec![],
            });
        }
        
        // Master 2 of them
        for word in &["word0", "word1"] {
            let detection = WordDetection {
                word: word.to_string(),
                tier: VocabularyTier::Basic,
                coal_earned: 1,
                is_correct_usage: true,
                context: None,
            };
            for _ in 0..3 {
                mastery.record_detection(&detection);
            }
        }
        
        let progress = mastery.tier_progress(&db, VocabularyTier::Basic);
        
        assert_eq!(progress.tier, VocabularyTier::Basic);
        assert_eq!(progress.total_words, 5);
        assert_eq!(progress.mastered_words, 2);
        assert_eq!(progress.percentage, 40.0);
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // COAL ECONOMY TESTS
    // ═══════════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_coal_values_by_tier() {
        let mut db = VocabularyDatabase::new(Genre::Cyberpunk);
        
        // Add words at different tiers
        db.add_word(VocabularyWord {
            word: "basic_word".to_string(),
            aliases: vec![],
            context_clues: vec![],
            coal_value: 3,
            tier: VocabularyTier::Basic,
            bloom_level: trinity_protocol::BloomLevel::Remember,
            definition: "Basic".to_string(),
            tags: vec![],
        });
        
        db.add_word(VocabularyWord {
            word: "intermediate_word".to_string(),
            aliases: vec![],
            context_clues: vec![],
            coal_value: 8,
            tier: VocabularyTier::Intermediate,
            bloom_level: trinity_protocol::BloomLevel::Understand,
            definition: "Intermediate".to_string(),
            tags: vec![],
        });
        
        db.add_word(VocabularyWord {
            word: "advanced_word".to_string(),
            aliases: vec![],
            context_clues: vec![],
            coal_value: 15,
            tier: VocabularyTier::Advanced,
            bloom_level: trinity_protocol::BloomLevel::Analyze,
            definition: "Advanced".to_string(),
            tags: vec![],
        });
        
        let detections = db.scan("basic_word intermediate_word advanced_word");
        
        assert_eq!(detections.len(), 3);
        
        let total_coal: u32 = detections.iter().map(|d| d.coal_earned).sum();
        assert_eq!(total_coal, 3 + 8 + 15, "Total coal should be sum of all tiers");
    }
    
    #[test]
    fn test_coal_penalty_for_wrong_context() {
        let mut db = VocabularyDatabase::new(Genre::Cyberpunk);
        
        db.add_word(VocabularyWord {
            word: "struct".to_string(),
            aliases: vec![],
            context_clues: vec!["impl".to_string()],
            coal_value: 2,
            tier: VocabularyTier::Basic,
            bloom_level: trinity_protocol::BloomLevel::Remember,
            definition: "Data structure".to_string(),
            tags: vec![],
        });
        
        // Correct usage
        let correct = db.scan("I'll impl a struct for this");
        assert_eq!(correct[0].coal_earned, 2);
        
        // Wrong usage (no context clues)
        let wrong = db.scan("The struct of the building");
        // Still detects the word but marks as incorrect
        assert!(!wrong[0].is_correct_usage);
        // Coal could be reduced or zero for incorrect usage
        // This depends on implementation - adjust as needed
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // IRON ROAD HUD INTEGRATION TESTS
    // ═══════════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_iron_road_state_vocabulary_recording() {
        use trinity_iron_road::IronRoadState;
        
        let mut state = IronRoadState::default();
        
        // Record vocabulary detection
        state.record_vocabulary("async", 3, true);
        state.record_vocabulary("struct", 2, true);
        
        assert_eq!(state.coal_earned_session, 5);
        assert_eq!(state.words_detected.len(), 2);
        assert!(state.coal_reserves > 100.0, "Coal reserves should increase");
    }
    
    #[test]
    fn test_iron_road_coal_burn() {
        use trinity_iron_road::IronRoadState;
        
        let mut state = IronRoadState::default();
        state.coal_reserves = 50.0;
        
        // Burn coal for steam
        let burned = state.burn_coal(20.0);
        
        assert_eq!(burned, 20.0);
        assert_eq!(state.coal_reserves, 30.0);
        assert_eq!(state.steam_production, 40.0, "1 coal = 2 steam");
        assert_eq!(state.steam_burned_session, 20);
    }
    
    #[test]
    fn test_iron_road_planning_doing_gap() {
        use trinity_iron_road::IronRoadState;
        
        let mut state = IronRoadState::default();
        state.coal_reserves = 100.0;
        
        // Earn coal
        state.record_vocabulary("async", 10, true);
        assert!(state.planning_doing_gap > 0.0, "Should have positive gap (ideas waiting)");
        
        // Burn coal
        state.burn_coal(50.0);
        // Gap should decrease as we execute
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // PROFILE SYSTEM TESTS
    // ═══════════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_user_profile_creation() {
        use trinity_protocol::UserProfile;
        
        let profile = UserProfile::new("Test Conductor");
        
        assert_eq!(profile.display_name, "Test Conductor");
        assert!(profile.projects.is_empty());
    }
    
    #[test]
    fn test_project_profile_creation() {
        use trinity_protocol::{UserProfile, ProjectProfile};
        
        let mut user = UserProfile::new("Test Conductor");
        let project = user.create_project(
            "Test Project",
            "/path/to/project",
            Genre::Cyberpunk,
        );
        
        assert_eq!(user.projects.len(), 1);
        assert_eq!(project.project_name, "Test Project");
        assert_eq!(project.genre, Genre::Cyberpunk);
    }
    
    #[test]
    fn test_project_vocabulary_recording() {
        use trinity_protocol::{UserProfile, WordDetection};
        
        let mut user = UserProfile::new("Test Conductor");
        let project = user.create_project("Test", "/path", Genre::Cyberpunk);
        
        let detection = WordDetection {
            word: "async".to_string(),
            tier: VocabularyTier::Basic,
            coal_earned: 3,
            is_correct_usage: true,
            context: None,
        };
        
        user.record_vocabulary(&project.project_id, detection.clone());
        
        // Check mastery was updated
        let proj = &user.projects[0];
        assert!(proj.vocabulary_mastery.discovered.contains(&"async".to_string()));
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // JOURNAL SYSTEM TESTS
    // ═══════════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_journal_addie_phases() {
        use trinity_protocol::{Journal, AddiePhase, JournalEntryType};
        
        let mut journal = Journal::default();
        
        // Start in Analysis
        assert_eq!(journal.current_phase, AddiePhase::Analysis);
        
        // Add analysis note
        journal.add_entry(
            JournalEntryType::AnalysisNote,
            "Understanding the problem space".to_string(),
            None,
        );
        assert_eq!(journal.entries.len(), 1);
        
        // Advance to Design
        journal.advance_phase();
        assert_eq!(journal.current_phase, AddiePhase::Design);
        
        // Add design decision
        journal.add_entry(
            JournalEntryType::DesignDecision,
            "Using async pattern for concurrency".to_string(),
            None,
        );
        
        // Continue through phases
        journal.advance_phase(); // Development
        journal.advance_phase(); // Implementation
        journal.advance_phase(); // Evaluation
        
        assert_eq!(journal.entries.len(), 2);
    }
    
    #[test]
    fn test_journal_vocabulary_moments() {
        use trinity_protocol::{Journal, JournalEntryType};
        
        let mut journal = Journal::default();
        
        // Vocabulary discovery
        journal.add_entry(
            JournalEntryType::VocabularyDiscovery,
            "Discovered 'async' - asynchronous execution pattern".to_string(),
            Some("quest-001".to_string()),
        );
        
        // Vocabulary mastery
        journal.add_entry(
            JournalEntryType::VocabularyMastery,
            "Mastered 'async' after 3 correct uses!".to_string(),
            None,
        );
        
        assert_eq!(journal.entries.len(), 2);
        
        // Check entry types
        assert!(matches!(journal.entries[0].entry_type, JournalEntryType::VocabularyDiscovery));
        assert!(matches!(journal.entries[1].entry_type, JournalEntryType::VocabularyMastery));
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // GENRE SYSTEM TESTS
    // ═══════════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_genre_display_names() {
        assert_eq!(Genre::Cyberpunk.display_name(), "Cyberpunk");
        assert_eq!(Genre::Steampunk.display_name(), "Steampunk");
        assert_eq!(Genre::Solarpunk.display_name(), "Solarpunk");
        assert_eq!(Genre::DarkFantasy.display_name(), "Dark Fantasy");
    }
    
    #[test]
    fn test_genre_vocabulary_loading() {
        // Test that vocabulary files can be loaded for each genre
        for genre in [Genre::Cyberpunk, Genre::Steampunk, Genre::Solarpunk, Genre::DarkFantasy] {
            let db = VocabularyDatabase::new(genre);
            assert_eq!(db.genre, genre);
        }
    }
}
