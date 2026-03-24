// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-quest/src/state.rs
// ═══════════════════════════════════════════════════════════════════════════════
//
// PURPOSE:     Quest state, objectives, player stats, persistence
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::hero::{HeroStage, Phase};
use crate::party::{default_party, PartyMember};
use serde::{Deserialize, Serialize};
use trinity_protocol::pearl::Pearl;

/// A quest objective within a phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub id: String,
    pub description: String,
    pub completed: bool,
}

/// Active quest state — one chapter of the Hero's Journey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestState {
    pub quest_id: String,
    pub quest_title: String,
    pub hero_stage: HeroStage,
    pub current_phase: Phase,
    pub phase_objectives: Vec<Objective>,
    pub completed_phases: Vec<Phase>,
    pub completed_chapters: Vec<u8>,
    pub xp_earned: u32,
    pub coal_used: f32,
    pub steam_generated: f32,
    pub subject: String,
    pub game_title: String,
    /// The PEARL — per-project alignment document.
    /// Created when the user selects a subject. Captures subject, medium, vision.
    /// Evaluated at every phase transition for scope alignment.
    #[serde(default)]
    pub pearl: Option<Pearl>,
}

impl QuestState {
    /// Create a new quest at the beginning of the Hero's Journey
    pub fn new(subject: &str) -> Self {
        let stage = HeroStage::OrdinaryWorld;
        let phase = Phase::Analysis;
        Self {
            quest_id: "journey".to_string(),
            quest_title: stage.title().to_string(),
            hero_stage: stage,
            current_phase: phase,
            phase_objectives: objectives_for_chapter(stage, phase),
            completed_phases: vec![],
            completed_chapters: vec![],
            xp_earned: 0,
            coal_used: 0.0,
            steam_generated: 0.0,
            subject: subject.to_string(),
            game_title: format!("{} Learning Experience", subject),
            pearl: Some(Pearl::new(subject)),
        }
    }

    /// Mark an objective as completed
    pub fn complete_objective(&mut self, objective_id: &str) -> bool {
        if let Some(obj) = self
            .phase_objectives
            .iter_mut()
            .find(|o| o.id == objective_id && !o.completed)
        {
            obj.completed = true;
            self.xp_earned += 10;
            self.coal_used += 2.0;
            self.steam_generated += 5.0;
            return true;
        }
        false
    }

    /// Check if all phase objectives are complete
    pub fn phase_complete(&self) -> bool {
        self.phase_objectives.iter().all(|o| o.completed)
    }

    /// Advance to next phase
    pub fn advance_phase(&mut self) -> bool {
        if !self.phase_complete() {
            return false;
        }

        self.completed_phases.push(self.current_phase);

        if let Some(next) = self.current_phase.next() {
            self.current_phase = next;
            self.phase_objectives = objectives_for_chapter(self.hero_stage, next);
            self.xp_earned += 50;
            self.steam_generated += 20.0;
            // Sync PEARL phase with ADDIECRAPEYE group
            if let Some(ref mut pearl) = self.pearl {
                pearl.sync_phase_from_station(self.completed_phases.len() as u8 + 1);
            }
            true
        } else {
            // Phase was Evaluation, need to advance chapter
            false
        }
    }

    /// Advance to next chapter (called after completing Evaluation phase)
    pub fn advance_chapter(&mut self) -> bool {
        self.completed_chapters.push(self.hero_stage.chapter());

        if let Some(next) = self.hero_stage.next() {
            self.hero_stage = next;
            self.current_phase = Phase::Analysis;
            self.phase_objectives = objectives_for_chapter(next, Phase::Analysis);
            self.quest_title = next.title().to_string();
            self.xp_earned += 100;
            self.steam_generated += 50.0;
            true
        } else {
            false // Journey complete!
        }
    }
}

impl Default for QuestState {
    fn default() -> Self {
        Self {
            quest_id: "journey".to_string(),
            quest_title: "The Ordinary World".to_string(),
            hero_stage: HeroStage::OrdinaryWorld,
            current_phase: Phase::Analysis,
            phase_objectives: objectives_for_chapter(HeroStage::OrdinaryWorld, Phase::Analysis),
            completed_phases: vec![],
            completed_chapters: vec![],
            xp_earned: 0,
            coal_used: 0.0,
            steam_generated: 0.0,
            subject: String::new(),
            game_title: String::new(),
            pearl: None,
        }
    }
}

/// Player stats tracked across the entire journey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub traction: u32,
    pub velocity: u32,
    pub combustion: u32,
    pub coal_reserves: f32,
    pub resonance: i32,
    pub total_xp: u32,
    pub quests_completed: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            traction: 3,
            velocity: 2,
            combustion: 1,
            coal_reserves: 87.0,
            resonance: 1,
            total_xp: 0,
            quests_completed: 0,
        }
    }
}

/// Full game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub quest: QuestState,
    pub stats: PlayerStats,
    pub party: Vec<PartyMember>,
    pub inventory: Vec<String>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            quest: QuestState::default(),
            stats: PlayerStats::default(),
            party: default_party(),
            inventory: vec![
                "📐 ADDIE Framework".into(),
                "🌸 Bloom's Taxonomy".into(),
                "🧠 Cognitive Load Theory".into(),
            ],
        }
    }
}

/// Generate objectives specific to each Hero's Journey chapter + ADDIE phase
pub fn objectives_for_chapter(stage: HeroStage, phase: Phase) -> Vec<Objective> {
    let ch = stage.chapter();
    let p = match phase {
        Phase::Analysis => "a",
        Phase::Design => "d",
        Phase::Development => "v",
        Phase::Implementation => "i",
        Phase::Evaluation => "e",
        Phase::Contrast => "c",
        Phase::Repetition => "r",
        Phase::Alignment => "al",
        Phase::Proximity => "p",
        Phase::Envision => "en",
        Phase::Yoke => "y",
        Phase::Evolve => "ev",
    };
    // Agentic Quests: Instead of a static hardcoded array, the Great Recycler 
    // dynamically generates Socratic objectives based on the user's answers.
    // We only seed the initial conversational prompt.
    vec![obj(ch, p, 1, "Speak with the Great Recycler about this station's focus")]
}

fn obj(ch: u8, phase: &str, n: u8, desc: &str) -> Objective {
    Objective {
        id: format!("ch{}_{}{}", ch, phase, n),
        description: desc.to_string(),
        completed: false,
    }
}

/// Complete an objective, returns true if phase advanced
pub fn complete_objective(state: &mut GameState, objective_id: &str) -> bool {
    let idx = state
        .quest
        .phase_objectives
        .iter()
        .position(|o| o.id == objective_id && !o.completed);

    if let Some(i) = idx {
        state.quest.phase_objectives[i].completed = true;
        state.stats.total_xp += 25;
        state.quest.xp_earned += 25;
        state.stats.coal_reserves = (state.stats.coal_reserves - 3.0).max(0.0);
        state.quest.coal_used += 3.0;
        state.quest.steam_generated += 5.0;

        // Check if all objectives in current phase are done
        let all_done = state.quest.phase_objectives.iter().all(|o| o.completed);
        if all_done {
            advance_phase(state);
            return true;
        }
    }
    false
}

/// Advance to next phase or chapter
fn advance_phase(state: &mut GameState) {
    let completed_phase = state.quest.current_phase;
    state.quest.completed_phases.push(completed_phase);

    // Stat boost for completing phase (mapped to 12 stations)
    match completed_phase {
        Phase::Analysis => state.stats.traction += 1,
        Phase::Design => state.stats.velocity += 1,
        Phase::Development => state.stats.combustion += 1,
        Phase::Implementation => state.stats.combustion += 1,
        Phase::Evaluation => {
            state.stats.traction += 1;
            state.stats.velocity += 1;
        }
        Phase::Contrast => state.stats.combustion += 1,
        Phase::Repetition => state.stats.resonance += 1,
        Phase::Alignment => state.stats.resonance += 1,
        Phase::Proximity => state.stats.velocity += 1,
        Phase::Envision => state.stats.combustion += 1,
        Phase::Yoke => state.stats.traction += 1,
        Phase::Evolve => state.stats.resonance += 2,
    }

    state.stats.total_xp += 100;
    state.quest.xp_earned += 100;

    if let Some(next_phase) = completed_phase.next() {
        state.quest.current_phase = next_phase;
        state.quest.phase_objectives = objectives_for_chapter(state.quest.hero_stage, next_phase);
    } else {
        // All 12 stations done — chapter complete! Advance Hero's Journey
        let ch = state.quest.hero_stage.chapter();
        state.quest.completed_chapters.push(ch);
        state.stats.quests_completed += 1;

        if let Some(next_stage) = state.quest.hero_stage.next() {
            state.quest.hero_stage = next_stage;
            state.quest.quest_title = next_stage.title().to_string();
            state.quest.current_phase = Phase::Analysis;
            state.quest.completed_phases.clear();
            state.quest.phase_objectives = objectives_for_chapter(next_stage, Phase::Analysis);
            state.stats.resonance += 1;
        } else {
            // All 12 chapters done — THE JOURNEY IS COMPLETE
            state.stats.resonance += 5;
        }
    }
}

/// Toggle a party member active/inactive
pub fn toggle_party_member(state: &mut GameState, member_id: &str) -> bool {
    let idx = state.party.iter().position(|m| m.id == member_id);
    if let Some(i) = idx {
        if !state.party[i].available {
            return false;
        }
        // Only one ART sidecar active at a time (besides Pete who is permanent)
        if state.party[i].id != "pete" {
            for m in &mut state.party {
                if m.id != "pete" {
                    m.active = false;
                }
            }
            state.party[i].active = true;
        }
        return true;
    }
    false
}
