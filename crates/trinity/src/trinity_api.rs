// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        trinity_api.rs
// PURPOSE:     Unified API — "Talk to Trinity like a person"
//
// ARCHITECTURE:
//   • POST /api/v1/trinity — single endpoint, routes by mode
//   • Returns structured JSON with diagnostics (VAAM, skills, etc.)
//   • Testable via curl, integration harness, or any API client
//   • Wraps existing chat/agent functionality into a clean interface
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{inference, rag, AppState, ChatMessage};

/// Unified request to Trinity
#[derive(Debug, Deserialize)]
pub struct TrinityRequest {
    /// The message to send
    pub message: String,
    /// Mode: "iron-road", "dev", "creative"
    #[serde(default = "default_mode")]
    pub mode: String,
    /// Session ID for conversation continuity
    #[serde(default = "default_session")]
    pub session_id: String,
    /// Include VAAM diagnostics, skill checks, etc.
    #[serde(default)]
    pub include_diagnostics: bool,
    /// Max tokens for response
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

fn default_mode() -> String {
    "iron-road".to_string()
}
fn default_session() -> String {
    format!("session-{}", chrono::Utc::now().timestamp())
}
fn default_max_tokens() -> u32 {
    16384
}

/// Unified response from Trinity
#[derive(Debug, Serialize)]
pub struct TrinityResponse {
    /// The AI's text response
    pub reply: String,
    /// Which mode was used
    pub mode: String,
    /// Session ID (echo back for client tracking)
    pub session_id: String,
    /// Optional diagnostics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Diagnostics>,
}

#[derive(Debug, Serialize)]
pub struct Diagnostics {
    /// VAAM-detected vocabulary words
    pub vocabulary_detected: Vec<String>,
    /// Total coal earned from this message
    pub coal_earned: u32,
    /// Current quest phase
    pub current_phase: String,
    /// Character info
    pub character: CharacterSummary,
}

#[derive(Debug, Serialize)]
pub struct CharacterSummary {
    pub alias: String,
    pub class: String,
    pub xp: u64,
    pub coal: f32,
    pub resonance_level: u32,
}

/// POST /api/v1/trinity — unified endpoint
pub async fn trinity_chat(
    State(state): State<AppState>,
    Json(request): Json<TrinityRequest>,
) -> Result<Json<TrinityResponse>, (StatusCode, String)> {
    info!(
        "[Trinity API] mode={} session={} message={}...",
        request.mode,
        request.session_id,
        &request.message[..request.message.len().min(50)]
    );

    let llm_url = state.inference_router.read().await.active_url().to_string();

    // Check inference health
    if !inference::check_health(&llm_url).await {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "LLM server not reachable. Start vLLM on :8001".to_string(),
        ));
    }

    // Build system prompt based on mode
    let system_prompt = match request.mode.as_str() {
        "iron-road" => build_iron_road_prompt(&state).await,
        "creative" => {
            "You are Trinity's creative engine. Help the user generate visual and musical assets for their educational game. You can describe images, suggest music moods, and plan creative assets."
                .to_string()
        }
        _ => {
            "You are Trinity, an AI coding and instructional design assistant running locally on a 128GB AMD workstation. Help with coding, game design, and system management."
                .to_string()
        }
    };

    // VAAM scan for vocabulary detection
    let vaam_result = state.vaam_bridge.vaam.scan_message(&request.message).await;
    let vocabulary_detected: Vec<String> = vaam_result
        .detections
        .iter()
        .map(|d| d.word.clone())
        .collect();
    let coal_earned = vaam_result.total_coal;

    // Update game state with vocabulary coal
    if !vocabulary_detected.is_empty() {
        let mut gs = state.project.game_state.write().await;
        gs.stats.coal_reserves = (gs.stats.coal_reserves + coal_earned as f32).min(100.0);

        // Persist VAAM mastery to database (non-blocking)
        let pool = state.db_pool.clone();
        let mastery_snapshot = state.vaam_bridge.vaam.mastery.read().await.clone();
        let detections = vaam_result.detections.clone();
        tokio::spawn(async move {
            let project_id = "default";
            if let Err(e) =
                crate::vaam::save_mastery_to_db(&pool, project_id, &mastery_snapshot).await
            {
                tracing::warn!("[VAAM] Mastery save failed: {}", e);
            }
            for detection in &detections {
                if let Err(e) =
                    crate::vaam::record_detection(&pool, project_id, detection, None).await
                {
                    tracing::warn!("[VAAM] Detection record failed: {}", e);
                }
            }
        });
    }

    // Add VAAM context to system prompt
    let vaam_context = state.vaam_bridge.prompt_context().await;
    let full_system = if vaam_context.is_empty() {
        system_prompt
    } else {
        format!("{}\n\nVAAM ALIGNMENT:\n{}", system_prompt, vaam_context)
    };

    // RAG (only if DB is available)
    let rag_context = rag::search_documents(&state.db_pool, &request.message)
        .await
        .unwrap_or_default();
    let rag_suffix = if rag_context.is_empty() {
        String::new()
    } else {
        format!(
            "\n\nRelevant context:\n{}",
            rag_context[..rag_context.len().min(3)].join("\n---\n")
        )
    };

    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: format!("{}{}", full_system, rag_suffix),
            timestamp: None,
            image_base64: None,
        },
        ChatMessage {
            role: "user".to_string(),
            content: request.message,
            timestamp: None,
            image_base64: None,
        },
    ];

    // Call LLM
    let reply = inference::chat_completion(&llm_url, &messages, request.max_tokens)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Inference failed: {}", e),
            )
        })?;

    // Build diagnostics if requested
    let diagnostics = if request.include_diagnostics {
        let gs = state.project.game_state.read().await;
        let sheet = state.player.character_sheet.read().await;
        Some(Diagnostics {
            vocabulary_detected,
            coal_earned,
            current_phase: gs.quest.current_phase.label().to_string(),
            character: CharacterSummary {
                alias: sheet.alias.clone(),
                class: format!("{:?}", sheet.user_class),
                xp: gs.stats.total_xp as u64,
                coal: gs.stats.coal_reserves,
                resonance_level: sheet.resonance_level,
            },
        })
    } else {
        None
    };

    Ok(Json(TrinityResponse {
        reply,
        mode: request.mode,
        session_id: request.session_id,
        diagnostics,
    }))
}

/// Build Iron Road system prompt with character context
async fn build_iron_road_prompt(state: &AppState) -> String {
    let sheet = state.player.character_sheet.read().await;
    let gs = state.project.game_state.read().await;

    format!(
        r#"You are Pete, a master Instructional Designer and AI coach inside TRINITY ID AI OS.
You guide teachers through the ADDIECRAPEYE framework to build gamified lesson plans.

Current conductor: {} ({:?})
Resonance Level: {}
Current Phase: {} ({})
XP: {} | Coal: {:.0}%

Guide the user through the current phase. Ask questions. Suggest activities.
When they've completed this phase's objectives, suggest advancing to the next phase."#,
        sheet.alias,
        sheet.user_class,
        sheet.resonance_level,
        gs.quest.current_phase.label(),
        gs.quest.current_phase.label(),
        gs.stats.total_xp,
        gs.stats.coal_reserves,
    )
}
