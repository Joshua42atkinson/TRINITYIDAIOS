// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity/src/quests.rs
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        quests.rs
// PURPOSE:     HTTP API endpoints for quest system
//
// ARCHITECTURE:
//   • Wraps trinity-quest crate for HTTP API
//   • Provides endpoints: get state, complete objective, advance, toggle party
//
// CHANGES:
//   2026-03-16  Cascade  Created from archive, adapted for 12-crate structure
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;

/// Get current game state (HTTP endpoint)
pub async fn get_game_state(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let game = state.project.game_state.read().await;

    // Product Maturity = how done AND how good the product is
    // Player XP bar = WHO you're becoming
    // Product Maturity bar = WHAT you're building
    let station_progress = game.quest.completed_phases.len() as f32 / 12.0;
    let pearl_alignment = game.quest.pearl.as_ref()
        .map(|p| p.evaluation.overall_alignment())
        .unwrap_or(0.0);
    let maturity_score = (station_progress * 0.5) + (pearl_alignment * 0.5);
    let maturity_label = match (maturity_score * 100.0) as u32 {
        0..=19 => "Raw Material",
        20..=39 => "Rough Cut",
        40..=59 => "Taking Shape",
        60..=79 => "Nearly Polished",
        80..=99 => "Production Ready",
        _ => "Ship It! 🚀",
    };

    // Build response JSON using trinity-quest types
    let response = serde_json::json!({
        "chapter": game.quest.hero_stage.chapter(),
        "chapter_title": game.quest.hero_stage.title(),
        "act": game.quest.hero_stage.act(),
        "phase": game.quest.current_phase.label(),
        "phase_index": game.quest.current_phase.phase_index(),
        "phase_icon": game.quest.current_phase.icon(),
        "subject": game.quest.subject,
        "game_title": game.quest.game_title,
        "objectives": game.quest.phase_objectives,
        "completed_phases": game.quest.completed_phases.iter().map(|p| p.label()).collect::<Vec<_>>(),
        "xp": game.quest.xp_earned,
        "steam": game.quest.steam_generated,
        "coal": 100.0 - game.quest.coal_used,
        "resonance": game.stats.resonance,
        "inventory": game.inventory,
        "phases": trinity_quest::hero::Phase::all_phases().iter().map(|p| {
            serde_json::json!({
                "name": p.label(),
                "icon": p.icon(),
                "blooms": p.blooms(),
                "group": p.group(),
                "circuit": p.circuit_name(),
            })
        }).collect::<Vec<_>>(),
        "party": game.party.iter().map(|m| {
            serde_json::json!({
                "id": m.id,
                "name": m.name,
                "role": m.role,
                "avatar": m.icon,
                "active": m.active,
                "can_help": m.specialty.contains(&game.quest.current_phase),
            })
        }).collect::<Vec<_>>(),
        "pearl": game.quest.pearl.as_ref().map(|p| serde_json::json!({
            "subject": p.subject,
            "medium": p.medium.display_name(),
            "medium_icon": p.medium.icon(),
            "vision": p.vision,
            "phase": p.phase.display_name(),
            "phase_icon": p.phase.icon(),
            "alignment": p.evaluation.overall_alignment(),
            "grade": p.evaluation.grade(),
            "addie_score": p.evaluation.addie_score,
            "crap_score": p.evaluation.crap_score,
            "eye_score": p.evaluation.eye_score,
            "refined_count": p.refined_count,
            "has_vision": p.has_vision(),
        })),
        "product_maturity": {
            "score": (maturity_score * 100.0).round(),
            "station_progress": (station_progress * 100.0).round(),
            "alignment": (pearl_alignment * 100.0).round(),
            "label": maturity_label,
        },
    });

    Ok(Json(response))
}

/// Get current Sacred Circuitry state (HTTP endpoint)
pub async fn get_circuitry_state(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let circuitry = state.vaam_bridge.circuitry.read().await;

    let response = serde_json::json!({
        "active": circuitry.active.name(),
        "active_order": circuitry.active.order(),
        "quadrant": circuitry.active.quadrant().name(),
        "activations": circuitry.activations,
        "summary": circuitry.summary(),
    });

    Ok(Json(response))
}

/// Get current Bevy ECS state for the DM HUD (HTTP endpoint)
pub async fn get_bevy_state(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let game = state.project.game_state.read().await;

    // Calculate simulated ZPD (Zone of Proximal Development)
    let zpd_range = format!(
        "{:.1} - {:.1}",
        game.stats.resonance as f32 * 0.8,
        game.stats.resonance as f32 * 1.2
    );

    let response = serde_json::json!({
        "entities": [
            {
                "id": "Player_Avatar",
                "components": {
                    "Resonance": game.stats.resonance,
                    "Coal": format!("{:.1}%", 100.0 - game.quest.coal_used),
                    "XP": game.stats.total_xp,
                    "ZPD_Zone": zpd_range
                }
            },
            {
                "id": "Dungeon_Master_Pete",
                "components": {
                    "State": "Active_Orchestration",
                    "Current_Quest": game.quest.hero_stage.title(),
                    "Bloom_Target": format!("{:?}", game.quest.current_phase),
                    "Cognitive_Load": "Low"
                }
            },
            {
                "id": "Encounter_Manager",
                "components": {
                    "Difficulty_Scale": format!("{:.2}", 1.0 + (game.stats.resonance as f32 * 0.05)),
                    "Active_Creeps": game.quest.phase_objectives.len(),
                    "Status": "Balancing"
                }
            },
            {
                "id": "ART_Sidecar_Process",
                "components": {
                    "Pipeline": "Multimodal",
                    "Status": "Idle",
                    "Memory_Budget": "Allocated"
                }
            }
        ]
    });

    Ok(Json(response))
}

/// Request to complete an objective
#[derive(Debug, Deserialize)]
pub struct CompleteRequest {
    pub objective_id: String,
}

/// Response from completing an objective
#[derive(Debug, Serialize)]
pub struct CompleteResponse {
    pub success: bool,
    pub xp_earned: u32,
    pub phase_advanced: bool,
    pub message: String,
}

/// Complete an objective (HTTP endpoint)
pub async fn complete_objective(
    State(state): State<AppState>,
    Json(req): Json<CompleteRequest>,
) -> Result<Json<CompleteResponse>, StatusCode> {
    let mut game = state.project.game_state.write().await;

    let success = game.quest.complete_objective(&req.objective_id);

    if success {
        game.stats.total_xp += 10;
        game.stats.combustion += 1;

        // Auto-advance phase if all objectives complete
        let advanced = if game.quest.phase_complete() {
            game.quest.advance_phase()
        } else {
            false
        };

        // Save to database
        let _ = trinity_quest::save_game_state(&state.db_pool, "default", &game).await;

        // Fire book update event — SSE clients get real-time quest progress
        let event = serde_json::json!({
            "type": "objective_completed",
            "objective_id": req.objective_id,
            "phase": game.quest.current_phase.label(),
            "chapter": game.quest.hero_stage.chapter(),
            "xp": game.stats.total_xp,
            "phase_advanced": advanced,
        });
        let _ = state.project.book_updates.send(event.to_string());

        Ok(Json(CompleteResponse {
            success: true,
            xp_earned: 10,
            phase_advanced: advanced,
            message: "Objective completed!".to_string(),
        }))
    } else {
        Ok(Json(CompleteResponse {
            success: false,
            xp_earned: 0,
            phase_advanced: false,
            message: "Objective not found or already complete".to_string(),
        }))
    }
}

/// Response from phase advance
#[derive(Debug, Serialize)]
pub struct PhaseResponse {
    pub success: bool,
    pub new_phase: String,
    pub chapter: u8,
    pub objectives: Vec<trinity_quest::Objective>,
}

/// Advance to next phase manually (HTTP endpoint)
pub async fn advance_phase(
    State(state): State<AppState>,
) -> Result<Json<PhaseResponse>, StatusCode> {
    let mut game = state.project.game_state.write().await;

    let advanced = game.quest.advance_phase();

    if advanced {
        game.stats.resonance += 1;

        // Sync Quest progress to Character Sheet skills
        {
            let mut sheet = state.player.character_sheet.write().await;
            use trinity_protocol::SkillType;

            // Map the completed phase to skill increases
            match game.quest.current_phase {
                trinity_quest::hero::Phase::Analysis | trinity_quest::hero::Phase::Design => {
                    *sheet
                        .skills
                        .entry(SkillType::CurriculumDesign)
                        .or_insert(0.0) += 2.5;
                }
                trinity_quest::hero::Phase::Development => {
                    *sheet
                        .skills
                        .entry(SkillType::GamificationDesign)
                        .or_insert(0.0) += 2.5;
                }
                trinity_quest::hero::Phase::Implementation => {
                    *sheet
                        .skills
                        .entry(SkillType::NarrativeDesign)
                        .or_insert(0.0) += 2.5;
                }
                trinity_quest::hero::Phase::Evaluation | trinity_quest::hero::Phase::Alignment => {
                    *sheet
                        .skills
                        .entry(SkillType::AssessmentDesign)
                        .or_insert(0.0) += 2.5;
                }
                _ => {
                    *sheet
                        .skills
                        .entry(SkillType::ContentCuration)
                        .or_insert(0.0) += 1.5;
                }
            }
            sheet.resonance_level = game.stats.resonance as u32;

            // ═══ SOFT SPOT 6: Friction reduction on phase advance ═══
            // Completing a phase = progress = reduced cognitive friction
            sheet.track_friction = (sheet.track_friction - 10.0).max(0.0);
            sheet.recalculate_vulnerability();

            let _ = crate::character_sheet::save_character_sheet(&sheet);
        }

        let _ = trinity_quest::save_game_state(&state.db_pool, "default", &game).await;

        // Fire book update event — SSE clients track phase transitions
        let event = serde_json::json!({
            "type": "phase_advanced",
            "new_phase": game.quest.current_phase.label(),
            "chapter": game.quest.hero_stage.chapter(),
            "resonance": game.stats.resonance,
        });
        let _ = state.project.book_updates.send(event.to_string());

        // ═══════════════════════════════════════════════════════════════
        // AUTO-SNAPSHOT: Git commit + Journal entry on phase advance
        // ═══════════════════════════════════════════════════════════════
        // Each phase completion creates:
        //   1. A git snapshot the user can roll back to
        //   2. A journal entry for portfolio/reflection review
        // Both run in background tasks — never block the HTTP response.
        {
            let prev_phase = game
                .quest
                .completed_phases
                .last()
                .map(|p| p.label().to_string())
                .unwrap_or_else(|| "Start".to_string());
            let new_phase = game.quest.current_phase.label().to_string();
            let subject = game.quest.subject.clone();

            // Capture journal snapshot from current state
            let quest_snap = crate::journal::QuestSnapshot {
                subject: game.quest.subject.clone(),
                phase: prev_phase.clone(),
                phase_index: game.quest.current_phase.phase_index().saturating_sub(1),
                chapter: game.quest.hero_stage.chapter(),
                chapter_title: game.quest.hero_stage.title().to_string(),
                completed_phases: game
                    .quest
                    .completed_phases
                    .iter()
                    .map(|p| p.label().to_string())
                    .collect(),
                objectives_total: game.quest.phase_objectives.len(),
                objectives_completed: game
                    .quest
                    .phase_objectives
                    .iter()
                    .filter(|o| o.completed)
                    .count(),
                xp: game.quest.xp_earned,
                coal_remaining: 100.0 - game.quest.coal_used,
                steam: game.quest.steam_generated,
            };
            let sheet = state.player.character_sheet.read().await;
            let char_snap = crate::journal::CharacterSnapshot {
                resonance: sheet.resonance_level,
                skills: sheet
                    .skills
                    .iter()
                    .map(|(k, v)| (format!("{:?}", k), *v))
                    .collect(),
                experience: sheet.experience.clone(),
                audience: sheet.audience.clone(),
                vision: sheet.success_vision.clone(),
            };
            drop(sheet);

            // Journal entry (synchronous — file I/O is fast)
            let journal_entry = crate::journal::create_entry(
                crate::journal::JournalEntryType::PhaseComplete,
                None,
                quest_snap,
                char_snap,
                vec![
                    format!("chapter-{}", game.quest.hero_stage.chapter()),
                    prev_phase.clone(),
                ],
            );
            let _ = crate::journal::save_entry(&journal_entry);

            // Git snapshot (async background)
            tokio::spawn(async move {
                let msg = format!(
                    "phase-complete: {} → {} ({})",
                    prev_phase, new_phase, subject
                );
                let result = tokio::process::Command::new("git")
                    .args(["add", "-A"])
                    .current_dir("/home/joshua/Workflow/desktop_trinity/trinity-genesis")
                    .output()
                    .await;
                if result.is_ok() {
                    let _ = tokio::process::Command::new("git")
                        .args(["commit", "-m", &msg, "--allow-empty"])
                        .current_dir("/home/joshua/Workflow/desktop_trinity/trinity-genesis")
                        .env("GIT_TERMINAL_PROMPT", "0")
                        .env("GIT_AUTHOR_NAME", "Trinity")
                        .env("GIT_COMMITTER_NAME", "Trinity")
                        .env("GIT_AUTHOR_EMAIL", "trinity@local")
                        .env("GIT_COMMITTER_EMAIL", "trinity@local")
                        .output()
                        .await;
                    tracing::info!("[Auto-Snapshot] {}", msg);
                }
            });
        }

        Ok(Json(PhaseResponse {
            success: true,
            new_phase: game.quest.current_phase.label().to_string(),
            chapter: game.quest.hero_stage.chapter(),
            objectives: game.quest.phase_objectives.clone(),
        }))
    } else {
        // Try to advance chapter instead
        let chapter_advanced = game.quest.advance_chapter();
        if chapter_advanced {
            let _ = trinity_quest::save_game_state(&state.db_pool, "default", &game).await;

            Ok(Json(PhaseResponse {
                success: true,
                new_phase: format!(
                    "{} - {}",
                    game.quest.hero_stage.title(),
                    game.quest.current_phase.label()
                ),
                chapter: game.quest.hero_stage.chapter(),
                objectives: game.quest.phase_objectives.clone(),
            }))
        } else {
            Ok(Json(PhaseResponse {
                success: false,
                new_phase: game.quest.current_phase.label().to_string(),
                chapter: game.quest.hero_stage.chapter(),
                objectives: game.quest.phase_objectives.clone(),
            }))
        }
    }
}

/// Request to toggle party member
#[derive(Debug, Deserialize)]
pub struct ToggleRequest {
    pub member_id: String,
    pub active: bool,
}

/// Toggle party member active status (HTTP endpoint)
pub async fn toggle_party_member(
    State(state): State<AppState>,
    Json(req): Json<ToggleRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut game = state.project.game_state.write().await;

    if let Some(member) = game.party.iter_mut().find(|m| m.id == req.member_id) {
        member.active = req.active;

        Ok(Json(serde_json::json!({
            "success": true,
            "member_id": member.id,
            "name": member.name,
            "active": member.active,
        })))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Request to set subject
#[derive(Debug, Deserialize)]
pub struct SubjectRequest {
    pub subject: String,
}

/// Set quest subject (HTTP endpoint)
pub async fn set_subject(
    State(state): State<AppState>,
    Json(req): Json<SubjectRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut game = state.project.game_state.write().await;
    game.quest.subject = req.subject.clone();
    game.quest.game_title = format!("{} Learning Experience", req.subject);
    // Create or update the PEARL from the subject
    game.quest.pearl = Some(trinity_protocol::Pearl::new(&req.subject));

    let _ = trinity_quest::save_game_state(&state.db_pool, "default", &game).await;

    Ok(Json(serde_json::json!({
        "success": true,
        "subject": req.subject,
        "game_title": game.quest.game_title,
    })))
}

// ═══════════════════════════════════════════════════════════════════
// PEARL ENDPOINTS
// ═══════════════════════════════════════════════════════════════════

/// Get current PEARL
pub async fn get_pearl(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let game = state.project.game_state.read().await;

    match &game.quest.pearl {
        Some(pearl) => Ok(Json(serde_json::json!({
            "subject": pearl.subject,
            "medium": pearl.medium.display_name(),
            "medium_icon": pearl.medium.icon(),
            "vision": pearl.vision,
            "phase": pearl.phase.display_name(),
            "phase_icon": pearl.phase.icon(),
            "alignment": pearl.evaluation.overall_alignment(),
            "grade": pearl.evaluation.grade(),
            "addie_score": pearl.evaluation.addie_score,
            "crap_score": pearl.evaluation.crap_score,
            "eye_score": pearl.evaluation.eye_score,
            "refined_count": pearl.refined_count,
            "has_vision": pearl.has_vision(),
            "suggested_tools": pearl.medium.suggested_tools(),
            "prompt_summary": pearl.prompt_summary(),
        }))),
        None => Ok(Json(serde_json::json!({
            "error": "No PEARL set. Select a subject first.",
        }))),
    }
}

/// Request to create or replace a PEARL
#[derive(Debug, Deserialize)]
pub struct CreatePearlRequest {
    pub subject: String,
    #[serde(default = "default_medium")]
    pub medium: String,
    #[serde(default)]
    pub vision: String,
}

fn default_medium() -> String {
    "Game".to_string()
}

fn parse_medium(s: &str) -> trinity_protocol::PearlMedium {
    match s.to_lowercase().as_str() {
        "game" => trinity_protocol::PearlMedium::Game,
        "storyboard" => trinity_protocol::PearlMedium::Storyboard,
        "simulation" => trinity_protocol::PearlMedium::Simulation,
        "lessonplan" | "lesson_plan" | "lesson plan" => trinity_protocol::PearlMedium::LessonPlan,
        "assessment" => trinity_protocol::PearlMedium::Assessment,
        "book" => trinity_protocol::PearlMedium::Book,
        other => trinity_protocol::PearlMedium::Other(other.to_string()),
    }
}

/// Create or replace the PEARL (HTTP endpoint)
pub async fn create_pearl(
    State(state): State<AppState>,
    Json(req): Json<CreatePearlRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut game = state.project.game_state.write().await;

    let medium = parse_medium(&req.medium);
    let pearl = if req.vision.is_empty() {
        trinity_protocol::Pearl::new(&req.subject)
    } else {
        trinity_protocol::Pearl::with_vision(&req.subject, medium.clone(), &req.vision)
    };

    // Reset quest back to Chapter 1 Analysis — new PEARL = new journey
    game.quest.subject = req.subject.clone();
    game.quest.game_title = format!("{} Learning Experience", req.subject);
    game.quest.pearl = Some(pearl);
    // Full journey reset
    game.quest.hero_stage = trinity_quest::hero::HeroStage::OrdinaryWorld;
    game.quest.current_phase = trinity_quest::hero::Phase::Analysis;
    game.quest.phase_objectives = trinity_quest::objectives_for_chapter(
        trinity_quest::hero::HeroStage::OrdinaryWorld,
        trinity_quest::hero::Phase::Analysis,
    );
    game.quest.completed_phases = vec![];
    game.quest.completed_chapters = vec![];
    game.quest.xp_earned = 0;
    game.quest.coal_used = 0.0;
    game.quest.steam_generated = 0.0;
    game.quest.quest_title = trinity_quest::hero::HeroStage::OrdinaryWorld
        .title()
        .to_string();
    // Reset stats too
    game.stats.total_xp = 0;
    game.stats.resonance = 1;
    game.stats.traction = 3;
    game.stats.velocity = 2;
    game.stats.combustion = 1;
    game.stats.coal_reserves = 87.0;

    let _ = trinity_quest::save_game_state(&state.db_pool, "default", &game).await;

    // Fire SSE event so frontend knows the journey has begun
    let event = serde_json::json!({
        "type": "journey_started",
        "subject": req.subject,
        "phase": "Analysis",
        "chapter": 1,
    });
    let _ = state.project.book_updates.send(event.to_string());

    Ok(Json(serde_json::json!({
        "success": true,
        "subject": req.subject,
        "medium": medium.display_name(),
        "vision": req.vision,
    })))
}

/// Request to refine a PEARL
#[derive(Debug, Deserialize)]
pub struct RefinePearlRequest {
    #[serde(default)]
    pub vision: Option<String>,
    #[serde(default)]
    pub medium: Option<String>,
    #[serde(default)]
    pub addie_score: Option<f32>,
    #[serde(default)]
    pub crap_score: Option<f32>,
    #[serde(default)]
    pub eye_score: Option<f32>,
}

/// Refine the PEARL — update vision, medium, or evaluation scores (HTTP endpoint)
pub async fn refine_pearl(
    State(state): State<AppState>,
    Json(req): Json<RefinePearlRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut game = state.project.game_state.write().await;

    // Scope the mutable pearl borrow — extract needed data, then release
    let pearl_data = {
        if let Some(pearl) = game.quest.pearl.as_mut() {
            let new_medium = req.medium.as_deref().map(parse_medium);
            pearl.refine(req.vision.clone(), new_medium);

            if let Some(score) = req.addie_score {
                pearl
                    .evaluation
                    .update_score(trinity_protocol::PearlPhase::Extracting, score);
            }
            if let Some(score) = req.crap_score {
                pearl
                    .evaluation
                    .update_score(trinity_protocol::PearlPhase::Placing, score);
            }
            if let Some(score) = req.eye_score {
                pearl
                    .evaluation
                    .update_score(trinity_protocol::PearlPhase::Refining, score);
            }

            if pearl.evaluation.overall_alignment() >= 0.9
                && pearl.phase == trinity_protocol::PearlPhase::Refining
            {
                pearl.polish();
            }

            Some((
                pearl.evaluation.overall_alignment(),
                pearl.evaluation.grade().to_string(),
                pearl.phase.display_name().to_string(),
                pearl.refined_count,
            ))
        } else {
            None
        }
    };

    match pearl_data {
        Some((alignment, grade, phase_name, refined_count)) => {
            let _ = trinity_quest::save_game_state(&state.db_pool, "default", &game).await;

            let event = serde_json::json!({
                "type": "pearl_refined",
                "phase": phase_name,
                "alignment": alignment,
                "grade": grade,
                "refined_count": refined_count,
            });
            let _ = state.project.book_updates.send(event.to_string());

            Ok(Json(serde_json::json!({
                "success": true,
                "alignment": alignment,
                "grade": grade,
                "phase": phase_name,
                "refined_count": refined_count,
            })))
        }
        None => Ok(Json(serde_json::json!({
            "success": false,
            "error": "No PEARL set. Create one first with POST /api/pearl.",
        }))),
    }
}
