// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        eye_container.rs
// PURPOSE:     EYE Container — bundle quest data into exportable artifact
//
// 🪟 THE LIVING CODE TEXTBOOK (EYE Framework: The Observer):
// This file is the central artifact packager. It is designed to be read, 
// modified, and authored by YOU. It takes everything you built in the quest
// and exports it. If you want a new export format (like PDF), add it here!
// ACTION: Edit `ExportFormat` and `compile_container()` to build a new export target.
//
// 📖 THE HOOK BOOK CONNECTION:
// This file powers the 'Export' Hook. It is the bridge between the Trinity 
// engine and the real world, turning your learning into a tangible product.
// For a full catalogue of system capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
//
// ARCHITECTURE:
//   • Compiles quest state + PEARL + vocabulary + assets into a single container
//   • Used by export.rs to generate HTML5 quizzes, adventures, PDFs
//   • The container IS the product — the thing the Gym Coach takes home
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use trinity_quest::state::GameState;

/// EYE Container — the exportable artifact from a completed quest journey
///
/// WHY: This is what makes Trinity useful. Without export, all the quest
/// work stays locked inside the database. The container bundles everything
/// the user created into something they can hand to their students.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EyeContainer {
    pub metadata: ContainerMeta,
    pub pearl_summary: String,
    pub vocabulary: Vec<VocabEntry>,
    pub objectives: Vec<CompletedObjective>,
    pub quest_summary: QuestSummary,
    pub assets: Vec<AssetRef>,
}

/// Container metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerMeta {
    pub title: String,
    pub subject: String,
    pub author: String,
    pub grade_level: String,
    pub created_at: String,
    pub pearl_phase: String,
    pub alignment_grade: String,
}

/// A vocabulary entry for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabEntry {
    pub word: String,
    pub definition: String,
}

/// An objective with completion status and phase context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedObjective {
    pub phase: String,
    pub description: String,
    pub completed: bool,
}

/// Summary of the quest journey for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestSummary {
    pub total_xp: u32,
    pub phases_completed: usize,
    pub total_phases: usize,
    pub coal_used: f32,
    pub steam_generated: f32,
}

/// Reference to a generated asset (image, audio, mesh)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRef {
    pub filename: String,
    pub asset_type: String,
    pub url: String,
}

/// Export format selection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ExportFormat {
    #[default]
    Html5Quiz,
    Html5Adventure,
    RawJson,
    /// Professional DOCX portfolio — the student deliverable
    DocxPortfolio,
}

/// Compile an EYE Container from the current game state
///
/// This is the "Great Recycler" at work — it takes the raw quest state
/// and transforms it into a structured artifact ready for export.
pub fn compile_container(game_state: &GameState) -> EyeContainer {
    let quest = &game_state.quest;

    // Build metadata
    let pearl_summary = quest
        .pearl
        .as_ref()
        .map(|p| p.prompt_summary())
        .unwrap_or_else(|| "No PEARL set".to_string());

    let alignment_grade = quest
        .pearl
        .as_ref()
        .map(|p| p.evaluation.grade().to_string())
        .unwrap_or_else(|| "N/A".to_string());

    let pearl_phase = quest
        .pearl
        .as_ref()
        .map(|p| p.phase.display_name().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let metadata = ContainerMeta {
        title: quest.game_title.clone(),
        subject: quest.subject.clone(),
        author: "Yardmaster".to_string(),
        grade_level: String::new(),
        created_at: chrono::Utc::now().to_rfc3339(),
        pearl_phase,
        alignment_grade,
    };

    // Collect objectives from current phase
    let objectives: Vec<CompletedObjective> = quest
        .phase_objectives
        .iter()
        .map(|o| CompletedObjective {
            phase: format!("{:?}", quest.current_phase),
            description: o.description.clone(),
            completed: o.completed,
        })
        .collect();

    // Vocabulary from bestiary/VAAM (placeholder — will integrate with VAAM bridge)
    let vocabulary = vec![];

    let quest_summary = QuestSummary {
        total_xp: game_state.stats.total_xp,
        phases_completed: quest.completed_phases.len(),
        total_phases: 12,
        coal_used: quest.coal_used,
        steam_generated: quest.steam_generated,
    };

    // Scan for generated assets
    let assets = scan_assets();

    EyeContainer {
        metadata,
        pearl_summary,
        vocabulary,
        objectives,
        quest_summary,
        assets,
    }
}

/// Scan the workspace asset directory for generated images/audio
fn scan_assets() -> Vec<AssetRef> {
    let home = std::env::var("HOME").unwrap_or_else(|_| dirs::home_dir().unwrap_or_default().to_string_lossy().to_string());
    let asset_dir =
        std::path::PathBuf::from(&home).join(".local/share/trinity/workspace/assets/images");

    let mut assets = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&asset_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                if filename.starts_with("trinity") {
                    let ext = path
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("unknown");
                    assets.push(AssetRef {
                        filename: filename.to_string(),
                        asset_type: ext.to_string(),
                        url: format!("/api/creative/assets/{}", filename),
                    });
                }
            }
        }
    }
    assets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_container_default() {
        let gs = GameState::default();
        let container = compile_container(&gs);
        assert_eq!(container.quest_summary.total_phases, 12);
        assert_eq!(container.quest_summary.phases_completed, 0);
        assert!(!container.objectives.is_empty());
    }

    #[test]
    fn test_compile_container_with_subject() {
        let mut gs = GameState::default();
        gs.quest = trinity_quest::state::QuestState::new("Photosynthesis");
        gs.stats.total_xp = 250;
        let container = compile_container(&gs);
        assert_eq!(container.metadata.subject, "Photosynthesis");
        assert_eq!(container.quest_summary.total_xp, 250);
        assert!(container.pearl_summary.contains("Photosynthesis"));
    }

    #[test]
    fn test_export_format_serde() {
        let fmt = ExportFormat::Html5Quiz;
        let json = serde_json::to_string(&fmt).unwrap();
        assert_eq!(json, "\"html5_quiz\"");
        let restored: ExportFormat = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, ExportFormat::Html5Quiz);
    }
}
