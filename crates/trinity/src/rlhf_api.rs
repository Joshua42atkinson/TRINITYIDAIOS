use axum::{extract::State, response::Json};
use trinity_protocol::character_sheet::ShadowStatus;

use crate::AppState;

/// Accept any JSON for RLHF feedback — the UI sends {message_id, score, phase}
/// but the original handler expected ResonanceFeedbackEvent. Accept both shapes.
///
/// ═══ SOFT SPOT 5: WIRED ═══
/// This is no longer a stub. Negative feedback cascades:
///   score < 0 → friction↑, consecutive_negatives↑, Shadow escalation
///   score > 0 → steam↑, friction↓, consecutive_negatives reset
/// Vulnerability is recalculated after every mutation.
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
    } else if score > 0 {
        // Positive feedback → steam rises, friction drops, Shadow calms
        sheet.consecutive_negatives = 0;
        sheet.current_steam = (sheet.current_steam + 3.0).min(100.0);
        sheet.track_friction = (sheet.track_friction - 2.0).max(0.0);
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
