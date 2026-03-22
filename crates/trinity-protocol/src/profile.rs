// Trinity AI Agent System
// Copyright (c) Joshua
//
// ═══════════════════════════════════════════════════════════════════════════════
// 📡 ZONE: PROTOCOL | Module: Player Profile
// ═══════════════════════════════════════════════════════════════════════════════
// One user, multiple projects, each with its own "player" profile.
//
// The profile system enables:
// - Single user identity across all projects
// - Multiple project workspaces with independent progress
// - Per-project genre selection (fixed at creation)
// - Unified quest board, journal, and character sheet per project
// - Bestiary for tracking Scope Creep
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

use crate::character_sheet::CharacterSheet;
use crate::vocabulary::{Genre, TierProgress, VocabularyMastery, WordDetection};

// -----------------------------------------------------------------------------
// SCOPE CREEP / BESTIARY
// -----------------------------------------------------------------------------

/// The spectrum of how aligned a Scope Creep is with the core project goals.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CreepAlignment {
    /// Highly aligned, mostly light/hope. Worth taming.
    Hope,
    /// Shadow side, too much tech debt or off-mission. Needs banishment.
    Nope,
    /// Neutral or undetermined alignment.
    Neutral,
}

/// A "Scope Creep" monster representing a feature that was amputated or deferred.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeCreepMonster {
    pub id: String,
    pub name: String,
    pub description: String,
    /// 1-100 indicating tech debt or entanglement. Higher is harder to tame.
    pub hp_tech_debt: u32,
    /// VRAM/RAM required, or cognitive load proxy.
    pub mana_cost: String,
    /// Which ADDIE phase this feature is currently stuck in or requires for taming.
    pub taming_phase: AddiePhase,
    pub alignment: CreepAlignment,
    /// Path to the archived file or git branch.
    pub git_artifact: String,
    /// Is this pet currently tamed (integrated) or wild (in Dumpster Universe)?
    pub is_tamed: bool,
    pub discovered_at: DateTime<Utc>,
}

/// The Dumpster Universe Bestiary attached to a project/profile.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bestiary {
    pub monsters: HashMap<String, ScopeCreepMonster>,
    pub total_tamed: u32,
    pub total_banished: u32,
}

impl Bestiary {
    pub fn add_monster(&mut self, monster: ScopeCreepMonster) {
        self.monsters.insert(monster.id.clone(), monster);
    }

    pub fn tame_monster(&mut self, id: &str) -> bool {
        if let Some(monster) = self.monsters.get_mut(id) {
            monster.is_tamed = true;
            self.total_tamed += 1;
            true
        } else {
            false
        }
    }
}

// -----------------------------------------------------------------------------
// PROJECT PROFILE
// -----------------------------------------------------------------------------

/// A user's global profile across all projects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// Unique user identifier (stable across all projects).
    pub user_id: Uuid,
    /// Display name shown in UI.
    pub display_name: String,
    /// All projects this user has created.
    pub projects: Vec<ProjectProfile>,
    /// Currently active project (if any).
    pub active_project_id: Option<Uuid>,
    /// When this profile was created.
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp.
    pub last_active: DateTime<Utc>,
}

impl UserProfile {
    /// Create a new user profile.
    pub fn new(display_name: impl Into<String>) -> Self {
        Self {
            user_id: Uuid::new_v4(),
            display_name: display_name.into(),
            projects: Vec::new(),
            active_project_id: None,
            created_at: Utc::now(),
            last_active: Utc::now(),
        }
    }

    /// Create a new project with the given parameters.
    pub fn create_project(
        &mut self,
        name: impl Into<String>,
        workspace_path: PathBuf,
        genre: Genre,
    ) -> &ProjectProfile {
        let project = ProjectProfile::new(name, workspace_path, genre);
        let id = project.project_id;
        self.projects.push(project);
        self.active_project_id = Some(id);
        self.projects.last().unwrap()
    }

    /// Get the active project (if any).
    pub fn active_project(&self) -> Option<&ProjectProfile> {
        self.active_project_id
            .and_then(|id| self.projects.iter().find(|p| p.project_id == id))
    }

    /// Get mutable active project.
    pub fn active_project_mut(&mut self) -> Option<&mut ProjectProfile> {
        self.active_project_id
            .and_then(|id| self.projects.iter_mut().find(|p| p.project_id == id))
    }

    /// Switch to a different project.
    pub fn switch_project(&mut self, project_id: Uuid) -> bool {
        if self.projects.iter().any(|p| p.project_id == project_id) {
            self.active_project_id = Some(project_id);
            self.last_active = Utc::now();
            true
        } else {
            false
        }
    }

    /// Save profile to disk.
    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load profile from disk.
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let profile: UserProfile = serde_json::from_str(&content)?;
        Ok(profile)
    }

    /// Default profile path.
    pub fn default_path() -> PathBuf {
        // Use home directory or fallback to current dir
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("trinity")
            .join("user_profile.json")
    }
}

impl Default for UserProfile {
    fn default() -> Self {
        Self::new("Conductor")
    }
}

/// A single project's profile - the "player" for that project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectProfile {
    /// Unique project identifier.
    pub project_id: Uuid,
    /// Human-readable project name.
    pub project_name: String,
    /// Path to the project workspace root.
    pub workspace_path: PathBuf,

    // === THE PLAYER ===
    /// Character sheet for this project (level, skills, etc.).
    pub character_sheet: CharacterSheet,
    /// Fixed genre for this project (determines vocabulary + narrative).
    pub genre: Genre,

    // === VAAM STATE ===
    /// Vocabulary mastery for this project.
    pub vocabulary_mastery: VocabularyMastery,
    /// Words detected in current session.
    pub session_detections: Vec<WordDetection>,
    /// Coal earned this session.
    pub session_coal_earned: u32,
    /// Steam generated this session.
    pub session_steam_generated: u32,

    // === PROGRESS ===
    /// Quests available or completed in this project.
    pub quest_board: QuestBoardState,
    /// Narrative journal documenting progress.
    pub journal: Journal,
    /// LitRPG Dumpster Universe bestiary for scope creep.
    pub bestiary: Bestiary,

    // === TIMESTAMPS ===
    /// When this project was created.
    pub created_at: DateTime<Utc>,
    /// Last time this project was active.
    pub last_active: DateTime<Utc>,
}

impl ProjectProfile {
    /// Create a new project profile.
    pub fn new(name: impl Into<String>, workspace_path: PathBuf, genre: Genre) -> Self {
        Self {
            project_id: Uuid::new_v4(),
            project_name: name.into(),
            workspace_path,
            character_sheet: CharacterSheet::default(),
            genre,
            vocabulary_mastery: VocabularyMastery::default(),
            session_detections: Vec::new(),
            session_coal_earned: 0,
            session_steam_generated: 0,
            quest_board: QuestBoardState::default(),
            journal: Journal::default(),
            bestiary: Bestiary::default(),
            created_at: Utc::now(),
            last_active: Utc::now(),
        }
    }

    /// Record a vocabulary detection and award coal.
    pub fn record_vocabulary(&mut self, detection: WordDetection) -> VocabularyUpdate {
        let coal = detection.coal_earned;
        self.session_coal_earned += coal;
        self.character_sheet.current_coal =
            (self.character_sheet.current_coal + coal as f32).min(100.0);
        self.session_detections.push(detection.clone());

        let mastery_update = self.vocabulary_mastery.record_detection(&detection);

        // Add journal entry for vocabulary discovery
        if mastery_update.times_used == 1 {
            self.journal.add_entry(
                JournalEntryType::VocabularyDiscovery,
                format!("Discovered word: '{}' (+{} coal)", detection.word, coal),
            );
        }

        // Add journal entry for mastery
        if mastery_update.newly_mastered {
            self.journal.add_entry(
                JournalEntryType::VocabularyMastery,
                format!("Mastered word: '{}' - now automatic!", detection.word),
            );
        }

        VocabularyUpdate {
            detection,
            mastery: mastery_update,
            total_coal: self.character_sheet.current_coal,
        }
    }

    /// Burn coal to generate steam (when delegating to AI).
    pub fn burn_coal_for_steam(&mut self, coal_amount: f32) -> SteamResult {
        let burned = self.character_sheet.consume_coal(coal_amount);
        let steam = burned * 2.0; // 1 coal = 2 steam

        self.session_steam_generated += steam as u32;

        SteamResult {
            coal_burned: burned,
            steam_generated: steam,
            remaining_coal: self.character_sheet.current_coal,
        }
    }

    /// Get vocabulary mastery progress by tier.
    pub fn vocabulary_progress(&self) -> Vec<TierProgress> {
        // This would need the VocabularyDatabase to calculate properly
        // For now, return empty - will be implemented with full integration
        Vec::new()
    }

    /// Get the planning/doing gap (coal - steam).
    pub fn planning_doing_gap(&self) -> f32 {
        self.character_sheet.current_coal - self.session_steam_generated as f32
    }
}

/// Result of recording a vocabulary detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyUpdate {
    pub detection: WordDetection,
    pub mastery: crate::vocabulary::MasteryUpdate,
    pub total_coal: f32,
}

/// Result of burning coal for steam.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamResult {
    pub coal_burned: f32,
    pub steam_generated: f32,
    pub remaining_coal: f32,
}

/// Quest board state for a project.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuestBoardState {
    /// Quests available to start.
    pub available: Vec<QuestSummary>,
    /// Quests currently in progress.
    pub active: Vec<QuestSummary>,
    /// Completed quests.
    pub completed: Vec<QuestSummary>,
    /// Failed quests.
    pub failed: Vec<QuestSummary>,
}

/// Summary of a quest for the board.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestSummary {
    pub quest_id: String,
    pub title: String,
    pub description: String,
    pub coal_cost: u32,      // Coal required to start
    pub steam_progress: u32, // Steam generated so far (for active)
    pub steam_required: u32, // Total steam needed
    pub status: QuestStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Quest lifecycle status.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QuestStatus {
    #[default]
    Available,
    Active,
    Completed,
    Failed,
}

/// Journal system - the "why" train of thought.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Journal {
    /// All journal entries.
    pub entries: Vec<JournalEntry>,
    /// Current ADDIE phase.
    pub current_phase: AddiePhase,
    /// Total entries count.
    pub total_entries: u64,
}

impl Journal {
    /// Add a new journal entry.
    pub fn add_entry(&mut self, entry_type: JournalEntryType, content: String) -> &JournalEntry {
        let entry = JournalEntry {
            timestamp: Utc::now(),
            entry_type,
            content,
            linked_quest: None,
            linked_files: Vec::new(),
            addie_phase: self.current_phase,
        };

        self.entries.push(entry);
        self.total_entries += 1;
        self.entries.last().unwrap()
    }

    /// Add an entry linked to a quest.
    pub fn add_quest_entry(
        &mut self,
        entry_type: JournalEntryType,
        content: String,
        quest_id: &str,
    ) -> &JournalEntry {
        let entry = JournalEntry {
            timestamp: Utc::now(),
            entry_type,
            content,
            linked_quest: Some(quest_id.to_string()),
            linked_files: Vec::new(),
            addie_phase: self.current_phase,
        };

        self.entries.push(entry);
        self.total_entries += 1;
        self.entries.last().unwrap()
    }

    /// Get entries for a specific ADDIE phase.
    pub fn entries_for_phase(&self, phase: AddiePhase) -> Vec<&JournalEntry> {
        self.entries
            .iter()
            .filter(|e| e.addie_phase == phase)
            .collect()
    }

    /// Get entries linked to a specific quest.
    pub fn entries_for_quest(&self, quest_id: &str) -> Vec<&JournalEntry> {
        self.entries
            .iter()
            .filter(|e| e.linked_quest.as_deref() == Some(quest_id))
            .collect()
    }

    /// Advance to the next ADDIE phase.
    pub fn advance_phase(&mut self) {
        self.current_phase = match self.current_phase {
            AddiePhase::Analysis => AddiePhase::Design,
            AddiePhase::Design => AddiePhase::Development,
            AddiePhase::Development => AddiePhase::Implementation,
            AddiePhase::Implementation => AddiePhase::Evaluation,
            AddiePhase::Evaluation => AddiePhase::Analysis, // Cycle back
        };
    }
}

/// A single journal entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub timestamp: DateTime<Utc>,
    pub entry_type: JournalEntryType,
    pub content: String,
    pub linked_quest: Option<String>,
    pub linked_files: Vec<String>,
    pub addie_phase: AddiePhase,
}

/// Types of journal entries.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum JournalEntryType {
    // ADDIE phases
    AnalysisNote,
    DesignDecision,
    Implementation,
    Evaluation,

    // VAAM moments
    VocabularyDiscovery,
    VocabularyMastery,

    // Quest moments
    QuestStarted,
    QuestCompleted,
    QuestFailed,

    // Reflection
    Insight,
    Frustration,
    Breakthrough,

    // System / Scope Creep
    SystemNote,
    ScopeCreepDiscovered,
    ScopeCreepTamed,
}

/// ADDIE instructional design phases.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AddiePhase {
    #[default]
    Analysis,
    Design,
    Development,
    Implementation,
    Evaluation,
}

impl AddiePhase {
    pub fn display_name(&self) -> &'static str {
        match self {
            AddiePhase::Analysis => "Analysis",
            AddiePhase::Design => "Design",
            AddiePhase::Development => "Development",
            AddiePhase::Implementation => "Implementation",
            AddiePhase::Evaluation => "Evaluation",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            AddiePhase::Analysis => "Understanding the problem and requirements",
            AddiePhase::Design => "Planning the solution architecture",
            AddiePhase::Development => "Building the solution",
            AddiePhase::Implementation => "Deploying and integrating",
            AddiePhase::Evaluation => "Assessing outcomes and learning",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_creation() {
        let mut user = UserProfile::new("Test User");
        assert_eq!(user.display_name, "Test User");
        assert!(user.projects.is_empty());

        user.create_project("Test Project", PathBuf::from("/tmp/test"), Genre::Cyberpunk);
        assert_eq!(user.projects.len(), 1);
        assert!(user.active_project().is_some());
    }

    #[test]
    fn test_project_switching() {
        let mut user = UserProfile::new("Test User");
        let p1 = user.create_project("Project 1", PathBuf::from("/tmp/p1"), Genre::Cyberpunk);
        let p1_id = p1.project_id;
        let p2 = user.create_project("Project 2", PathBuf::from("/tmp/p2"), Genre::Steampunk);
        let p2_id = p2.project_id;

        assert_eq!(user.active_project_id, Some(p2_id));

        user.switch_project(p1_id);
        assert_eq!(user.active_project_id, Some(p1_id));
    }

    #[test]
    fn test_journal_addie_phases() {
        let mut journal = Journal::default();
        assert_eq!(journal.current_phase, AddiePhase::Analysis);

        journal.add_entry(JournalEntryType::AnalysisNote, "Test analysis".to_string());
        assert_eq!(journal.entries.len(), 1);

        journal.advance_phase();
        assert_eq!(journal.current_phase, AddiePhase::Design);
    }
}
