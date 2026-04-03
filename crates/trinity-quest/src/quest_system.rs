// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-quest/src/quest_system.rs
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        quest_system.rs
// PURPOSE:     Quest objectives generation and database persistence
//
// ARCHITECTURE:
//   • objectives_for_chapter: generates quest objectives for each Hero's Journey stage
//   • Database functions: ensure_quest_tables, save/load game state
//   • NO HTTP handlers here - those belong in trinity crate
//
// CHANGES:
//   2026-03-16  Cascade  Removed duplicate types, kept only DB functions
//
// ═══════════════════════════════════════════════════════════════════════════════

use sqlx::SqlitePool;
use tracing::info;

// Import canonical types from sibling modules
use crate::hero::{HeroStage, Phase};
use crate::party::default_party;
use crate::state::{GameState, Objective, PlayerStats, QuestState};

use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, serde::Deserialize)]
struct BaseObjective {
    id: String,
    description: String,
}

type ObjectivesMap = HashMap<String, HashMap<String, Vec<BaseObjective>>>;

static OBJECTIVES_JSON: &str = include_str!("objectives.json");

fn load_objectives() -> &'static ObjectivesMap {
    static MAP: OnceLock<ObjectivesMap> = OnceLock::new();
    MAP.get_or_init(|| {
        serde_json::from_str(OBJECTIVES_JSON).unwrap_or_default()
    })
}

/// Generate objectives for a specific chapter and phase
pub fn objectives_for_chapter(stage: HeroStage, phase: Phase) -> Vec<Objective> {
    let ch = stage.chapter();
    let p = phase.label();
    
    let map = load_objectives();
    
    let stage_str = format!("{:?}", stage);
    let phase_str = format!("{:?}", phase);

    if let Some(stage_phases) = map.get(&stage_str) {
        if let Some(base_objs) = stage_phases.get(&phase_str) {
            return base_objs
                .iter()
                .map(|bo| obj(ch, p, bo.id.clone(), &bo.description))
                .collect();
        }
    }

    // CRAP + EYE phases fallback if not defined in JSON
    vec![
        obj(ch, p, "1".to_string(), &format!("Reflect on {}: what question must you answer before moving forward?", p)),
        obj(ch, p, "2".to_string(), &format!("Apply {} thinking to your PEARL subject — list 3 observations", p)),
        obj(ch, p, "3".to_string(), &format!("Ask Pete: 'How does {} apply to {}?' — log the insight", p, stage.title())),
    ]
}

fn obj(ch: u8, phase: &str, n: String, desc: &str) -> Objective {
    Objective {
        id: format!("ch{}_{}{}", ch, phase, n),
        description: desc.to_string(),
        completed: false,
    }
}

// ═══════════════════════════════════════════════════════════════════
// SQLITE PERSISTENCE
// ═══════════════════════════════════════════════════════════════════

/// Ensure quest state tables exist
pub async fn ensure_quest_tables(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS quest_state (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            player_id TEXT NOT NULL DEFAULT 'default',
            chapter INT NOT NULL DEFAULT 1,
            phase TEXT NOT NULL DEFAULT 'analysis',
            xp INT NOT NULL DEFAULT 0,
            coal REAL NOT NULL DEFAULT 87.0,
            steam REAL NOT NULL DEFAULT 0.0,
            resonance INT NOT NULL DEFAULT 1,
            stats TEXT NOT NULL DEFAULT '{"traction":3,"velocity":2,"combustion":1,"coal_reserves":87.0,"resonance":1,"total_xp":0,"quests_completed":0}',
            inventory TEXT NOT NULL DEFAULT '["📐 ADDIE Framework","🌸 Bloom''s Taxonomy","🧠 Cognitive Load Theory"]',
            subject TEXT NOT NULL DEFAULT '',
            game_title TEXT NOT NULL DEFAULT '',
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            UNIQUE(player_id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_quest_state_player ON quest_state(player_id)")
        .execute(pool)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS quest_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            player_id TEXT NOT NULL DEFAULT 'default',
            quest_id TEXT NOT NULL,
            quest_title TEXT NOT NULL,
            status TEXT NOT NULL,
            xp_earned INT NOT NULL DEFAULT 0,
            duration_secs INT,
            completed_at TEXT NOT NULL DEFAULT (datetime('now')),
            results TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_quest_history_player ON quest_history(player_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_quest_history_quest ON quest_history(quest_id)")
        .execute(pool)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO quest_state (player_id)
        VALUES ('default')
        ON CONFLICT (player_id) DO NOTHING
        "#,
    )
    .execute(pool)
    .await?;

    info!("Quest state tables ensured");
    Ok(())
}

/// Save game state to SQLite
#[allow(dead_code)] // Called from /api/quest/advance when game state changes
pub async fn save_game_state(
    pool: &SqlitePool,
    player_id: &str,
    state: &GameState,
) -> anyhow::Result<()> {
    let stats_json = serde_json::to_value(&state.stats)?;
    let inventory_json = serde_json::to_value(&state.inventory)?;
    let chapter = state.quest.hero_stage.chapter();

    sqlx::query(
        r#"
        INSERT INTO quest_state (player_id, chapter, phase, xp, coal, steam, resonance, stats, inventory, subject, game_title, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, datetime('now'))
        ON CONFLICT (player_id) DO UPDATE SET
            chapter = EXCLUDED.chapter,
            phase = EXCLUDED.phase,
            xp = EXCLUDED.xp,
            coal = EXCLUDED.coal,
            steam = EXCLUDED.steam,
            resonance = EXCLUDED.resonance,
            stats = EXCLUDED.stats,
            inventory = EXCLUDED.inventory,
            subject = EXCLUDED.subject,
            game_title = EXCLUDED.game_title,
            updated_at = datetime('now')
        "#
    )
    .bind(player_id)
    .bind(chapter as i32)
    .bind(state.quest.current_phase.label())
    .bind(state.quest.xp_earned as i32)
    .bind(state.stats.coal_reserves)
    .bind(state.quest.steam_generated)
    .bind(state.stats.resonance)
    .bind(stats_json)
    .bind(inventory_json)
    .bind(&state.quest.subject)
    .bind(&state.quest.game_title)
    .execute(pool)
    .await?;

    Ok(())
}

/// Load game state from SQLite
#[allow(clippy::type_complexity)]
pub async fn load_game_state(pool: &SqlitePool, player_id: &str) -> anyhow::Result<GameState> {
    let row: Option<(
        i32,
        String,
        i32,
        f32,
        f32,
        i32,
        serde_json::Value,
        serde_json::Value,
        String,
        String,
    )> = sqlx::query_as(
        r#"
        SELECT chapter, phase, xp, coal, steam, resonance, stats, inventory, subject, game_title
        FROM quest_state
        WHERE player_id = $1
        "#,
    )
    .bind(player_id)
    .fetch_optional(pool)
    .await?;

    if let Some((
        chapter,
        phase_str,
        xp,
        coal,
        steam,
        _resonance,
        stats_json,
        inventory_json,
        subject,
        game_title,
    )) = row
    {
        let current_phase = match phase_str.as_str() {
            "Analysis" => Phase::Analysis,
            "Design" => Phase::Design,
            "Development" => Phase::Development,
            "Implementation" => Phase::Implementation,
            "Evaluation" => Phase::Evaluation,
            "Contrast" => Phase::Contrast,
            "Repetition" => Phase::Repetition,
            "Alignment" => Phase::Alignment,
            "Proximity" => Phase::Proximity,
            "Envision" => Phase::Envision,
            "Yoke" => Phase::Yoke,
            "Evolve" => Phase::Evolve,
            _ => Phase::Analysis,
        };

        let stats: PlayerStats = serde_json::from_value(stats_json).unwrap_or_default();
        let inventory: Vec<String> = serde_json::from_value(inventory_json).unwrap_or_default();

        let hero_stage = match chapter {
            1 => HeroStage::OrdinaryWorld,
            2 => HeroStage::CallToAdventure,
            3 => HeroStage::RefusalOfTheCall,
            4 => HeroStage::MeetingTheMentor,
            5 => HeroStage::CrossingTheThreshold,
            6 => HeroStage::TestsAlliesEnemies,
            7 => HeroStage::ApproachToInmostCave,
            8 => HeroStage::TheOrdeal,
            9 => HeroStage::TheReward,
            10 => HeroStage::TheRoadBack,
            11 => HeroStage::TheResurrection,
            12 => HeroStage::ReturnWithElixir,
            _ => HeroStage::OrdinaryWorld,
        };

        let quest_state = QuestState {
            quest_id: "journey".to_string(),
            quest_title: hero_stage.title().to_string(),
            hero_stage,
            current_phase,
            phase_objectives: objectives_for_chapter(hero_stage, current_phase),
            completed_phases: vec![],
            completed_chapters: vec![],
            xp_earned: xp as u32,
            coal_used: 100.0 - coal,
            steam_generated: steam,
            subject: subject.clone(),
            game_title,
            pearl: if subject.is_empty() {
                None
            } else {
                Some(trinity_protocol::Pearl::new(&subject))
            },
        };

        Ok(GameState {
            quest: quest_state,
            stats,
            party: default_party(),
            inventory,
        })
    } else {
        Ok(GameState::default())
    }
}
