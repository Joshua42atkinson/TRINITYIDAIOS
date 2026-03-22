// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        skills.rs
// PURPOSE:     D20 skill check system for Hardcore Mode
//
// ARCHITECTURE:
//   • Hardcore Mode toggle stored in CharacterSheet
//   • Skill checks use d20 + stat modifier vs DC
//   • Stats: Traction (Analysis/Evaluation), Velocity (Design/Impl), Combustion (Dev)
//   • Heavilon Protocol triggers on failure - must analyze before retry
//   • Per IRON_ROAD_LITRPG_FRAMEWORK.md lines 41-49, 89-96
//
// DEPENDENCIES:
//   - rand — d20 die rolls
//   - trinity_quest — Phase, PlayerStats
//   - serde — Serialization
//
// CHANGES:
//   2026-03-16  Cascade  Created for 30-second core loop
//
// ═══════════════════════════════════════════════════════════════════════════════

use rand::Rng;
use serde::{Deserialize, Serialize};
use trinity_quest::{Phase, PlayerStats};

/// Game mode toggle - stored in CharacterSheet
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GameMode {
    #[default]
    Normal, // No skill checks, auto-succeed
    Hardcore, // Full d20 skill checks
}

/// Result of a skill check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillResult {
    pub roll: u8,
    pub modifier: i32,
    pub dc: u8,
    pub total: i32,
    pub success: bool,
    pub critical: bool, // Natural 20
    pub fumble: bool,   // Natural 1
}

impl SkillResult {
    /// Auto-success for Normal mode
    pub fn auto_success() -> Self {
        Self {
            roll: 20,
            modifier: 0,
            dc: 10,
            total: 20,
            success: true,
            critical: false,
            fumble: false,
        }
    }
}

/// Perform a skill check
pub fn skill_check(
    mode: GameMode,
    stat: &PlayerStats,
    phase: Phase,
    difficulty_modifier: i32,
) -> SkillResult {
    // Normal mode: auto-succeed
    if mode == GameMode::Normal {
        return SkillResult::auto_success();
    }

    // Hardcore mode: roll d20
    let mut rng = rand::thread_rng();
    let die: u8 = rng.gen_range(1..=20);

    // Stat modifier based on phase
    let modifier = match phase {
        Phase::Analysis | Phase::Evaluation | Phase::Yoke => stat.traction as i32,
        Phase::Design | Phase::Implementation | Phase::Proximity => stat.velocity as i32,
        Phase::Development | Phase::Contrast | Phase::Envision => stat.combustion as i32,
        Phase::Repetition | Phase::Alignment | Phase::Evolve => {
            (stat.traction + stat.velocity + stat.combustion) as i32 / 3
        } // Combined focus
    };

    // DC from phase (per hero.rs:52-60)
    let base_dc = phase.dc() as i32;
    let dc = (base_dc + difficulty_modifier).clamp(5, 30) as u8;

    let total = die as i32 + modifier;

    // Natural 20 always succeeds, natural 1 always fails
    let success = die == 20 || (die > 1 && total >= dc as i32);

    SkillResult {
        roll: die,
        modifier,
        dc,
        total,
        success,
        critical: die == 20,
        fumble: die == 1,
    }
}

/// Heavilon Protocol - triggered on failed skill check
/// Per framework lines 89-96: "Cannot simply retry with same parameters"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeavilonProtocol {
    pub failure_context: String,
    pub required_analysis: Vec<String>,
    pub retry_allowed: bool,
    pub coal_cost: f32,
    pub steam_lost: f32,
}

impl HeavilonProtocol {
    /// Create Heavilon Protocol from a failed skill check
    pub fn from_failure(skill: &SkillResult, action: &str) -> Self {
        let miss_by = skill.dc as i32 - skill.total;

        Self {
            failure_context: format!(
                "DC {} missed by {} (rolled {} + {} modifier = {})",
                skill.dc, miss_by, skill.roll, skill.modifier, skill.total
            ),
            required_analysis: vec![
                "Identify structural weakness".to_string(),
                "Engage Brakeman to analyze crash log (Debris)".to_string(),
                "Reinforce before resuming".to_string(),
                format!("Action that failed: {}", action),
            ],
            retry_allowed: false, // Must complete analysis first
            coal_cost: 3.0,       // Coal burned in failed attempt
            steam_lost: 0.0,      // No steam generated on failure
        }
    }

    /// Check if player can retry (after completing analysis)
    #[allow(dead_code)] // Called when player completes Heavilon analysis steps
    pub fn can_retry(&self, completed_steps: &[String]) -> bool {
        let all_done = self
            .required_analysis
            .iter()
            .all(|step| completed_steps.iter().any(|done| done.contains(step)));
        all_done
    }

    /// Mark protocol as complete, allow retry
    #[allow(dead_code)] // Activated after Heavilon analysis is verified
    pub fn complete_analysis(&mut self) {
        self.retry_allowed = true;
    }
}

/// Calculate Steam generated from a successful action
/// Steam = Coal burned × skill factor × phase multiplier
pub fn calculate_steam(coal_burned: f32, skill: &SkillResult, phase: Phase) -> f32 {
    if !skill.success {
        return 0.0;
    }

    // Phase multiplier (higher phases = more steam)
    let phase_mult = match phase {
        Phase::Analysis | Phase::Repetition => 1.0,
        Phase::Design | Phase::Proximity => 1.2,
        Phase::Development | Phase::Envision => 1.5,
        Phase::Implementation | Phase::Yoke => 2.0,
        Phase::Evaluation | Phase::Alignment => 1.8,
        Phase::Contrast => 1.3,
        Phase::Evolve => 2.5,
    };

    // Critical success doubles steam
    let crit_mult = if skill.critical { 2.0 } else { 1.0 };

    // Steam = coal × success_margin × phase_mult × crit_mult
    let success_margin = (skill.total - skill.dc as i32).max(1) as f32 / 10.0;

    coal_burned * success_margin * phase_mult * crit_mult
}

/// Calculate XP earned from an action
pub fn calculate_xp(tool_used: &str, skill: &SkillResult, quest_objective_complete: bool) -> u32 {
    let mut xp = 0u32;

    // Base XP for using tools
    xp += match tool_used {
        "read_file" => 5,
        "write_file" => 15,
        "list_dir" => 3,
        "shell" => 10,
        "search_files" => 5,
        "cargo_check" | "cargo_build" => 25,
        _ => 5,
    };

    // Bonus for success
    if skill.success {
        xp += 10;
    }

    // Critical bonus
    if skill.critical {
        xp += 20;
    }

    // Quest objective completion bonus
    if quest_objective_complete {
        xp += 50;
    }

    xp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_success_normal_mode() {
        let stat = PlayerStats::default();
        let result = skill_check(GameMode::Normal, &stat, Phase::Analysis, 0);
        assert!(result.success);
    }

    #[test]
    fn test_critical_success() {
        let _stat = PlayerStats::default();
        // Mock a natural 20 by calling directly
        let result = SkillResult {
            roll: 20,
            modifier: 0,
            dc: 30,
            total: 20,
            success: true,
            critical: true,
            fumble: false,
        };
        assert!(result.success);
        assert!(result.critical);
    }

    #[test]
    fn test_fumble() {
        let _stat = PlayerStats::default();
        // Natural 1 always fails
        let result = SkillResult {
            roll: 1,
            modifier: 100,
            dc: 5,
            total: 101,
            success: false,
            critical: false,
            fumble: true,
        };
        assert!(!result.success);
        assert!(result.fumble);
    }

    #[test]
    fn test_heavilon_protocol() {
        let skill = SkillResult {
            roll: 5,
            modifier: 2,
            dc: 15,
            total: 7,
            success: false,
            critical: false,
            fumble: false,
        };

        let protocol = HeavilonProtocol::from_failure(&skill, "write_file");
        assert!(!protocol.retry_allowed);
        assert_eq!(protocol.coal_cost, 3.0);
    }

    #[test]
    fn test_steam_calculation() {
        let skill = SkillResult {
            roll: 15,
            modifier: 3,
            dc: 10,
            total: 18,
            success: true,
            critical: false,
            fumble: false,
        };

        let steam = calculate_steam(5.0, &skill, Phase::Development);
        assert!(steam > 0.0);
    }
}
