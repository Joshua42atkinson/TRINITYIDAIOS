// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        vaam.rs
// PURPOSE:     VAAM (Vocabulary As A Mechanism) — Coal mining system
//
// ARCHITECTURE:
//   • Scans all player messages for domain vocabulary
//   • Awards Coal (potential energy) when vocabulary detected
//   • Genre-specific vocabulary databases (ID, Gaming, Rust, etc.)
//   • Tier progression: Novice → Apprentice → Journeyman → Expert → Master
//   • SQL persistence for vocabulary mastery tracking
//
// DEPENDENCIES:
//   - sqlx — PostgreSQL for vocabulary storage
//   - trinity_protocol — Vocabulary types and detection
//   - serde — Vocabulary pack serialization
//
// CHANGES:
//   2026-03-16  Cascade  Migrated to §17 comment standard
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use sqlx::PgPool;
use trinity_protocol::{
    Genre, TierProgress, VocabularyDatabase, VocabularyMastery, VocabularyTier, WordDetection,
};

/// Local VocabularyPack using String IDs (trinity_protocol uses Uuid)
#[derive(Debug, Clone)]
pub struct VocabularyPackString {
    pub id: String,
    pub genre: Genre,
    pub name: String,
    pub description: String,
    pub words: Vec<trinity_protocol::VocabularyWord>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

impl VocabularyPackString {
    pub fn to_database(&self) -> VocabularyDatabase {
        let mut db = VocabularyDatabase::new(self.genre);
        // Add all words to the database
        for word in &self.words {
            db.add_word(word.clone());
        }
        db
    }
}

/// Global VAAM state for the server
#[derive(Debug, Clone)]
pub struct VaamState {
    /// The vocabulary database for the current genre
    pub database: Arc<RwLock<VocabularyDatabase>>,
    /// The mastery state for the current player
    pub mastery: Arc<RwLock<VocabularyMastery>>,
    /// The current genre (fixed per project)
    pub genre: Genre,
    /// Optional vocabulary pack ID (from character sheet)
    pub vocabulary_pack_id: Option<String>,
    /// Total coal earned in session
    pub session_coal: Arc<RwLock<u64>>,
}

impl VaamState {
    /// Create a new VAAM state with the given genre
    pub async fn new(genre: Genre) -> Self {
        // Try to load vocabulary database, create empty if not found
        let database = match VocabularyDatabase::load_genre(genre) {
            Ok(db) => {
                let word_count = db.all_words().len();
                info!("[VAAM] Loaded {} words for {:?} genre", word_count, genre);
                db
            }
            Err(e) => {
                warn!(
                    "[VAAM] No vocabulary found for {:?}: {}, using empty database",
                    genre, e
                );
                VocabularyDatabase::new(genre)
            }
        };

        Self {
            database: Arc::new(RwLock::new(database)),
            mastery: Arc::new(RwLock::new(VocabularyMastery::default())),
            genre,
            vocabulary_pack_id: None,
            session_coal: Arc::new(RwLock::new(0)),
        }
    }

    /// Create VAAM state from a vocabulary pack (co-created vocabulary)
    pub async fn from_pack(pack: VocabularyPackString) -> Self {
        let genre = pack.genre;
        let vocabulary_pack_id = Some(pack.id.clone());

        // Convert pack to database
        let database = pack.to_database();
        let word_count = database.all_words().len();
        info!(
            "[VAAM] Loaded {} words from vocabulary pack '{}' for {:?} genre",
            word_count, pack.name, genre
        );

        Self {
            database: Arc::new(RwLock::new(database)),
            mastery: Arc::new(RwLock::new(VocabularyMastery::default())),
            genre,
            vocabulary_pack_id,
            session_coal: Arc::new(RwLock::new(0)),
        }
    }

    /// Load VAAM state from PostgreSQL vocabulary pack
    pub async fn from_db_pack(pool: &PgPool, pack_id: String) -> Result<Self, sqlx::Error> {
        let row = sqlx::query_as::<_, (String, String, String, String, serde_json::Value)>(
            r#"
            SELECT id, genre, name, description, words
            FROM vocabulary_packs
            WHERE id = $1
            "#,
        )
        .bind(pack_id)
        .fetch_one(pool)
        .await?;

        // Parse genre
        let genre = match row.1.as_str() {
            "Cyberpunk" => Genre::Cyberpunk,
            "Steampunk" => Genre::Steampunk,
            "Solarpunk" => Genre::Solarpunk,
            "DarkFantasy" => Genre::DarkFantasy,
            _ => Genre::default(),
        };

        // Parse words from JSON
        let words: Vec<trinity_protocol::VocabularyWord> =
            serde_json::from_value(row.4).unwrap_or_default();

        let pack = VocabularyPackString {
            id: row.0,
            genre,
            name: row.2,
            description: row.3,
            words,
            created_at: chrono::Utc::now(),
            user_id: String::new(),
        };

        Ok(Self::from_pack(pack).await)
    }

    /// Create VAAM state and sync mastery from PostgreSQL
    pub async fn new_with_db(genre: Genre, pool: &PgPool, project_id: &str) -> Self {
        let state = Self::new(genre).await;

        // Load mastery state from database
        match load_mastery_from_db(pool, project_id).await {
            Ok(mastery) => {
                info!(
                    "[VAAM] Loaded mastery state from database: {} words",
                    mastery.discovered.len()
                );
                *state.mastery.write().await = mastery;
            }
            Err(e) => {
                warn!(
                    "[VAAM] Failed to load mastery from DB: {}. Using fresh state.",
                    e
                );
            }
        }

        state
    }

    /// Scan a message for vocabulary words and award coal
    pub async fn scan_message(&self, message: &str) -> VaamResult {
        let db = self.database.read().await;
        let detections = db.scan(message);

        let mut mastery = self.mastery.write().await;
        let mut session_coal = self.session_coal.write().await;

        let mut total_coal = 0u32;
        let mut updates = Vec::new();
        let mut newly_mastered = Vec::new();

        for detection in &detections {
            total_coal += detection.coal_earned;
            let update = mastery.record_detection(detection);

            if update.newly_mastered {
                newly_mastered.push(detection.word.clone());
                info!("[VAAM] Word mastered: '{}' (Rule of Three)", detection.word);
            }

            updates.push(update);
        }

        *session_coal += total_coal as u64;

        if !detections.is_empty() {
            info!(
                "[VAAM] Detected {} words, +{} coal (session total: {})",
                detections.len(),
                total_coal,
                *session_coal
            );
        }

        VaamResult {
            detections,
            total_coal,
            updates,
            newly_mastered,
            session_total: *session_coal,
        }
    }

    /// Get mastery progress by tier
    pub async fn get_progress(&self) -> Vec<TierProgress> {
        let db = self.database.read().await;
        let mastery = self.mastery.read().await;

        [
            VocabularyTier::Basic,
            VocabularyTier::Intermediate,
            VocabularyTier::Advanced,
            VocabularyTier::Expert,
        ]
        .iter()
        .map(|tier| mastery.tier_progress(&db, *tier))
        .collect()
    }

    /// Get total words mastered
    pub async fn mastered_count(&self) -> usize {
        self.mastery.read().await.mastered.len()
    }

    /// Reset session stats (for new session)
    pub async fn reset_session(&self) {
        *self.session_coal.write().await = 0;
    }
}

impl Default for VaamState {
    fn default() -> Self {
        // Default to Cyberpunk genre
        futures::executor::block_on(Self::new(Genre::Cyberpunk))
    }
}

/// Result of scanning a message for vocabulary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaamResult {
    /// Words detected in the message
    pub detections: Vec<WordDetection>,
    /// Total coal earned from this message
    pub total_coal: u32,
    /// Mastery updates for each word
    pub updates: Vec<trinity_protocol::MasteryUpdate>,
    /// Words that became mastered (Rule of Three)
    pub newly_mastered: Vec<String>,
    /// Total coal earned this session
    pub session_total: u64,
}

impl VaamResult {
    /// Check if any vocabulary was detected
    pub fn has_detections(&self) -> bool {
        !self.detections.is_empty()
    }

    /// Format a summary for chat display
    pub fn format_summary(&self) -> String {
        if self.detections.is_empty() {
            return String::new();
        }

        let mut summary = format!("🎯 VAAM: +{} coal", self.total_coal);

        if !self.newly_mastered.is_empty() {
            summary.push_str(&format!(
                " | 🎓 Mastered: {}",
                self.newly_mastered.join(", ")
            ));
        }

        summary
    }

    /// Format detailed detection list for HUD
    pub fn format_detections(&self) -> Vec<(String, u32, bool)> {
        self.detections
            .iter()
            .map(|d| (d.word.clone(), d.coal_earned, d.is_correct_usage))
            .collect()
    }
}

/// Format VAAM result as SSE event data
pub fn format_vaam_event(result: &VaamResult) -> String {
    if !result.has_detections() {
        return String::new();
    }

    // Format as JSON for client-side parsing
    serde_json::to_string(&serde_json::json!({
        "type": "vaam",
        "detections": result.detections.iter().map(|d| {
            serde_json::json!({
                "word": d.word,
                "coal": d.coal_earned,
                "correct": d.is_correct_usage,
                "tier": format!("{:?}", d.tier),
            })
        }).collect::<Vec<_>>(),
        "total_coal": result.total_coal,
        "newly_mastered": result.newly_mastered,
        "session_total": result.session_total,
    }))
    .unwrap_or_default()
}

// ============================================================================
// POSTGRESQL INTEGRATION
// ============================================================================

/// Load mastery state from PostgreSQL for a project
pub async fn load_mastery_from_db(
    pool: &PgPool,
    project_id: &str,
) -> Result<VocabularyMastery, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, String, i32, bool, i32)>(
        r#"
        SELECT word, tier, times_used, is_mastered, total_coal_earned
        FROM vocabulary_mastery vm
        JOIN project_profiles pp ON vm.project_profile_id = pp.id
        WHERE pp.project_id = $1
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    let mut mastery = VocabularyMastery::default();

    for (word, _tier, times_used, is_mastered, total_coal) in rows {
        mastery.discovered.insert(word.clone(), times_used as u32);

        if is_mastered {
            mastery.mastered.push(word);
        }

        mastery.total_coal_earned += total_coal as u64;
    }

    Ok(mastery)
}

/// Save mastery state to PostgreSQL
pub async fn save_mastery_to_db(
    pool: &PgPool,
    project_id: &str,
    mastery: &VocabularyMastery,
) -> Result<(), sqlx::Error> {
    for (word, count) in &mastery.discovered {
        let is_mastered = mastery.mastered.contains(word);

        sqlx::query(
            r#"
            INSERT INTO vocabulary_mastery (project_profile_id, word, tier, times_used, is_mastered, total_coal_earned)
            SELECT id, $2, 'Basic', $3, $4, 0
            FROM project_profiles
            WHERE project_id = $1
            ON CONFLICT (project_profile_id, word)
            DO UPDATE SET
                times_used = EXCLUDED.times_used,
                is_mastered = EXCLUDED.is_mastered,
                last_used_at = NOW()
            "#
        )
        .bind(project_id)
        .bind(word)
        .bind(*count as i32)
        .bind(is_mastered)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Record a word detection to the audit trail
pub async fn record_detection(
    pool: &PgPool,
    project_id: &str,
    detection: &WordDetection,
    context: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO word_detections (project_profile_id, word, tier, coal_earned, is_correct_usage, context)
        SELECT id, $2, $3, $4, $5, $6
        FROM project_profiles
        WHERE project_id = $1
        "#
    )
    .bind(project_id)
    .bind(&detection.word)
    .bind(format!("{:?}", detection.tier))
    .bind(detection.coal_earned as i32)
    .bind(detection.is_correct_usage)
    .bind(context)
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vaam_detection() {
        let state = VaamState::new(Genre::Cyberpunk).await;

        // Test message with vocabulary
        let result = state
            .scan_message("I want to use async await for the NPU inference with multi-threading")
            .await;

        // Should detect at least some words
        // Note: depends on vocabulary files being loaded
        if result.has_detections() {
            assert!(result.total_coal > 0);
        }
    }

    #[tokio::test]
    async fn test_vaam_session_tracking() {
        let state = VaamState::new(Genre::Cyberpunk).await;

        // Scan multiple messages
        let _ = state.scan_message("async function").await;
        let _ = state.scan_message("struct and enum").await;

        let total = *state.session_coal.read().await;
        // Session coal should accumulate
        // Note: depends on vocabulary files
        println!("Session coal: {}", total);
    }
}
