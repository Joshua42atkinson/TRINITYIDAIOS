// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Spatial UI (Native Bevy, No JS, No egui)
// ═══════════════════════════════════════════════════════════════════════════════
//
// PURPOSE:  Cross-platform HUD that works on BOTH desktop and XR headsets.
//           Uses native Bevy UI (Node, Text, BackgroundColor) instead of egui.
//           On desktop: renders as screen-space overlay (identical to egui UX).
//           On XR: renders as head-locked HUD (follows gaze, always visible).
//
// PANELS:
//   ┌─────────────────────────────────┐  ┌──────────────────────────┐
//   │  🌙 PEARL COMPASS               │  │  🚂 P-ART-Y FLEET       │
//   │  Subject: Physics                │  │  T: Tempo 4B ● ONLINE   │
//   │  Medium:  Game                   │  │  P: Prog 26B ○ STANDBY  │
//   │  Vision:  Feel like Newton       │  │  R: Reason 31B ○ HOTEL  │
//   │  Phase:   Extracting ████░░░░░░  │  │  A: Aesth Janus ○ HOTEL │
//   └─────────────────────────────────┘  └──────────────────────────┘
//
//   ┌─────────────────────────────────┐  ┌──────────────────────────┐
//   │  🎯 QUEST DASHBOARD             │  │  ⚡ RESOURCE GAUGES      │
//   │  Station 3: Development          │  │  Coal:   ████████░░ 80  │
//   │  Phase: ADDIE (Extracting)       │  │  Steam:  ███░░░░░░░ 30  │
//   │  ☐ Define learning objectives    │  │  XP:     127 / 500      │
//   │  ☐ Map concept vocabulary        │  │  Traction: ██████░░ 65  │
//   │  ☑ Identify target audience      │  │  Creeps Slain: 3        │
//   └─────────────────────────────────┘  └──────────────────────────┘
//
// DESIGN:
//   Glassmorphism dark panels with gold (PEARL) and cyan (Trinity) accents.
//   Translucent backgrounds with subtle border glow.
//   All text uses Bevy's default_font (glyph-based, no asset loading).
//
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;

// ─── Colors ──────────────────────────────────────────────────────────────────

/// Deep navy panel background (glassmorphism base)
const PANEL_BG: Color = Color::srgba(0.06, 0.06, 0.10, 0.85);
/// Panel border accent
const PANEL_BORDER: Color = Color::srgba(0.16, 0.18, 0.24, 0.9);
/// Purdue Old Gold — PEARL accent
const OLD_GOLD: Color = Color::srgb(0.812, 0.725, 0.569);
/// Trinity Cyan — system accent
const CYAN: Color = Color::srgb(0.0, 1.0, 1.0);
/// Muted text
const DIM_TEXT: Color = Color::srgb(0.65, 0.65, 0.70);
/// Bright text
const BRIGHT_TEXT: Color = Color::srgb(0.88, 0.88, 0.92);
/// Bar background (empty portion)
const BAR_BG: Color = Color::srgba(0.15, 0.15, 0.20, 0.6);
/// Coal bar fill (warm ember)
const COAL_COLOR: Color = Color::srgb(0.95, 0.3, 0.15);
/// Steam bar fill (cool cyan)
const STEAM_COLOR: Color = Color::srgb(0.1, 0.7, 0.95);
/// XP bar fill (gold)
const XP_COLOR: Color = Color::srgb(0.85, 0.75, 0.35);
/// Healthy status indicator
const STATUS_ONLINE: Color = Color::srgb(0.2, 0.9, 0.3);
/// Standby status indicator
const STATUS_STANDBY: Color = Color::srgb(0.6, 0.6, 0.3);

// ─── Plugin ──────────────────────────────────────────────────────────────────

/// Native Bevy UI for the DAYDREAM HUD — works on desktop AND XR.
pub struct SpatialUiPlugin;

impl Plugin for SpatialUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialDashboardState>()
            .add_systems(Startup, spawn_spatial_hud)
            .add_systems(Update, (
                update_pearl_compass,
                update_quest_dashboard,
                update_resource_gauges,
                update_party_status,
            ));
    }
}

// ─── Dashboard State (synced from bridge) ────────────────────────────────────

/// Live dashboard state — updated by bridge polling systems.
#[derive(Resource)]
pub struct SpatialDashboardState {
    // PEARL
    pub pearl_subject: String,
    pub pearl_medium: String,
    pub pearl_vision: String,
    pub pearl_phase: String,
    pub pearl_progress: f32, // 0.0–1.0

    // Quest
    pub station: u8,
    pub station_name: String,
    pub addiecrapeye_label: String,
    pub objectives: Vec<(String, bool)>, // (label, completed)

    // Resources
    pub coal: f32,     // 0–100
    pub steam: f32,    // 0–100
    pub xp: u32,
    pub xp_max: u32,
    pub traction: f32, // 0–100
    pub creeps_slain: u32,

    // P-ART-Y Fleet
    pub tempo_status: FleetStatus,
    pub programming_status: FleetStatus,
    pub reasoning_status: FleetStatus,
    pub aesthetics_status: FleetStatus,
}

/// Status of a model in the P-ART-Y fleet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FleetStatus {
    Online,
    Standby,
    HotelSwapping,
    Offline,
}

impl FleetStatus {
    fn label(&self) -> &'static str {
        match self {
            FleetStatus::Online => "● ONLINE",
            FleetStatus::Standby => "○ STANDBY",
            FleetStatus::HotelSwapping => "◉ SWAPPING",
            FleetStatus::Offline => "⊘ OFFLINE",
        }
    }

    fn color(&self) -> Color {
        match self {
            FleetStatus::Online => STATUS_ONLINE,
            FleetStatus::Standby => STATUS_STANDBY,
            FleetStatus::HotelSwapping => CYAN,
            FleetStatus::Offline => Color::srgb(0.5, 0.2, 0.2),
        }
    }
}

impl Default for SpatialDashboardState {
    fn default() -> Self {
        Self {
            pearl_subject: "Awaiting PEARL...".into(),
            pearl_medium: "—".into(),
            pearl_vision: "—".into(),
            pearl_phase: "Extracting".into(),
            pearl_progress: 0.0,

            station: 1,
            station_name: "Station 1: Analysis".into(),
            addiecrapeye_label: "ADDIE → Analyze".into(),
            objectives: vec![
                ("Identify target audience".into(), false),
                ("Define learning objectives".into(), false),
                ("Map concept vocabulary".into(), false),
            ],

            coal: 100.0,
            steam: 0.0,
            xp: 0,
            xp_max: 500,
            traction: 0.0,
            creeps_slain: 0,

            tempo_status: FleetStatus::Standby,
            programming_status: FleetStatus::Standby,
            reasoning_status: FleetStatus::Standby,
            aesthetics_status: FleetStatus::Standby,
        }
    }
}

// ─── Marker Components ───────────────────────────────────────────────────────

/// Root node for the entire spatial HUD
#[derive(Component)]
struct SpatialHudRoot;

/// PEARL Compass panel elements
#[derive(Component)]
struct PearlSubjectText;

#[derive(Component)]
struct PearlMediumText;

#[derive(Component)]
struct PearlVisionText;

#[derive(Component)]
struct PearlPhaseText;

#[derive(Component)]
struct PearlProgressBar;

/// Quest Dashboard elements
#[derive(Component)]
struct QuestStationText;

#[derive(Component)]
struct QuestPhaseText;

#[derive(Component)]
struct QuestObjectiveText(usize);

/// Resource gauge elements
#[derive(Component)]
struct CoalBar;

#[derive(Component)]
struct CoalText;

#[derive(Component)]
struct SteamBar;

#[derive(Component)]
struct SteamText;

#[derive(Component)]
struct XpText;

/// P-ART-Y status elements
#[derive(Component)]
struct FleetStatusText(usize); // 0=T, 1=P, 2=R, 3=A

// ─── HUD Spawn ───────────────────────────────────────────────────────────────

/// Build the entire HUD hierarchy using native Bevy UI nodes.
fn spawn_spatial_hud(mut commands: Commands) {
    // ── Root container: full-screen overlay ───────────────────────
    commands
        .spawn((
            SpatialHudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(16.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            // The root is transparent — panels have their own BG
            BackgroundColor(Color::NONE),
        ))
        .with_children(|root| {
            // ── Top Row: PEARL Compass + P-ART-Y Fleet ───────────
            root.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                column_gap: Val::Px(12.0),
                margin: UiRect::bottom(Val::Px(12.0)),
                ..default()
            })
            .with_children(|row| {
                spawn_pearl_compass(row);
                spawn_party_status(row);
            });

            // ── Bottom Row: Quest Dashboard + Resource Gauges ────
            root.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                column_gap: Val::Px(12.0),
                ..default()
            })
            .with_children(|row| {
                spawn_quest_dashboard(row);
                spawn_resource_gauges(row);
            });
        });

    info!("🎯 Spatial HUD spawned (native Bevy UI — desktop + XR compatible)");
}

// ─── Panel Builders ──────────────────────────────────────────────────────────

/// Reusable glassmorphism panel frame.
fn panel_node() -> Node {
    Node {
        padding: UiRect::all(Val::Px(14.0)),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(6.0),
        min_width: Val::Px(280.0),
        border_radius: BorderRadius::all(Val::Px(8.0)),
        ..default()
    }
}

fn panel_bg() -> BackgroundColor {
    BackgroundColor(PANEL_BG)
}

fn panel_border() -> BorderColor {
    BorderColor::all(PANEL_BORDER)
}


/// Panel title text.
fn title_text(text: &str, color: Color) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(color),
    )
}

/// Label text (dim).
fn label_text(text: &str) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(DIM_TEXT),
    )
}

/// Value text (bright).
fn value_text(text: &str, color: Color) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(color),
    )
}

/// A horizontal label: value row.
fn spawn_kv_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    value: &str,
    value_color: Color,
    marker: impl Component,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|row| {
            row.spawn(label_text(label));
            row.spawn((value_text(value, value_color), marker));
        });
}

/// A progress / resource bar.
fn spawn_bar(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    fill_pct: f32,
    fill_color: Color,
    bar_marker: impl Component,
    text_marker: impl Component,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            // Label
            row.spawn(label_text(label));

            // Bar background
            row.spawn((
                Node {
                    width: Val::Px(120.0),
                    height: Val::Px(10.0),
                    border_radius: BorderRadius::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(BAR_BG),
            ))
            .with_children(|bar_bg| {
                // Bar fill
                bar_bg.spawn((
                    Node {
                        width: Val::Percent(fill_pct * 100.0),
                        height: Val::Percent(100.0),
                        border_radius: BorderRadius::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(fill_color),
                    bar_marker,
                ));
            });

            // Value text
            row.spawn((
                value_text(&format!("{:.0}", fill_pct * 100.0), BRIGHT_TEXT),
                text_marker,
            ));
        });
}

// ─── PEARL Compass ───────────────────────────────────────────────────────────

fn spawn_pearl_compass(parent: &mut ChildSpawnerCommands) {
    let mut panel = panel_node();
    panel.border = UiRect::all(Val::Px(1.0));

    parent
        .spawn((panel, panel_bg(), panel_border()))
        .with_children(|panel| {
            // Title
            panel.spawn(title_text("🌙 PEARL COMPASS", OLD_GOLD));

            // Separator line
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    margin: UiRect::vertical(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(PANEL_BORDER),
            ));

            // Subject
            spawn_kv_row(panel, "Subject:", "Awaiting PEARL...", OLD_GOLD, PearlSubjectText);

            // Medium
            spawn_kv_row(panel, "Medium:", "—", BRIGHT_TEXT, PearlMediumText);

            // Vision
            spawn_kv_row(panel, "Vision:", "—", BRIGHT_TEXT, PearlVisionText);

            // Phase
            spawn_kv_row(panel, "Phase:", "Extracting", CYAN, PearlPhaseText);

            // Progress bar
            spawn_bar(panel, "Progress:", 0.0, XP_COLOR, PearlProgressBar, PearlPhaseText);
        });
}

// ─── Quest Dashboard ─────────────────────────────────────────────────────────

fn spawn_quest_dashboard(parent: &mut ChildSpawnerCommands) {
    let mut panel = panel_node();
    panel.border = UiRect::all(Val::Px(1.0));

    parent
        .spawn((panel, panel_bg(), panel_border()))
        .with_children(|panel| {
            // Title
            panel.spawn(title_text("🎯 QUEST DASHBOARD", CYAN));

            // Separator
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    margin: UiRect::vertical(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(PANEL_BORDER),
            ));

            // Station
            spawn_kv_row(panel, "Station:", "1: Analysis", CYAN, QuestStationText);

            // Phase
            spawn_kv_row(panel, "ADDIECRAPEYE:", "Analyze", OLD_GOLD, QuestPhaseText);

            // Objectives header
            panel.spawn(label_text("Objectives:"));

            // Objective slots
            for i in 0..5 {
                let text = if i < 3 {
                    match i {
                        0 => "☐ Identify target audience",
                        1 => "☐ Define learning objectives",
                        _ => "☐ Map concept vocabulary",
                    }
                } else {
                    ""
                };
                panel.spawn((
                    value_text(text, DIM_TEXT),
                    QuestObjectiveText(i),
                ));
            }
        });
}

// ─── Resource Gauges ─────────────────────────────────────────────────────────

fn spawn_resource_gauges(parent: &mut ChildSpawnerCommands) {
    let mut panel = panel_node();
    panel.border = UiRect::all(Val::Px(1.0));

    parent
        .spawn((panel, panel_bg(), panel_border()))
        .with_children(|panel| {
            // Title
            panel.spawn(title_text("⚡ RESOURCE GAUGES", CYAN));

            // Separator
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    margin: UiRect::vertical(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(PANEL_BORDER),
            ));

            // Coal bar
            spawn_bar(panel, "Coal:", 1.0, COAL_COLOR, CoalBar, CoalText);

            // Steam bar
            spawn_bar(panel, "Steam:", 0.0, STEAM_COLOR, SteamBar, SteamText);

            // XP text
            spawn_kv_row(panel, "XP:", "0 / 500", XP_COLOR, XpText);

            // Bestiary
            panel.spawn((
                label_text("Creeps Slain: 0"),
                QuestObjectiveText(99), // reused marker — separate component would be cleaner
            ));
        });
}

// ─── P-ART-Y Fleet Status ────────────────────────────────────────────────────

fn spawn_party_status(parent: &mut ChildSpawnerCommands) {
    let mut panel = panel_node();
    panel.border = UiRect::all(Val::Px(1.0));

    parent
        .spawn((panel, panel_bg(), panel_border()))
        .with_children(|panel| {
            // Title
            panel.spawn(title_text("🚂 P·A·R·T·Y FLEET", OLD_GOLD));

            // Separator
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    margin: UiRect::vertical(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(PANEL_BORDER),
            ));

            // Fleet rows: T, P, R, A
            let fleet_labels = [
                ("T: Tempo E4B", 0usize),
                ("P: Programming 26B", 1),
                ("R: Reasoning 31B", 2),
                ("A: Aesthetics Janus", 3),
            ];

            for (label, idx) in &fleet_labels {
                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn(label_text(label));
                        row.spawn((
                            value_text("○ STANDBY", STATUS_STANDBY),
                            FleetStatusText(*idx),
                        ));
                    });
            }
        });
}

// ─── Update Systems ──────────────────────────────────────────────────────────

/// Update PEARL compass text from dashboard state.
fn update_pearl_compass(
    state: Res<SpatialDashboardState>,
    mut subject_q: Query<(&mut Text, &mut TextColor), With<PearlSubjectText>>,
    mut medium_q: Query<(&mut Text, &mut TextColor), (With<PearlMediumText>, Without<PearlSubjectText>)>,
    mut vision_q: Query<(&mut Text, &mut TextColor), (With<PearlVisionText>, Without<PearlSubjectText>, Without<PearlMediumText>)>,
) {
    if !state.is_changed() {
        return;
    }

    for (mut text, _color) in &mut subject_q {
        *text = Text::new(&state.pearl_subject);
    }
    for (mut text, _color) in &mut medium_q {
        *text = Text::new(&state.pearl_medium);
    }
    for (mut text, _color) in &mut vision_q {
        *text = Text::new(&state.pearl_vision);
    }
}

/// Update quest station and objectives.
fn update_quest_dashboard(
    state: Res<SpatialDashboardState>,
    mut station_q: Query<&mut Text, With<QuestStationText>>,
    mut phase_q: Query<&mut Text, (With<QuestPhaseText>, Without<QuestStationText>)>,
    mut obj_q: Query<(&mut Text, &QuestObjectiveText), (Without<QuestStationText>, Without<QuestPhaseText>)>,
) {
    if !state.is_changed() {
        return;
    }

    for mut text in &mut station_q {
        *text = Text::new(&state.station_name);
    }
    for mut text in &mut phase_q {
        *text = Text::new(&state.addiecrapeye_label);
    }
    for (mut text, idx) in &mut obj_q {
        if idx.0 < state.objectives.len() {
            let (label, done) = &state.objectives[idx.0];
            let prefix = if *done { "☑" } else { "☐" };
            *text = Text::new(format!("{} {}", prefix, label));
        }
    }
}

/// Update resource gauge bars and text.
fn update_resource_gauges(
    state: Res<SpatialDashboardState>,
    mut coal_bar_q: Query<&mut Node, With<CoalBar>>,
    mut coal_text_q: Query<&mut Text, With<CoalText>>,
    mut steam_bar_q: Query<&mut Node, (With<SteamBar>, Without<CoalBar>)>,
    mut steam_text_q: Query<&mut Text, (With<SteamText>, Without<CoalText>)>,
    mut xp_q: Query<&mut Text, (With<XpText>, Without<CoalText>, Without<SteamText>)>,
) {
    if !state.is_changed() {
        return;
    }

    for mut node in &mut coal_bar_q {
        node.width = Val::Percent(state.coal);
    }
    for mut text in &mut coal_text_q {
        *text = Text::new(format!("{:.0}", state.coal));
    }
    for mut node in &mut steam_bar_q {
        node.width = Val::Percent(state.steam);
    }
    for mut text in &mut steam_text_q {
        *text = Text::new(format!("{:.0}", state.steam));
    }
    for mut text in &mut xp_q {
        *text = Text::new(format!("{} / {}", state.xp, state.xp_max));
    }
}

/// Update P-ART-Y fleet status indicators.
fn update_party_status(
    state: Res<SpatialDashboardState>,
    mut fleet_q: Query<(&mut Text, &mut TextColor, &FleetStatusText)>,
) {
    if !state.is_changed() {
        return;
    }

    for (mut text, mut color, idx) in &mut fleet_q {
        let status = match idx.0 {
            0 => &state.tempo_status,
            1 => &state.programming_status,
            2 => &state.reasoning_status,
            3 => &state.aesthetics_status,
            _ => continue,
        };
        *text = Text::new(status.label());
        color.0 = status.color();
    }
}
