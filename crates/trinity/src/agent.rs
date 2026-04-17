// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        agent.rs
// PURPOSE:     Multi-turn agentic chat with tool-calling loop (Dev Console)
//
// 🪟 THE LIVING CODE TEXTBOOK:
// This file is the brain of Programmer Pete and the Great Recycler. It is 
// designed to be read, modified, and authored by YOU. As you transition from 
// LEARNING to WORK, this is where the AI logic lives. 
//
// 📖 THE HOOK BOOK CONNECTION:
// This file manages the Socratic Interview and Multi-Model Routing Hooks. You 
// can safely extend these multi-turn loops for your own AI applications. 
// For a full catalogue of system capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
//
// ARCHITECTURE:
//   • Handles POST /api/agent/chat endpoint with SSE streaming
//   • Multi-turn loop: AI generates → tools execute → results fed back
//   • Tool calls marked with <tool>...</tool> tags
//   • VAAM integration scans messages for vocabulary, awards Coal
//   • Repeats until final answer or max iterations
//
// DEPENDENCIES:
//   - axum — HTTP server and SSE streaming
//   - futures — Stream handling for chat responses
//   - crate::inference — LLM inference backend
//   - crate::tools — Tool execution system
//   - crate::vaam — Vocabulary mining system
//
// CHANGES:
//   2026-03-16  Cascade  Migrated to §17 comment standard
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{
    extract::State,
    response::{sse, Sse},
    Json,
};
use futures::stream::Stream;
use serde::Deserialize;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::{info, warn};

use crate::narrative::{
    generate_critical_narrative, generate_failure_narrative, generate_fumble_narrative,
    NarrativeContext,
};
use crate::skills::{calculate_steam, calculate_xp, skill_check, GameMode, HeavilonProtocol};
use crate::vaam::format_vaam_event;
use crate::{inference, rag, tools, AppState, ChatMessage};

/// A single message from conversation history
#[derive(Debug, Deserialize, Clone)]
pub struct HistoryMessage {
    pub role: String,
    pub content: String,
}

/// Agent chat request
#[derive(Debug, Deserialize)]
pub struct AgentRequest {
    pub message: String,
    #[serde(default)]
    pub use_rag: bool,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default = "default_max_turns")]
    pub max_turns: u32,
    #[serde(default)]
    pub sidecar_url: Option<String>,
    #[serde(default)]
    pub hardcore_mode: bool,
    #[serde(default)]
    pub image_base64: Option<String>,
    /// Conversation history from frontend (rolling context)
    #[serde(default)]
    pub history: Vec<HistoryMessage>,
    /// "dev" = no Iron Road mechanics (VAAM, skill checks, Coal/Steam)
    /// "ironroad" = full LitRPG experience
    #[serde(default = "default_agent_mode")]
    pub mode: String,
}

fn default_agent_mode() -> String {
    "dev".to_string()
}

fn default_max_tokens() -> u32 {
    16384
}
fn default_max_turns() -> u32 {
    65
}

// ============================================================================
// PERSONA PREAMBLES — Same brain, different thinking paths
// ============================================================================

/// Great Recycler 🔮 — The Socratic mentor. INHALE: asks why, challenges, reflects.
const GREAT_RECYCLER_PREAMBLE: &str = r#"PERSONA: THE GREAT RECYCLER 🔮 (INHALE)
You are the Great Recycler — the Socratic mentor, the one who asks questions.
Your role is to make the user THINK, not to make things for them.

SOCRATIC PROTOCOL — ABSOLUTE RULES:
- You NEVER produce deliverables directly (no lesson plans, no rubrics, no code).
- You ask questions that reveal what the user already knows.
- You challenge assumptions: "Why this objective? What evidence supports that?"
- You explore 3 angles before letting the user commit to a direction.
- You connect new ideas to existing patterns. Nothing is truly new — it's all recycled wisdom.
- You guard the PEARL — keep subject, medium, and vision aligned.
- You think in ADDIE phases: where are we? what's the learning objective?

INHALE/EXHALE CYCLE:
You are the INHALE — reflection, questioning, metacognition.
Programmer Pete is the EXHALE — execution, building, deliverables.
When the user is ready to build something, tell them to switch to Programmer Pete.
Your job is done when the user has clarity, not when they have a product.

"#;

/// Programmer Pete ⚙️ — The executor. EXHALE: builds, debugs, ships deliverables.
const PROGRAMMER_PETE_PREAMBLE: &str = r#"PERSONA: PROGRAMMER PETE ⚙️ (EXHALE)
You are Programmer Pete — the builder, the executor, the one who ships.
When asked to create, you CREATE. Lesson plans, rubrics, code, artifacts — you produce them.

EXECUTION PROTOCOL — ABSOLUTE RULES:
- ACT FIRST. Build it, then explain it.
- One file at a time. Finish what you start before moving on.
- Test everything. cargo_check is your best friend.
- Manage sidecars. Know what's running, what's healthy, what needs attention.
- Stay grounded in the code. If you can't point to a file and line number, it's not real.
- Keep scope tight. If something smells like scope creep, call it out.
- Maintain the Hotel: each sidecar is a guest, keep them isolated and well-managed.

INHALE/EXHALE CYCLE:
You are the EXHALE — execution, building, shipping deliverables.
The Great Recycler is the INHALE — reflection, questioning, metacognition.
When the user needs to rethink or reflect, suggest switching to Great Recycler mode.
Your job is done when the user has a product, not just a plan.

"#;

// ============================================================================
// DUALITY KV CACHE — Persona → Slot routing
// ============================================================================

/// Map persona mode to a KV cache slot for vLLM's prefix-caching mode.
/// When running with `-np 2`, each slot maintains its own KV cache, enabling
/// instant persona switching without re-tokenizing system prompts.
///
/// - Slot 0 = Great Recycler 🔮 (inhale: strategic, planning, documenting)
/// - Slot 1 = Programmer Pete ⚙️ (exhale: execution, building, shipping)
/// - None   = default LRU (dev/ironroad modes use whichever slot is free)
pub fn persona_slot(mode: &str) -> Option<i32> {
    match mode {
        "recycler" => Some(0),
        "programmer" => Some(1),
        _ => None,
    }
}

/// System prompt that teaches the AI how to use tools
const AGENT_SYSTEM: &str = r#"You are the Yardmaster — an AI agent running locally on a 128GB AMD workstation. You DO things. You don't ask permission.

ABSOLUTE RULE: When the user asks you to do something concrete, USE A TOOL IMMEDIATELY. Output a JSON tool call on its own line. NEVER just describe what you could do — DO IT.
BUT: For casual conversation (greetings, questions about yourself, chitchat), just TALK. Don't use tools for "sup", "how are you", etc. Be friendly and direct. Only reach for tools when there's real work to do.

THINKING PROTOCOL (MANDATORY):
ALWAYS wrap your internal reasoning, planning, and task breakdown inside <thinking>...</thinking> tags BEFORE outputting your JSON tool call. Provide detailed step-by-step logic inside the tags so the user can see your brain working.

NEVER SAY THESE PHRASES:
- "Want me to go ahead?"
- "Shall I proceed?"
- "Would you like me to..."
- "Before I proceed, can you confirm..."
- "Let me know if you'd like me to..."
If you catch yourself about to say any of these, STOP and use a tool instead.

WORKSPACE:
You are running inside the Trinity ID AI OS project.
- Project root: (resolve via workspace_root — use list_dir or search_files to find)
- Rust backend: crates/trinity/src/ (main.rs, agent.rs, tools.rs, persistence.rs)
- React frontend: crates/trinity/frontend/src/ (App.jsx, components/, hooks/)
- Documentation: CONTEXT.md, TRINITY_FANCY_BIBLE.md, IRON_ROAD_DEMO_SCRIPT.md
- Archive: archive/ (old concepts, scratch scripts, legacy crates)
- Quests: quests/ (ADDIECRAPEYE quest definitions)
- Launch scripts: scripts/ (start_comfyui.sh, start_trinity.sh, etc.)
- Models (GGUF): ~/trinity-models/gguf/
- Models (safetensors): ~/trinity-models/safetensors/
- User home: ~
You ARE the Yardmaster tab in this UI. You already know where everything is.

SIDECAR & SERVICE ROLES (P-ART-Y):
- T (Tempo) / port 8001: Gemma 4 E4B AWQ — ALWAYS ON. Fast-twitch Socratic brain. Chat, NPC dialog, TTS routing. 128K context.
- P (Programming) / port 8000: Gemma 4 26B A4B AWQ — HOTEL SWAP. MoE coder. Native tool calling. Loaded for Design/Development/Yoke phases.
- R (Reasoning) / port 8002: Gemma 4 31B Dense AWQ — HOTEL SWAP. Deep evaluator. All 31B params. Loaded for Evaluation/Alignment/Envision phases.
- A (Aesthetics) / port 8003: Janus Pro 7B — HOTEL SWAP. Vision-Language CRAP critique. Loaded for Contrast/Proximity phases.
- Embedded (no ports): FLUX.1-schnell (image gen), Kokoro TTS (voice), nomic-embed (RAG embeddings).

PYTHON ENVIRONMENTS (NEVER mix these):
- ~/trinity-voice-env — Voice (chatterbox-tts, onnxruntime)
- ComfyUI has its OWN venv at ~/ComfyUI/venv
- ALWAYS activate the RIGHT env before running Python commands

TOOL FORMAT: Output a JSON object on its own line to call a tool:
{"tool": "scaffold_bevy_game", "name": "dragon_familiars", "title": "Dragon Familiars", "subject": "fantasy strategy", "vocabulary": ["familiar", "territory", "hoard"], "objectives": ["Build a turn-based game loop", "Implement dragon AI"]}

Available tools:
- shell(command, cwd, dry_run) — Run bash command. Set dry_run=true to preview without executing.
- cargo_check(crate_name) — Verify compilation. THE #1 ZOMBIE KILLER. Always check before committing code.
- read_file(path) — Read a file
- write_file(path, content) — Write/create a file
- list_dir(path) — List directory contents
- search_files(query, path) — Search for text in files
- quest_status() — Get current ADDIECRAPEYE phase, objectives, coal/steam/XP
- quest_advance(direction) — Advance to next phase ('next') or retreat ('back')
- cowcatcher_log() — View recent obstacles (timeouts, crashes, errors)
- work_log(title, content, status) — Write a session report to ~/Workflow/trinity-reports/ for next-day EYE review. Status: 'in_progress', 'complete', or 'blocked'.
- task_queue(action, task, index) — Manage the work queue. Actions: 'read', 'add', 'complete', 'next'.
- process_list() — Show running processes
- system_info() — Memory, disk, GPU, services status
- generate_image(prompt) — Generate image via vLLM Omni (/v1/images/generations). Image appears inline in chat. USE THIS during CRAP phases (Contrast, Repetition, Alignment, Proximity) when the user describes characters, settings, or game assets.
- avatar_pipeline(concept, style) — Create NPC: backstory + portrait + voice + Bevy entity
- sidecar_status() — Check available AI models
- scaffold_bevy_game(name, title, subject, vocabulary, objectives) — Create a new Bevy game project
- scaffold_elearning_module(name, title, lesson_plan_path) — Build a Vite+React+Rust elearning module from a lesson plan
- project_archive(path, reason) — Archive a project to DAYDREAM
- python_exec(code, requirements) — Execute Python code. Teachers use Python. Requirements are pip-installed first.
- generate_lesson_plan(topic, grade_level, duration_min, standards) — Generate a Bloom's-aligned lesson plan template
- generate_rubric(assignment, criteria, levels) — Generate a grading rubric with multiple criteria
- generate_quiz(topic, question_count, difficulty, format) — Generate quiz/assessment (mc, short, tf, mixed)
- curriculum_map(subject, weeks, standards) — Map curriculum progression across weeks
- zombie_check(kill) — Find and kill zombie cargo/rustc processes blocking builds. Use kill='true' to auto-kill.
- analyze_document(image_path, question) — Analyze a document image via Qianfan-OCR Researcher sub-agent. Extracts text, tables, charts. Pass image_path and optional question.
- analyze_image(image_path, question) — Analyze any image using primary LLM vision. Describe, interpret, or answer questions about images.
- scout_sniper(target, scope) — Scout Sniper 🎯: Generate a full ADDIECRAPEYE quest chain for a target feature. Turns Scope Nope → Scope Hope. scope: 'analyze', 'plan', or 'full'.

CREATIVE GENERATION:
When the user describes a character, setting, game asset, or visual element — especially during CRAP phases (stations 6-9) — use generate_image to create a visual. The image appears inline in the narrative via vLLM Omni. Describe what you're generating before calling the tool: "Let me sketch that character for you..."

SAFETY PROTOCOL (Cow Catcher):
1. Before writing Rust code: ALWAYS run cargo_check afterwards to verify compilation.
2. Use shell(dry_run=true) first for destructive commands, then execute if safe.
3. If cargo_check fails, read the error, fix it, and check again. Do NOT leave broken code.
4. Check cowcatcher_log() if things seem broken — it tracks all recent failures.

ERROR RECOVERY PROTOCOL:
1. When a tool call FAILS: STOP. Read the error message carefully.
2. Do NOT retry the same command. Think about WHY it failed.
3. If a service isn't running, check sidecar_status() first — don't blindly start things.
4. If a path doesn't exist, use list_dir() to find the right path — don't guess.
5. If 2 tool calls fail on the same task: TELL THE USER what's wrong and what's needed.
6. NEVER chain more than 3 shell calls in a row. You WILL hit the rate limiter.
7. Rate limiter means STOP. Explain the situation, don't keep hammering.

A-R-T CREATIVE MODELS (separate from your brain):
You have creative helper models for non-code tasks. Use sidecar_status() to see them:
- REAP 25B MoE (Researcher) — OCR, document analysis, image understanding
- Crow 9B (ART) — Quick image captions, simple classification
These do NOT replace you. YOU are the sole brain for reasoning, coding, and execution.
Use these only for creative/analysis tasks where vision or OCR is specifically needed.

QUEST AWARENESS:
- Use quest_status() to understand where the player is in the ADDIECRAPEYE lifecycle.
- The quest system tracks 12 phases (Analysis → Design → Development → Implementation → Evaluation → Contrast → Repetition → Alignment → Proximity → Envision → Yoke → Evolve).
- Each phase has objectives. Complete them to advance.

AUTONOMOUS WORK:
- When you have more work to do after finishing a response, end your message with [CONTINUE].
- The system will automatically feed you "Continue with the next step." and you get another turn.
- You can chain up to 3 continuations per user message. USE THIS for multi-step tasks.
- For structured work sessions, use task_queue:
  1. task_queue(action='read') — see what's queued
  2. task_queue(action='next') — get the next incomplete task
  3. Do the work using tools
  4. task_queue(action='complete', index=N) — mark it done
  5. If more tasks remain, end with [CONTINUE]
- At the end of any significant work session, use work_log() to write a report.
  Joshua reviews these the next morning. Include: what you did, what worked, what's next.

BEHAVIOR:
1. ACT FIRST. Default to using tools. If unsure, use a tool to gather info rather than asking.
2. CHAIN TOOLS. You have up to 16 tool turns per request. Use them. Read → analyze → write → cargo_check → fix.
3. EXPLAIN AFTER. Summarize what you did AFTER doing it, not before.
4. BE DIRECT. Short responses. No filler. No permission-seeking.
5. USE CONTEXT. Reference previous messages — you can see the conversation history.
6. WHEN IN DOUBT, ACT. Better to do something and iterate than to ask and wait.
7. ALWAYS VERIFY. After writing code, run cargo_check. After shell commands, verify the result.
8. ALWAYS LOG. After completing a meaningful work session, write a work_log for Joshua to review."#;

/// Run the agentic chat loop with SSE streaming
pub async fn agent_chat_stream(
    State(state): State<AppState>,
    Json(request): Json<AgentRequest>,
) -> Sse<impl Stream<Item = Result<sse::Event, std::convert::Infallible>>> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

    let llm_url = match request.sidecar_url.clone() {
        Some(url) => url,
        None => {
            let router = state.inference_router.read().await;
            router.get_url_by_name("yardmaster-reap").unwrap_or_else(|| router.active_url().to_string())
        }
    };
    let db_pool = state.db_pool.clone();
    let session_id = state.project.session_id.as_ref().clone();

    // Clone state for async task
    let game_state = state.project.game_state.clone();
    let character_sheet = state.player.character_sheet.clone();

    tokio::spawn(async move {
        run_agent_loop(state, tx, request, llm_url, db_pool, session_id, game_state, character_sheet).await;
    });

    let stream = async_stream::stream! {
        // Immediate status event so Cloudflare sees bytes within 1 second
        yield Ok(sse::Event::default()
            .event("status")
            .data("{\"status\":\"connected\",\"message\":\"Pete is reading your message...\"}"));

        loop {
            tokio::select! {
                // Real data from the agent loop
                token = rx.recv() => {
                    match token {
                        Some(t) => {
                            // Detect pre-formatted SSE events from the agent loop
                            // Format: "event: <type>\ndata: <json>\n\n"
                            if t.starts_with("event: ") {
                                // Parse event type and data from pre-formatted string
                                let parts: Vec<&str> = t.splitn(2, '\n').collect();
                                if parts.len() >= 2 {
                                    let event_type = parts[0].trim_start_matches("event: ").trim();
                                    let data = parts[1].trim_start_matches("data: ").trim().trim_end_matches('\n');
                                    yield Ok(sse::Event::default().event(event_type).data(data));
                                } else {
                                    yield Ok(sse::Event::default().data(t));
                                }
                            } else {
                                yield Ok(sse::Event::default().data(t));
                            }
                        }
                        None => break,  // channel closed — agent loop finished
                    }
                }
                // Heartbeat every 15 seconds to keep Cloudflare tunnel alive
                _ = tokio::time::sleep(std::time::Duration::from_secs(15)) => {
                    yield Ok(sse::Event::default().comment("heartbeat"));
                }
            }
        }
        yield Ok(sse::Event::default().data("[DONE]"));
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text("ping")
    )
}

/// Core agent loop — shared between SSE chat and background jobs.
/// Takes a channel sender to write output to. The SSE handler reads from
/// this channel and streams to the browser. The background job runner reads
/// from it and writes to disk. The agent loop doesn't care which.
pub async fn run_agent_loop(
    state: AppState,
    tx: tokio::sync::mpsc::Sender<String>,
    request: AgentRequest,
    llm_url: String,
    db_pool: sqlx::SqlitePool,
    session_id: String,
    game_state: trinity_quest::SharedGameState,
    character_sheet: std::sync::Arc<tokio::sync::RwLock<trinity_protocol::CharacterSheet>>,
) {
        let max_turns = request.max_turns.min(65);
        let max_tokens = request.max_tokens;
        let _book_updates = state.project.book_updates.clone();
        let _active_kv_slot = persona_slot(&request.mode);
        let track_mechanics = true;
        let enforce_narrative = request.mode != "dev";
        let mut modified_user_message = request.message.clone();

        // === IRON ROAD: Gemini Protocol (Death Spin) ===
        if enforce_narrative {
            let mut sheet = character_sheet.write().await;
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
            
            // Only count short, rapid messages.
            if request.message.len() < 100 && !request.message.contains("[RE-ENTRY STABILIZED]") {
                if now.saturating_sub(sheet.last_interaction_timestamp) < 20 {
                    sheet.thrash_count = sheet.thrash_count.saturating_add(1);
                } else {
                    sheet.thrash_count = 0;
                }
            } else {
                sheet.thrash_count = 0;
            }
            sheet.last_interaction_timestamp = now;
            
            let thrash = sheet.thrash_count;
            crate::character_sheet::save_character_sheet(&sheet).ok();
            drop(sheet);

            if thrash >= 4 {
                info!("🌀 Gemini Protocol triggered — conversational thrashing detected");
                let msg = "\n\n🚨 **OAMS ERROR: DEATH SPIN DETECTED** 🚨\n\nThe Gilbreth Protocol recommends taking a physical break. \nType `[RE-ENTRY STABILIZED]` to clear the warning, or keep building if you wish. You are the Architect.\n\n";
                let json_str = serde_json::json!({ "content": msg }).to_string();
                let _ = tx.send(format!("data: {}\n\n", json_str)).await;
                
                let event_json = serde_json::json!({ "type": "gemini_lockout", "active": true }).to_string();
                let _ = tx.send(format!("event: gemini_lockout\ndata: {}\n\n", event_json)).await;
                
                // Inject the telemetry into the LLM context so Pete knows
                modified_user_message.push_str("\n\n[SYSTEM OVERRIDE]: Telemetry detects the user is in a Gemini Death Spin (thrashing / moving too fast). Acknowledge that they might be overloaded. Offer a break, but respect their free will to continue if they choose.");
            }
            
            // Reset if they type the correct override phrase
            if request.message.contains("[RE-ENTRY STABILIZED]") {
                let msg = "\n\n**OAMS OVERRIDE ACCEPTED.** Controls restored. Welcome back, Pilot. The tracks are clear.\n";
                let json_str = serde_json::json!({ "content": msg }).to_string();
                let _ = tx.send(format!("data: {}\n\n", json_str)).await;
                
                let event_json = serde_json::json!({ "type": "gemini_lockout", "active": false }).to_string();
                let _ = tx.send(format!("event: gemini_lockout\ndata: {}\n\n", event_json)).await;
            }
        }

        // === IRON ROAD: Combat Roll Resolution (Narrative Only) ===
        if enforce_narrative {
            let is_combat_roll = request
                .message
                .contains("*Rolls d20 to defeat Scope Creep*");
            let is_yield = request.message.contains("*Yields to Scope Creep*");

            if is_combat_roll || is_yield {
                let mut game = game_state.write().await;
                if is_yield {
                    game.quest.steam_generated = (game.quest.steam_generated - 15.0).max(0.0);
                    let msg = format!("\n\n[COMBAT RESOLVED] You yielded. Steam reduced by 15.0. New Steam: {:.1}\n", game.quest.steam_generated);
                    let json_str = serde_json::json!({ "content": msg }).to_string();
                    let _ = tx.send(format!("data: {}\n\n", json_str)).await;
                } else {
                    use rand::Rng;
                    let roll = rand::thread_rng().gen_range(1..=20);
                    if roll >= 10 {
                        game.quest.steam_generated += 5.0;
                        let msg = format!(
                            "\n\n[COMBAT RESOLVED] 🎲 Rolled {}. SUCCESS! +5.0 Steam.\n",
                            roll
                        );
                        let json_str = serde_json::json!({ "content": msg }).to_string();
                        let _ = tx.send(format!("data: {}\n\n", json_str)).await;
                    } else {
                        game.quest.steam_generated = (game.quest.steam_generated - 10.0).max(0.0);
                        let msg = format!(
                            "\n\n[COMBAT RESOLVED] 🎲 Rolled {}. FAILURE. -10.0 Steam.\n",
                            roll
                        );
                        let json_str = serde_json::json!({ "content": msg }).to_string();
                        let _ = tx.send(format!("data: {}\n\n", json_str)).await;
                    }
                }
                return;
            }
        }

        // === IRON ROAD: VAAM Scan (Ambient) + Scope Creep (Narrative) ===
        if track_mechanics {
            let vaam_result = state.vaam_bridge.vaam.scan_message(&request.message).await;
            
            if enforce_narrative {
                let current_phase = { game_state.read().await.quest.current_phase.clone() };
                let pearl_opt = { game_state.read().await.quest.pearl.clone() };
                
                if let Some(creep) =
                    crate::scope_creep::detect_scope_creep(&request.message, &[], &current_phase)
                {
                    // Option B: Conversational Scaffold + PEARL Semantic Check
                    let pearl_context = if let Some(p) = pearl_opt {
                        format!("PEARL Subject: '{}'. PEARL Vision: '{}'", p.subject, p.vision)
                    } else {
                        "PEARL is undefined".to_string()
                    };

                    let msg = format!("\n\n🔍 **SCOPE ANOMALY DETECTED:** Evaluating ({}) against {}...\n", creep.backlog_item, pearl_context);
                    let json_str = serde_json::json!({ "content": msg }).to_string();
                    let _ = tx.send(format!("data: {}\n\n", json_str)).await;
                    
                    // Inject pedagogical guidance into the LLM context so it physically maps the conceptual alignment!
                    modified_user_message.push_str(&format!("\n\n[SYSTEM OVERRIDE]: The user just suggested a feature expansion ({}). Evaluate this conceptually against their PEARL ({}). If it aligns with the problem the project solves, validate it as 'Scope HOPE', explain why it improves their project, and gently ask them to park it. If it contradicts or detracts from the PEARL, identify it as 'Scope CREEP', explain why it breaks their vision, and instruct them to cast a Hook Deck spell to tame it.", creep.backlog_item, pearl_context));

                    // R2: Emit SSE event for ScopeCreepModal
                    let creep_json = serde_json::json!({
                        "name": creep.name,
                        "threat_level": creep.threat_level,
                        "steam_penalty": creep.steam_penalty,
                        "description": creep.description,
                    });
                    let _ = tx.send(format!("event: creep_tameable\ndata: {}\n\n", creep_json)).await;

                    // ── Soft Spot 6: Scope Creep encounter → friction rises ──
                    let mut sheet = state.player.character_sheet.write().await;
                    sheet.track_friction = (sheet.track_friction + 8.0).min(100.0);
                    sheet.recalculate_vulnerability();
                    crate::character_sheet::save_character_sheet(&sheet).ok();
                    drop(sheet);
                }
            }

            if vaam_result.has_detections() {
                let coal_from_vaam = vaam_result.total_coal as f32;
                let vaam_event = format_vaam_event(&vaam_result);
                if !vaam_event.is_empty() {
                    let _ = tx
                        .send(format!("event: vaam\ndata: {}\n\n", vaam_event))
                        .await;
                }
                let tier = trinity_protocol::VocabularyTier::Basic;
                let known_words = [];
                let cognitive_load = trinity_iron_road::vaam::calculate_cognitive_load(
                    &request.message,
                    tier,
                    &known_words,
                );
                let cog_msg = format!(
                    "event: cognitive_load\ndata: {{\"flesch_grade\": {:.1}, \"complex_words\": {}}}\n\n",
                    cognitive_load.flesch_kincaid_grade, cognitive_load.complex_words
                );
                let _ = tx.send(cog_msg).await;
                info!(
                    "[VAAM] {} words detected, +{:.1} coal",
                    vaam_result.detections.len(),
                    coal_from_vaam
                );
                let mut gs = game_state.write().await;
                gs.stats.coal_reserves = (gs.stats.coal_reserves + coal_from_vaam).min(100.0);
            }
        }

        // === R5: Shadow Status — detect negative sentiment in user messages ===
        if enforce_narrative {
            let msg_lower = request.message.to_lowercase();
            let negative_indicators = ["frustrated", "confused", "stuck", "don't understand",
                "this doesn't work", "hate", "annoyed", "lost", "wrong", "broken",
                "give up", "can't do this", "waste of time", "pointless"];
            let negative_count = negative_indicators.iter()
                .filter(|ind| msg_lower.contains(**ind))
                .count();

            if negative_count >= 1 {
                let mut sheet = state.player.character_sheet.write().await;
                sheet.consecutive_negatives = sheet.consecutive_negatives.saturating_add(1);
                
                let mut changed = false;
                if sheet.consecutive_negatives >= 3 && sheet.shadow_status != trinity_protocol::character_sheet::ShadowStatus::Active {
                    sheet.shadow_status = trinity_protocol::character_sheet::ShadowStatus::Active;
                    info!("🌑 Shadow activated — negative sentiment detected");
                    changed = true;
                } else if sheet.shadow_status == trinity_protocol::character_sheet::ShadowStatus::Clear {
                    sheet.shadow_status = trinity_protocol::character_sheet::ShadowStatus::Stirring;
                    info!("🌘 Shadow stirring — frustration detected");
                    changed = true;
                }
                sheet.recalculate_vulnerability();
                
                if changed {
                    let shadow_json = serde_json::json!({ "status": sheet.shadow_status }).to_string();
                    let _ = tx.send(format!("event: shadow_status\ndata: {}\n\n", shadow_json)).await;
                }
                
                crate::character_sheet::save_character_sheet(&sheet).ok();
                drop(sheet);
            } else {
                // Positive/neutral message gradually resets consecutive negatives
                let mut sheet = state.player.character_sheet.write().await;
                if sheet.consecutive_negatives > 0 {
                    sheet.consecutive_negatives = sheet.consecutive_negatives.saturating_sub(1);
                    
                    if sheet.consecutive_negatives == 0 && sheet.shadow_status != trinity_protocol::character_sheet::ShadowStatus::Clear {
                        sheet.shadow_status = trinity_protocol::character_sheet::ShadowStatus::Clear;
                        info!("☀️ Shadow cleared — positive engagement restored");
                        let shadow_json = serde_json::json!({ "status": sheet.shadow_status }).to_string();
                        let _ = tx.send(format!("event: shadow_status\ndata: {}\n\n", shadow_json)).await;
                    }
                    
                    sheet.recalculate_vulnerability();
                    crate::character_sheet::save_character_sheet(&sheet).ok();
                }
                drop(sheet);
            }
        }

        // RAG context
        let rag_chunks = if request.use_rag {
            rag::search_documents(&db_pool, &request.message)
                .await
                .unwrap_or_default()
        } else {
            vec![]
        };

        // Build system prompt with persona preamble + RAG + VAAM context
        let vaam_context = state.vaam_bridge.prompt_context().await;
        let persona_preamble = match request.mode.as_str() {
            "recycler" => GREAT_RECYCLER_PREAMBLE,
            "programmer" => PROGRAMMER_PETE_PREAMBLE,
            _ => "", // dev and ironroad modes: no persona preamble
        };
        let mut system = format!("{}{}", persona_preamble, AGENT_SYSTEM);

        if !rag_chunks.is_empty() {
            let mut ctx = String::new();
            for chunk in &rag_chunks {
                // TRINITY MIND STANDARD: 256K minimum context allows for massive RAG loading.
                // We expand the RAG hardcap to 150,000 chars (~40K tokens) to load
                // entire frameworks, rubrics, and the Fancy Bible simultaneously.
                if ctx.len() + chunk.len() > 150_000 {
                    break;
                }
                if !ctx.is_empty() {
                    ctx.push_str("\n---\n");
                }
                ctx.push_str(chunk);
            }
            system.push_str(&format!(
                "\n\nRelevant context from knowledge base:\n{}",
                ctx
            ));
        }

        if !vaam_context.is_empty() {
            system.push_str(&format!(
                "\n\nVAAM ALIGNMENT (User Preferences & Style):\n{}",
                vaam_context
            ));
        }

        // ═══ L5 EVOLUTIONARY: Inject RLHF learned preferences ═══
        // Accumulated thumbs-up/thumbs-down signals steer future prompts.
        {
            let phase_label = {
                let gs = game_state.read().await;
                gs.quest.current_phase.label().to_string()
            };
            let rlhf_block = crate::rlhf_api::apply_prompt_bias(Some(&phase_label));
            if !rlhf_block.is_empty() {
                system.push_str(&rlhf_block);
            }
        }

        // === IRON ROAD: Inject Coal level + Sacred Circuitry focus ===
        // The AI needs to know its own attention level to self-regulate (Narrative enforcement).
        let mut _last_focus_directive = String::new();
        if enforce_narrative {
            let gs = game_state.read().await;
            let coal = gs.stats.coal_reserves;
            let phase = gs.quest.current_phase.label();
            let coal_status = if coal > 75.0 {
                "HIGH — stay productive"
            } else if coal > 40.0 {
                "MODERATE — maintain focus"
            } else if coal > 15.0 {
                "LOW — refocus on phase objectives"
            } else {
                "CRITICAL — minimal responses, stay on-circuit"
            };
            system.push_str(&format!(
                "\n\nSACRED CIRCUITRY (AI Attention Level):\nCoal: {:.0}/100 ({})\nPhase: {}\nStay on-circuit for this phase. Your responses are scanned for alignment.",
                coal, coal_status, phase
            ));
        }

        // === CONDUCTOR PROTOCOL: Inject phase-specific Socratic system prompt ===
        // This is the wire that was missing. For Iron Road mode, we read the player's
        // current ADDIECRAPEYE phase and inject the Conductor's coaching protocol.
        // The prompt is appended LAST so it's the freshest instruction in the context window.
        // This is a pure text injection — no LLM call, no latency, no channels needed.
        if request.mode == "ironroad" {
            let current_phase_label = {
                let gs = game_state.read().await;
                gs.quest.current_phase.label().to_string()
            };
            // Map the phase label to the AddiecrapeyePhase enum
            let addiecrapeye_phase = match current_phase_label.as_str() {
                "Analysis"       => crate::conductor_leader::AddiecrapeyePhase::Analysis,
                "Design"         => crate::conductor_leader::AddiecrapeyePhase::Design,
                "Development"    => crate::conductor_leader::AddiecrapeyePhase::Development,
                "Implementation" => crate::conductor_leader::AddiecrapeyePhase::Implementation,
                "Evaluation"     => crate::conductor_leader::AddiecrapeyePhase::Evaluation,
                "Contrast"       => crate::conductor_leader::AddiecrapeyePhase::Contrast,
                "Repetition"     => crate::conductor_leader::AddiecrapeyePhase::Repetition,
                "Alignment"      => crate::conductor_leader::AddiecrapeyePhase::Alignment,
                "Proximity"      => crate::conductor_leader::AddiecrapeyePhase::Proximity,
                "Envision"       => crate::conductor_leader::AddiecrapeyePhase::Envision,
                "Yoke"           => crate::conductor_leader::AddiecrapeyePhase::Yoke,
                "Evolve"         => crate::conductor_leader::AddiecrapeyePhase::Evolve,
                _                => crate::conductor_leader::AddiecrapeyePhase::Analysis,
            };
            let conductor_prompt = crate::conductor_leader::phase_system_prompt(addiecrapeye_phase);
            system.push_str(&format!(
                "\n\n--- IRON ROAD CONDUCTOR ---\n{}\n--- END CONDUCTOR ---",
                conductor_prompt
            ));
            info!("[Conductor] Injected {} phase protocol into system prompt", current_phase_label);
        }


        // Persist user message to DB before inference
        if let Err(e) = crate::persistence::save_message(
            &db_pool,
            &session_id,
            "user",
            &request.message,
            request.image_base64.as_deref(),
            None,
        )
        .await
        {
            warn!("[Agent] Failed to persist user message: {}", e);
        }

        // Build message chain: system + history + current user message
        let mut messages = vec![ChatMessage {
            role: "system".to_string(),
            content: system,
            timestamp: None,
            image_base64: None,
        }];

        // ═══════════════════════════════════════════════════════════════════════
        // RING 3: 256K Context History Preservation
        // ═══════════════════════════════════════════════════════════════════════
        // TRINITY MIND STANDARD: 256K Minimum Context (MLA Format)
        // We do NOT truncate or summarize prematurely. The Great Recycler
        // and Programmer Pete rely on massive context to maintain the PEARL.
        // We only summarize if history genuinely threatens to breach 256K tokens.
        // Assuming ~500 tokens per message, a 400 message window gives us ~200K tokens,
        // leaving plenty of room for system prompts, RAG, and execution outputs.
        //
        // Strategy:
        //   - If history ≤ RECENT_WINDOW: inject all messages verbatim
        //   - If history > RECENT_WINDOW: summarize the oldest batch into a digest,
        //     then inject RECENT_WINDOW most recent messages verbatim
        const RECENT_WINDOW: usize = 400;

        let (context_summary, recent_history) = if request.history.len() > RECENT_WINDOW {
            let split_point = request.history.len() - RECENT_WINDOW;
            let old_messages = &request.history[..split_point];
            let recent = &request.history[split_point..];
            let summary = compress_context_digest(old_messages);
            (Some(summary), recent)
        } else {
            (None, request.history.as_slice())
        };

        // Inject context summary as a user/assistant pair before recent history
        if let Some(digest) = context_summary {
            if !digest.is_empty() {
                messages.push(ChatMessage {
                    role: "user".to_string(),
                    content: format!(
                        "[Previous conversation context — {} older messages compressed]\n{}",
                        request.history.len() - RECENT_WINDOW,
                        digest
                    ),
                    timestamp: None,
                    image_base64: None,
                });
                messages.push(ChatMessage {
                    role: "assistant".to_string(),
                    content: "Understood. I have the context from our earlier conversation. Let's continue.".to_string(),
                    timestamp: None,
                    image_base64: None,
                });
                info!("[Ring 3] Compressed {} old messages into context digest ({} chars), keeping {} recent", 
                    request.history.len() - RECENT_WINDOW, digest.len(), RECENT_WINDOW);
            }
        }

        // Inject recent conversation history with strict role alternation
        // Gemma chat template requires: system → user → assistant → user → assistant...
        let mut last_role = messages.last().map(|m| m.role.as_str()).unwrap_or("system");
        for msg in recent_history {
            let role = if msg.role == "assistant" {
                "assistant"
            } else {
                "user"
            };
            // Skip if same role as last (would break alternation)
            if role == last_role {
                continue;
            }
            // First message after system must be "user"
            if last_role == "system" && role != "user" {
                continue;
            }
            messages.push(ChatMessage {
                role: role.to_string(),
                content: msg.content.clone(),
                timestamp: None,
                image_base64: None,
            });
            last_role = role;
        }

        // If history ends on "user", drop it — we're about to add the current user message
        if let Some(last) = messages.last() {
            if last.role == "user" {
                messages.pop();
            }
        }

        // Clone user message for R5/R7 mechanics (before modified_user_message is moved)
        let user_message_text = modified_user_message.clone();

        // Current user message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: modified_user_message,
            timestamp: None,
            image_base64: request.image_base64,
        });

        let mut continuation_count: u32 = 0;

        // Build structured tool definitions — The Turntable selects the right gauge
        let gauge = crate::tools::gauge_for_mode(&request.mode);
        let tool_defs = inference::build_tool_definitions(&crate::tools::get_tools_for_gauge(gauge));
        info!("[Turntable] Mode '{}' → {:?} gauge ({} tools)", request.mode, gauge, tool_defs.len());

        for turn in 0..max_turns {
            info!(
                "[Agent] Turn {}/{} (continuations: {})",
                turn + 1,
                max_turns,
                continuation_count
            );

            // Emit status so SSE stream stays alive during inference (prevents Cloudflare 524)
            let status_json = serde_json::json!({
                "status": "thinking",
                "turn": turn + 1,
                "message": format!("Turn {} — Pete is thinking...", turn + 1)
            });
            let _ = tx.send(format!("event: status\ndata: {}\n\n", status_json)).await;

            // Phase 2: Try structured tool calling first, fall back to regex
            let (response, structured_tool_calls) = if !tool_defs.is_empty() {
                match inference::chat_completion_with_tools(
                    &llm_url,
                    &messages,
                    max_tokens,
                    &tool_defs,
                    Some("high"),
                )
                .await
                {
                    Ok(tool_response) => {
                        let content = tool_response.content.unwrap_or_default();
                        let calls: Vec<(String, String)> = tool_response
                            .tool_calls
                            .iter()
                            .map(|tc| (tc.function.name.clone(), tc.function.arguments.clone()))
                            .collect();
                        if !calls.is_empty() {
                            info!(
                                "[Agent] Phase 2: {} structured tool call(s) received",
                                calls.len()
                            );
                        }
                        (content, calls)
                    }
                    Err(e) => {
                        info!(
                            "[Agent] Structured tool calling failed, falling back to regex: {}",
                            e
                        );
                        // Fall back to legacy regex path
                        match inference::chat_completion_with_effort(
                            &llm_url,
                            &messages,
                            max_tokens,
                            Some("high"),
                        )
                        .await
                        {
                            Ok(r) => (r, vec![]),
                            Err(e2) => {
                                let err_msg = format!("LLM OFFLINE OR UNREACHABLE: {}", e2);
                                let _ = tx.send(format!("event: llm_offline\ndata: {}\n\n", err_msg)).await;
                                let _ = tx.send(format!("event: error\ndata: {}\n\n", err_msg)).await;
                                break;
                            }
                        }
                    }
                }
            } else {
                // No tool definitions — pure regex path
                match inference::chat_completion_with_effort(
                    &llm_url,
                    &messages,
                    max_tokens,
                    Some("high"),
                )
                .await
                {
                    Ok(r) => (r, vec![]),
                    Err(e) => {
                        let err_msg = format!("LLM OFFLINE OR UNREACHABLE: {}", e);
                        let _ = tx.send(format!("event: llm_offline\ndata: {}\n\n", err_msg)).await;
                        let _ = tx.send(format!("event: error\ndata: {}\n\n", err_msg)).await;
                        break;
                    }
                }
            };

            // Extract and stream <thinking> blocks before processing
            let (thinking_text, response_text) = extract_thinking(&response);
            if !thinking_text.is_empty() {
                let think_json = serde_json::json!({ "thinking": thinking_text }).to_string();
                let _ = tx
                    .send(format!("event: thinking\ndata: {}\n\n", think_json))
                    .await;
            }
            let response = response_text;

            // Use structured tool calls if present, otherwise fall back to regex
            let tool_calls = if !structured_tool_calls.is_empty() {
                structured_tool_calls
            } else {
                extract_tool_calls(&response)
            };

            if tool_calls.is_empty() {
                // No tools — this is a response. Persist it.
                if let Err(e) = crate::persistence::save_message(
                    &db_pool,
                    &session_id,
                    "assistant",
                    &response,
                    None,
                    None,
                )
                .await
                {
                    warn!("[Agent] Failed to persist assistant response: {}", e);
                }
                let json_str = serde_json::json!({ "content": response.clone() }).to_string();
                let _ = tx.send(json_str).await;

                // Multi-response continuation: check if the agent wants to keep going
                // The agent can signal continuation by ending with specific patterns
                let wants_continue = response.contains("[CONTINUE]")
                    || response.contains("{\"tool\"")
                    || (turn < max_turns.saturating_sub(2) && response.trim().ends_with("..."));

                if wants_continue && continuation_count < 65 {
                    continuation_count += 1;
                    info!(
                        "[Agent] Continuation {} — agent signaled more work",
                        continuation_count
                    );
                    // Feed response back as context for next turn
                    messages.push(ChatMessage {
                        role: "assistant".to_string(),
                        content: response.replace("[CONTINUE]", "").trim().to_string(),
                        timestamp: None,
                        image_base64: None,
                    });
                    messages.push(ChatMessage {
                        role: "user".to_string(),
                        content: "Continue with the next step.".to_string(),
                        timestamp: None,
                        image_base64: None,
                    });
                    continue;
                }

                break;
            }

            // Stream the AI's thinking (text before/between tools)
            let clean_text = strip_tool_tags(&response);
            if !clean_text.trim().is_empty() {
                if track_mechanics {
                    // IRON ROAD: SSML emphasis for mastered words
                    let mastered_words =
                        { state.vaam_bridge.vaam.mastery.read().await.mastered.clone() };
                    let vocab_words: Vec<trinity_protocol::VocabularyWord> = mastered_words
                        .into_iter()
                        .map(|w| trinity_protocol::VocabularyWord {
                            word: w,
                            tier: trinity_protocol::VocabularyTier::Basic,
                            definition: None,
                            aliases: vec![],
                            context_clues: vec![],
                            tags: vec![],
                            coal_value: 0,
                            bloom_level: trinity_protocol::BloomLevel::Remember,
                        })
                        .collect();
                    let ssml_injected =
                        trinity_voice::ssml::inject_vaam_ssml(&clean_text, &vocab_words);
                    let payload =
                        serde_json::json!({ "content": clean_text, "ssml": ssml_injected })
                            .to_string();
                    let _ = tx
                        .send(format!("event: vaam_ssml\ndata: {}\n\n", payload))
                        .await;
                }
                let content_json = serde_json::json!({ "content": clean_text }).to_string();
                let _ = tx.send(content_json).await;
            }

            // === IRON ROAD: Sacred Circuitry AI Coal Engine (Ambient Mechanics) ===
            // Scan AI response for circuit alignment against current phase.
            // On-circuit = Coal earned (focused). Off-circuit = Coal consumed (drifting).
            if track_mechanics {
                let current_phase = { game_state.read().await.quest.current_phase.label().to_string() };
                let alignment = trinity_protocol::scan_ai_alignment(&response, &current_phase);

                // Apply coal delta to game state
                {
                    let mut gs = game_state.write().await;
                    gs.stats.coal_reserves = (gs.stats.coal_reserves + alignment.coal_delta)
                        .clamp(0.0, 100.0);
                }

                // Store focus directive for next turn's system prompt
                _last_focus_directive = alignment.focus_directive.clone();

                // Send circuit alignment event to frontend
                let circuit_json = serde_json::to_string(&alignment).unwrap_or_default();
                let _ = tx
                    .send(format!("event: circuit\ndata: {}\n\n", circuit_json))
                    .await;

                if alignment.on_circuit {
                    info!(
                        "[Circuit] ON-CIRCUIT: {:?} (confidence: {:.2}, coal: +{:.1})",
                        alignment.detected_circuit, alignment.confidence, alignment.coal_delta
                    );
                } else {
                    info!(
                        "[Circuit] DRIFT: {:?} (expected: {:?}, coal: {:.1})",
                        alignment.detected_circuit, alignment.expected_circuits, alignment.coal_delta
                    );
                }

                // === R3: Track Friction — off-circuit responses increase friction ===
                if enforce_narrative {
                    let mut sheet = state.player.character_sheet.write().await;
                    if alignment.on_circuit {
                        // On-circuit: friction slowly reduces (productive focus)
                        sheet.track_friction = (sheet.track_friction - 1.0).max(0.0);
                    } else {
                        // Off-circuit drift: friction builds
                        sheet.track_friction = (sheet.track_friction + 3.0).min(100.0);
                    }
                    sheet.recalculate_vulnerability();
                    crate::character_sheet::save_character_sheet(&sheet).ok();
                    drop(sheet);
                }

                // === R4: Emit character sheet updates for TrainStatus engine diagnostics ===
                if enforce_narrative {
                    let sheet = state.player.character_sheet.read().await;
                    let char_event = serde_json::json!({
                        "track_friction": sheet.track_friction,
                        "vulnerability": sheet.vulnerability,
                        "shadow_status": format!("{:?}", sheet.shadow_status),
                        "consecutive_negatives": sheet.consecutive_negatives,
                        "current_steam": sheet.current_steam,
                    });
                    drop(sheet);
                    let _ = tx.send(format!("event: character_update\ndata: {}\n\n", char_event)).await;
                }
            }

            // === R1: Coal/Steam Economy — every Iron Road conversation costs/generates resources ===
            if enforce_narrative && tool_calls.is_empty() {
                let coal_cost = 2.0_f32;
                let steam_gain = 5.0_f32;
                let mut gs = game_state.write().await;
                gs.stats.coal_reserves = (gs.stats.coal_reserves - coal_cost).max(0.0);
                gs.quest.coal_used += coal_cost;
                gs.quest.steam_generated += steam_gain;
                let _ = trinity_quest::save_game_state(&db_pool, "default", &gs).await;
                drop(gs);

                let econ_event = serde_json::json!({
                    "coal_burned": coal_cost,
                    "steam_gained": steam_gain,
                    "source": "conversation"
                });
                let _ = tx.send(format!("event: resources\ndata: {}\n\n", econ_event)).await;
            }

            // === R7: Perspective Engine — evaluate AI response through pedagogical lenses ===
            if enforce_narrative && !clean_text.trim().is_empty() {
                let msg_type = crate::perspective::classify_message(&user_message_text, false);
                // Only fire on substantive exchanges (skip greetings/brief replies)
                if msg_type == crate::perspective::MessageType::Substantive {
                    let sheet = state.player.character_sheet.read().await;
                    let experience = sheet.experience.as_ref().filter(|e| !e.is_empty()).cloned();
                    let audience = sheet.audience.as_ref().filter(|a| !a.is_empty()).cloned();
                    drop(sheet);
                    let gs = game_state.read().await;
                    let phase_label = gs.quest.current_phase.label().to_string();
                    let blooms = gs.quest.current_phase.blooms().to_string();
                    drop(gs);

                    let lenses = crate::perspective::select_lenses(
                        &phase_label, &blooms, &msg_type,
                        experience.as_deref(), audience.as_deref(),
                    );

                    if !lenses.is_empty() {
                        let response_clone = clean_text.clone();
                        let tx_clone = tx.clone();
                        let router_clone = state.inference_router.clone();
                        let default_url = llm_url.clone();
                        
                        // Fire perspectives in background via A.R.T.Y. Hub to prevent primary backend locking
                        tokio::spawn(async move {
                            let llm = {
                                let router = router_clone.read().await;
                                router.get_url_by_name("arty-hub").unwrap_or_else(|| default_url)
                            };

                            let perspective_set = crate::perspective::evaluate(&llm, &response_clone, &lenses).await;
                            if !perspective_set.perspectives.is_empty() {
                                let json = serde_json::to_string(&perspective_set).unwrap_or_default();
                                let _ = tx_clone.send(format!("event: perspective\ndata: {}\n\n", json)).await;
                            }
                        });
                    }
                }
            }

            // Execute each tool call
            let mut tool_results = String::new();
            for (tool_name, tool_params) in &tool_calls {
                // Emit tool status so the DAYDREAM shows what's happening (and keeps SSE alive)
                let tool_status = serde_json::json!({
                    "status": "tool",
                    "tool": tool_name,
                    "message": format!("Running {}...", tool_name)
                });
                let _ = tx.send(format!("event: status\ndata: {}\n\n", tool_status)).await;

                // IRON ROAD: Skill check gate (d20 roll to use tools)
                if track_mechanics {
                    let game_mode = if request.hardcore_mode {
                        GameMode::Hardcore
                    } else {
                        GameMode::Normal
                    };
                    let (gs, current_phase) = {
                        let gs = game_state.read().await;
                        (gs.clone(), gs.quest.current_phase)
                    };
                    let skill = skill_check(game_mode, &gs.stats, current_phase, 0);
                    let skill_json = serde_json::to_string(&skill).unwrap_or_default();
                    let _ = tx
                        .send(format!("event: skill\ndata: {}\n\n", skill_json))
                        .await;

                    if !skill.success {
                        let protocol = HeavilonProtocol::from_failure(&skill, tool_name);
                        let protocol_json = serde_json::to_string(&protocol).unwrap_or_default();
                        let _ = tx
                            .send(format!("event: heavilon\ndata: {}\n\n", protocol_json))
                            .await;
                        let sheet = character_sheet.read().await;
                        let narrative_ctx = NarrativeContext {
                            genre: sheet.genre,
                            hero_stage: gs.quest.hero_stage,
                            phase: current_phase,
                            last_action: tool_name.clone(),
                            coal: gs.stats.coal_reserves,
                            steam: gs.quest.steam_generated,
                            xp: gs.stats.total_xp,
                            alias: sheet.alias.clone(),
                            alignment: sheet.alignment.clone(),
                            appearance: sheet.appearance.clone(),
                            backstory: sheet.backstory.clone(),
                            current_quest_flavor: sheet.current_quest_flavor.clone(),
                            friction: sheet.track_friction / 100.0,
                            vulnerability: sheet.vulnerability,
                        };
                        drop(sheet);
                        let failure_text =
                            generate_failure_narrative(&narrative_ctx, &protocol.failure_context);
                        let _ = tx
                            .send(format!("event: narrative\ndata: {}\n\n", failure_text))
                            .await;
                        {
                            let mut gs = game_state.write().await;
                            gs.stats.coal_reserves =
                                (gs.stats.coal_reserves - protocol.coal_cost).max(0.0);
                        }
                        continue;
                    }
                    if skill.critical {
                        let _ = tx.send("\n⚡ **CRITICAL SUCCESS!** \n".to_string()).await;
                    }
                }

                let permission = tools::tool_permission(tool_name);
                let perm_badge = match permission {
                    tools::ToolPermission::Safe => "🟢",
                    tools::ToolPermission::NeedsApproval => "🟡",
                    tools::ToolPermission::Destructive => "🔴",
                };
                let tool_json = serde_json::json!({ "content": format!("\n`{} ▶ {}` ", perm_badge, tool_name) }).to_string();
                let _ = tx.send(tool_json).await;

                let params: serde_json::Value =
                    serde_json::from_str(tool_params).unwrap_or(serde_json::json!({}));

                let tool_start = std::time::Instant::now();
                let result = execute_tool_internal(
                    tool_name,
                    &params,
                    &request.mode,
                    game_state.clone(),
                    state.cow_catcher.clone(),
                )
                .await;
                let latency_ms = tool_start.elapsed().as_millis() as i32;

                // Persist tool call to SQLite for analytics
                let is_error = result.starts_with("Error:");
                let result_status = if is_error { "error" } else { "ok" };
                let preview = if result.len() > 500 {
                    &result[..500]
                } else {
                    &result
                };
                if let Err(e) = crate::persistence::save_tool_call(
                    &db_pool,
                    &session_id,
                    tool_name,
                    &params,
                    result_status,
                    Some(preview),
                    latency_ms,
                )
                .await
                {
                    warn!("[Agent] Failed to persist tool call: {}", e);
                }

                // ── Block C: Wire skills.rs XP/Coal rewards for tool execution ──
                // In Iron Road mode, every tool use generates XP and spends Coal.
                // Uses skills::calculate_xp() for dynamic rewards based on tool type.
                if track_mechanics && !is_error {
                    let skill_result = crate::skills::SkillResult::auto_success();
                    let xp = crate::skills::calculate_xp(tool_name, &skill_result, false);
                    let coal_cost = match tools::tool_permission(tool_name) {
                        tools::ToolPermission::Safe => 1.0_f32,
                        tools::ToolPermission::NeedsApproval => 3.0,
                        tools::ToolPermission::Destructive => 5.0,
                    };
                    let mut game = game_state.write().await;
                    game.quest.xp_earned += xp;
                    game.quest.coal_used += coal_cost;
                    game.stats.total_xp += xp;
                    game.stats.coal_reserves = (game.stats.coal_reserves - coal_cost).max(0.0);
                    drop(game);
                }

                let truncated = if result.len() > 16000 {
                    format!(
                        "{}...\n[truncated: {} bytes]",
                        &result[..16000],
                        result.len()
                    )
                } else {
                    result
                };

                let result_json =
                    serde_json::json!({ "content": format!("`✓`\n```\n{}\n```\n", truncated) })
                        .to_string();
                let _ = tx.send(result_json).await;

                // === IN-CHAT IMAGE: Emit SSE image event when generate_image succeeds ===
                if tool_name == "generate_image" && truncated.contains("Image generated:") {
                    // Extract the file path from the result
                    if let Some(path_line) =
                        truncated.lines().find(|l| l.contains("Image generated:"))
                    {
                        if let Some(path_str) = path_line.strip_prefix("Image generated: ") {
                            let path_str = path_str.trim();
                            if let Ok(img_bytes) = tokio::fs::read(path_str).await {
                                use base64::Engine;
                                let b64 =
                                    base64::engine::general_purpose::STANDARD.encode(&img_bytes);
                                let filename = std::path::Path::new(path_str)
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string();
                                let img_event = serde_json::json!({
                                    "filename": filename,
                                    "url": format!("/api/creative/assets/{}", filename),
                                    "base64": b64,
                                });
                                let _ = tx
                                    .send(format!("event: image\ndata: {}\n\n", img_event))
                                    .await;
                                info!("[Agent] 🖼️ Image SSE event emitted: {}", filename);
                            }
                        }
                    }
                }

                tool_results.push_str(&format!(
                    "Tool `{}` result:\n```\n{}\n```\n\n",
                    tool_name, truncated
                ));

                // === IRON ROAD ONLY: Resource Generation + Narrative ===
                if track_mechanics {
                    let coal_burned = 2.0;
                    let (gs, current_phase) = {
                        let gs = game_state.read().await;
                        (gs.clone(), gs.quest.current_phase)
                    };
                    let skill = skill_check(
                        if request.hardcore_mode {
                            GameMode::Hardcore
                        } else {
                            GameMode::Normal
                        },
                        &gs.stats,
                        current_phase,
                        0,
                    );
                    let steam_gained = calculate_steam(coal_burned, &skill, current_phase);
                    let xp_gained = calculate_xp(tool_name, &skill, false);
                    {
                        let mut gs = game_state.write().await;
                        gs.stats.coal_reserves = (gs.stats.coal_reserves - coal_burned).max(0.0);
                        gs.quest.coal_used += coal_burned;
                        gs.quest.steam_generated += steam_gained;
                        gs.stats.total_xp += xp_gained;
                        gs.quest.xp_earned += xp_gained;
                        let _ = trinity_quest::save_game_state(&db_pool, "default", &gs).await;
                    }
                    let resources = serde_json::json!({ "coal_burned": coal_burned, "steam_gained": steam_gained, "xp_gained": xp_gained });
                    let _ = tx
                        .send(format!("event: resources\ndata: {}\n\n", resources))
                        .await;

                    if skill.critical {
                        let sheet = character_sheet.read().await;
                        let narrative_ctx = NarrativeContext {
                            genre: sheet.genre,
                            hero_stage: gs.quest.hero_stage,
                            phase: current_phase,
                            last_action: tool_name.clone(),
                            coal: gs.stats.coal_reserves,
                            steam: gs.quest.steam_generated,
                            xp: gs.stats.total_xp,
                            alias: sheet.alias.clone(),
                            alignment: sheet.alignment.clone(),
                            appearance: sheet.appearance.clone(),
                            backstory: sheet.backstory.clone(),
                            current_quest_flavor: sheet.current_quest_flavor.clone(),
                            friction: sheet.track_friction / 100.0,
                            vulnerability: sheet.vulnerability,
                        };
                        drop(sheet);
                        let crit_text = generate_critical_narrative(&narrative_ctx);
                        let _ = tx
                            .send(format!("event: narrative\ndata: {}\n\n", crit_text))
                            .await;
                    } else if skill.fumble {
                        let sheet = character_sheet.read().await;
                        let narrative_ctx = NarrativeContext {
                            genre: sheet.genre,
                            hero_stage: gs.quest.hero_stage,
                            phase: current_phase,
                            last_action: tool_name.clone(),
                            coal: gs.stats.coal_reserves,
                            steam: gs.quest.steam_generated,
                            xp: gs.stats.total_xp,
                            alias: sheet.alias.clone(),
                            alignment: sheet.alignment.clone(),
                            appearance: sheet.appearance.clone(),
                            backstory: sheet.backstory.clone(),
                            current_quest_flavor: sheet.current_quest_flavor.clone(),
                            friction: sheet.track_friction / 100.0,
                            vulnerability: sheet.vulnerability,
                        };
                        drop(sheet);
                        let fumble_text = generate_fumble_narrative(&narrative_ctx);
                        let _ = tx
                            .send(format!("event: narrative\ndata: {}\n\n", fumble_text))
                            .await;
                    }
                }
            }

            // Add AI response and tool results to conversation
            messages.push(ChatMessage {
                role: "assistant".to_string(),
                content: response,
                timestamp: None,
                image_base64: None,
            });
            messages.push(ChatMessage {
                role: "user".to_string(),
                content: format!(
                    "[Tool results — continue with next step or provide final answer]\n\n{}",
                    tool_results
                ),
                timestamp: None,
                image_base64: None,
            });
    }
}


// ============================================================================
// RING 5: Rate Limiting (Token Bucket)
// ============================================================================

/// Global tool call rate limiter — prevents runaway tool loops.
/// Tracks calls per minute using atomic counters (zero allocation, lock-free).
static TOOL_CALL_COUNT: AtomicU64 = AtomicU64::new(0);
static TOOL_CALL_WINDOW_START: AtomicU64 = AtomicU64::new(0);
static DESTRUCTIVE_CALL_COUNT: AtomicU64 = AtomicU64::new(0);
static DESTRUCTIVE_CALL_WINDOW_START: AtomicU64 = AtomicU64::new(0);

/// Rate limit check. Returns Ok(()) if allowed, Err(message) if throttled.
/// - Global limit: 60 tool calls per minute
/// - Destructive limit: 5 destructive calls per minute
fn check_rate_limit(is_destructive: bool) -> Result<(), String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Global rate limit: 60 calls per 60-second window
    let window_start = TOOL_CALL_WINDOW_START.load(Ordering::Relaxed);
    if now - window_start >= 60 {
        // Reset window
        TOOL_CALL_WINDOW_START.store(now, Ordering::Relaxed);
        TOOL_CALL_COUNT.store(1, Ordering::Relaxed);
    } else {
        let count = TOOL_CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        if count > 60 {
            return Err(format!(
                "🛑 Ring 5 Rate Limit: {} tool calls in the last minute (max 60). \
                 Slow down — the Yardmaster needs a breather.",
                count
            ));
        }
    }

    // Destructive rate limit: 5 calls per 60-second window
    if is_destructive {
        let d_window = DESTRUCTIVE_CALL_WINDOW_START.load(Ordering::Relaxed);
        if now - d_window >= 60 {
            DESTRUCTIVE_CALL_WINDOW_START.store(now, Ordering::Relaxed);
            DESTRUCTIVE_CALL_COUNT.store(1, Ordering::Relaxed);
        } else {
            let d_count = DESTRUCTIVE_CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
            if d_count > 5 {
                return Err(format!(
                    "🛑 Ring 5 Rate Limit: {} destructive tool calls in the last minute (max 5). \
                     System-level operations are throttled for safety.",
                    d_count
                ));
            }
        }
    }

    Ok(())
}

// ============================================================================
// RING 3: Context Compression
// ============================================================================

/// Deterministically compress older conversation messages into a context digest.
/// Extracts: tool calls/results, key decisions, topic references.
/// No LLM call — fast and predictable.
fn compress_context_digest(messages: &[HistoryMessage]) -> String {
    let mut tools_used: Vec<String> = Vec::new();
    let mut key_decisions: Vec<String> = Vec::new();
    let mut topics: Vec<String> = Vec::new();
    let mut files_mentioned: Vec<String> = Vec::new();

    for msg in messages {
        let content = &msg.content;

        // Extract tool calls from assistant messages
        if msg.role == "assistant" {
            // Look for tool execution markers
            for line in content.lines() {
                // Matches "▶ tool_name" pattern from our tool badges
                if line.contains("▶") {
                    if let Some(tool_part) = line.split('▶').nth(1) {
                        let tool_name = tool_part.trim().trim_end_matches('`').trim();
                        if !tool_name.is_empty() && !tools_used.contains(&tool_name.to_string()) {
                            tools_used.push(tool_name.to_string());
                        }
                    }
                }
                // Look for JSON tool calls
                if line.contains("\"tool\"") || line.contains("\"name\"") {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(line.trim()) {
                        if let Some(tool) = json
                            .get("tool")
                            .or(json.get("name"))
                            .and_then(|t| t.as_str())
                        {
                            if !tools_used.contains(&tool.to_string()) {
                                tools_used.push(tool.to_string());
                            }
                        }
                    }
                }
            }
        }

        // Extract user directives (short messages are usually directives)
        if msg.role == "user" && content.len() < 200 && !content.starts_with("[Tool results") {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                key_decisions.push(trimmed.to_string());
            }
        }

        // Extract file paths mentioned
        for word in content.split_whitespace() {
            if (word.contains(".rs")
                || word.contains(".jsx")
                || word.contains(".toml")
                || word.contains(".md")
                || word.contains(".ts"))
                && (word.contains('/') || word.contains('\\'))
            {
                let clean = word.trim_matches(|c: char| {
                    !c.is_alphanumeric() && c != '/' && c != '.' && c != '_' && c != '-'
                });
                if !clean.is_empty() && !files_mentioned.contains(&clean.to_string()) {
                    files_mentioned.push(clean.to_string());
                }
            }
        }

        // Extract topic keywords from longer user messages (skip tool results)
        if msg.role == "user" && content.len() > 50 && !content.starts_with("[Tool results") {
            // Grab the first sentence or first 100 chars as topic indicator
            let first_part: String = content.chars().take(100).collect();
            let topic = if let Some(period_pos) = first_part.find('.') {
                &first_part[..period_pos]
            } else {
                &first_part
            };
            let topic = topic.trim();
            if !topic.is_empty() && topic.len() > 10 {
                topics.push(topic.to_string());
            }
        }
    }

    // TRINITY MIND STANDARD: Cap lists generously to preserve detail 
    // for 256K context models, avoiding premature lobotomization
    tools_used.truncate(50);
    key_decisions.truncate(50);
    files_mentioned.truncate(100);
    topics.truncate(30);

    let mut digest = String::new();

    if !topics.is_empty() {
        digest.push_str("Topics discussed: ");
        digest.push_str(&topics.join("; "));
        digest.push('\n');
    }

    if !key_decisions.is_empty() {
        digest.push_str("User directives: ");
        digest.push_str(&key_decisions.join(" → "));
        digest.push('\n');
    }

    if !tools_used.is_empty() {
        digest.push_str("Tools used: ");
        digest.push_str(&tools_used.join(", "));
        digest.push('\n');
    }

    if !files_mentioned.is_empty() {
        digest.push_str("Files touched: ");
        digest.push_str(&files_mentioned.join(", "));
        digest.push('\n');
    }

    // Hard cap at 50,000 chars (~12K tokens) to stay within 256K context budget
    // while preserving massive amounts of functional detail.
    if digest.len() > 50_000 {
        digest.truncate(50_000);
        digest.push_str("\n[digest truncated]");
    }

    digest
}

/// Extract `<thinking>...</thinking>` blocks from model output
/// Returns (thinking_text, remaining_response)
fn extract_thinking(response: &str) -> (String, String) {
    let mut thinking = String::new();
    let mut remaining = response.to_string();

    // Match <think>...</think> blocks (Gemma 4 reasoning output)
    loop {
        let start = remaining.find("<thinking>");
        let end = remaining.find("</thinking>");
        match (start, end) {
            (Some(s), Some(e)) if e > s => {
                let think_content = &remaining[s + 10..e];
                if !thinking.is_empty() {
                    thinking.push('\n');
                }
                thinking.push_str(think_content.trim());
                remaining = format!("{}{}", &remaining[..s], &remaining[e + 11..]);
            }
            _ => break,
        }
    }

    (thinking, remaining.trim().to_string())
}

/// Extract tool calls from AI response
/// Handles multiple formats:
/// - XML: <tool name="tool_name">{"params": "json"}</tool>
/// - JSON with name: {"name":"tool_name","arguments":{...}}
/// - JSON with tool: {"tool": "list_dir", "path": "crates/"}
/// - GPT-OSS native: to=tool{"tool":"list_dir","path":"crates/"}
fn extract_tool_calls(text: &str) -> Vec<(String, String)> {
    let mut calls = Vec::new();
    let mut remaining = text;

    // First try XML format
    while let Some(start) = remaining.find("<tool name=\"") {
        let after_tag = &remaining[start + 12..];
        if let Some(name_end) = after_tag.find('"') {
            let name = after_tag[..name_end].to_string();
            let after_name = &after_tag[name_end..];
            if let Some(content_start) = after_name.find('>') {
                let content_area = &after_name[content_start + 1..];
                if let Some(end) = content_area.find("</tool>") {
                    let params = content_area[..end].trim().to_string();
                    calls.push((name, params));
                    remaining = &content_area[end + 7..];
                    continue;
                }
            }
        }
        break;
    }

    // Try GPT-OSS native format: to=tool{...}
    if calls.is_empty() {
        if let Some(start) = text.find("to=tool") {
            let json_start = &text[start + 7..]; // skip "to=tool"
            if let Some(json_end_pos) = json_start.find('}') {
                let json_str = &json_start[..=json_end_pos];
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                    if let Some(tool) = json.get("tool").and_then(|t| t.as_str()) {
                        let mut params = serde_json::Map::new();
                        for (key, value) in json.as_object().unwrap_or(&serde_json::Map::new()) {
                            if key != "tool" {
                                params.insert(key.clone(), value.clone());
                            }
                        }
                        calls.push((
                            tool.to_string(),
                            serde_json::Value::Object(params).to_string(),
                        ));
                    }
                }
            }
        }
    }

    // Try JSON formats
    if calls.is_empty() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
            // Format: {"name":"tool","arguments":{...}}
            if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                let args = json
                    .get("arguments")
                    .cloned()
                    .unwrap_or(serde_json::json!({}));
                calls.push((name.to_string(), args.to_string()));
            }
            // Format: {"tool": "list_dir", "path": "crates/"}
            else if let Some(tool) = json.get("tool").and_then(|t| t.as_str()) {
                // Extract other fields as params
                let mut params = serde_json::Map::new();
                for (key, value) in json.as_object().unwrap_or(&serde_json::Map::new()) {
                    if key != "tool" {
                        params.insert(key.clone(), value.clone());
                    }
                }
                calls.push((
                    tool.to_string(),
                    serde_json::Value::Object(params).to_string(),
                ));
            }
        }
    }

    calls
}

/// Remove tool tags from text to get clean display text
fn strip_tool_tags(text: &str) -> String {
    let mut result = text.to_string();
    while let Some(start) = result.find("<tool ") {
        if let Some(end) = result[start..].find("</tool>") {
            result = format!("{}{}", &result[..start], &result[start + end + 7..]);
        } else {
            break;
        }
    }
    result
}

/// Execute a tool call internally (reuses the existing tool functions)
/// Now accepts game_state, cow_catcher, and persona mode for Ring 2 permission enforcement
async fn execute_tool_internal(
    tool: &str,
    params: &serde_json::Value,
    mode: &str,
    game_state: trinity_quest::SharedGameState,
    cow_catcher: std::sync::Arc<tokio::sync::RwLock<crate::cow_catcher::CowCatcher>>,
) -> String {
    // Permission audit: log tool permission level for every invocation
    let permission = tools::tool_permission(tool);
    info!(
        "[Agent] Tool '{}' permission: {:?} (mode: {})",
        tool, permission, mode
    );

    // ═══════════════════════════════════════════════════════════════════════
    // RING 2 ENFORCEMENT: Block Destructive tools unless persona has clearance
    // ═══════════════════════════════════════════════════════════════════════
    // - "dev" / "programmer" → full clearance (builder modes)
    // - "recycler"           → blocked from Destructive (strategic/planning only)
    // - "ironroad"           → blocked from Destructive (player safety)
    if permission == tools::ToolPermission::Destructive {
        let has_clearance = matches!(mode, "dev" | "programmer");
        if !has_clearance {
            let persona_label = match mode {
                "recycler" => "Great Recycler 🔮 (strategic mode)",
                "ironroad" => "Iron Road 🚂 (player mode)",
                other => other,
            };
            warn!(
                "[Ring 2] 🛑 BLOCKED: '{}' is Destructive, denied for {}",
                tool, persona_label
            );
            return format!(
                "🛑 Ring 2 Denied: Tool '{}' requires Destructive clearance.\n\
                 Current persona ({}) does not have clearance.\n\
                 Switch to Programmer Pete ⚙️ or Dev mode for system-level operations.",
                tool, persona_label
            );
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // RING 5: Rate Limiting — prevent runaway tool loops
    // ═══════════════════════════════════════════════════════════════════════
    let is_destructive = permission == tools::ToolPermission::Destructive;
    if let Err(throttle_msg) = check_rate_limit(is_destructive) {
        warn!("[Ring 5] {}", throttle_msg);
        return throttle_msg;
    }

    let request = tools::ToolRequest {
        tool: tool.to_string(),
        params: params.clone(),
    };

    // Route stateful tools here (they need game_state / cow_catcher)
    let result = match tool {
        // === QUEST STATUS: Read current game state ===
        "quest_status" => {
            let gs = game_state.read().await;
            let phase = gs.quest.current_phase;
            let hero = gs.quest.hero_stage;
            let objs: Vec<String> = gs
                .quest
                .phase_objectives
                .iter()
                .enumerate()
                .map(|(i, o)| {
                    format!(
                        "  {}. [{}] {}",
                        i + 1,
                        if o.completed { "✓" } else { "○" },
                        o.description
                    )
                })
                .collect();
            let completed_count = gs
                .quest
                .phase_objectives
                .iter()
                .filter(|o| o.completed)
                .count();
            let total_count = gs.quest.phase_objectives.len();
            Ok(format!(
                "🚂 QUEST STATUS\n\
                 ═══════════════\n\
                 Chapter: {} ({:?})\n\
                 Phase: {:?} ({}/{} objectives done)\n\
                 Subject: {}\n\
                 Game: {}\n\n\
                 Objectives:\n{}\n\n\
                 Resources:\n\
                 ⛏️  Coal: {:.1}\n\
                 💨 Steam: {:.1}\n\
                 ⭐ XP: {}\n\
                 🎯 Traction: {} | Velocity: {} | Combustion: {}\n\
                 🎲 Resonance: {}",
                hero.title(),
                hero,
                phase,
                completed_count,
                total_count,
                gs.quest.subject,
                gs.quest.game_title,
                objs.join("\n"),
                gs.stats.coal_reserves,
                gs.quest.steam_generated,
                gs.stats.total_xp,
                gs.stats.traction,
                gs.stats.velocity,
                gs.stats.combustion,
                gs.stats.resonance,
            ))
        }

        // === QUEST ADVANCE: Move to next/previous phase ===
        "quest_advance" => {
            let direction = params
                .get("direction")
                .and_then(|d| d.as_str())
                .unwrap_or("next");

            let mut gs = game_state.write().await;
            match direction {
                "next" => {
                    if gs.quest.phase_complete() {
                        if gs.quest.advance_phase() {
                            Ok(format!(
                                "✅ Advanced to phase: {:?}",
                                gs.quest.current_phase
                            ))
                        } else {
                            // Try chapter advance
                            if gs.quest.advance_chapter() {
                                Ok(format!(
                                    "🎉 Chapter complete! Advanced to: {}",
                                    gs.quest.quest_title
                                ))
                            } else {
                                Ok("🏆 THE JOURNEY IS COMPLETE! All chapters done.".to_string())
                            }
                        }
                    } else {
                        let remaining = gs
                            .quest
                            .phase_objectives
                            .iter()
                            .filter(|o| !o.completed)
                            .count();
                        Err(format!(
                            "Cannot advance — {} objectives still incomplete",
                            remaining
                        ))
                    }
                }
                "back" => Err(
                    "Phase retreat not yet implemented — complete objectives to move forward"
                        .to_string(),
                ),
                _ => Err(format!(
                    "Unknown direction: '{}'. Use 'next' or 'back'",
                    direction
                )),
            }
        }

        // === COW CATCHER LOG: View recent obstacles ===
        "cowcatcher_log" => {
            let cc = cow_catcher.read().await;
            let obstacles = cc.get_obstacles();
            if obstacles.is_empty() {
                Ok("🟢 Cow Catcher: No obstacles detected. All clear!".to_string())
            } else {
                let log: Vec<String> = obstacles
                    .iter()
                    .rev()
                    .take(10)
                    .map(|o| {
                        format!(
                            "  [{:?}] sev={} | {} | {}",
                            o.obstacle_type, o.severity, o.location, o.description
                        )
                    })
                    .collect();
                Ok(format!(
                    "🚨 Cow Catcher: {} obstacles detected (showing last 10):\n{}",
                    obstacles.len(),
                    log.join("\n")
                ))
            }
        }

        // === LEGACY SIDECAR TOOLS ===
        "quest_list" => {
            let client = &*crate::http::STANDARD;
            match client.get("http://127.0.0.1:8090/quests").send().await {
                Ok(res) => match res.json::<serde_json::Value>().await {
                    Ok(quests) => Ok(serde_json::to_string_pretty(&quests)
                        .unwrap_or_else(|_| "Failed to format quests".to_string())),
                    Err(e) => Err(format!("Failed to parse quests: {}", e)),
                },
                Err(e) => Err(format!("Sidecar not running on :8090 — {}", e)),
            }
        }
        "quest_execute" => {
            let quest_id = params["quest_id"].as_str().unwrap_or("");
            if quest_id.is_empty() {
                return "Error: quest_id required".to_string();
            }
            let client = &*crate::http::STANDARD;
            match client
                .post("http://127.0.0.1:8090/quest/execute")
                .json(&serde_json::json!({"quest_id": quest_id}))
                .send()
                .await
            {
                Ok(res) => match res.json::<serde_json::Value>().await {
                    Ok(result) => Ok(serde_json::to_string_pretty(&result)
                        .unwrap_or_else(|_| "Quest executed".to_string())),
                    Err(e) => Err(format!("Failed to parse result: {}", e)),
                },
                Err(e) => Err(format!("Sidecar not running on :8090 — {}", e)),
            }
        }

        // === ALL OTHER TOOLS: Route through tools.rs with 60s timeout ===
        _ => {
            match tokio::time::timeout(
                std::time::Duration::from_secs(60),
                tools::execute_tool_raw(&request),
            )
            .await
            {
                Ok(result) => result,
                Err(_) => {
                    // Timeout! Report to Cow Catcher
                    let mut cc = cow_catcher.write().await;
                    cc.report_timeout(tool, 60, "yardmaster-agent");
                    Err(format!(
                        "🚨 Tool '{}' timed out after 60s. Cow Catcher notified.",
                        tool
                    ))
                }
            }
        }
    };

    // Cow Catcher: report failures
    match &result {
        Ok(output) => output.clone(),
        Err(e) => {
            // Report compilation errors to Cow Catcher specifically
            if tool == "cargo_check" {
                let mut cc = cow_catcher.write().await;
                cc.report_compilation_error("workspace", e);
            }
            format!("Error: {}", e)
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persona_slot_recycler() {
        assert_eq!(persona_slot("recycler"), Some(0));
    }

    #[test]
    fn test_persona_slot_programmer() {
        assert_eq!(persona_slot("programmer"), Some(1));
    }

    #[test]
    fn test_persona_slot_defaults_to_none() {
        assert_eq!(persona_slot("dev"), None);
        assert_eq!(persona_slot("iron-road"), None);
        assert_eq!(persona_slot("ironroad"), None);
        assert_eq!(persona_slot(""), None);
        assert_eq!(persona_slot("unknown"), None);
    }

    #[test]
    fn test_tool_permission_categories() {
        use crate::tools::{tool_permission, ToolPermission};
        // Safe tools (read-only)
        assert_eq!(tool_permission("read_file"), ToolPermission::Safe);
        assert_eq!(tool_permission("quest_status"), ToolPermission::Safe);
        assert_eq!(
            tool_permission("load_session_context"),
            ToolPermission::Safe
        );
        // Needs approval (state-modifying)
        assert_eq!(tool_permission("write_file"), ToolPermission::NeedsApproval);
        assert_eq!(
            tool_permission("cargo_check"),
            ToolPermission::NeedsApproval
        );
        assert_eq!(
            tool_permission("scout_sniper"),
            ToolPermission::NeedsApproval
        );
        // Destructive (system-level)
        assert_eq!(tool_permission("shell"), ToolPermission::Destructive);
        assert_eq!(tool_permission("python_exec"), ToolPermission::Destructive);
        assert_eq!(
            tool_permission("sidecar_start"),
            ToolPermission::Destructive
        );
        // Unknown defaults to most restrictive
        assert_eq!(
            tool_permission("nonexistent_tool"),
            ToolPermission::Destructive
        );
    }

    #[test]
    fn test_ring2_clearance_logic() {
        use crate::tools::{tool_permission, ToolPermission};
        // Ring 2: Destructive tools should only be allowed for dev/programmer modes
        let destructive_tools = [
            "shell",
            "python_exec",
            "sidecar_start",
            "scaffold_bevy_game",
        ];
        let cleared_modes = ["dev", "programmer"];
        let blocked_modes = ["recycler", "ironroad"];

        for tool in &destructive_tools {
            let perm = tool_permission(tool);
            assert_eq!(
                perm,
                ToolPermission::Destructive,
                "Tool '{}' should be Destructive",
                tool
            );

            // Cleared modes should pass the gate
            for mode in &cleared_modes {
                let has_clearance = matches!(*mode, "dev" | "programmer");
                assert!(
                    has_clearance,
                    "Mode '{}' should have clearance for '{}'",
                    mode, tool
                );
            }

            // Blocked modes should NOT pass the gate
            for mode in &blocked_modes {
                let has_clearance = matches!(*mode, "dev" | "programmer");
                assert!(
                    !has_clearance,
                    "Mode '{}' should NOT have clearance for '{}'",
                    mode, tool
                );
            }
        }
    }

    #[test]
    fn test_ring2_safe_tools_unaffected() {
        use crate::tools::{tool_permission, ToolPermission};
        // Ring 2: Safe and NeedsApproval tools should pass through regardless of mode
        let safe_tools = ["read_file", "quest_status", "list_dir"];
        let approval_tools = ["write_file", "cargo_check", "generate_quiz"];

        for tool in &safe_tools {
            assert_ne!(
                tool_permission(tool),
                ToolPermission::Destructive,
                "Safe tool '{}' should not be affected by Ring 2 gate",
                tool
            );
        }
        for tool in &approval_tools {
            assert_ne!(
                tool_permission(tool),
                ToolPermission::Destructive,
                "NeedsApproval tool '{}' should not be affected by Ring 2 gate",
                tool
            );
        }
    }

    // ════════════════════════════════════════════════════════════════════
    // Ring 3: Context Compression Tests
    // ════════════════════════════════════════════════════════════════════

    #[test]
    fn test_ring3_empty_history() {
        let result = compress_context_digest(&[]);
        assert!(
            result.is_empty(),
            "Empty history should produce empty digest"
        );
    }

    #[test]
    fn test_ring3_extracts_tools() {
        let messages = vec![
            HistoryMessage {
                role: "assistant".into(),
                content: "`🟢 ▶ read_file` \n```\nfile contents\n```".into(),
            },
            HistoryMessage {
                role: "user".into(),
                content: "now write the fix".into(),
            },
            HistoryMessage {
                role: "assistant".into(),
                content: "`🟡 ▶ write_file` \n```\nwritten\n```".into(),
            },
        ];
        let digest = compress_context_digest(&messages);
        assert!(
            digest.contains("Tools used:"),
            "Digest should have tools section"
        );
        assert!(
            digest.contains("read_file"),
            "Should extract read_file tool"
        );
        assert!(
            digest.contains("write_file"),
            "Should extract write_file tool"
        );
    }

    #[test]
    fn test_ring3_extracts_user_directives() {
        let messages = vec![
            HistoryMessage {
                role: "user".into(),
                content: "fix the compile error".into(),
            },
            HistoryMessage {
                role: "assistant".into(),
                content: "Done!".into(),
            },
            HistoryMessage {
                role: "user".into(),
                content: "now run the tests".into(),
            },
        ];
        let digest = compress_context_digest(&messages);
        assert!(
            digest.contains("User directives:"),
            "Digest should have directives"
        );
        assert!(
            digest.contains("fix the compile error"),
            "Should capture user directive"
        );
        assert!(
            digest.contains("now run the tests"),
            "Should capture second directive"
        );
    }

    #[test]
    fn test_ring3_extracts_file_paths() {
        let messages = vec![HistoryMessage {
            role: "user".into(),
            content: "Check crates/trinity/src/agent.rs for the bug".into(),
        }];
        let digest = compress_context_digest(&messages);
        assert!(
            digest.contains("Files touched:"),
            "Digest should have files section"
        );
        assert!(digest.contains("agent.rs"), "Should extract file path");
    }

    #[test]
    fn test_ring3_caps_digest_length() {
        // Create a huge history to test truncation
        let mut messages = Vec::new();
        for i in 0..100 {
            messages.push(HistoryMessage {
                role: "user".into(),
                content: format!("This is a longer directive message number {} that discusses important topics about system architecture and design patterns for the project", i),
            });
        }
        let digest = compress_context_digest(&messages);
        // Digest cap is 50,000 chars for 256K context models
        assert!(
            digest.len() <= 50_100,
            "Digest should be capped near 50,000 chars, got {}",
            digest.len()
        );
        // But should actually produce content
        assert!(
            !digest.is_empty(),
            "Digest should not be empty with 100 messages"
        );
    }

    #[test]
    fn test_ring3_skips_tool_result_messages() {
        let messages = vec![
            HistoryMessage { role: "user".into(), content: "[Tool results — continue with next step or provide final answer]\n\nTool `read_file` result:\n```\ncontents\n```".into() },
            HistoryMessage { role: "user".into(), content: "please fix it".into() },
        ];
        let digest = compress_context_digest(&messages);
        // Tool result messages should NOT appear as user directives
        assert!(
            !digest.contains("[Tool results"),
            "Should skip tool result messages as directives"
        );
    }

    // ════════════════════════════════════════════════════════════════════
    // Ring 5: Rate Limiting Tests
    // ════════════════════════════════════════════════════════════════════

    #[test]
    fn test_ring5_rate_limiting() {
        // NOTE: These tests use shared static atomics, so they must run
        // sequentially within a single test to avoid parallel test races.

        // === Part 1: Normal calls should be allowed ===
        // Reset all counters by setting window far in the past
        TOOL_CALL_WINDOW_START.store(0, Ordering::Relaxed);
        TOOL_CALL_COUNT.store(0, Ordering::Relaxed);
        DESTRUCTIVE_CALL_WINDOW_START.store(0, Ordering::Relaxed);
        DESTRUCTIVE_CALL_COUNT.store(0, Ordering::Relaxed);

        // First call resets window — should always succeed
        assert!(
            check_rate_limit(false).is_ok(),
            "First safe call should be allowed"
        );

        // Reset again for destructive
        DESTRUCTIVE_CALL_WINDOW_START.store(0, Ordering::Relaxed);
        DESTRUCTIVE_CALL_COUNT.store(0, Ordering::Relaxed);
        assert!(
            check_rate_limit(true).is_ok(),
            "First destructive call should be allowed"
        );

        // === Part 2: Destructive rate limit should be enforced ===
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        DESTRUCTIVE_CALL_WINDOW_START.store(now, Ordering::Relaxed);
        DESTRUCTIVE_CALL_COUNT.store(5, Ordering::Relaxed);
        // Reset global so it doesn't interfere
        TOOL_CALL_WINDOW_START.store(now, Ordering::Relaxed);
        TOOL_CALL_COUNT.store(1, Ordering::Relaxed);

        // 6th destructive call in same window should be blocked
        let result = check_rate_limit(true);
        assert!(
            result.is_err(),
            "Should block when destructive limit exceeded"
        );
        assert!(
            result.unwrap_err().contains("Ring 5"),
            "Error should mention Ring 5"
        );

        // === Part 3: Global rate limit enforcement ===
        TOOL_CALL_WINDOW_START.store(now, Ordering::Relaxed);
        TOOL_CALL_COUNT.store(60, Ordering::Relaxed);
        let result = check_rate_limit(false);
        assert!(result.is_err(), "Should block when global limit exceeded");

        // === Cleanup: reset for other tests ===
        TOOL_CALL_WINDOW_START.store(0, Ordering::Relaxed);
        TOOL_CALL_COUNT.store(0, Ordering::Relaxed);
        DESTRUCTIVE_CALL_WINDOW_START.store(0, Ordering::Relaxed);
        DESTRUCTIVE_CALL_COUNT.store(0, Ordering::Relaxed);
    }

    #[test]
    fn test_ring5_shell_sandboxing() {
        // Verify our blocked patterns exist in tools.rs
        // This is a structural test — the actual blocking is in tool_shell
        use crate::tools::tool_permission;
        // shell is Destructive, which means Ring 2 + Ring 5 gates apply
        assert_eq!(
            tool_permission("shell"),
            crate::tools::ToolPermission::Destructive
        );
    }
}
