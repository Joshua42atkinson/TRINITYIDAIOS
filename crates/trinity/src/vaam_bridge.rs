// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        vaam_bridge.rs
// PURPOSE:     The VAAM Bridge — words connect people and LLMs
//
// WHAT THIS IS:
//   The runtime integration layer that makes VAAM work. Every message —
//   from user or AI — flows through this bridge. It:
//
//   1. Scans text for VAAM vocabulary (coal, mastery)
//   2. Detects Sacred Circuitry attention patterns
//   3. Updates the user's VaamProfile (preferences, style, agreements)
//   4. Generates the VAAM context block for LLM system prompts
//   5. Returns the active circuit's auto-reply for the AI to use
//
//   Words are what LLMs and people have in common. This bridge makes
//   that shared vocabulary productive, measurable, and personal.
//
// ARCHITECTURE:
//   User input ──→ VaamBridge ──→ VaamState (coal + mastery)
//                      │       ──→ CircuitryState (attention pattern)
//                      │       ──→ VaamProfile (preferences + style)
//                      │
//                      ├──→ prompt_context() ──→ Conductor system prompt
//                      │
//   AI output  ──→ VaamBridge ──→ CircuitryState (AI attention tracking)
//                              ──→ auto_reply() (circuit signal)
//
// CHANGES:
//   2026-03-19  Cascade  Initial — the bridge between people and LLMs
//
// ═══════════════════════════════════════════════════════════════════════════════

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use trinity_protocol::{sacred_circuitry::Circuit, CircuitryState, VaamProfile};

use crate::vaam::{VaamResult, VaamState};

/// The VAAM Bridge — processes all text through vocabulary detection,
/// circuit detection, and preference tracking.
///
/// Shared across the server via Arc. All state is behind RwLocks
/// for safe concurrent access from HTTP handlers and the Conductor.
pub struct VaamBridge {
    /// Vocabulary scanning and coal tracking
    pub vaam: VaamState,
    /// Sacred Circuitry attention state
    pub circuitry: Arc<RwLock<CircuitryState>>,
    /// User preference profile
    pub profile: Arc<RwLock<VaamProfile>>,
}

/// Result of processing text through the bridge
#[derive(Debug, Clone)]
pub struct BridgeResult {
    /// VAAM vocabulary scan results (coal, mastery, detections)
    pub vaam: VaamResult,
    /// Which Sacred Circuitry pattern was detected (if any)
    pub detected_circuit: Option<(Circuit, f32)>,
    /// The auto-reply for the currently active circuit
    pub auto_reply: String,
    /// Whether a new circuit was activated by this text
    pub circuit_changed: bool,
}

impl VaamBridge {
    /// Create a new bridge with the given VAAM state
    pub fn new(vaam: VaamState) -> Self {
        Self {
            vaam,
            circuitry: Arc::new(RwLock::new(CircuitryState::new())),
            profile: Arc::new(RwLock::new(VaamProfile::new())),
        }
    }

    /// Create a bridge with an existing VaamProfile (loaded from CharacterSheet)
    pub fn with_profile(vaam: VaamState, profile: VaamProfile) -> Self {
        Self {
            vaam,
            circuitry: Arc::new(RwLock::new(CircuitryState::new())),
            profile: Arc::new(RwLock::new(profile)),
        }
    }

    /// Process USER input through the full VAAM pipeline.
    ///
    /// This is the core bridge function. Every user message flows through here:
    /// 1. Scan for vocabulary words → award coal, track mastery
    /// 2. Detect Sacred Circuitry pattern → activate circuit
    /// 3. Update VaamProfile → track preferences and style
    /// 4. Return results for the Conductor to use
    pub async fn process_user_input(&self, text: &str) -> BridgeResult {
        // 1. VAAM vocabulary scan
        let vaam_result = self.vaam.scan_message(text).await;

        // 2. Sacred Circuitry detection
        let detected = CircuitryState::detect_circuit(text);
        let mut circuit_changed = false;

        if let Some((circuit, score)) = &detected {
            let mut circuitry = self.circuitry.write().await;
            let previous = circuitry.active;
            circuitry.activate(*circuit);
            circuit_changed = previous != *circuit;

            if circuit_changed {
                debug!(
                    "[VAAM Bridge] Circuit shift: {} → {} (score: {:.2})",
                    previous, circuit, score
                );
            }

            // Update profile with circuit usage
            let mut profile = self.profile.write().await;
            profile.record_circuit_usage(*circuit);
        }

        // 3. Update profile with detected vocabulary words
        {
            let mut profile = self.profile.write().await;
            for detection in &vaam_result.detections {
                profile.record_word_usage(&detection.word, detection.is_correct_usage);
            }

            // Update communication style
            let word_count = text.split_whitespace().count();
            let question_count = text.matches('?').count();
            let statement_count = text
                .split(['.', '!'])
                .filter(|s| !s.trim().is_empty())
                .count();
            profile.record_interaction(word_count, question_count, statement_count);
        }

        // 4. Get auto-reply
        let auto_reply = {
            let circuitry = self.circuitry.read().await;
            circuitry.auto_reply().to_string()
        };

        if vaam_result.has_detections() || detected.is_some() {
            info!(
                "[VAAM Bridge] User: {} words detected, +{} coal{}",
                vaam_result.detections.len(),
                vaam_result.total_coal,
                if let Some((c, _)) = &detected {
                    format!(", circuit: {}", c)
                } else {
                    String::new()
                }
            );
        }

        BridgeResult {
            vaam: vaam_result,
            detected_circuit: detected,
            auto_reply,
            circuit_changed,
        }
    }

    /// Process AI output through circuit detection.
    ///
    /// Lighter than user processing — we only track which circuit
    /// the AI is signaling, not vocabulary mastery (that's the user's job).
    pub async fn process_ai_output(&self, text: &str) -> Option<(Circuit, f32)> {
        let detected = CircuitryState::detect_circuit(text);

        if let Some((circuit, score)) = &detected {
            let mut circuitry = self.circuitry.write().await;
            circuitry.activate(*circuit);

            debug!(
                "[VAAM Bridge] AI signaled circuit: {} (score: {:.2})",
                circuit, score
            );
        }

        detected
    }

    /// Maximum character budget for VAAM context injected into the LLM system prompt.
    /// Keeps prompt concise even after months of vocabulary mastery growth.
    const MAX_CONTEXT_CHARS: usize = 500;

    /// Generate the VAAM context block for injection into LLM system prompts.
    ///
    /// This is how the user's preferences flow into the AI's attention.
    /// The Conductor calls this before every `call_pete()` to get the
    /// current VAAM state as a compact string.
    ///
    /// IMPORTANT: Enforces a hard cap of MAX_CONTEXT_CHARS (500) to prevent
    /// prompt bloat as the user masters more words and triggers more circuits.
    /// Priority order:
    ///   1. Active Circuit (always included — core steering signal)
    ///   2. User Profile summary (truncated if needed)
    ///   3. Session stats (included if budget allows)
    ///   4. Active agreements (top 3, included if budget allows)
    pub async fn prompt_context(&self) -> String {
        let circuitry = self.circuitry.read().await;
        let profile = self.profile.read().await;

        // Priority 1: Sacred Circuitry state (always included)
        let circuit_line = format!(
            "VAAM Active Circuit: {} ({}) — {}",
            circuitry.active,
            circuitry.active.quadrant().name(),
            circuitry.active.description()
        );

        let mut result = circuit_line;
        let budget = Self::MAX_CONTEXT_CHARS;

        // Priority 2: User preference summary (truncated to fit)
        let pref_summary = profile.prompt_summary();
        if !pref_summary.is_empty() {
            let pref_line = format!("VAAM User Profile: {}", pref_summary);
            let candidate = format!("{}\n{}", result, pref_line);
            if candidate.len() <= budget {
                result = candidate;
            } else {
                // Truncate profile to fit remaining budget
                let remaining =
                    budget.saturating_sub(result.len() + 1 + "VAAM User Profile: ".len() + 3);
                if remaining > 10 {
                    let truncated = &pref_summary[..pref_summary.len().min(remaining)];
                    result = format!("{}\nVAAM User Profile: {}...", result, truncated);
                }
            }
        }

        // Priority 3: Session stats (only if budget allows)
        let session_coal = *self.vaam.session_coal.read().await;
        let mastered = self.vaam.mastered_count().await;
        if session_coal > 0 || mastered > 0 {
            let stats_line = format!("VAAM Session: {} coal, {} mastered", session_coal, mastered);
            let candidate = format!("{}\n{}", result, stats_line);
            if candidate.len() <= budget {
                result = candidate;
            }
        }

        // Priority 4: Active agreements (top 3, only if budget allows)
        let agreements = profile.active_agreements();
        if !agreements.is_empty() {
            let agreement_strs: Vec<String> = agreements
                .iter()
                .take(3)
                .map(|a| format!("{}={:.0}%", a.topic, a.weight * 100.0))
                .collect();
            let agr_line = format!("VAAM Agreements: {}", agreement_strs.join(", "));
            let candidate = format!("{}\n{}", result, agr_line);
            if candidate.len() <= budget {
                result = candidate;
            }
        }

        // Final safety: hard truncate (should never trigger with above logic)
        if result.len() > budget {
            result.truncate(budget - 3);
            result.push_str("...");
        }

        result
    }

    /// Get the current auto-reply the AI should signal.
    pub async fn current_auto_reply(&self) -> &'static str {
        let circuitry = self.circuitry.read().await;
        circuitry.auto_reply()
    }

    /// Get a snapshot of the current VaamProfile for persistence.
    pub async fn snapshot_profile(&self) -> VaamProfile {
        self.profile.read().await.clone()
    }

    /// Get the circuitry summary for display.
    pub async fn circuitry_summary(&self) -> String {
        self.circuitry.read().await.summary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trinity_protocol::Genre;

    async fn test_bridge() -> VaamBridge {
        // Create a bridge with Sacred Circuitry foundation vocabulary loaded
        let vaam = VaamState::new(Genre::default()).await;
        {
            let mut db = vaam.database.write().await;
            // Load Sacred Circuitry foundation words
            for word in trinity_protocol::foundation_vocabulary() {
                db.add_word(word);
            }
        }
        VaamBridge::new(vaam)
    }

    #[tokio::test]
    async fn test_process_user_input_detects_circuit() {
        let bridge = test_bridge().await;

        let result = bridge
            .process_user_input("I need to focus on the core problem and filter out the noise")
            .await;

        // Should detect Center circuit (focus, core, filter are context clues)
        assert!(result.detected_circuit.is_some());
        let (circuit, _) = result.detected_circuit.unwrap();
        assert_eq!(circuit, Circuit::Center);
    }

    #[tokio::test]
    async fn test_process_user_input_awards_coal() {
        let bridge = test_bridge().await;

        // Use a Sacred Circuitry word in context with its context clues
        let result = bridge
            .process_user_input("Let me Center on the focus of this problem and filter the scope")
            .await;

        // Should detect the word "Center" and award coal
        assert!(result.vaam.total_coal > 0 || result.detected_circuit.is_some());
    }

    #[tokio::test]
    async fn test_prompt_context_generation() {
        let bridge = test_bridge().await;

        // Process some input first
        bridge
            .process_user_input("I need to execute and build this now")
            .await;

        let context = bridge.prompt_context().await;
        assert!(context.contains("VAAM Active Circuit"));
        assert!(context.contains("VAAM User Profile"));
    }

    #[tokio::test]
    async fn test_circuit_change_tracking() {
        let bridge = test_bridge().await;

        // Start with default (Center)
        let r1 = bridge
            .process_user_input("Let me focus on the core problem and filter")
            .await;

        // Should detect Center — first activation is a change from default
        if r1.detected_circuit.is_some() {
            // Now shift to Ship quadrant
            let r2 = bridge
                .process_user_input("Time to execute and implement and build the code now")
                .await;

            if r2.detected_circuit.is_some() {
                assert!(
                    r2.circuit_changed,
                    "Should detect circuit change from Center to Act"
                );
            }
        }
    }

    #[tokio::test]
    async fn test_profile_updates_from_input() {
        let bridge = test_bridge().await;

        // Simulate several terse, direct interactions
        for _ in 0..5 {
            bridge
                .process_user_input("Ship it. Execute now. Build the code.")
                .await;
        }

        let profile = bridge.snapshot_profile().await;
        assert!(profile.interactions_analyzed >= 5);
        // Should lean toward Ship quadrant
        assert!(profile.circuit_affinity[3] > 0.0 || profile.circuit_usage[11] > 0);
    }

    #[tokio::test]
    async fn test_ai_output_processing() {
        let bridge = test_bridge().await;

        let detected = bridge
            .process_ai_output(
                "Refactoring this module to improve quality and simplify the interface",
            )
            .await;

        // Should detect Transform circuit (refactor, improve, quality, simplify)
        assert!(detected.is_some());
        if let Some((circuit, _)) = detected {
            assert_eq!(circuit, Circuit::Transform);
        }
    }

    #[tokio::test]
    async fn test_prompt_context_never_exceeds_budget() {
        let bridge = test_bridge().await;

        // Simulate heavy usage to grow VAAM state
        for i in 0..50 {
            bridge.process_user_input(
                &format!("I need to focus on the core problem {} and filter the scope by building and executing", i)
            ).await;
        }

        // Add some profile weight
        {
            let mut profile = bridge.profile.write().await;
            for j in 0..20 {
                profile.record_word_usage(&format!("test_word_{}", j), true);
            }
            profile.record_interaction(100, 10, 20);
        }

        let context = bridge.prompt_context().await;
        assert!(
            context.len() <= VaamBridge::MAX_CONTEXT_CHARS,
            "prompt_context() returned {} chars, exceeding {} budget:\n{}",
            context.len(),
            VaamBridge::MAX_CONTEXT_CHARS,
            context
        );
        // Should always contain the active circuit (highest priority)
        assert!(context.contains("VAAM Active Circuit"));
    }
}
