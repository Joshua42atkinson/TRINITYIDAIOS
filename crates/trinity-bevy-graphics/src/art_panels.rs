// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — ART Studio Panels (bevy_egui)
// ═══════════════════════════════════════════════════════════════════════════════
//
// The core UI for the ART Studio desktop app. Renders egui panels inside a
// Bevy window:
//
//   ┌─────────────────────────────────────────────────────────────────┐
//   │  TOP: 12-station ADDIECRAPEYE navigation rail                  │
//   ├──────────┬──────────────────────────────────┬──────────────────┤
//   │  LEFT    │  CENTER                          │  RIGHT           │
//   │  Project │  Phase-specific workspace        │  ART Sidebar     │
//   │  PEARL   │  (changes per active phase)      │  Image/3D/Music  │
//   │          │                                  │  Sidecar Status  │
//   ├──────────┴──────────────────────────────────┴──────────────────┤
//   │  BOTTOM: Connection status │ XP │ Coal │ Steam │ Chapter       │
//   └─────────────────────────────────────────────────────────────────┘
//
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

use crate::addiecrapeye::{Act, Phase};
use crate::bridge::TrinityServerState;
use crate::creative_bridge::{
    request_image_generation, ArtSidecarState, CreativeMailbox, ImageGenRequest,
};

// ─── Plugin ──────────────────────────────────────────────────────────────────

/// The ART Studio panel system plugin.
pub struct ArtPanelsPlugin;

impl Plugin for ArtPanelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .insert_resource(StudioState::default())
            .add_systems(EguiPrimaryContextPass, render_studio_ui);
    }
}

// ─── Studio State ────────────────────────────────────────────────────────────

/// Persistent UI state for the ART Studio.
#[derive(Resource)]
pub struct StudioState {
    /// Currently selected ADDIECRAPEYE phase.
    pub active_phase: Phase,

    /// Project fields
    pub project_title: String,
    pub project_subject: String,
    pub project_audience: String,
    pub pearl_vision: String,

    /// Phase-specific text buffers
    pub sme_notes: String,
    pub learning_objectives: Vec<String>,
    pub content_draft: String,

    /// ART sidebar
    pub image_prompt: String,
    pub image_width: u32,
    pub image_height: u32,
    pub generated_assets: Vec<String>,

    /// Show phase detail panel
    pub show_hero_journey: bool,
}

impl Default for StudioState {
    fn default() -> Self {
        Self {
            active_phase: Phase::Analyze,
            project_title: String::new(),
            project_subject: String::new(),
            project_audience: String::new(),
            pearl_vision: String::new(),
            sme_notes: String::new(),
            learning_objectives: vec![String::new()],
            content_draft: String::new(),
            image_prompt: String::new(),
            image_width: 1024,
            image_height: 1024,
            generated_assets: Vec::new(),
            show_hero_journey: false,
        }
    }
}

// ─── Main Render System ─────────────────────────────────────────────────────

fn render_studio_ui(
    mut contexts: EguiContexts,
    mut studio: ResMut<StudioState>,
    server: Res<TrinityServerState>,
    sidecar: Res<ArtSidecarState>,
    mailbox: Res<CreativeMailbox>,
) {
    let ctx = match contexts.ctx_mut() {
        Ok(ctx) => ctx,
        Err(_) => return,
    };

    // Apply Trinity dark theme
    apply_trinity_theme(ctx);

    // ── Top Panel: ADDIECRAPEYE Navigation ─────────────────────────
    egui::TopBottomPanel::top("phase_nav").show(ctx, |ui| {
        render_phase_navigation(ui, &mut studio);
    });

    // ── Bottom Panel: Status Bar ───────────────────────────────────
    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        render_status_bar(ui, &server, &sidecar);
    });

    // ── Left Panel: Project Overview / PEARL ───────────────────────
    egui::SidePanel::left("project_panel")
        .min_width(220.0)
        .max_width(320.0)
        .default_width(260.0)
        .show(ctx, |ui| {
            render_project_panel(ui, &mut studio);
        });

    // ── Right Panel: ART Sidebar ──────────────────────────────────
    egui::SidePanel::right("art_sidebar")
        .min_width(240.0)
        .max_width(380.0)
        .default_width(300.0)
        .show(ctx, |ui| {
            render_art_sidebar(ui, &mut studio, &sidecar, &mailbox);
        });

    // ── Center Panel: Phase Workspace ─────────────────────────────
    egui::CentralPanel::default().show(ctx, |ui| {
        render_phase_workspace(ui, &mut studio);
    });
}

// ─── Theme ───────────────────────────────────────────────────────────────────

fn apply_trinity_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Dark railroad theme — Old Gold accents on dark slate
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::from_rgb(18, 18, 24);
    visuals.window_fill = egui::Color32::from_rgb(22, 22, 30);
    visuals.extreme_bg_color = egui::Color32::from_rgb(12, 12, 16);

    // Old Gold for selections and accents
    visuals.selection.bg_fill = egui::Color32::from_rgb(180, 150, 50);
    visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 190, 80));

    // Widget styling
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(50, 45, 35);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(40, 38, 32);
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 28, 24);

    style.visuals = visuals;

    // Spacing
    style.spacing.item_spacing = egui::vec2(6.0, 4.0);
    style.spacing.window_margin = egui::Margin::same(8);

    ctx.set_style(style);
}

// ─── Phase Navigation (Top Bar) ──────────────────────────────────────────────

fn render_phase_navigation(ui: &mut egui::Ui, studio: &mut StudioState) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;

        ui.label(
            egui::RichText::new("🚂 TRINITY ART STUDIO")
                .strong()
                .color(egui::Color32::from_rgb(220, 190, 80)),
        );

        ui.separator();

        // Current act label
        let act = studio.active_phase.act();
        ui.label(
            egui::RichText::new(act.label())
                .small()
                .color(egui::Color32::from_rgb(140, 140, 160)),
        );

        ui.separator();

        // Phase buttons
        for phase in Phase::ALL {
            let is_active = studio.active_phase == phase;
            let label = format!("{}{}", phase.emoji(), phase.station());

            let button = if is_active {
                egui::Button::new(
                    egui::RichText::new(&label)
                        .strong()
                        .color(egui::Color32::BLACK),
                )
                .fill(egui::Color32::from_rgb(220, 190, 80))
            } else {
                let act_color = match phase.act() {
                    Act::Departure => egui::Color32::from_rgb(60, 80, 60),
                    Act::Initiation => egui::Color32::from_rgb(70, 60, 80),
                    Act::Return => egui::Color32::from_rgb(60, 70, 90),
                };
                egui::Button::new(egui::RichText::new(&label)).fill(act_color)
            };

            let resp = ui.add(button);
            if resp.clicked() {
                studio.active_phase = phase;
            }
            if resp.hovered() {
                resp.on_hover_text(format!(
                    "{} — {}\n{}\n📍 {}",
                    phase.label(),
                    phase.description(),
                    phase.hero_stage(),
                    phase.location()
                ));
            }
        }
    });
}

// ─── Project Panel (Left) ────────────────────────────────────────────────────

fn render_project_panel(ui: &mut egui::Ui, studio: &mut StudioState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading(
            egui::RichText::new("📋 Project")
                .color(egui::Color32::from_rgb(220, 190, 80)),
        );
        ui.separator();

        // Title
        ui.label("Title:");
        ui.text_edit_singleline(&mut studio.project_title);
        ui.add_space(4.0);

        // Subject (PEARL)
        ui.label("Subject (SME Wisdom):");
        ui.text_edit_singleline(&mut studio.project_subject);
        ui.add_space(4.0);

        // Target Audience
        ui.label("Audience:");
        ui.text_edit_singleline(&mut studio.project_audience);
        ui.add_space(4.0);

        // PEARL Vision
        ui.label("PEARL Vision:");
        ui.add(
            egui::TextEdit::multiline(&mut studio.pearl_vision)
                .desired_rows(3)
                .hint_text("When this works, the learner will feel..."),
        );

        ui.add_space(12.0);
        ui.separator();

        // ── Phase Info Card ───────────────────────────────────────
        ui.heading(
            egui::RichText::new(format!("{}", studio.active_phase))
                .color(egui::Color32::from_rgb(180, 200, 220)),
        );

        ui.label(
            egui::RichText::new(studio.active_phase.description())
                .italics()
                .color(egui::Color32::from_rgb(160, 160, 175)),
        );

        ui.add_space(6.0);

        // Detail toggleable
        ui.checkbox(&mut studio.show_hero_journey, "Show Hero's Journey details");
        if studio.show_hero_journey {
            egui::Grid::new("phase_detail_grid")
                .num_columns(2)
                .spacing([8.0, 4.0])
                .show(ui, |ui| {
                    ui.label("🗺️ Stage:");
                    ui.label(studio.active_phase.hero_stage());
                    ui.end_row();

                    ui.label("🦴 Body:");
                    ui.label(studio.active_phase.body_part());
                    ui.end_row();

                    ui.label("📍 Location:");
                    ui.label(studio.active_phase.location());
                    ui.end_row();

                    ui.label("⚔️ Party:");
                    ui.label(studio.active_phase.party_member());
                    ui.end_row();
                });
        }

        ui.add_space(12.0);
        ui.separator();

        // ── Learning Objectives ──────────────────────────────────
        ui.heading(
            egui::RichText::new("🎯 Learning Objectives")
                .color(egui::Color32::from_rgb(180, 200, 220)),
        );

        let mut to_remove: Option<usize> = None;
        let obj_count = studio.learning_objectives.len();
        for (i, obj) in studio.learning_objectives.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("{}.", i + 1));
                ui.text_edit_singleline(obj);
                if obj_count > 1 && ui.small_button("✕").clicked() {
                    to_remove = Some(i);
                }
            });
        }
        if let Some(idx) = to_remove {
            studio.learning_objectives.remove(idx);
        }
        if ui.small_button("➕ Add Objective").clicked() {
            studio.learning_objectives.push(String::new());
        }
    });
}

// ─── Phase Workspace (Center) ────────────────────────────────────────────────

fn render_phase_workspace(ui: &mut egui::Ui, studio: &mut StudioState) {
    let phase = studio.active_phase;

    egui::ScrollArea::vertical().show(ui, |ui| {
        // Phase header
        ui.heading(
            egui::RichText::new(format!(
                "Station {} — {} {}",
                phase.station(),
                phase.emoji(),
                phase.label()
            ))
            .color(egui::Color32::from_rgb(220, 190, 80)),
        );
        ui.label(
            egui::RichText::new(phase.description())
                .italics()
                .color(egui::Color32::from_rgb(160, 160, 175)),
        );
        ui.separator();

        match phase {
            Phase::Analyze => render_analyze_workspace(ui, studio),
            Phase::Design => render_design_workspace(ui, studio),
            Phase::Develop => render_develop_workspace(ui, studio),
            Phase::Implement => render_implement_workspace(ui, studio),
            Phase::Evaluate => render_evaluate_workspace(ui, studio),
            Phase::Contrast => render_crap_workspace(ui, phase),
            Phase::Repetition => render_crap_workspace(ui, phase),
            Phase::Alignment => render_crap_workspace(ui, phase),
            Phase::Proximity => render_crap_workspace(ui, phase),
            Phase::Envision => render_eye_workspace(ui, phase, studio),
            Phase::Yoke => render_eye_workspace(ui, phase, studio),
            Phase::Evolve => render_eye_workspace(ui, phase, studio),
        }
    });
}

// ─── Phase-Specific Workspaces ───────────────────────────────────────────────

fn render_analyze_workspace(ui: &mut egui::Ui, studio: &mut StudioState) {
    ui.add_space(8.0);

    // SME Interview Section
    ui.heading("🎤 SME Interview Notes");
    ui.label("Record notes from your Subject Matter Expert conversations.");
    ui.add_space(4.0);

    ui.label(egui::RichText::new("Standard SME Questions (STAR Method):").strong());
    let questions = [
        "What business need does this address?",
        "What challenges do learners currently face?",
        "How would you explain this to an 8-year-old?",
        "Describe a real situation where this knowledge was critical (STAR).",
        "What's the single most important takeaway?",
    ];
    for (i, q) in questions.iter().enumerate() {
        ui.label(format!("  {}. {}", i + 1, q));
    }

    ui.add_space(8.0);
    ui.label("SME Notes:");
    ui.add(
        egui::TextEdit::multiline(&mut studio.sme_notes)
            .desired_rows(10)
            .hint_text("Record SME interview responses here...")
            .desired_width(f32::INFINITY),
    );

    ui.add_space(12.0);
    ui.separator();

    // Needs Assessment
    ui.heading("📊 Needs Assessment");
    ui.label("Identify the gap between current state and desired outcomes.");

    egui::Grid::new("needs_grid")
        .num_columns(2)
        .spacing([12.0, 8.0])
        .show(ui, |ui| {
            ui.label("Current State:");
            ui.label("(What learners know/do now)");
            ui.end_row();

            ui.label("Desired State:");
            ui.label("(What learners should know/do after)");
            ui.end_row();

            ui.label("Gap:");
            ui.label("(The specific knowledge/skill deficit)");
            ui.end_row();
        });
}

fn render_design_workspace(ui: &mut egui::Ui, studio: &mut StudioState) {
    ui.add_space(8.0);

    // Backward Design
    ui.heading("📐 Backward Design (Wiggins & McTighe)");
    ui.label("Design from outcomes backward: Results → Evidence → Content.");

    ui.add_space(8.0);

    egui::Grid::new("backward_design")
        .num_columns(2)
        .spacing([12.0, 8.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("1. Desired Results").strong());
            ui.label("What measurable outcomes should learners achieve?");
            ui.end_row();

            ui.label(egui::RichText::new("2. Evidence of Learning").strong());
            ui.label("What assessment will prove they learned it?");
            ui.end_row();

            ui.label(egui::RichText::new("3. Learning Plan").strong());
            ui.label("What content and activities will get them there?");
            ui.end_row();
        });

    ui.add_space(12.0);
    ui.separator();

    // Bloom's Taxonomy
    ui.heading("🌱 Bloom's Taxonomy Alignment");
    ui.label("Map each objective to a cognitive level:");

    ui.horizontal_wrapped(|ui| {
        let levels = [
            ("Remember", "📝"),
            ("Understand", "💡"),
            ("Apply", "🔧"),
            ("Analyze", "🔍"),
            ("Evaluate", "⚖️"),
            ("Create", "🎨"),
        ];
        for (level, icon) in levels {
            ui.label(
                egui::RichText::new(format!("{} {}", icon, level))
                    .color(egui::Color32::from_rgb(160, 180, 200)),
            );
            ui.label("→");
        }
    });

    ui.add_space(8.0);

    // Learning objectives with Bloom's level
    for (i, obj) in studio.learning_objectives.iter().enumerate() {
        if !obj.is_empty() {
            ui.horizontal(|ui| {
                ui.label(format!("Objective {}: ", i + 1));
                ui.label(egui::RichText::new(obj).strong());
                ui.label(" → Bloom's: ___");
            });
        }
    }
}

fn render_develop_workspace(ui: &mut egui::Ui, studio: &mut StudioState) {
    ui.add_space(8.0);

    ui.heading("🛠️ Content Creation");
    ui.label("Draft your learning content. Use the ART sidebar (right) to generate visual assets.");

    ui.add_space(8.0);

    ui.label("Content Draft:");
    ui.add(
        egui::TextEdit::multiline(&mut studio.content_draft)
            .desired_rows(15)
            .hint_text("Write your learning content here...\n\nTip: Use the ART sidebar to generate images, then reference them in your content.")
            .desired_width(f32::INFINITY),
    );

    ui.add_space(12.0);
    ui.separator();

    // Cognitive Load Management
    ui.heading("🧠 Cognitive Load Check");
    egui::Grid::new("cog_load")
        .num_columns(2)
        .spacing([12.0, 6.0])
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Intrinsic Load:").strong());
            ui.label("Inherent difficulty of the subject matter");
            ui.end_row();

            ui.label(egui::RichText::new("Extraneous Load:").strong());
            ui.label("Unnecessary effort from poor design — minimize this!");
            ui.end_row();

            ui.label(egui::RichText::new("Germane Load:").strong());
            ui.label("Productive effort for schema construction — maximize this!");
            ui.end_row();
        });
}

fn render_implement_workspace(ui: &mut egui::Ui, _studio: &mut StudioState) {
    ui.add_space(8.0);

    ui.heading("🚀 Implementation Checklist");
    ui.label("Package and deploy your learning experience.");

    ui.add_space(8.0);

    let checklist = [
        "Content reviewed for accuracy",
        "All media assets generated and embedded",
        "Assessment questions linked to objectives",
        "Navigation/flow tested",
        "Accessibility check (WCAG AA)",
        "Quality Matters rubric pre-check",
        "Delivery platform configured",
        "Pilot test with 1-2 learners",
    ];

    for item in checklist {
        ui.horizontal(|ui| {
            ui.label("☐");
            ui.label(item);
        });
    }
}

fn render_evaluate_workspace(ui: &mut egui::Ui, _studio: &mut StudioState) {
    ui.add_space(8.0);

    ui.heading("📊 Evaluation Framework");
    ui.label("Define how you'll measure whether learning occurred.");

    ui.add_space(8.0);

    // Kirkpatrick's Four Levels
    ui.heading("Kirkpatrick's Four Levels:");

    egui::Grid::new("kirkpatrick")
        .num_columns(3)
        .spacing([12.0, 6.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Level").strong());
            ui.label(egui::RichText::new("Question").strong());
            ui.label(egui::RichText::new("Method").strong());
            ui.end_row();

            ui.label("1. Reaction");
            ui.label("Did they like it?");
            ui.label("Surveys, feedback forms");
            ui.end_row();

            ui.label("2. Learning");
            ui.label("Did they learn it?");
            ui.label("Pre/post tests, quizzes");
            ui.end_row();

            ui.label("3. Behavior");
            ui.label("Do they use it?");
            ui.label("Observation, performance data");
            ui.end_row();

            ui.label("4. Results");
            ui.label("Did it matter?");
            ui.label("Business metrics, ROI");
            ui.end_row();
        });
}

fn render_crap_workspace(ui: &mut egui::Ui, phase: Phase) {
    ui.add_space(8.0);

    match phase {
        Phase::Contrast => {
            ui.heading("🧥 Contrast — Stand Out from the Forgettable");
            ui.label("Find a bad example — name exactly what makes it forgettable.");
            ui.add_space(8.0);
            ui.label("Contrast is about making elements that should be different OBVIOUSLY different.");
            ui.add_space(4.0);
            ui.label("• Size contrast: headlines vs body text");
            ui.label("• Color contrast: call-to-action vs background");
            ui.label("• Shape contrast: rounded vs angular elements");
            ui.label("• Weight contrast: bold emphasis vs regular flow");
        }
        Phase::Repetition => {
            ui.heading("❤️ Repetition — The Core Concept Loop");
            ui.label("Identify the ONE core concept that must be encountered multiple times.");
            ui.add_space(8.0);
            ui.label("Repetition creates unity. Repeat visual elements, structural patterns, and key vocabulary:");
            ui.add_space(4.0);
            ui.label("• Color scheme consistency across all materials");
            ui.label("• Typography hierarchy used everywhere");
            ui.label("• Layout patterns learners can predict");
            ui.label("• Key vocabulary reinforced in every section");
        }
        Phase::Alignment => {
            ui.heading("🦴 Alignment — The Spine of Your Design");
            ui.label("Check: does your hook connect directly to your measurable objective?");
            ui.add_space(8.0);
            ui.label("Every element should have a visual connection to something else on the page:");
            ui.add_space(4.0);
            ui.label("• Left-align or center-align — pick ONE and commit");
            ui.label("• Grid systems create invisible structure");
            ui.label("• Objectives → Content → Assessment must ALIGN");
            ui.label("• If something looks arbitrarily placed, it IS");
        }
        Phase::Proximity => {
            ui.heading("✋ Proximity — Cluster Related Content");
            ui.label("What belongs in Act 1 vs Act 2 vs Act 3?");
            ui.add_space(8.0);
            ui.label("Group related items together. Separate unrelated items:");
            ui.add_space(4.0);
            ui.label("• Related concepts → same section, close together");
            ui.label("• Unrelated concepts → clear visual separation");
            ui.label("• White space is a design tool, not wasted space");
            ui.label("• If items are related, they should LOOK related");
        }
        _ => {}
    }
}

fn render_eye_workspace(ui: &mut egui::Ui, phase: Phase, studio: &mut StudioState) {
    ui.add_space(8.0);

    match phase {
        Phase::Envision => {
            ui.heading("👁️ Envision — The Road Back");
            ui.label("Write your PEARL Vision: 'When this works, the learner will feel...'");
            ui.add_space(8.0);

            ui.label("PEARL Vision Statement:");
            ui.add(
                egui::TextEdit::multiline(&mut studio.pearl_vision)
                    .desired_rows(5)
                    .hint_text(
                        "When this learning experience works as intended, \
                         the learner will feel..."
                    )
                    .desired_width(f32::INFINITY),
            );

            ui.add_space(8.0);
            ui.label("Questions to consider:");
            ui.label("• What emotion should the learner feel at completion?");
            ui.label("• What will they be able to DO that they couldn't before?");
            ui.label("• How will they describe this experience to a peer?");
        }
        Phase::Yoke => {
            ui.heading("🔗 Yoke — The Grand Coupling");
            ui.label("Connect your learning objective to a real-world moment the student will face.");
            ui.add_space(8.0);

            ui.label("A yoke joins two things together. Here, we join:");
            ui.add_space(4.0);
            ui.label("  📚 What was learned  ↔  🌍 Where it's used");
            ui.add_space(8.0);

            ui.label("Transfer scenarios:");
            ui.label("• When will the learner next encounter this knowledge?");
            ui.label("• What real-world problem does this solve?");
            ui.label("• Who will benefit from the learner applying this?");
        }
        Phase::Evolve => {
            ui.heading("🫁 Evolve — Return with the Elixir");
            ui.label("Commit the design to your Book — the Iron Road continues.");
            ui.add_space(8.0);

            ui.label("The Great Recycler asks:");
            ui.add_space(4.0);
            ui.label("• What worked well that should be kept for next iteration?");
            ui.label("• What didn't work that should be redesigned?");
            ui.label("• What new questions emerged during this journey?");
            ui.label("• What has the DESIGNER learned about their own practice?");

            ui.add_space(12.0);
            ui.separator();

            ui.label(
                egui::RichText::new("🔄 Autopoiesis — the system evolves through use.")
                    .italics()
                    .color(egui::Color32::from_rgb(140, 160, 180)),
            );
        }
        _ => {}
    }
}

// ─── ART Sidebar (Right) ────────────────────────────────────────────────────

fn render_art_sidebar(
    ui: &mut egui::Ui,
    studio: &mut StudioState,
    sidecar: &ArtSidecarState,
    mailbox: &CreativeMailbox,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading(
            egui::RichText::new("🎨 ART Studio")
                .color(egui::Color32::from_rgb(220, 190, 80)),
        );
        ui.separator();

        // ── Sidecar Status ───────────────────────────────────────
        ui.collapsing("⚡ Sidecar Status", |ui| {
            egui::Grid::new("sidecar_status")
                .num_columns(2)
                .spacing([8.0, 4.0])
                .show(ui, |ui| {
                    ui.label("Trinity :3000");
                    ui.label(format!("{}", sidecar.trinity));
                    ui.end_row();

                    ui.label("ComfyUI :8188");
                    ui.label(format!("{}", sidecar.comfyui));
                    ui.end_row();

                    ui.label("LLM :8080");
                    ui.label(format!("{}", sidecar.llm));
                    ui.end_row();
                });
        });

        ui.add_space(8.0);
        ui.separator();

        // ── Image Generation ─────────────────────────────────────
        ui.heading(
            egui::RichText::new("🖼️ Generate Image")
                .color(egui::Color32::from_rgb(180, 200, 220)),
        );

        ui.label("Prompt:");
        ui.add(
            egui::TextEdit::multiline(&mut studio.image_prompt)
                .desired_rows(3)
                .hint_text("Describe the educational visual you need...")
                .desired_width(f32::INFINITY),
        );

        ui.horizontal(|ui| {
            ui.label("Size:");
            ui.add(egui::DragValue::new(&mut studio.image_width).range(256..=2048));
            ui.label("×");
            ui.add(egui::DragValue::new(&mut studio.image_height).range(256..=2048));
        });

        let can_generate = !studio.image_prompt.is_empty();
        if ui
            .add_enabled(can_generate, egui::Button::new("🎨 Generate"))
            .clicked()
        {
            let req = ImageGenRequest {
                prompt: studio.image_prompt.clone(),
                negative_prompt: None,
                width: studio.image_width,
                height: studio.image_height,
            };
            request_image_generation(mailbox, req);
            info!("🎨 Image generation requested: {}", studio.image_prompt);
        }

        // Pending indicator
        if let Ok(count) = mailbox.pending_count.try_lock() {
            if *count > 0 {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(format!("{} generation(s) pending...", *count));
                });
            }
        }

        // Show completed results
        if let Ok(results) = mailbox.image_results.try_lock() {
            if !results.is_empty() {
                ui.add_space(4.0);
                ui.label(format!("✅ {} image(s) generated", results.len()));
                for result in results.iter() {
                    ui.horizontal(|ui| {
                        ui.label(format!("  ID: {}", result.id));
                        if let Some(ref url) = result.url {
                            ui.label(format!(" → {}", url));
                        }
                    });
                }
            }
        }

        ui.add_space(12.0);
        ui.separator();

        // ── Quick Asset Generators ───────────────────────────────
        ui.heading(
            egui::RichText::new("⚡ Quick Generate")
                .color(egui::Color32::from_rgb(180, 200, 220)),
        );

        let quick_presets = [
            ("🎵 Music", "Generate background music"),
            ("📹 Video", "Generate instructional video"),
            ("🎲 3D Mesh", "Generate 3D model"),
        ];

        for (label, tooltip) in quick_presets {
            let btn = ui.add_enabled(false, egui::Button::new(label).min_size(egui::vec2(120.0, 0.0)));
            if btn.hovered() {
                btn.on_hover_text(format!("{} (coming soon)", tooltip));
            }
        }

        ui.add_space(12.0);
        ui.separator();

        // ── Generated Asset Gallery ──────────────────────────────
        ui.heading(
            egui::RichText::new("🖼️ Asset Gallery")
                .color(egui::Color32::from_rgb(180, 200, 220)),
        );

        if studio.generated_assets.is_empty() {
            ui.label(
                egui::RichText::new("No assets generated yet.\nUse the controls above to create visual content.")
                    .italics()
                    .color(egui::Color32::from_rgb(120, 120, 140)),
            );
        } else {
            for asset in &studio.generated_assets {
                ui.label(asset);
            }
        }
    });
}

// ─── Status Bar (Bottom) ─────────────────────────────────────────────────────

fn render_status_bar(ui: &mut egui::Ui, server: &TrinityServerState, sidecar: &ArtSidecarState) {
    ui.horizontal(|ui| {
        // Connection status
        let trinity_icon = match &sidecar.trinity {
            crate::creative_bridge::SidecarStatus::Healthy => "🟢",
            _ => if server.healthy { "🟡" } else { "🔴" },
        };
        ui.label(format!("{} Trinity", trinity_icon));

        ui.separator();

        // Phase from server
        let phase_display = if server.phase.is_empty() {
            "—".to_string()
        } else {
            server.phase.clone()
        };
        ui.label(format!("📍 Phase: {}", phase_display));

        ui.separator();

        // Game stats
        ui.label(format!("Ch.{}", server.chapter));
        ui.separator();
        ui.label(format!("⭐ XP: {}", server.xp));
        ui.separator();
        ui.label(format!("🔥 Coal: {:.0}%", server.coal));
        ui.separator();
        ui.label(format!("💨 Steam: {:.0}", server.steam));

        // Right-aligned comfyui status
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let comfy_icon = match &sidecar.comfyui {
                crate::creative_bridge::SidecarStatus::Healthy => "🟢",
                crate::creative_bridge::SidecarStatus::Unknown => "⚪",
                crate::creative_bridge::SidecarStatus::Unhealthy(_) => "🔴",
            };
            ui.label(format!("{} ComfyUI", comfy_icon));

            let llm_icon = match &sidecar.llm {
                crate::creative_bridge::SidecarStatus::Healthy => "🟢",
                crate::creative_bridge::SidecarStatus::Unknown => "⚪",
                crate::creative_bridge::SidecarStatus::Unhealthy(_) => "🔴",
            };
            ui.label(format!("{} LLM", llm_icon));
        });
    });
}
