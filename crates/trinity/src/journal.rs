// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        journal.rs
// PURPOSE:     Journal States — chapter milestone snapshots + weekly reflections
//
// ARCHITECTURE:
//   Journal States capture the complete learning state at a point in time:
//     - Quest progress (phase, objectives, completed phases)
//     - Character sheet (skills, VAAM profile, resonance)
//     - Quality scores (if documents are uploaded)
//     - User reflection text (optional)
//     - Timestamp + auto-generated summary
//
//   Use cases:
//     1. Chapter milestones: auto-captured on phase/chapter completion
//     2. Weekly reflections: user-triggered with optional reflection text
//     3. Portfolio export: shareable JSON/HTML artifact
//     4. Demo mode: automated sequence for presentations
//
//   Storage:
//     - JSON files in ~/.local/share/trinity/journal/
//     - Each entry is a self-contained snapshot
//     - Can be loaded, compared, and exported
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// A Journal State — complete snapshot of learning progress at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    /// Unique ID (timestamp-based)
    pub id: String,
    /// When this snapshot was taken
    pub timestamp: String,
    /// Type of entry
    pub entry_type: JournalEntryType,
    /// User-written reflection (optional)
    pub reflection: String,
    /// Auto-generated summary
    pub summary: String,
    /// Quest state snapshot
    pub quest: QuestSnapshot,
    /// Character sheet snapshot
    pub character: CharacterSnapshot,
    /// Tags for filtering
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JournalEntryType {
    /// Auto-captured on phase completion
    PhaseComplete,
    /// Auto-captured on chapter completion  
    ChapterComplete,
    /// User-triggered weekly reflection
    WeeklyReflection,
    /// Manual checkpoint
    ManualCheckpoint,
    /// Demo/presentation bookmark
    DemoBookmark,
}

impl JournalEntryType {
    pub fn label(&self) -> &str {
        match self {
            Self::PhaseComplete => "Phase Complete",
            Self::ChapterComplete => "Chapter Complete",
            Self::WeeklyReflection => "Weekly Reflection",
            Self::ManualCheckpoint => "Checkpoint",
            Self::DemoBookmark => "Demo Bookmark",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::PhaseComplete => "🚉",
            Self::ChapterComplete => "🏆",
            Self::WeeklyReflection => "📓",
            Self::ManualCheckpoint => "📌",
            Self::DemoBookmark => "🎬",
        }
    }
}

/// Snapshot of quest state at journal time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestSnapshot {
    pub subject: String,
    pub phase: String,
    pub phase_index: u8,
    pub chapter: u8,
    pub chapter_title: String,
    pub completed_phases: Vec<String>,
    pub objectives_total: usize,
    pub objectives_completed: usize,
    pub xp: u32,
    pub coal_remaining: f32,
    pub steam: f32,
}

/// Snapshot of character state at journal time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSnapshot {
    pub resonance: u32,
    pub skills: Vec<(String, f32)>,
    pub experience: Option<String>,
    pub audience: Option<String>,
    pub vision: Option<String>,
}

// ============================================================================
// JOURNAL STORAGE
// ============================================================================

fn journal_dir() -> PathBuf {
    let dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("trinity")
        .join("journal");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Save a journal entry to disk
pub fn save_entry(entry: &JournalEntry) -> Result<PathBuf, String> {
    let dir = journal_dir();
    let filename = format!("{}.json", entry.id);
    let path = dir.join(&filename);

    let json = serde_json::to_string_pretty(entry)
        .map_err(|e| format!("Failed to serialize journal entry: {}", e))?;

    std::fs::write(&path, json).map_err(|e| format!("Failed to write journal entry: {}", e))?;

    info!(
        "[Journal] Saved entry: {} ({})",
        entry.id,
        entry.entry_type.label()
    );
    Ok(path)
}

/// Load all journal entries, sorted by timestamp (newest first)
pub fn load_entries() -> Vec<JournalEntry> {
    let dir = journal_dir();
    let mut entries = Vec::new();

    if let Ok(read_dir) = std::fs::read_dir(&dir) {
        for entry in read_dir.flatten() {
            if entry.path().extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Ok(journal_entry) = serde_json::from_str::<JournalEntry>(&content) {
                        entries.push(journal_entry);
                    }
                }
            }
        }
    }

    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    entries
}

/// Load a specific journal entry by ID
pub fn load_entry(id: &str) -> Option<JournalEntry> {
    let path = journal_dir().join(format!("{}.json", id));
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
}

/// Delete a journal entry by ID
pub fn delete_entry(id: &str) -> bool {
    let path = journal_dir().join(format!("{}.json", id));
    std::fs::remove_file(&path).is_ok()
}

// ============================================================================
// ENTRY CREATION HELPERS
// ============================================================================

/// Generate a unique ID for a journal entry
fn generate_id() -> String {
    let now = chrono::Utc::now();
    now.format("%Y%m%d-%H%M%S-%3f").to_string()
}

/// Generate an auto-summary from quest + character state
fn auto_summary(
    entry_type: &JournalEntryType,
    quest: &QuestSnapshot,
    character: &CharacterSnapshot,
) -> String {
    let progress = if quest.objectives_total > 0 {
        format!(
            "{}/{} objectives",
            quest.objectives_completed, quest.objectives_total
        )
    } else {
        "No objectives set".to_string()
    };

    let phase_progress = format!("{} completed phases", quest.completed_phases.len());

    match entry_type {
        JournalEntryType::PhaseComplete => {
            format!(
                "Completed {} phase in {} ({}). {} at Resonance {}. {}.",
                quest.phase,
                quest.subject,
                quest.chapter_title,
                progress,
                character.resonance,
                phase_progress
            )
        }
        JournalEntryType::ChapterComplete => {
            format!(
                "Completed chapter {} — {} in {}. Resonance {} with {}.",
                quest.chapter,
                quest.chapter_title,
                quest.subject,
                character.resonance,
                phase_progress
            )
        }
        JournalEntryType::WeeklyReflection => {
            format!(
                "Weekly reflection: {} phase, {} chapter in {}. {}. Resonance {}.",
                quest.phase, quest.chapter_title, quest.subject, progress, character.resonance
            )
        }
        JournalEntryType::ManualCheckpoint => {
            format!(
                "Checkpoint: {} → {} ({}).",
                quest.phase, quest.chapter_title, progress
            )
        }
        JournalEntryType::DemoBookmark => {
            format!(
                "Demo bookmark: {} phase, {} chapter. {} at Resonance {}.",
                quest.phase, quest.chapter_title, progress, character.resonance
            )
        }
    }
}

/// Create a journal entry from current game state
pub fn create_entry(
    entry_type: JournalEntryType,
    reflection: Option<String>,
    quest: QuestSnapshot,
    character: CharacterSnapshot,
    tags: Vec<String>,
) -> JournalEntry {
    let id = generate_id();
    let timestamp = chrono::Utc::now().to_rfc3339();
    let summary = auto_summary(&entry_type, &quest, &character);

    JournalEntry {
        id,
        timestamp,
        entry_type,
        reflection: reflection.unwrap_or_default(),
        summary,
        quest,
        character,
        tags,
    }
}

/// Export a journal entry as a standalone HTML page (for sharing/portfolio)
pub fn export_html(entry: &JournalEntry) -> String {
    let skills_html: String = entry
        .character
        .skills
        .iter()
        .map(|(name, level)| format!("<li><strong>{}</strong>: {:.1}</li>", name, level))
        .collect::<Vec<_>>()
        .join("\n");

    let phases_html: String = entry
        .quest
        .completed_phases
        .iter()
        .map(|p| format!("<span class='phase-badge'>{}</span>", p))
        .collect::<Vec<_>>()
        .join(" ");

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>Trinity Journal — {subject} — {phase}</title>
<style>
  body {{ font-family: 'Georgia', serif; max-width: 800px; margin: 2rem auto; padding: 0 1rem; background: #1a1410; color: #cfb991; }}
  h1 {{ font-family: 'Cinzel', serif; color: #e8d5a3; border-bottom: 2px solid rgba(207,185,145,0.2); padding-bottom: 0.5rem; }}
  h2 {{ color: #4ec9b0; font-size: 1.1rem; }}
  .meta {{ color: rgba(207,185,145,0.6); font-size: 0.85rem; }}
  .reflection {{ background: rgba(207,185,145,0.08); padding: 1rem; border-left: 3px solid #cfb991; margin: 1rem 0; border-radius: 4px; }}
  .phase-badge {{ display: inline-block; background: rgba(78,201,176,0.15); border: 1px solid rgba(78,201,176,0.3); border-radius: 12px; padding: 0.2rem 0.6rem; font-size: 0.75rem; margin: 0.1rem; }}
  .stats {{ display: grid; grid-template-columns: repeat(3, 1fr); gap: 0.5rem; margin: 1rem 0; }}
  .stat {{ background: rgba(207,185,145,0.05); padding: 0.5rem; border-radius: 4px; text-align: center; }}
  .stat-value {{ font-size: 1.5rem; font-weight: bold; color: #e8d5a3; }}
  .stat-label {{ font-size: 0.7rem; color: rgba(207,185,145,0.5); text-transform: uppercase; letter-spacing: 0.1em; }}
  footer {{ margin-top: 2rem; padding-top: 1rem; border-top: 1px solid rgba(207,185,145,0.1); font-size: 0.75rem; color: rgba(207,185,145,0.4); }}
</style>
</head>
<body>
<h1>{icon} {entry_type} — {subject}</h1>
<p class="meta">{timestamp} · {chapter_title} · {phase} phase</p>

<div class="stats">
  <div class="stat"><div class="stat-value">{resonance}</div><div class="stat-label">Resonance</div></div>
  <div class="stat"><div class="stat-value">{xp}</div><div class="stat-label">XP Earned</div></div>
  <div class="stat"><div class="stat-value">{coal:.0}%</div><div class="stat-label">Coal</div></div>
</div>

<h2>Summary</h2>
<p>{summary}</p>

{reflection_section}

<h2>Completed Phases</h2>
<p>{phases}</p>

<h2>Skills</h2>
<ul>{skills}</ul>

<footer>Generated by Trinity ID AI OS · The Iron Road</footer>
</body>
</html>"#,
        subject = entry.quest.subject,
        phase = entry.quest.phase,
        icon = entry.entry_type.icon(),
        entry_type = entry.entry_type.label(),
        timestamp = entry.timestamp,
        chapter_title = entry.quest.chapter_title,
        resonance = entry.character.resonance,
        xp = entry.quest.xp,
        coal = entry.quest.coal_remaining,
        summary = entry.summary,
        reflection_section = if entry.reflection.is_empty() {
            String::new()
        } else {
            format!(
                "<h2>Reflection</h2>\n<div class='reflection'>{}</div>",
                entry.reflection
            )
        },
        phases = phases_html,
        skills = skills_html,
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_quest() -> QuestSnapshot {
        QuestSnapshot {
            subject: "Biology".to_string(),
            phase: "Analysis".to_string(),
            phase_index: 0,
            chapter: 1,
            chapter_title: "The Ordinary World".to_string(),
            completed_phases: vec!["Analysis".to_string()],
            objectives_total: 3,
            objectives_completed: 3,
            xp: 30,
            coal_remaining: 85.0,
            steam: 15.0,
        }
    }

    fn sample_character() -> CharacterSnapshot {
        CharacterSnapshot {
            resonance: 2,
            skills: vec![("CurriculumDesign".to_string(), 5.0)],
            experience: Some("5 years K-12".to_string()),
            audience: Some("10th grade".to_string()),
            vision: Some("Students understand ecosystems".to_string()),
        }
    }

    #[test]
    fn test_create_phase_entry() {
        let entry = create_entry(
            JournalEntryType::PhaseComplete,
            None,
            sample_quest(),
            sample_character(),
            vec!["chapter-1".to_string()],
        );
        assert!(!entry.id.is_empty());
        assert!(entry.summary.contains("Biology"));
        assert!(entry.summary.contains("Analysis"));
        assert_eq!(entry.tags, vec!["chapter-1"]);
    }

    #[test]
    fn test_create_weekly_reflection() {
        let entry = create_entry(
            JournalEntryType::WeeklyReflection,
            Some("This week I learned about ecosystems and food webs.".to_string()),
            sample_quest(),
            sample_character(),
            vec![],
        );
        assert!(!entry.reflection.is_empty());
        assert!(entry.summary.contains("Weekly reflection"));
    }

    #[test]
    fn test_save_and_load() {
        let entry = create_entry(
            JournalEntryType::ManualCheckpoint,
            None,
            sample_quest(),
            sample_character(),
            vec![],
        );
        let id = entry.id.clone();

        // Save
        let path = save_entry(&entry).expect("Should save");
        assert!(path.exists());

        // Load
        let loaded = load_entry(&id).expect("Should load");
        assert_eq!(loaded.id, id);
        assert_eq!(loaded.quest.subject, "Biology");

        // Cleanup
        delete_entry(&id);
    }

    #[test]
    fn test_export_html() {
        let entry = create_entry(
            JournalEntryType::ChapterComplete,
            Some("Great progress this chapter!".to_string()),
            sample_quest(),
            sample_character(),
            vec![],
        );
        let html = export_html(&entry);
        assert!(html.contains("Biology"));
        assert!(html.contains("Resonance"));
        assert!(html.contains("Great progress this chapter!"));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn test_entry_type_labels() {
        assert_eq!(JournalEntryType::PhaseComplete.label(), "Phase Complete");
        assert_eq!(JournalEntryType::ChapterComplete.icon(), "🏆");
        assert_eq!(JournalEntryType::WeeklyReflection.icon(), "📓");
        assert_eq!(JournalEntryType::DemoBookmark.icon(), "🎬");
    }
}
