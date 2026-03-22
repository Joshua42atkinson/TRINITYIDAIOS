//! Sword & Shield Workflow Orchestrator
//!
//! The autonomous work loop that coordinates Opus (Shield/Thinker) and REAP (Sword/Coder).
//!
//! Turn-based combat metaphor:
//! 1. Shield analyzes the quest and creates a battle plan
//! 2. For each step:
//!    - If code generation: Sword strikes (generates code)
//!    - Shield reviews the result (parry check)
//!    - If review fails: Sword retries with feedback (riposte)
//! 3. Shield declares victory or retreat

use chrono::Utc;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use crate::llama::{ChatMessage, LlamaClient};
use crate::prompts;
use crate::quest::{LogEntry, Quest, QuestBoard, QuestResults, QuestStatus};

/// Shared state for the workflow engine
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowState {
    pub role_id: String,
    pub role_name: String,
    pub status: EngineStatus,
    pub current_quest: Option<String>,
    pub quests_completed: u32,
    pub quests_failed: u32,
    pub total_code_generated_lines: u64,
    pub uptime_secs: u64,
    pub opus_healthy: bool,
    pub reap_healthy: bool,
    pub timeouts_hit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineStatus {
    Starting,
    Idle,
    Working,
    Paused,
    Stopped,
    Error,
}

/// Configuration for quest execution timeouts
const MAX_STEP_DURATION_SECS: u64 = 300; // 5 minutes per step
const MAX_PLAN_DURATION_SECS: u64 = 600; // 10 minutes for planning

impl Default for WorkflowState {
    fn default() -> Self {
        Self {
            role_id: "tempo".to_string(),
            role_name: "The Engineer".to_string(),
            status: EngineStatus::Starting,
            current_quest: None,
            quests_completed: 0,
            quests_failed: 0,
            total_code_generated_lines: 0,
            uptime_secs: 0,
            opus_healthy: false,
            reap_healthy: false,
            timeouts_hit: 0,
        }
    }
}

/// The workflow engine that runs the autonomous loop
pub struct WorkflowEngine {
    pub opus: LlamaClient,
    pub reap: LlamaClient,
    pub quest_board: QuestBoard,
    pub workspace_root: PathBuf,
    pub state: Arc<RwLock<WorkflowState>>,
    pub autonomous: Arc<RwLock<bool>>,
    pub role_id: String,
    pub cow_catcher: Arc<RwLock<crate::cow_catcher::CowCatcher>>,
    pub creative: trinity_comfy::CreativeStudio,
}

impl WorkflowEngine {
    pub fn new(
        opus_url: &str,
        reap_url: &str,
        quest_dir: PathBuf,
        workspace_root: PathBuf,
        state: Arc<RwLock<WorkflowState>>,
        role_id: String,
    ) -> Self {
        Self {
            opus: LlamaClient::new(opus_url),
            reap: LlamaClient::new(reap_url),
            quest_board: QuestBoard::new(quest_dir),
            workspace_root,
            state,
            autonomous: Arc::new(RwLock::new(false)),
            role_id,
            cow_catcher: Arc::new(RwLock::new(crate::cow_catcher::CowCatcher::new())),
            creative: trinity_comfy::CreativeStudio::new(),
        }
    }

    /// Start the autonomous work loop
    pub async fn run_autonomous(&self) {
        info!("Autonomous work loop starting...");
        {
            let mut auto = self.autonomous.write().await;
            *auto = true;
        }

        loop {
            // Check if we should keep running
            {
                let auto = self.autonomous.read().await;
                if !*auto {
                    info!("Autonomous mode disabled, stopping work loop");
                    break;
                }
            }

            // Update health status
            {
                let mut state = self.state.write().await;
                state.opus_healthy = self.opus.is_healthy().await;
                state.reap_healthy = self.reap.is_healthy().await;
            }

            // Check for available quests
            match self.quest_board.list_available().await {
                Ok(quests) if !quests.is_empty() => {
                    let quest = &quests[0];
                    info!("Found quest: {} — {}", quest.id, quest.title);

                    // Claim and execute
                    match self.quest_board.claim(&quest.id).await {
                        Ok(mut claimed) => {
                            {
                                let mut state = self.state.write().await;
                                state.status = EngineStatus::Working;
                                state.current_quest = Some(claimed.title.clone());
                            }

                            match self.execute_quest(&mut claimed).await {
                                Ok(results) => {
                                    if let Err(e) =
                                        self.quest_board.complete(&mut claimed, results).await
                                    {
                                        error!("Failed to mark quest complete: {}", e);
                                    }
                                    let mut state = self.state.write().await;
                                    state.quests_completed += 1;
                                    state.current_quest = None;
                                    state.status = EngineStatus::Idle;
                                }
                                Err(e) => {
                                    error!("Quest execution failed: {}", e);
                                    // Report to Cow Catcher
                                    let mut cow_catcher = self.cow_catcher.write().await;
                                    cow_catcher.report_quest_failure(&claimed.id, &e.to_string());

                                    if let Err(e2) =
                                        self.quest_board.fail(&mut claimed, &e.to_string()).await
                                    {
                                        error!("Failed to mark quest failed: {}", e2);
                                    }
                                    let mut state = self.state.write().await;
                                    state.quests_failed += 1;
                                    state.current_quest = None;
                                    state.status = EngineStatus::Idle;
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to claim quest: {}", e);
                        }
                    }
                }
                Ok(_) => {
                    // No quests available — idle
                    {
                        let mut state = self.state.write().await;
                        state.status = EngineStatus::Idle;
                        state.current_quest = None;
                    }
                    // Sleep before checking again (30 second poll interval)
                    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                }
                Err(e) => {
                    warn!("Failed to list quests: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                }
            }

            // Brief pause between quests
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }

    /// Stop the autonomous loop
    pub async fn stop_autonomous(&self) {
        let mut auto = self.autonomous.write().await;
        *auto = false;
        info!("Autonomous mode stop requested");
    }

    /// Execute a single quest using the Sword & Shield pattern
    pub async fn execute_quest(&self, quest: &mut Quest) -> anyhow::Result<QuestResults> {
        info!("=== EXECUTING QUEST: {} ===", quest.title);

        // Git safety: track modified files for rollback on failure
        let mut all_modified_files: Vec<String> = Vec::new();

        // Phase 1: Read context files
        let context_files = self.read_context_files(&quest.context_files).await;

        // Phase 2: Shield plans (Opus)
        quest.status = QuestStatus::Planning;
        self.quest_board.update_active(quest).await?;

        let plan_prompt =
            prompts::opus_plan_prompt(&quest.title, &quest.description, &context_files);

        quest.log.push(LogEntry {
            timestamp: Utc::now(),
            step: "plan".to_string(),
            model: "opus".to_string(),
            message: "Shield analyzing quest and creating plan...".to_string(),
        });

        let primary_system = prompts::primary_prompt(&self.role_id);
        let secondary_system = prompts::secondary_prompt(&self.role_id);

        info!("[Primary] Planning quest...");
        let plan_response = match tokio::time::timeout(
            std::time::Duration::from_secs(MAX_PLAN_DURATION_SECS),
            self.opus.chat(
                &[
                    ChatMessage::system(primary_system),
                    ChatMessage::user(&plan_prompt),
                ],
                4096,
                0.3,
            ),
        )
        .await
        {
            Ok(Ok(response)) => response,
            Ok(Err(e)) => {
                error!("Planning failed: {}", e);
                return Err(e);
            }
            Err(_) => {
                error!("Planning timed out after {}s", MAX_PLAN_DURATION_SECS);
                let mut state = self.state.write().await;
                state.timeouts_hit += 1;

                // Report to Cow Catcher for debugging
                let mut cow_catcher = self.cow_catcher.write().await;
                cow_catcher.report_timeout("planning", MAX_PLAN_DURATION_SECS, "opus");

                return Err(anyhow::anyhow!(
                    "Planning timed out after {}s",
                    MAX_PLAN_DURATION_SECS
                ));
            }
        };

        info!("[Shield] Plan created ({} chars)", plan_response.len());

        // Try to parse plan as JSON, fall back to raw text
        let plan_json: serde_json::Value =
            serde_json::from_str(&plan_response).unwrap_or_else(|_| {
                // Try to extract JSON from the response (Opus might wrap it in text)
                if let Some(start) = plan_response.find('{') {
                    if let Some(end) = plan_response.rfind('}') {
                        serde_json::from_str(&plan_response[start..=end])
                            .unwrap_or_else(|_| serde_json::json!({"raw_plan": plan_response}))
                    } else {
                        serde_json::json!({"raw_plan": plan_response})
                    }
                } else {
                    serde_json::json!({"raw_plan": plan_response})
                }
            });

        quest.plan = Some(plan_json.clone());
        self.quest_board.update_active(quest).await?;

        // Phase 3: Execute plan steps
        quest.status = QuestStatus::InProgress;
        self.quest_board.update_active(quest).await?;

        let mut files_modified = Vec::new();
        let mut files_created = Vec::new();
        let mut verification_output = String::new();

        // Extract steps from plan
        let steps = plan_json["steps"].as_array();
        if let Some(steps) = steps {
            for step in steps {
                let step_type = step["type"].as_str().unwrap_or("unknown");
                let target = step["target"].as_str().unwrap_or("");
                let spec = step.get("spec").and_then(|s| s.as_str()).unwrap_or("");
                let command = step.get("command").and_then(|s| s.as_str()).unwrap_or("");
                let _reason = step.get("reason").and_then(|s| s.as_str()).unwrap_or("");

                info!("[Step] type={}, target={}", step_type, target);

                match step_type {
                    "read_file" => {
                        // Read a file for context (already done in context phase, but Opus may want more)
                        let path = self.workspace_root.join(target);
                        match tokio::fs::read_to_string(&path).await {
                            Ok(content) => {
                                quest.log.push(LogEntry {
                                    timestamp: Utc::now(),
                                    step: format!("read_file:{}", target),
                                    model: "system".to_string(),
                                    message: format!("Read {} ({} bytes)", target, content.len()),
                                });
                            }
                            Err(e) => {
                                warn!("Failed to read {}: {}", target, e);
                            }
                        }
                    }

                    "generate_code" => {
                        // Sword generates code (patch mode for large files)
                        let target_path = self.workspace_root.join(target);
                        let existing = tokio::fs::read_to_string(&target_path).await.ok();

                        let code_prompt =
                            prompts::reap_code_prompt(spec, target, existing.as_deref());

                        info!("[Secondary] Generating code for {}...", target);
                        let generated = match tokio::time::timeout(
                            std::time::Duration::from_secs(MAX_STEP_DURATION_SECS),
                            self.reap.chat(
                                &[
                                    ChatMessage::system(secondary_system),
                                    ChatMessage::user(&code_prompt),
                                ],
                                8192,
                                0.2,
                            ),
                        )
                        .await
                        {
                            Ok(Ok(response)) => response,
                            Ok(Err(e)) => {
                                warn!("Code generation failed for {}: {}", target, e);
                                quest.log.push(LogEntry {
                                    timestamp: Utc::now(),
                                    step: format!("generate_code:{}", target),
                                    model: "reap".to_string(),
                                    message: format!("ERROR: {}", e),
                                });
                                continue; // Skip this step
                            }
                            Err(_) => {
                                warn!(
                                    "Code generation timed out for {} after {}s",
                                    target, MAX_STEP_DURATION_SECS
                                );
                                let mut state = self.state.write().await;
                                state.timeouts_hit += 1;

                                // Report to Cow Catcher
                                let mut cow_catcher = self.cow_catcher.write().await;
                                cow_catcher.report_timeout(
                                    &format!("generate_code:{}", target),
                                    MAX_STEP_DURATION_SECS,
                                    "reap",
                                );

                                quest.log.push(LogEntry {
                                    timestamp: Utc::now(),
                                    step: format!("generate_code:{}", target),
                                    model: "reap".to_string(),
                                    message: format!("TIMEOUT after {}s", MAX_STEP_DURATION_SECS),
                                });
                                continue; // Skip this step
                            }
                        };

                        // Determine if output is a patch or complete file
                        let raw_output = extract_code_block(&generated);
                        let (final_code, mode) = if let Some(ref orig) = existing {
                            if is_patch(&raw_output) {
                                match apply_patch(orig, &raw_output) {
                                    Ok(patched) => (patched, "patch"),
                                    Err(e) => {
                                        warn!("Patch apply failed: {}. Using raw output.", e);
                                        (raw_output.clone(), "raw_fallback")
                                    }
                                }
                            } else if orig.lines().count() > 100
                                && raw_output.lines().count() < orig.lines().count() / 2
                            {
                                // Safety: if existing file is large and output is suspiciously small, reject
                                warn!("Generated output ({} lines) is <50% of original ({} lines). Skipping write to prevent truncation.",
                                    raw_output.lines().count(), orig.lines().count());
                                quest.log.push(LogEntry {
                                    timestamp: Utc::now(),
                                    step: format!("generate_code:{}", target),
                                    model: "reap".to_string(),
                                    message: format!("REJECTED: output {} lines vs original {} lines (truncation guard)",
                                        raw_output.lines().count(), orig.lines().count()),
                                });
                                continue; // Skip this step
                            } else {
                                (raw_output.clone(), "complete")
                            }
                        } else {
                            (raw_output.clone(), "new_file")
                        };

                        let line_count = final_code.lines().count() as u64;

                        quest.log.push(LogEntry {
                            timestamp: Utc::now(),
                            step: format!("generate_code:{}", target),
                            model: "reap".to_string(),
                            message: format!(
                                "Generated {} lines for {} (mode: {})",
                                line_count, target, mode
                            ),
                        });

                        // Shield reviews
                        info!("[Primary] Reviewing generated code (mode: {})...", mode);
                        let review_content = if mode == "patch" {
                            format!("Applied patch to {}. Patch:\n{}", target, raw_output)
                        } else {
                            final_code.clone()
                        };
                        let review_prompt =
                            prompts::opus_review_prompt(spec, &review_content, target);
                        let review = match tokio::time::timeout(
                            std::time::Duration::from_secs(MAX_STEP_DURATION_SECS),
                            self.opus.chat(
                                &[
                                    ChatMessage::system(primary_system),
                                    ChatMessage::user(&review_prompt),
                                ],
                                2048,
                                0.2,
                            ),
                        )
                        .await
                        {
                            Ok(Ok(response)) => response,
                            Ok(Err(e)) => {
                                warn!("Review failed for {}: {}", target, e);
                                "Review failed - auto-approving".to_string()
                            }
                            Err(_) => {
                                warn!(
                                    "Review timed out for {} after {}s - auto-approving",
                                    target, MAX_STEP_DURATION_SECS
                                );
                                let mut state = self.state.write().await;
                                state.timeouts_hit += 1;

                                // Report to Cow Catcher
                                let mut cow_catcher = self.cow_catcher.write().await;
                                cow_catcher.report_timeout(
                                    &format!("review:{}", target),
                                    MAX_STEP_DURATION_SECS,
                                    "opus",
                                );

                                "Review timed out - auto-approving".to_string()
                            }
                        };

                        let approved = review.contains("\"approved\": true")
                            || review.contains("\"approved\":true")
                            || review.to_lowercase().contains("approved");

                        quest.log.push(LogEntry {
                            timestamp: Utc::now(),
                            step: format!("review:{}", target),
                            model: "opus".to_string(),
                            message: format!(
                                "Review: {}",
                                if approved {
                                    "APPROVED"
                                } else {
                                    "NEEDS REVISION"
                                }
                            ),
                        });

                        if approved {
                            if let Some(parent) = target_path.parent() {
                                tokio::fs::create_dir_all(parent).await?;
                            }
                            tokio::fs::write(&target_path, &final_code).await?;

                            if existing.is_some() {
                                files_modified.push(target.to_string());
                                all_modified_files.push(target.to_string());
                            } else {
                                files_created.push(target.to_string());
                            }

                            let mut state = self.state.write().await;
                            state.total_code_generated_lines += line_count;
                        } else {
                            // One retry — for patches, ask for a corrected patch
                            info!("[Secondary] Retrying with review feedback...");
                            let retry_instruction = if existing
                                .as_ref()
                                .map(|e| e.lines().count() > 100)
                                .unwrap_or(false)
                            {
                                "Fix the issues and output a corrected PATCH (not the whole file)."
                            } else {
                                "Fix the issues and output the corrected COMPLETE file."
                            };
                            let retry_prompt = format!(
                                "{}\n\n## Review Feedback:\n{}\n\n{}",
                                code_prompt, review, retry_instruction
                            );
                            let retried = match tokio::time::timeout(
                                std::time::Duration::from_secs(MAX_STEP_DURATION_SECS),
                                self.reap.chat(
                                    &[
                                        ChatMessage::system(secondary_system),
                                        ChatMessage::user(&retry_prompt),
                                    ],
                                    8192,
                                    0.15,
                                ),
                            )
                            .await
                            {
                                Ok(Ok(response)) => response,
                                Ok(Err(e)) => {
                                    warn!("Retry failed for {}: {}", target, e);
                                    continue; // Skip this step
                                }
                                Err(_) => {
                                    warn!(
                                        "Retry timed out for {} after {}s",
                                        target, MAX_STEP_DURATION_SECS
                                    );
                                    let mut state = self.state.write().await;
                                    state.timeouts_hit += 1;

                                    // Report to Cow Catcher
                                    let mut cow_catcher = self.cow_catcher.write().await;
                                    cow_catcher.report_timeout(
                                        &format!("retry:{}", target),
                                        MAX_STEP_DURATION_SECS,
                                        "reap",
                                    );

                                    continue; // Skip this step
                                }
                            };

                            let retry_raw = extract_code_block(&retried);
                            let retry_final = if let Some(ref orig) = existing {
                                if is_patch(&retry_raw) {
                                    apply_patch(orig, &retry_raw).unwrap_or(retry_raw)
                                } else {
                                    retry_raw
                                }
                            } else {
                                retry_raw
                            };

                            if let Some(parent) = target_path.parent() {
                                tokio::fs::create_dir_all(parent).await?;
                            }
                            tokio::fs::write(&target_path, &retry_final).await?;

                            if existing.is_some() {
                                files_modified.push(target.to_string());
                                all_modified_files.push(target.to_string());
                            } else {
                                files_created.push(target.to_string());
                            }

                            let line_count = retry_final.lines().count() as u64;
                            let mut state = self.state.write().await;
                            state.total_code_generated_lines += line_count;
                        }
                    }

                    "write_file" => {
                        // Direct file write (specs, docs, configs)
                        let target_path = self.workspace_root.join(target);
                        let content = step.get("content").and_then(|c| c.as_str()).unwrap_or(spec);
                        if let Some(parent) = target_path.parent() {
                            tokio::fs::create_dir_all(parent).await?;
                        }
                        tokio::fs::write(&target_path, content).await?;
                        files_created.push(target.to_string());
                    }

                    "shell" => {
                        // Execute a shell command (for verification)
                        info!("[Shell] Running: {}", command);
                        let output = tokio::process::Command::new("bash")
                            .arg("-c")
                            .arg(command)
                            .current_dir(&self.workspace_root)
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .output()
                            .await?;

                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        let exit_code = output.status.code().unwrap_or(-1);

                        let shell_result =
                            format!("$ {}\nExit: {}\n{}\n{}", command, exit_code, stdout, stderr);
                        verification_output.push_str(&shell_result);
                        verification_output.push('\n');

                        quest.log.push(LogEntry {
                            timestamp: Utc::now(),
                            step: format!("shell:{}", command),
                            model: "system".to_string(),
                            message: format!("Exit code: {}", exit_code),
                        });
                    }

                    "review" | "analyze" => {
                        // Opus-only analysis step
                        let target_path = self.workspace_root.join(target);
                        let content = tokio::fs::read_to_string(&target_path)
                            .await
                            .unwrap_or_default();
                        let criteria = step
                            .get("criteria")
                            .and_then(|c| c.as_str())
                            .unwrap_or("general review");

                        let analysis = self
                            .opus
                            .chat(
                                &[
                                    ChatMessage::system(primary_system),
                                    ChatMessage::user(&format!(
                                        "Analyze `{}`:\n```\n{}\n```\nCriteria: {}",
                                        target, content, criteria
                                    )),
                                ],
                                2048,
                                0.3,
                            )
                            .await?;

                        quest.log.push(LogEntry {
                            timestamp: Utc::now(),
                            step: format!("analyze:{}", target),
                            model: "opus".to_string(),
                            message: analysis,
                        });
                    }

                    "generate_image" => {
                        // Artist role: Generate image via ComfyUI
                        info!("[ComfyUI] Generating image: {}", spec);

                        match self.creative.comfyui.generate_image(&trinity_comfy::ImageRequest {
                            prompt: spec.to_string(),
                            negative_prompt: None,
                            width: 512,
                            height: 512,
                            steps: 4,
                            seed: None,
                        }).await {
                            Ok(img) => {
                                // Save to assets directory
                                let assets_dir = self.workspace_root.join("assets/generated");
                                if let Err(e) = tokio::fs::create_dir_all(&assets_dir).await {
                                    warn!("Failed to create assets dir: {}", e);
                                }

                                let filename = img.filename.clone();
                                let image_path = assets_dir.join(&filename);

                                match tokio::fs::write(&image_path, &img.data).await {
                                    Ok(_) => {
                                        info!(
                                            "[ComfyUI] Saved image: {}",
                                            image_path.display()
                                        );
                                        quest.log.push(LogEntry {
                                            timestamp: Utc::now(),
                                            step: format!("generate_image:{}", filename),
                                            model: "comfyui".to_string(),
                                            message: format!(
                                                "Generated {} ({} bytes)",
                                                filename,
                                                img.data.len()
                                            ),
                                        });
                                        files_created
                                            .push(format!("assets/generated/{}", filename));
                                    }
                                    Err(e) => {
                                        warn!("Failed to save image: {}", e);
                                        quest.log.push(LogEntry {
                                            timestamp: Utc::now(),
                                            step: "generate_image".to_string(),
                                            model: "comfyui".to_string(),
                                            message: format!("ERROR saving: {}", e),
                                        });
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("ComfyUI generation failed: {}", e);
                                    quest.log.push(LogEntry {
                                        timestamp: Utc::now(),
                                        step: "generate_image".to_string(),
                                        model: "comfyui".to_string(),
                                        message: format!("ERROR: {}", e),
                                    });
                                }
                            }
                        }

                    _ => {
                        warn!("Unknown step type: {}", step_type);
                    }
                }

                self.quest_board.update_active(quest).await?;
            }
        } else {
            // No structured steps — use the raw plan as a single generation task
            info!("[Secondary] No structured plan — generating code from raw plan...");
            let generated = self
                .reap
                .chat(
                    &[
                        ChatMessage::system(secondary_system),
                        ChatMessage::user(&format!(
                            "## Quest: {}\n\n{}\n\n## Plan:\n{}\n\nGenerate the required code.",
                            quest.title, quest.description, plan_response
                        )),
                    ],
                    8192,
                    0.2,
                )
                .await?;

            quest.log.push(LogEntry {
                timestamp: Utc::now(),
                step: "generate_code:raw".to_string(),
                model: "reap".to_string(),
                message: format!("Generated {} chars from raw plan", generated.len()),
            });
        }

        // Run verification commands
        let mut verification_failed = false;
        for cmd in &quest.verify_commands {
            info!("[Verify] Running: {}", cmd);
            let output = tokio::process::Command::new("bash")
                .arg("-c")
                .arg(cmd)
                .current_dir(&self.workspace_root)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await?;

            let exit_code = output.status.code().unwrap_or(-1);
            let result = format!(
                "$ {}\nExit: {}\n{}\n{}",
                cmd,
                exit_code,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
            verification_output.push_str(&result);

            if exit_code != 0 {
                verification_failed = true;
                // Report to Cow Catcher for debugging
                let mut cow_catcher = self.cow_catcher.write().await;
                cow_catcher.report_compilation_error(cmd, &format!("Exit code: {}", exit_code));
            }
        }

        // Git safety: if verification failed, restore modified files
        if verification_failed && !all_modified_files.is_empty() {
            warn!(
                "[Git] Verification failed — restoring {} modified files",
                all_modified_files.len()
            );
            git_restore_files(&self.workspace_root, &all_modified_files).await;
            quest.log.push(LogEntry {
                timestamp: Utc::now(),
                step: "git_restore".to_string(),
                model: "system".to_string(),
                message: format!(
                    "Restored files after verification failure: {:?}",
                    all_modified_files
                ),
            });
        }

        // Final Shield verdict
        let verdict = self.opus.chat(
            &[
                ChatMessage::system(primary_system),
                ChatMessage::user(&format!(
                    "Quest '{}' execution complete.\n\nFiles modified: {:?}\nFiles created: {:?}\nVerification:\n{}\n\nProvide a brief verdict: was this quest successful?",
                    quest.title, files_modified, files_created, verification_output
                )),
            ],
            1024,
            0.3,
        ).await.unwrap_or_else(|_| "Verdict unavailable".to_string());

        info!("=== QUEST COMPLETE: {} ===", quest.title);

        Ok(QuestResults {
            files_modified,
            files_created,
            verification_output,
            opus_verdict: verdict,
            xp_earned: quest.difficulty_class as u32 * 10,
        })
    }

    /// Read context files from the workspace
    async fn read_context_files(&self, paths: &[String]) -> Vec<(String, String)> {
        let mut results = Vec::new();
        for path in paths {
            let full_path = self.workspace_root.join(path);
            match tokio::fs::read_to_string(&full_path).await {
                Ok(content) => {
                    // Truncate very large files
                    let truncated = if content.len() > 20_000 {
                        format!(
                            "{}...\n[truncated: {} bytes total]",
                            &content[..20_000],
                            content.len()
                        )
                    } else {
                        content
                    };
                    results.push((path.clone(), truncated));
                }
                Err(e) => {
                    warn!("Failed to read context file {}: {}", path, e);
                }
            }
        }
        results
    }

    /// Execute a single quest by ID (manual trigger)
    pub async fn execute_quest_by_id(&self, quest_id: &str) -> anyhow::Result<QuestResults> {
        let mut quest = self.quest_board.claim(quest_id).await?;
        {
            let mut state = self.state.write().await;
            state.status = EngineStatus::Working;
            state.current_quest = Some(quest.title.clone());
        }

        let result = self.execute_quest(&mut quest).await;

        match result {
            Ok(results) => {
                self.quest_board
                    .complete(&mut quest, results.clone())
                    .await?;
                let mut state = self.state.write().await;
                state.quests_completed += 1;
                state.current_quest = None;
                state.status = EngineStatus::Idle;
                Ok(results)
            }
            Err(e) => {
                // Report to Cow Catcher for debugging
                let mut cow_catcher = self.cow_catcher.write().await;
                cow_catcher.report_quest_failure(&quest.id, &e.to_string());

                self.quest_board.fail(&mut quest, &e.to_string()).await?;
                let mut state = self.state.write().await;
                state.quests_failed += 1;
                state.current_quest = None;
                state.status = EngineStatus::Idle;
                Err(e)
            }
        }
    }
}

/// Extract code from markdown fences
fn extract_code_block(text: &str) -> String {
    // Try to find ```rust ... ``` or ``` ... ```
    if let Some(start) = text.find("```") {
        let after_fence = &text[start + 3..];
        // Skip language identifier
        let code_start = after_fence.find('\n').map(|i| i + 1).unwrap_or(0);
        let code_content = &after_fence[code_start..];
        if let Some(end) = code_content.find("```") {
            return code_content[..end].trim().to_string();
        }
    }
    // No fences found — return as-is
    text.trim().to_string()
}

/// Check if the generated output is a unified diff patch
fn is_patch(text: &str) -> bool {
    text.contains("--- a/") || text.contains("@@ -")
}

/// Apply a unified diff patch to existing file content
/// Returns the patched content or an error
fn apply_patch(original: &str, patch_text: &str) -> Result<String, String> {
    let orig_lines: Vec<&str> = original.lines().collect();
    let mut result = orig_lines.iter().map(|s| s.to_string()).collect::<Vec<_>>();

    // Parse hunks from the patch
    let mut in_hunk = false;
    let mut _hunk_start: usize = 0;
    let mut removals: Vec<usize> = Vec::new();
    let mut additions: Vec<(usize, String)> = Vec::new();
    let mut current_line: usize = 0;

    for line in patch_text.lines() {
        if line.starts_with("@@ -") {
            // Parse hunk header: @@ -START,COUNT +START,COUNT @@
            in_hunk = true;
            removals.clear();
            additions.clear();
            if let Some(start_str) = line.split('-').nth(1) {
                if let Some(num) = start_str.split(',').next() {
                    if let Ok(n) = num.trim().parse::<usize>() {
                        _hunk_start = n.saturating_sub(1); // Convert 1-indexed to 0-indexed
                        current_line = _hunk_start;
                    }
                }
            }
        } else if in_hunk {
            if line.starts_with('-') {
                // Line to remove
                removals.push(current_line);
                current_line += 1;
            } else if line.starts_with('+') {
                // Line to add
                if let Some(stripped) = line.strip_prefix('+') {
                    additions.push((current_line, stripped.to_string()));
                }
            } else if line.starts_with(' ') || line.is_empty() {
                // Context line
                current_line += 1;
            } else if line.starts_with("--- ") || line.starts_with("+++ ") {
                // File header, skip
            } else {
                // End of hunk or unknown line
                in_hunk = false;
            }
        }
    }

    // Apply removals in reverse order (so indices don't shift)
    let mut removals_sorted = removals.clone();
    removals_sorted.sort_unstable();
    removals_sorted.dedup();
    for &idx in removals_sorted.iter().rev() {
        if idx < result.len() {
            result.remove(idx);
        }
    }

    // Apply additions (adjust for removed lines)
    let removed_before =
        |pos: usize| -> usize { removals_sorted.iter().filter(|&&r| r < pos).count() };
    for (pos, content) in &additions {
        let adjusted = pos.saturating_sub(removed_before(*pos));
        let insert_at = adjusted.min(result.len());
        result.insert(insert_at, content.clone());
    }

    if result.is_empty() {
        return Err("Patch produced empty result".to_string());
    }

    Ok(result.join("\n") + "\n")
}

/// Git safety: create a checkpoint before modifying files
#[allow(dead_code)] // Git safety: creates a stash checkpoint before modifying files
async fn git_stash_create(workspace: &std::path::Path) -> Option<String> {
    let output = tokio::process::Command::new("git")
        .args(["stash", "create", "trinity-quest-checkpoint"])
        .current_dir(workspace)
        .output()
        .await
        .ok()?;
    let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if hash.is_empty() {
        None
    } else {
        Some(hash)
    }
}

/// Git safety: restore a file from the index if quest fails
async fn git_restore_files(workspace: &std::path::Path, files: &[String]) {
    for file in files {
        let _ = tokio::process::Command::new("git")
            .args(["checkout", "--", file])
            .current_dir(workspace)
            .output()
            .await;
        info!("[Git] Restored: {}", file);
    }
}
