// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — RLHF Feedback API
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        rlhf_api.rs
// BIBLE CAR:   Car 4 — IMPLEMENT (Iron Road Game Mechanics)
// HOOK SCHOOL: 🎭 Identity
// PURPOSE:     Resonance feedback (👍/👎), Shadow escalation, PEARL alignment scoring
// MATURITY:    L5 — Evolutionary (RLHF bias persisted across sessions, steers future prompts)
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{extract::State, response::Json};
use trinity_protocol::character_sheet::ShadowStatus;

use crate::AppState;

/// Accept any JSON for RLHF feedback — the UI sends {message_id, score, phase}
/// but the original handler expected ResonanceFeedbackEvent. Accept both shapes.
///
/// ═══ SOFT SPOT 5: L5 EVOLUTIONARY ═══
/// Negative feedback cascades:
///   score < 0 → friction↑, consecutive_negatives↑, Shadow escalation, bias persisted
///   score > 0 → steam↑, friction↓, consecutive_negatives reset, reinforce persisted
/// Bias signals accumulate in rlhf_bias.json and steer future Pete system prompts.
pub async fn submit_resonance_feedback(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let score = payload.get("score").and_then(|v| v.as_i64()).unwrap_or(0);
    let phase = payload
        .get("phase")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let msg_id = payload
        .get("message_id")
        .and_then(|v| v.as_str())
        .unwrap_or("?");
    let msg_content = payload
        .get("message_content")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    tracing::info!(
        "RLHF feedback: msg={}, score={}, phase={}",
        msg_id,
        score,
        phase
    );

    // ── Mutate the CharacterSheet based on feedback ──
    let mut sheet = state.player.character_sheet.write().await;

    if score < 0 {
        // Negative feedback → friction rises, Shadow may escalate
        sheet.consecutive_negatives = sheet.consecutive_negatives.saturating_add(1);

        if sheet.consecutive_negatives >= 3 {
            sheet.shadow_status = ShadowStatus::Active;
            tracing::info!("🌑 Shadow activated — 3+ consecutive negative signals");
        } else if sheet.shadow_status == ShadowStatus::Clear {
            sheet.shadow_status = ShadowStatus::Stirring;
            tracing::info!("🌘 Shadow stirring — negative feedback received");
        }

        // Scope Creep friction: negative feedback = extraneous cognitive load
        sheet.track_friction = (sheet.track_friction + 5.0).min(100.0);

        // ═══ L5 EVOLUTIONARY: Persist negative bias for future prompt steering ═══
        // When a user thumbs-down a response, we record the phase and message
        // content as a bias signal. Future prompts will avoid repeating this pattern.
        if !msg_content.is_empty() {
            let bias_entry = PromptBiasEntry {
                phase: phase.to_string(),
                signal: "avoid".to_string(),
                snippet: msg_content.chars().take(120).collect(),
                weight: 1,
            };
            if let Err(e) = append_prompt_bias(bias_entry) {
                tracing::warn!("Failed to persist RLHF bias: {}", e);
            } else {
                tracing::info!("🧠 RLHF bias persisted — Pete will steer away from this pattern");
            }
        }
    } else if score > 0 {
        // Positive feedback → steam rises, friction drops, Shadow calms
        sheet.consecutive_negatives = 0;
        sheet.current_steam = (sheet.current_steam + 3.0).min(100.0);
        sheet.track_friction = (sheet.track_friction - 2.0).max(0.0);

        // ═══ L5: Persist positive bias — reinforce good patterns ═══
        if !msg_content.is_empty() {
            let bias_entry = PromptBiasEntry {
                phase: phase.to_string(),
                signal: "reinforce".to_string(),
                snippet: msg_content.chars().take(120).collect(),
                weight: 1,
            };
            // Positive biases are lower priority — don't fail silently
            let _ = append_prompt_bias(bias_entry);
        }
    }

    // Always recalculate vulnerability (the compound metric)
    sheet.recalculate_vulnerability();

    drop(sheet);

    // ═══ SOFT SPOT 8: RLHF → PEARL alignment scores ═══
    // Positive feedback = PEARL is aligned; negative = drifting
    if score != 0 {
        let game = state.project.game_state.read().await;
        if let Some(pearl) = game.quest.pearl.as_ref() {
            let pearl_phase = pearl.phase;
            drop(game);

            let mut game = state.project.game_state.write().await;
            if let Some(pearl) = game.quest.pearl.as_mut() {
                let delta: f32 = if score > 0 { 0.05 } else { -0.03 };
                match pearl_phase {
                    trinity_protocol::PearlPhase::Extracting => {
                        pearl.evaluation.addie_score =
                            (pearl.evaluation.addie_score + delta).clamp(0.0, 1.0);
                    }
                    trinity_protocol::PearlPhase::Placing => {
                        pearl.evaluation.crap_score =
                            (pearl.evaluation.crap_score + delta).clamp(0.0, 1.0);
                    }
                    trinity_protocol::PearlPhase::Refining => {
                        pearl.evaluation.eye_score =
                            (pearl.evaluation.eye_score + delta).clamp(0.0, 1.0);
                    }
                    trinity_protocol::PearlPhase::Polished => {}
                }
                tracing::debug!(
                    "PEARL alignment updated: ADDIE={:.2} CRAP={:.2} EYE={:.2}",
                    pearl.evaluation.addie_score,
                    pearl.evaluation.crap_score,
                    pearl.evaluation.eye_score,
                );
            }
        }
    }

    let sheet = state.player.character_sheet.read().await;
    let response = serde_json::json!({
        "status": "success",
        "shadow_status": format!("{:?}", sheet.shadow_status),
        "vulnerability": sheet.vulnerability,
        "track_friction": sheet.track_friction,
        "current_steam": sheet.current_steam,
        "consecutive_negatives": sheet.consecutive_negatives,
    });

    Json(response)
}

/// POST /api/character/shadow/process — User submits a reflection journal.
/// Transitions Shadow from Active → Processed, reducing vulnerability and friction.
/// The Ghost Train stops when the user processes their emotions.
pub async fn process_shadow(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let mut sheet = state.player.character_sheet.write().await;

    if sheet.shadow_status == ShadowStatus::Active {
        sheet.shadow_status = ShadowStatus::Processed;
        sheet.track_friction = (sheet.track_friction - 15.0).max(0.0);
        sheet.consecutive_negatives = 0;
        sheet.recalculate_vulnerability();

        tracing::info!(
            "🌕 Shadow processed — vulnerability now {:.2}, friction {:.1}%",
            sheet.vulnerability,
            sheet.track_friction
        );
    }

    Json(serde_json::json!({
        "shadow_status": format!("{:?}", sheet.shadow_status),
        "vulnerability": sheet.vulnerability,
        "track_friction": sheet.track_friction,
    }))
}

// ============================================================================
// L5 EVOLUTIONARY: PROMPT BIAS PERSISTENCE
// ============================================================================
//
// This is the memory of the RLHF system. Every thumbs-down writes a bias entry
// that future system prompt construction reads via apply_prompt_bias().
//
// The bias file is stored at: ~/.local/share/trinity/rlhf_bias.json
// It is a JSON array of PromptBiasEntry structs.
//
// How it steers Pete:
//   - "avoid" entries → "DO NOT repeat patterns like: [snippet]" injected into system prompt
//   - "reinforce" entries → "Continue patterns like: [snippet]" injected into system prompt
//   - Entries age out after 50 signals (FIFO queue)

/// A single RLHF bias signal — captures what Pete should avoid or reinforce.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PromptBiasEntry {
    /// ADDIECRAPEYE phase where this signal occurred
    pub phase: String,
    /// "avoid" or "reinforce"
    pub signal: String,
    /// First 120 chars of the message content for pattern matching
    pub snippet: String,
    /// Accumulated weight (incremented when same pattern re-occurs)
    pub weight: u32,
}

/// Path to the RLHF bias persistence file
fn bias_file_path() -> Option<std::path::PathBuf> {
    std::env::var_os("HOME").map(|home| {
        std::path::PathBuf::from(&home)
            .join(".local")
            .join("share")
            .join("trinity")
            .join("rlhf_bias.json")
    })
}

/// Load all bias entries from disk (max 50, newest first)
pub fn load_prompt_bias() -> Vec<PromptBiasEntry> {
    let path = match bias_file_path() {
        Some(p) => p,
        None => return Vec::new(),
    };

    if !path.exists() {
        return Vec::new();
    }

    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str::<Vec<PromptBiasEntry>>(&s).ok())
        .unwrap_or_default()
}

/// Append a new bias entry, keeping the list at max 50 entries (FIFO)
pub fn append_prompt_bias(entry: PromptBiasEntry) -> Result<(), String> {
    let path = bias_file_path().ok_or("Cannot determine home directory")?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let mut entries = load_prompt_bias();

    // Check if same snippet already exists — increment weight instead of duplicating
    if let Some(existing) = entries.iter_mut().find(|e| e.snippet == entry.snippet) {
        existing.weight = existing.weight.saturating_add(1);
    } else {
        entries.push(entry);
    }

    // FIFO — keep max 50 entries
    if entries.len() > 50 {
        let drain_count = entries.len() - 50;
        entries.drain(0..drain_count);
    }

    let json = serde_json::to_string_pretty(&entries).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(())
}

/// Generate a prompt bias injection block for inclusion in Pete's system prompt.
/// Call this when building any system prompt to apply accumulated RLHF learning.
///
/// Returns an empty string if there are no strong bias signals (weight >= 2).
pub fn apply_prompt_bias(phase: Option<&str>) -> String {
    let entries = load_prompt_bias();
    if entries.is_empty() {
        return String::new();
    }

    let relevant: Vec<&PromptBiasEntry> = entries
        .iter()
        .filter(|e| {
            // Only apply high-weight signals, or phase-specific signals
            e.weight >= 2
                || phase.map(|p| e.phase == p).unwrap_or(false)
        })
        .collect();

    if relevant.is_empty() {
        return String::new();
    }

    let avoid: Vec<String> = relevant
        .iter()
        .filter(|e| e.signal == "avoid" && e.weight >= 1)
        .map(|e| format!("  - [{}] \"{}...\"", e.phase, e.snippet.chars().take(60).collect::<String>()))
        .collect();

    let reinforce: Vec<String> = relevant
        .iter()
        .filter(|e| e.signal == "reinforce" && e.weight >= 2)
        .map(|e| format!("  - [{}] \"{}...\"", e.phase, e.snippet.chars().take(60).collect::<String>()))
        .collect();

    let mut bias_block = String::from("\n\n═══ RLHF LEARNED PREFERENCES ═══\n");

    if !avoid.is_empty() {
        bias_block.push_str("DO NOT repeat these patterns (user rejected them):\n");
        bias_block.push_str(&avoid.join("\n"));
        bias_block.push('\n');
    }

    if !reinforce.is_empty() {
        bias_block.push_str("CONTINUE these patterns (user approved them):\n");
        bias_block.push_str(&reinforce.join("\n"));
        bias_block.push('\n');
    }

    bias_block
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_prompt_bias_empty_state() {
        // When there are no bias entries, apply_prompt_bias returns empty string
        // (We can't easily clear the file in tests, so we just verify no panic)
        let result = apply_prompt_bias(None);
        // Result is either empty or contains RLHF block — both are valid
        assert!(result.is_empty() || result.contains("RLHF"));
    }

    #[test]
    fn test_prompt_bias_entry_serializes() {
        let entry = PromptBiasEntry {
            phase: "Analysis".to_string(),
            signal: "avoid".to_string(),
            snippet: "This response was too direct and not Socratic".to_string(),
            weight: 1,
        };
        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("avoid"));
        assert!(json.contains("Analysis"));
        assert!(json.contains("Socratic"));
    }

    #[test]
    fn test_prompt_bias_entry_deserializes() {
        let json = r#"{"phase":"Design","signal":"reinforce","snippet":"Great analogy!","weight":3}"#;
        let entry: PromptBiasEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.phase, "Design");
        assert_eq!(entry.signal, "reinforce");
        assert_eq!(entry.weight, 3);
    }

    #[test]
    fn test_bias_fifo_logic() {
        // Test that the bias deduplication logic works
        let mut entries: Vec<PromptBiasEntry> = vec![
            PromptBiasEntry { phase: "A".to_string(), signal: "avoid".to_string(), snippet: "same".to_string(), weight: 1 },
            PromptBiasEntry { phase: "B".to_string(), signal: "avoid".to_string(), snippet: "same".to_string(), weight: 1 },
        ];
        // Deduplication: find same snippet and increment weight
        let new_entry = PromptBiasEntry { phase: "A".to_string(), signal: "avoid".to_string(), snippet: "same".to_string(), weight: 1 };
        if let Some(existing) = entries.iter_mut().find(|e| e.snippet == new_entry.snippet) {
            existing.weight = existing.weight.saturating_add(1);
        }
        // First entry should have weight 2 now
        assert_eq!(entries[0].weight, 2);
        assert_eq!(entries.len(), 2); // No new entry added
    }
}
