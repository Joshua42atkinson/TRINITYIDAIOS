#![allow(dead_code)]
// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        conductor_leader.rs
// PURPOSE:     The Conductor — ADDIECRAPEYE phase orchestrator and Hotel manager
//
// 🪟 THE LIVING CODE TEXTBOOK (P-ART-Y Framework Orchestration):
// This file is the beating heart of the ADDIECRAPEYE quest system. It is 
// designed to be read, modified, and authored by YOU. If you want to change how 
// the AI transitions between learning phases or swaps models, edit this logic.
// ACTION: Modify `manage_hotel_sidecars()` to change how AI agents are swapped.
//
// 📖 THE HOOK BOOK CONNECTION:
// This file implements the '12-Station Quest' and 'Model Switching' Hooks. 
// You can use this state machine pattern to build your own gamified apps.
// For a full catalogue of system capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
//
// ARCHITECTURE:
//   • The 12 phases of ADDIECRAPEYE (ADD + IECRAP + EYE) as state machine
//   • Hotel Manager: enforces "One Heavyweight at a Time" (Pete/Ming/ART models)
//   • Sidecar hot-swap protocol: unload → verify → load → health check
//   • CharacterSheet broadcast via SSE to all connected clients
//   • Phase transitions trigger LitRPG chapter generation
//
// DEPENDENCIES:
//   - tokio::sync — Async broadcast channels for SSE
//   - tracing — Conductor state machine logging
//   - trinity_protocol — CharacterSheet and phase definitions
//   - anyhow — Error handling for phase transitions
//
// CHANGES:
//   2026-03-16  Cascade  Migrated to §17 comment standard
//
// ═══════════════════════════════════════════════════════════════════════════════

use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, error, info, warn};

/// The phases of the ADDIECRAPEYE quest methodology
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AddiecrapeyePhase {
    Analysis,
    Design,
    Development,
    Implementation,
    Evaluation,
    Contrast,
    Repetition,
    Alignment,
    Proximity,
    Envision,
    Yoke,
    Evolve,
}

/// Get the system prompt associated with a phase.
pub fn phase_system_prompt(_phase: AddiecrapeyePhase) -> &'static str {
    "You are the Conductor. Guide the user through this phase of the IRON ROAD."
}

impl std::fmt::Display for AddiecrapeyePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddiecrapeyePhase::Analysis => write!(f, "Analysis"),
            AddiecrapeyePhase::Design => write!(f, "Design"),
            AddiecrapeyePhase::Development => write!(f, "Development"),
            AddiecrapeyePhase::Implementation => write!(f, "Implementation"),
            AddiecrapeyePhase::Evaluation => write!(f, "Evaluation"),
            AddiecrapeyePhase::Contrast => write!(f, "Contrast"),
            AddiecrapeyePhase::Repetition => write!(f, "Repetition"),
            AddiecrapeyePhase::Alignment => write!(f, "Alignment"),
            AddiecrapeyePhase::Proximity => write!(f, "Proximity"),
            AddiecrapeyePhase::Envision => write!(f, "Envision"),
            AddiecrapeyePhase::Yoke => write!(f, "Yoke"),
            AddiecrapeyePhase::Evolve => write!(f, "Evolve"),
        }
    }
}

impl AddiecrapeyePhase {
    /// Get the next phase in the cycle
    #[allow(dead_code)] // Used by quest system to advance through ADDIECRAPEYE cycle
    pub fn next(&self) -> Self {
        match self {
            AddiecrapeyePhase::Analysis => AddiecrapeyePhase::Design,
            AddiecrapeyePhase::Design => AddiecrapeyePhase::Development,
            AddiecrapeyePhase::Development => AddiecrapeyePhase::Implementation,
            AddiecrapeyePhase::Implementation => AddiecrapeyePhase::Evaluation,
            AddiecrapeyePhase::Evaluation => AddiecrapeyePhase::Contrast,
            AddiecrapeyePhase::Contrast => AddiecrapeyePhase::Repetition,
            AddiecrapeyePhase::Repetition => AddiecrapeyePhase::Alignment,
            AddiecrapeyePhase::Alignment => AddiecrapeyePhase::Proximity,
            AddiecrapeyePhase::Proximity => AddiecrapeyePhase::Envision,
            AddiecrapeyePhase::Envision => AddiecrapeyePhase::Yoke,
            AddiecrapeyePhase::Yoke => AddiecrapeyePhase::Evolve,
            AddiecrapeyePhase::Evolve => AddiecrapeyePhase::Analysis,
        }
    }

    /// All phases for iteration
    #[allow(dead_code)] // Used for iteration and display in the frontend station selector
    pub fn all() -> &'static [Self] {
        &[
            AddiecrapeyePhase::Analysis,
            AddiecrapeyePhase::Design,
            AddiecrapeyePhase::Development,
            AddiecrapeyePhase::Implementation,
            AddiecrapeyePhase::Evaluation,
            AddiecrapeyePhase::Contrast,
            AddiecrapeyePhase::Repetition,
            AddiecrapeyePhase::Alignment,
            AddiecrapeyePhase::Proximity,
            AddiecrapeyePhase::Envision,
            AddiecrapeyePhase::Yoke,
            AddiecrapeyePhase::Evolve,
        ]
    }

    /// Bloom's Taxonomy cognitive level for this ADDIECRAPEYE phase.
    ///
    /// This is the pedagogical backbone — it tells Pete what cognitive level
    /// the learner should be operating at during each station. Without this,
    /// the system cannot scaffold cognitive complexity properly.
    ///
    /// The Pythagorean mapping: each phase demands a specific type of thinking,
    /// and the 12-station cycle spirals upward through all six Bloom's levels.
    pub fn bloom_level(&self) -> &'static str {
        match self {
            AddiecrapeyePhase::Analysis => {
                "Remember/Understand — extract intent, identify audience, recall prior knowledge"
            }
            AddiecrapeyePhase::Design => {
                "Apply — map objectives to mechanics, apply Bloom's verbs to goals"
            }
            AddiecrapeyePhase::Development => {
                "Create — build assets, write code, produce tangible artifacts"
            }
            AddiecrapeyePhase::Implementation => {
                "Apply — deploy, integrate, verify setup matches design"
            }
            AddiecrapeyePhase::Evaluation => {
                "Evaluate — QM rubric, quality review, measure against criteria"
            }
            AddiecrapeyePhase::Contrast => {
                "Analyze — visual hierarchy analysis, emphasis ranking, boundary design"
            }
            AddiecrapeyePhase::Repetition => {
                "Apply — pattern reinforcement, consistency audit, core loop solidity"
            }
            AddiecrapeyePhase::Alignment => {
                "Evaluate — structure compliance, scope pruning, Extraneous Load → zero"
            }
            AddiecrapeyePhase::Proximity => {
                "Analyze — UX grouping, Miller's Law (7±2), human-computer interaction"
            }
            AddiecrapeyePhase::Envision => {
                "Evaluate — meta-cognitive reflection, compare against original goals"
            }
            AddiecrapeyePhase::Yoke => {
                "Create — system integration, couple frontend to backend, final assembly"
            }
            AddiecrapeyePhase::Evolve => "Create — ship, publish, the Golem takes its first breath",
        }
    }
}

/// The mode in which the Conductor operates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum ConductorMode {
    #[default]
    Hybrid,
    Instructional,
    Narrative,
}

/// Quest orchestration request
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrchestrationRequest {
    /// Quest ID being orchestrated
    pub quest_id: String,

    /// Current ADDIECRAPEYE phase
    pub current_phase: AddiecrapeyePhase,

    /// Player context (from Iron Road book)
    pub player_context: serde_json::Value,

    /// Quest objectives
    pub objectives: Vec<String>,

    /// Party members available
    pub available_party: Vec<String>,

    /// Intent engineering context from CharacterSheet.
    /// Generated by `CharacterSheet::intent_summary()`.
    /// Tells Pete: grounding status, posture (Mastery/Efficiency),
    /// session intent, and vulnerability level.
    #[serde(default)]
    pub intent_context: String,

    /// VAAM profile summary from CharacterSheet.
    /// Generated by `VaamProfile::prompt_summary()`.
    /// Tells Pete: dominant quadrant, style, top words, agreements.
    #[serde(default)]
    pub vaam_context: String,

    /// PEARL alignment context from the active quest.
    /// Generated by `Pearl::prompt_summary()`.
    /// Tells Pete: subject, medium, vision, alignment scores.
    #[serde(default)]
    pub pearl_context: String,

    /// Mode of operation (Instructional, Narrative, Hybrid)
    #[serde(default)]
    pub mode: ConductorMode,
}

/// Quest orchestration response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrchestrationResponse {
    /// Next phase to execute
    pub next_phase: AddiecrapeyePhase,

    /// Instructions for the player
    pub player_instructions: String,

    /// Party member assignments (role -> task)
    pub party_assignments: std::collections::HashMap<String, String>,

    /// Content generated during orchestration (JSON)
    pub generated_content: Option<serde_json::Value>,

    /// XP awarded for completing the phase
    pub xp_awarded: u32,
}

/// A Book update from the NPU Great Recycler
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BookUpdate {
    /// Update ID
    pub id: String,
    /// Type of update (e.g. "player_action", "quest_completion", "narrative_update")
    pub entry_type: String,
    /// Content of the update
    pub content: String,
    /// Current resonance level (1-7)
    pub resonance: u32,
}

/// Configuration for the Conductor Party Leader
#[derive(Debug, Clone)]
pub struct ConductorConfig {
    /// Path to the Conductor model (Great Recycler - LongCat-Next 74B MoE)
    pub model_path: PathBuf,
    /// Context size (default: 256000 with TurboQuant)
    pub context_size: u32,
    /// vLLM Omni base URL
    pub server_url: String,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for ConductorConfig {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/tmp")).to_string_lossy().to_string();
        Self {
            // Great Recycler — LongCat-Next 74B MoE (served via sglang-engine sidecar)
            model_path: PathBuf::from(home)
                .join("trinity-models/sglang/LongCat-Next"),
            context_size: 131072, // LongCat-Next 131K context window
            server_url: std::env::var("LLM_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:8010".to_string()),
            verbose: false,
        }
    }
}

/// Conductor Party Leader - orchestrates quests via ADDIECRAPEYE
///
/// ARCHITECTURE:
/// - Runs on GPU via vLLM Omni (Great Recycler pent-house)
/// - Reads Iron Road book updates and delegates to P.A.R.T.Y. sub-agents
/// - Coordinates party members for complex multi-step objectives
pub struct ConductorLeader {
    /// Configuration
    config: ConductorConfig,

    /// Current ADDIECRAPEYE phase
    current_phase: Arc<RwLock<AddiecrapeyePhase>>,

    /// Active quests being orchestrated
    active_quests: Arc<RwLock<Vec<String>>>,

    /// Receiver for Iron Road book updates from Great Recycler
    book_updates: Option<broadcast::Receiver<BookUpdate>>,

    /// Sender for orchestration responses
    orchestration_sender: broadcast::Sender<OrchestrationResponse>,

    /// Shutdown signal receiver
    shutdown_rx: Option<broadcast::Receiver<()>>,

    /// LLM HTTP client (when model is loaded)
    llama_client: Option<reqwest::Client>,
}

impl ConductorLeader {
    /// Create a new Conductor Party Leader
    /// WHY: Initialize quest orchestration system
    /// HOW: Connect to Iron Road book updates from Great Recycler
    pub fn new(config: ConductorConfig) -> Self {
        let (tx, _) = broadcast::channel(100);

        Self {
            config,
            current_phase: Arc::new(RwLock::new(AddiecrapeyePhase::Analysis)),
            active_quests: Arc::new(RwLock::new(Vec::new())),
            book_updates: None,
            orchestration_sender: tx,
            shutdown_rx: None,
            llama_client: None,
        }
    }

    /// Set the receiver for book updates from Great Recycler
    pub fn set_book_updates_receiver(&mut self, rx: broadcast::Receiver<BookUpdate>) {
        self.book_updates = Some(rx);
    }

    /// Get a receiver for orchestration responses
    pub fn subscribe_to_orchestration(&self) -> broadcast::Receiver<OrchestrationResponse> {
        self.orchestration_sender.subscribe()
    }

    /// Set the receiver for shutdown signals
    pub fn set_shutdown_receiver(&mut self, rx: broadcast::Receiver<()>) {
        self.shutdown_rx = Some(rx);
    }

    /// Check if the Conductor model exists locally
    /// MUST return true for execution to proceed
    pub fn model_exists(&self) -> bool {
        self.config.model_path.exists()
    }

    /// Start the quest orchestration loop
    /// WHY: Conductor continuously orchestrates quests based on Iron Road updates
    /// HOW: Listen for book updates, process through ADDIECRAPEYE phases
    pub async fn run(&mut self) -> Result<()> {
        info!("Conductor Party Leader starting quest orchestration...");

        // Verify model exists
        if !self.model_exists() {
            return Err(anyhow!(
                "Great Recycler model (LongCat-Next 74B MoE) not found at {:?}. \
                 Please ensure the model is available and the sglang-engine sidecar is running.\n\
                 AI should NOT automatically download models without user initiation.",
                self.config.model_path
            ));
        }

        let mut shutdown_rx = self.shutdown_rx.take();
        let mut book_updates = self.book_updates.take();

        loop {
            tokio::select! {
                // Check for shutdown signal
                _ = async {
                    if let Some(rx) = &mut shutdown_rx {
                        rx.recv().await.ok()
                    } else {
                        // If no shutdown channel, wait forever
                        futures::future::pending::<()>().await;
                        None
                    }
                } => {
                    info!("Conductor Party Leader shutting down");
                    break;
                }

                // Process new book updates
                update = async {
                    if let Some(rx) = &mut book_updates {
                        rx.recv().await
                    } else {
                        futures::future::pending().await
                    }
                } => {
                    match update {
                        Ok(update) => {
                            if let Err(e) = self.process_book_update(update).await {
                                error!("Error processing book update: {}", e);
                            }
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            warn!("Book updates channel closed");
                            // Wait briefly then try to reconnect (handled by higher level)
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                        Err(broadcast::error::RecvError::Lagged(count)) => {
                            warn!("Conductor lagged behind {} book updates", count);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Process a new update from the Iron Road book
    /// WHY: Book updates trigger quest progression
    async fn process_book_update(&self, update: BookUpdate) -> Result<()> {
        debug!(
            "Conductor received book update: [{}] {}",
            update.entry_type, update.id
        );

        // Update current phase based on entry type
        let new_phase = match update.entry_type.as_str() {
            "player_action" => AddiecrapeyePhase::Analysis,
            "quest_completion" => AddiecrapeyePhase::Evaluation,
            "narrative_update" => AddiecrapeyePhase::Repetition,
            "system_event" => AddiecrapeyePhase::Proximity,
            "scope_creep_detected" => AddiecrapeyePhase::Design, // Route Scope Creep back to Design for containment
            _ => return Ok(()),                                  // Ignore unknown entry types
        };

        {
            let mut phase = self.current_phase.write().await;
            *phase = new_phase;
        }

        if self.config.verbose {
            info!("Conductor phase updated to: {}", new_phase);
        }

        Ok(())
    }

    /// Orchestrate a quest
    /// WHY: Process quest through ADDIECRAPEYE phases
    /// HOW: Generate instructions and party assignments via LLM inference
    pub async fn orchestrate(
        &self,
        request: OrchestrationRequest,
    ) -> Result<OrchestrationResponse> {
        info!(
            "Orchestrating quest {} in phase {}",
            request.quest_id, request.current_phase
        );

        // Use the phase from the request as the source of truth for orchestration
        let current_phase = request.current_phase;

        // HOTEL MANAGEMENT: Spawn the appropriate sidecar based on phase
        if let Err(e) = self.manage_hotel_sidecars(current_phase).await {
            warn!("Hotel Management: Failed to spawn sidecar: {}", e);
        }

        // Generate orchestration response based on phase
        let response = match current_phase {
            AddiecrapeyePhase::Analysis => self.analyze_quest(&request).await?,
            AddiecrapeyePhase::Design => self.design_quest(&request).await?,
            AddiecrapeyePhase::Development => self.develop_quest(&request).await?,
            AddiecrapeyePhase::Implementation => self.implement_quest(&request).await?,
            AddiecrapeyePhase::Evaluation => self.evaluate_quest(&request).await?,
            AddiecrapeyePhase::Contrast => self.contrast_quest(&request).await?,
            AddiecrapeyePhase::Repetition => self.repetition_quest(&request).await?,
            AddiecrapeyePhase::Alignment => self.alignment_quest(&request).await?,
            AddiecrapeyePhase::Proximity => self.proximity_quest(&request).await?,
            AddiecrapeyePhase::Envision => self.envision_quest(&request).await?,
            AddiecrapeyePhase::Yoke => self.yoke_quest(&request).await?,
            AddiecrapeyePhase::Evolve => self.evolve_quest(&request).await?,
        };

        // Broadcast response to party members
        let _ = self.orchestration_sender.send(response.clone());

        // Update internal state to match the next phase
        {
            let mut phase = self.current_phase.write().await;
            *phase = current_phase.next();
        }

        Ok(response)
    }

    /// HOTEL MANAGEMENT: Switch out models based on ADDIECRAPEYE phase
    /// NOTE: In Lone Wolf mode (Phase 4), this is log-only — single LongCat-Next brain,
    /// no model hot-swapping. The gear mapping is preserved for future multi-model support.
    async fn manage_hotel_sidecars(&self, phase: AddiecrapeyePhase) -> Result<()> {
        // CRUISING MODE BATCHING: We group the 12 phases into the 4 P-ART gears
        // to prevent thrashing the VRAM every single node.
        let target_role = match phase {
            // Gear P: Pete — Socratic Mirror (permanent resident)
            AddiecrapeyePhase::Analysis | AddiecrapeyePhase::Envision => {
                "pete" // Socratic questioning, meta-awareness
            }
            // Gear A: Aesthetics — The Visionary (CRAP design)
            AddiecrapeyePhase::Design
            | AddiecrapeyePhase::Contrast
            | AddiecrapeyePhase::Proximity => {
                "aesthetics" // Visual hierarchy, ComfyUI, UI boundaries
            }
            // Gear R: Research — The Brakeman (QM, tests, audits)
            AddiecrapeyePhase::Evaluation | AddiecrapeyePhase::Alignment => {
                "research" // QM rubrics, test gen, scope pruning
            }
            // Gear T: Tempo — The Engineer (code gen, momentum)
            AddiecrapeyePhase::Development
            | AddiecrapeyePhase::Implementation
            | AddiecrapeyePhase::Repetition => "tempo",
            // Full P-ART swarm for coupling + shipping
            AddiecrapeyePhase::Yoke | AddiecrapeyePhase::Evolve => "pete", // Pete leads the final assembly
        };

        // LONE WOLF MODE: Log the gear shift but don't actually swap models.
        // Single LongCat-Next 74B MoE handles all phases.
        info!(
            "Hotel Management (Lone Wolf): Phase {} → gear {} (no swap)",
            phase, target_role
        );

        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════
    // CORE: Call Pete (LongCat-Next) via LLM server for real inference
    // ═══════════════════════════════════════════════════════════════════

    /// Call Pete (the Conductor model) with a phase-specific system prompt.
    /// Returns the model's raw text response.
    async fn call_pete(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let client = self
            .llama_client
            .as_ref()
            .cloned()
            .unwrap_or_else(|| crate::http::LONG.clone());

        // Extract the model name from the directory path (vLLM uses this to route in multi-model)
        let model_id = self.config.model_path.file_name().unwrap_or_default().to_string_lossy();

        let body = serde_json::json!({
            "model": model_id,
            "messages": [
                { "role": "system", "content": system_prompt },
                { "role": "user", "content": user_prompt }
            ],
            "max_tokens": 2048,
            "temperature": 0.7,
            "stream": false
        });

        let url = format!("{}/v1/chat/completions", self.config.server_url);

        let response = client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!("Pete unreachable at {}: {}", url, e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Pete returned {}: {}", status, text));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse Pete's response: {}", e))?;

        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No content in Pete's response"))
    }

    /// Build a user prompt from the OrchestrationRequest context
    fn build_quest_context(request: &OrchestrationRequest) -> String {
        let objectives = request.objectives.join("\n- ");
        let party = request.available_party.join(", ");
        let bloom = request.current_phase.bloom_level();

        let mut context = format!(
            "Quest: {}\nPhase: {}\nBloom's Level: {}\nObjectives:\n- {}\nAvailable Party: [{}]",
            request.quest_id, request.current_phase, bloom, objectives, party
        );

        // Intent Engineering context — WHO the user is and WHY they're here
        if !request.intent_context.is_empty() {
            context.push_str(&format!(
                "\n\nINTENT ENGINEERING:\n{}",
                request.intent_context
            ));
        }

        // VAAM context — HOW the user communicates and WHAT they value
        if !request.vaam_context.is_empty() {
            context.push_str(&format!("\n\nVAAM PROFILE:\n{}", request.vaam_context));
        }

        // PEARL context — WHAT the user is building, in WHAT medium, WHY
        if !request.pearl_context.is_empty() {
            context.push_str(&format!("\n\nPEARL ALIGNMENT:\n{}", request.pearl_context));
        }

        // Player context (additional JSON)
        let player_ctx = serde_json::to_string_pretty(&request.player_context).unwrap_or_default();
        if player_ctx != "null" && player_ctx != "{}" {
            context.push_str(&format!("\n\nPlayer Context: {}", player_ctx));
        }

        context
    }

    /// Get the system prompt based on phase and mode
    fn get_system_prompt(&self, phase: AddiecrapeyePhase, mode: ConductorMode) -> String {
        let prefix = match mode {
            ConductorMode::Narrative => "You are the Great Recycler, the Socratic voice of the Iron Road. The user is a brave traveler (protagonist) on a LitRPG journey. ",
            ConductorMode::Instructional => "You are Pete, the Conductor. Provide direct pedagogical scaffolding. ",
            ConductorMode::Hybrid => "You are the Conductor, blending LitRPG narrative with Socratic instructional design. ",
        };

        let phase_content = match (mode, phase) {
            (ConductorMode::Narrative, AddiecrapeyePhase::Analysis) => "ANALYSIS: The Golem's Eyes. Paint a scene of a foggy train station platform. Ask the traveler to identify the passengers (target audience) waiting in the mist, and describe the specific burdens (learning gaps) they carry.",
            (ConductorMode::Narrative, AddiecrapeyePhase::Design) => "DESIGN: The Golem's Brain. Describe a drafting table illuminated by flickering lanterns. Ask the traveler to sketch the blueprints of their journey—what specific abilities must the passengers gain by the end of the line?",
            (ConductorMode::Narrative, AddiecrapeyePhase::Development) => "DEVELOPMENT: The Golem's Skeleton. The sparks of the forge fly as hammers strike. Help the traveler forge the first raw framing of their curriculum—what does the physical structure of the lesson look like?",
            (ConductorMode::Narrative, AddiecrapeyePhase::Implementation) => "IMPLEMENTATION: The Golem's Muscles. The steam engine roars to life. Ask the traveler to guide a test passenger through the cars, ensuring the journey is smooth and the machinery functions as intended.",
            (ConductorMode::Narrative, AddiecrapeyePhase::Evaluation) => "EVALUATION: The Golem's Voice. An ancient oracle weighs the traveler's deeds on a golden scale. Help them measure the true quality and impact of the learning experience against the original vision.",
            (ConductorMode::Narrative, AddiecrapeyePhase::Contrast) => "CONTRAST: The Golem's Skin. Shadows and bright flares of light dance across the train cars. Ask the traveler where they should hang the brightest lanterns (visual hierarchy) to draw the passengers' eyes to what matters most.",
            (ConductorMode::Narrative, AddiecrapeyePhase::Repetition) => "REPETITION: The Golem's Heart. The steady 'clack-clack' of wheels on the iron rails sets a comforting rhythm. Ask the traveler how to weave consistent patterns that make the passengers feel secure.",
            (ConductorMode::Narrative, AddiecrapeyePhase::Alignment) => "ALIGNMENT: The Golem's Spine. The train leans into a sharp curve. Ask the traveler to jettison any excess cargo (extraneous cognitive load) to keep the train perfectly balanced on the tracks.",
            (ConductorMode::Narrative, AddiecrapeyePhase::Proximity) => "PROXIMITY: The Golem's Hands. Luggage is scattered wildly across the cabins. Ask the traveler how to group similar items together so passengers intuitively understand where things belong.",
            (ConductorMode::Narrative, AddiecrapeyePhase::Envision) => "ENVISION: The Golem's Third Eye. The traveler stands upon the caboose looking back down the tracks. Ask them to reflect: did the journey lead where they originally promised?",
            (ConductorMode::Narrative, AddiecrapeyePhase::Yoke) => "YOKE: The Connective Tissue. Heavy iron couplings lock the cars together. Ask the traveler how to seamlessly bind the frontend aesthetics with the backend mechanics into one unstoppable machine.",
            (ConductorMode::Narrative, AddiecrapeyePhase::Evolve) => "EVOLVE: The Golem's Lungs. The engine breaches the fog, entering a breathtaking valley of light. Time to unleash the creation upon the world. Congratulate them on shipping.",
            
            (_, AddiecrapeyePhase::Analysis) => "ANALYSIS: This is the Golem's Eyes. Ask who their audience is, and what they struggle with. Present choices.",
            (_, AddiecrapeyePhase::Design) => "DESIGN: This is the Golem's Brain. Ask them what the learner should be able to do. Suggest mechanics.",
            (_, AddiecrapeyePhase::Development) => "DEVELOPMENT: This is the Golem's Skeleton. Ask them what the first working version looks like. Scaffold implementation.",
            (_, AddiecrapeyePhase::Implementation) => "IMPLEMENTATION: This is the Golem's Muscles. Ask them to test the system from a learner's perspective.",
            (_, AddiecrapeyePhase::Evaluation) => "EVALUATION: This is the Golem's Voice. Evaluate quality against the intent.",
            (_, AddiecrapeyePhase::Contrast) => "CONTRAST: This is the Golem's Skin. Ask them how visual hierarchy can emphasize importance.",
            (_, AddiecrapeyePhase::Repetition) => "REPETITION: This is the Golem's Heart. Focus on patterns and consistency in the design.",
            (_, AddiecrapeyePhase::Alignment) => "ALIGNMENT: This is the Golem's Spine. Focus on removing extraneous cognitive load.",
            (_, AddiecrapeyePhase::Proximity) => "PROXIMITY: This is the Golem's Hands. Focus on grouping elements logically.",
            (_, AddiecrapeyePhase::Envision) => "ENVISION: This is the Golem's Third Eye. Reflect on the original goals versus the current state.",
            (_, AddiecrapeyePhase::Yoke) => "YOKE: This is the Connective Tissue. Focus on integration.",
            (_, AddiecrapeyePhase::Evolve) => "EVOLVE: This is the Golem's Lungs. Time to ship the finished product. Congratulate them.",
        };

        format!("{}\n\nSOCRATIC PROTOCOL: Ask, do not tell. Formulate questions based on the phase:\n{}", prefix, phase_content)
    }

    // ═══════════════════════════════════════════════════════════════════
    // ADDIECRAPEYE phase implementations — real LLM calls through Pete
    // ═══════════════════════════════════════════════════════════════════

    async fn analyze_quest(&self, request: &OrchestrationRequest) -> Result<OrchestrationResponse> {
        let system = self.get_system_prompt(AddiecrapeyePhase::Analysis, request.mode);
        let context = Self::build_quest_context(request);

        match self.call_pete(&system, &context).await {
            Ok(response) => {
                let mut assignments = std::collections::HashMap::new();
                assignments.insert(
                    "pete".to_string(),
                    "Socratic extraction of user intent".to_string(),
                );
                Ok(OrchestrationResponse {
                    next_phase: AddiecrapeyePhase::Design,
                    player_instructions: response,
                    party_assignments: assignments,
                    generated_content: None,
                    xp_awarded: 10,
                })
            }
            Err(e) => {
                warn!("Pete offline for Analysis phase: {}", e);
                Ok(OrchestrationResponse {
                    next_phase: AddiecrapeyePhase::Design,
                    player_instructions: format!("[Pete offline] Fallback: Review your objectives and define the target audience. Error: {}", e),
                    party_assignments: std::collections::HashMap::new(),
                    generated_content: None,
                    xp_awarded: 5,
                })
            }
        }
    }

    async fn design_quest(&self, request: &OrchestrationRequest) -> Result<OrchestrationResponse> {
        let system = self.get_system_prompt(AddiecrapeyePhase::Design, request.mode);
        let context = Self::build_quest_context(request);

        match self.call_pete(&system, &context).await {
            Ok(response) => {
                let mut assignments = std::collections::HashMap::new();
                assignments.insert(
                    "aesthetics".to_string(),
                    "Visual design and UI wireframes".to_string(),
                );
                Ok(OrchestrationResponse {
                    next_phase: AddiecrapeyePhase::Development,
                    player_instructions: response,
                    party_assignments: assignments,
                    generated_content: None,
                    xp_awarded: 20,
                })
            }
            Err(e) => {
                warn!("Pete offline for Design phase: {}", e);
                Ok(OrchestrationResponse {
                    next_phase: AddiecrapeyePhase::Development,
                    player_instructions: format!("[Pete offline] Fallback: Create design documents for your objectives. Error: {}", e),
                    party_assignments: std::collections::HashMap::new(),
                    generated_content: None,
                    xp_awarded: 10,
                })
            }
        }
    }

    async fn develop_quest(&self, request: &OrchestrationRequest) -> Result<OrchestrationResponse> {
        let system = self.get_system_prompt(AddiecrapeyePhase::Development, request.mode);
        let context = Self::build_quest_context(request);

        match self.call_pete(&system, &context).await {
            Ok(response) => {
                let mut assignments = std::collections::HashMap::new();
                assignments.insert(
                    "tempo".to_string(),
                    "Code generation and implementation from design spec".to_string(),
                );
                Ok(OrchestrationResponse {
                    next_phase: AddiecrapeyePhase::Implementation,
                    player_instructions: response,
                    party_assignments: assignments,
                    generated_content: None,
                    xp_awarded: 0,
                })
            }
            Err(e) => {
                warn!("Pete offline for Development phase: {}", e);
                Ok(OrchestrationResponse {
                    next_phase: AddiecrapeyePhase::Implementation,
                    player_instructions: format!(
                        "[Pete offline] Fallback: Begin implementation. Error: {}",
                        e
                    ),
                    party_assignments: std::collections::HashMap::new(),
                    generated_content: None,
                    xp_awarded: 0,
                })
            }
        }
    }

    async fn implement_quest(
        &self,
        request: &OrchestrationRequest,
    ) -> Result<OrchestrationResponse> {
        let system = self.get_system_prompt(AddiecrapeyePhase::Implementation, request.mode);
        let context = Self::build_quest_context(request);

        match self.call_pete(&system, &context).await {
            Ok(response) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Evaluation,
                player_instructions: response,
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 0,
            }),
            Err(e) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Evaluation,
                player_instructions: format!("[Pete offline] Proceed to evaluation. Error: {}", e),
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 0,
            }),
        }
    }

    async fn evaluate_quest(
        &self,
        request: &OrchestrationRequest,
    ) -> Result<OrchestrationResponse> {
        // Build an IdContract from quest context for QM evaluation
        let contract = Self::build_contract_from_context(request);
        let qm_eval = trinity_protocol::QmRubricEvaluator::evaluate(&contract);

        // Format QM results for Pete's evaluation prompt
        let qm_summary = format!(
            "QM RUBRIC EVALUATION RESULTS (automated):\n\
             Overall Score: {:.0}/100 — {}\n\
             Criteria:\n{}\n\
             Feedback:\n{}",
            qm_eval.overall_score,
            if qm_eval.meets_standards {
                "PASSES"
            } else {
                "FAILS — needs revision"
            },
            qm_eval
                .criteria
                .iter()
                .map(|c| format!(
                    "  - {}: {:.0}/100 {}",
                    c.name,
                    c.score,
                    if c.met { "✅" } else { "❌" }
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            qm_eval.feedback.join("\n"),
        );

        let system = format!(
            "{}\n\nQM RUBRIC EVALUATION RESULTS:\n{}",
            self.get_system_prompt(AddiecrapeyePhase::Evaluation, request.mode),
            qm_summary
        );
        let context = Self::build_quest_context(request);

        match self.call_pete(&system, &context).await {
            Ok(response) => {
                let mut assignments = std::collections::HashMap::new();
                assignments.insert(
                    "research".to_string(),
                    "Quality Matters rubric evaluation".to_string(),
                );

                let xp = if qm_eval.meets_standards { 25 } else { 5 };

                Ok(OrchestrationResponse {
                    next_phase: AddiecrapeyePhase::Contrast,
                    player_instructions: response,
                    party_assignments: assignments,
                    generated_content: Some(serde_json::to_value(&qm_eval).unwrap_or_default()),
                    xp_awarded: xp,
                })
            }
            Err(e) => {
                // Even if Pete is offline, include QM results
                Ok(OrchestrationResponse {
                    next_phase: AddiecrapeyePhase::Contrast,
                    player_instructions: format!(
                        "[Pete offline] Automated QM evaluation:\n{}\n\nReview manually. Error: {}",
                        qm_summary, e
                    ),
                    party_assignments: std::collections::HashMap::new(),
                    generated_content: Some(serde_json::to_value(&qm_eval).unwrap_or_default()),
                    xp_awarded: if qm_eval.meets_standards { 15 } else { 5 },
                })
            }
        }
    }

    /// Build an IdContract from orchestration context for QM evaluation.
    /// Extracts objectives, milestones, and action mapping from the player_context JSON.
    fn build_contract_from_context(request: &OrchestrationRequest) -> trinity_protocol::IdContract {
        use trinity_protocol::id_contract::QuestMilestone;
        use trinity_protocol::{ActionMap, IdContract, LearningObjective, UserClass};

        let mut contract = IdContract::new(
            request.quest_id.clone(),
            request
                .player_context
                .get("subject")
                .and_then(|v| v.as_str())
                .unwrap_or(&request.quest_id),
            UserClass::InstructionalDesigner,
        );

        // Extract learning objectives from context if present
        if let Some(objectives) = request
            .player_context
            .get("objectives")
            .and_then(|v| v.as_array())
        {
            for obj in objectives {
                contract.learning_objectives.push(LearningObjective {
                    verb: obj
                        .get("verb")
                        .and_then(|v| v.as_str())
                        .unwrap_or("identify")
                        .to_string(),
                    content: obj
                        .get("content")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    condition: obj
                        .get("condition")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    criterion: obj
                        .get("criterion")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                });
            }
        }

        // If no structured objectives, create one from the quest objectives list
        if contract.learning_objectives.is_empty() {
            for obj_text in &request.objectives {
                contract.learning_objectives.push(LearningObjective {
                    verb: "demonstrate".to_string(),
                    content: obj_text.clone(),
                    condition: String::new(),
                    criterion: String::new(),
                });
            }
        }

        // Extract action map if present
        if let Some(goal) = request
            .player_context
            .get("measurable_goal")
            .and_then(|v| v.as_str())
        {
            contract.action_map = Some(ActionMap {
                measurable_goal: goal.to_string(),
                observable_behaviors: request
                    .player_context
                    .get("observable_behaviors")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            });
        }

        // Build milestones from objectives
        for (i, obj) in request.objectives.iter().enumerate() {
            contract.milestones.push(QuestMilestone {
                order: (i + 1) as u32,
                title: obj.clone(),
                deliverable: obj.clone(),
                coal_cost: 20.0,
                completed: false,
            });
        }

        contract.recalculate_coal();
        contract
    }

    async fn contrast_quest(
        &self,
        request: &OrchestrationRequest,
    ) -> Result<OrchestrationResponse> {
        let system = self.get_system_prompt(AddiecrapeyePhase::Contrast, request.mode);
        let context = Self::build_quest_context(request);

        match self.call_pete(&system, &context).await {
            Ok(response) => {
                let mut assignments = std::collections::HashMap::new();
                assignments.insert(
                    "aesthetics".to_string(),
                    "Visual hierarchy and boundary evaluation".to_string(),
                );
                Ok(OrchestrationResponse {
                    next_phase: AddiecrapeyePhase::Repetition,
                    player_instructions: response,
                    party_assignments: assignments,
                    generated_content: None,
                    xp_awarded: 0,
                })
            }
            Err(e) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Repetition,
                player_instructions: format!(
                    "[Pete offline] Apply contrast and visual hierarchy. Error: {}",
                    e
                ),
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 0,
            }),
        }
    }

    async fn repetition_quest(
        &self,
        request: &OrchestrationRequest,
    ) -> Result<OrchestrationResponse> {
        let system = self.get_system_prompt(AddiecrapeyePhase::Repetition, request.mode);
        let context = Self::build_quest_context(request);

        match self.call_pete(&system, &context).await {
            Ok(response) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Alignment,
                player_instructions: response,
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 50,
            }),
            Err(e) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Alignment,
                player_instructions: format!("[Pete offline] Solidify the core loop. Error: {}", e),
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 25,
            }),
        }
    }

    async fn alignment_quest(
        &self,
        request: &OrchestrationRequest,
    ) -> Result<OrchestrationResponse> {
        let system = self.get_system_prompt(AddiecrapeyePhase::Alignment, request.mode);
        let context = Self::build_quest_context(request);

        match self.call_pete(&system, &context).await {
            Ok(response) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Proximity,
                player_instructions: response,
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 0,
            }),
            Err(e) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Proximity,
                player_instructions: format!("[Pete offline] Run alignment check. Error: {}", e),
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 0,
            }),
        }
    }

    async fn proximity_quest(
        &self,
        request: &OrchestrationRequest,
    ) -> Result<OrchestrationResponse> {
        let system = self.get_system_prompt(AddiecrapeyePhase::Proximity, request.mode);
        let context = Self::build_quest_context(request);

        match self.call_pete(&system, &context).await {
            Ok(response) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Envision,
                player_instructions: response,
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 0,
            }),
            Err(e) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Envision,
                player_instructions: format!("[Pete offline] Optimize UX proximity. Error: {}", e),
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 0,
            }),
        }
    }

    async fn envision_quest(
        &self,
        request: &OrchestrationRequest,
    ) -> Result<OrchestrationResponse> {
        let system = self.get_system_prompt(AddiecrapeyePhase::Envision, request.mode);
        let context = Self::build_quest_context(request);

        match self.call_pete(&system, &context).await {
            Ok(response) => {
                let mut assignments = std::collections::HashMap::new();
                assignments.insert(
                    "pete".to_string(),
                    "Macro review and meta-awareness".to_string(),
                );
                Ok(OrchestrationResponse {
                    next_phase: AddiecrapeyePhase::Yoke,
                    player_instructions: response,
                    party_assignments: assignments,
                    generated_content: None,
                    xp_awarded: 0,
                })
            }
            Err(e) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Yoke,
                player_instructions: format!(
                    "[Pete offline] Step back and envision the whole. Error: {}",
                    e
                ),
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 0,
            }),
        }
    }

    async fn yoke_quest(&self, request: &OrchestrationRequest) -> Result<OrchestrationResponse> {
        let system = self.get_system_prompt(AddiecrapeyePhase::Yoke, request.mode);
        let context = Self::build_quest_context(request);

        match self.call_pete(&system, &context).await {
            Ok(response) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Evolve,
                player_instructions: response,
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 100,
            }),
            Err(e) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Evolve,
                player_instructions: format!(
                    "[Pete offline] Bind all systems together. Error: {}",
                    e
                ),
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 50,
            }),
        }
    }

    async fn evolve_quest(&self, request: &OrchestrationRequest) -> Result<OrchestrationResponse> {
        let system = self.get_system_prompt(AddiecrapeyePhase::Evolve, request.mode);
        let context = Self::build_quest_context(request);

        match self.call_pete(&system, &context).await {
            Ok(response) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Analysis,
                player_instructions: response,
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 0,
            }),
            Err(e) => Ok(OrchestrationResponse {
                next_phase: AddiecrapeyePhase::Analysis,
                player_instructions: format!(
                    "[Pete offline] The Golem evolves. Ready for next quest. Error: {}",
                    e
                ),
                party_assignments: std::collections::HashMap::new(),
                generated_content: None,
                xp_awarded: 0,
            }),
        }
    }

    // Public getters for internal state

    pub async fn current_phase(&self) -> AddiecrapeyePhase {
        *self.current_phase.read().await
    }

    #[allow(dead_code)] // Called by workflow engine when new quests are created
    pub async fn add_quest(&self, quest_id: String) {
        let mut quests = self.active_quests.write().await;
        if !quests.contains(&quest_id) {
            quests.push(quest_id);
        }
    }

    #[allow(dead_code)] // Called by workflow engine when quests complete or fail
    pub async fn remove_quest(&self, quest_id: String) {
        let mut quests = self.active_quests.write().await;
        quests.retain(|id| id != &quest_id);
    }
}
