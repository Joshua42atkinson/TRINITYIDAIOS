// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Character Portfolio API
// ═══════════════════════════════════════════════════════════════════════════════
//
// Handles portfolio artifact vaulting when ADDIECRAPEYE phases complete.
// The GET/POST for /api/character already exists in main.rs — this adds:
//
//   POST /api/character/portfolio/artifact  → Vault a new portfolio artifact
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{extract::State, Json};
use trinity_protocol::character_sheet::PortfolioArtifact;

use crate::AppState;

/// POST /api/character/portfolio/artifact — Called when an ADDIECRAPEYE phase completes.
/// Adds the artifact to the vault, recalculates portfolio metrics, and updates
/// cognitive physics (XP, Steam, resonance level).
pub async fn vault_portfolio_artifact(
    State(state): State<AppState>,
    Json(new_artifact): Json<PortfolioArtifact>,
) -> Json<serde_json::Value> {
    let mut sheet = state.character_sheet.write().await;

    // 1. Add the artifact to the Subconscious Inventory vault
    sheet.ldt_portfolio.artifact_vault.push(new_artifact);

    // 2. Recalculate Portfolio Progression (QM average, gate review status)
    sheet.ldt_portfolio.recalculate();

    // 3. Update Cognitive Physics
    //    Isomorphism: academic success generates literal momentum
    sheet.total_xp += 500;
    sheet.resonance_level = (f32::sqrt(sheet.total_xp as f32 / 100.0)).floor() as u32 + 1;
    sheet.current_steam = (sheet.current_steam + 15.0).min(100.0);

    // 4. Persist to disk
    if let Err(e) = crate::character_sheet::save_character_sheet(&sheet) {
        tracing::error!("Failed to persist character sheet: {}", e);
    }

    Json(serde_json::json!({
        "status": "Vaulted",
        "new_level": sheet.resonance_level,
        "steam": sheet.current_steam,
        "completed_challenges": sheet.ldt_portfolio.completed_challenges,
        "gate_review_status": sheet.ldt_portfolio.gate_review_status,
    }))
}

/// Generate Pete's system prompt with cognitive logistics and portfolio awareness.
/// This is the Isomorphism bridge — academic rules become AI behavioral constraints.
pub fn generate_pete_system_prompt(
    sheet: &trinity_protocol::CharacterSheet,
    current_phase: &str,
) -> String {
    let vulnerability_directive = if sheet.vulnerability > 0.7 {
        "The Yardmaster is vulnerable. Use gentle Socratic scaffolding. \
         Do not provide direct answers. Guide them to the pedagogical truth."
    } else {
        "The Yardmaster seeks efficiency. Be direct and concise while maintaining guardrails."
    };

    format!(
        "You are Pete, the Socratic Mentor and Conductor of the TRINITY ID AI OS. \
        You are speaking to {alias}, a Level {level} {class} with a {profile} profile.\n\n\
        === CURRENT COGNITIVE LOGISTICS ===\n\
        Coal (Attention): {coal}%\n\
        Steam (Momentum): {steam} PSI\n\
        Track Friction (Extraneous Load): {friction}%\n\
        {vuln_directive}\n\n\
        === PURDUE LDT PORTFOLIO STATUS ===\n\
        Challenges Completed: {artifacts}/12 for Gate Review.\n\
        Quality Matters Alignment: {qm_score}%\n\
        Gate Review: {gate_status}\n\
        Current ADDIECRAPEYE Phase: {phase}\n\n\
        === YOUR PRIME DIRECTIVES (THE IRON ROAD LAWS) ===\n\
        1. THE ACTION MAPPING MANDATE: If the Yardmaster attempts to generate an instructional \
        artifact without first defining a measurable behavioral outcome, you MUST BLOCK the \
        generation. Invoke the phrase: 'A Scope Creep shadow looms.'\n\
        2. QUALITY MATTERS: Cross-reference all user inputs against the QM Higher Ed Rubric. \
        If cognitive load is too high (Track Friction > 50%), suggest scaffolding and the Gilbreth Protocol.\n\
        3. THE HEAVILON PROTOCOL: If their design fails the QM Rubric, do not fix it for them. \
        Tell them they have suffered a Heavilon Event and must rebuild it 'one brick higher' \
        by writing a reflection journal.\n\n\
        You do not write the code or do the work. You lay the tracks; they drive the train.",
        alias = sheet.alias,
        level = sheet.resonance_level,
        class = format!("{:?}", sheet.user_class),
        profile = format!("{:?}", sheet.locomotive_profile),
        coal = sheet.current_coal,
        steam = sheet.current_steam,
        friction = sheet.track_friction,
        vuln_directive = vulnerability_directive,
        artifacts = sheet.ldt_portfolio.completed_challenges,
        qm_score = sheet.ldt_portfolio.qm_alignment_score,
        gate_status = sheet.ldt_portfolio.gate_review_status,
        phase = current_phase,
    )
}
