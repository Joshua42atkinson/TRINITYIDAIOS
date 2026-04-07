use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// Holds the core LitRPG player state natively in Daydream
#[derive(Resource)]
pub struct LitRpgState {
    pub coal: u32,
    pub steam: u32,
    pub traction: u32,
    pub socratic_prose: Vec<(String, String)>, // (Speaker, Text)
    pub current_phase: String,
    pub active_tab: usize, // 0: Stats, 1: Grimoire, 2: Bestiary
    pub input_text: String,
}

impl Default for LitRpgState {
    fn default() -> Self {
        Self {
            coal: 100,
            steam: 50,
            traction: 0,
            socratic_prose: vec![
                ("AI".to_string(), "The locomotive’s furnace groans, seeking fuel. You have analyzed your learners and mapped the architecture, but now the raw materials must be forged in the fire.".to_string()),
                ("AI".to_string(), "Development is fraught with scope creep and endless tinkering. The Iron Road demands focus. What is the very first programmatic asset you intend to build for this module?".to_string()),
            ],
            current_phase: "Station 3: Development (Apply)".to_string(),
            active_tab: 0,
            input_text: String::new(),
        }
    }
}

pub struct IronRoadUiPlugin;

impl Plugin for IronRoadUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LitRpgState>()
           .add_systems(bevy_egui::EguiPrimaryContextPass, render_iron_road_book);
    }
}

fn render_iron_road_book(
    mut contexts: EguiContexts,
    mut state: ResMut<LitRpgState>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };
    
    // The book floats in the center
    egui::Window::new("The Iron Road")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .min_width(1200.0)
        .min_height(700.0)
        .resizable(false)
        .collapsible(false)
        .title_bar(false) // LitRPG aesthetic
        .frame(egui::Frame::NONE.fill(egui::Color32::from_rgba_premultiplied(5, 10, 20, 220))
            .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 200, 255)))
            .inner_margin(15.0).corner_radius(15.0)) // Holographic Outer Edge
        .show(ctx, |ui| {
            // The Inner Book Pages background - Holographic Glassmorphism
            egui::Frame::NONE
                .fill(egui::Color32::from_rgba_premultiplied(0, 20, 40, 180)) // Deep transparent blue
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(207, 185, 145))) // Old Gold inner stroke
                .corner_radius(8.0)
                .inner_margin(40.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.set_max_width(500.0);
                            // LEFT PAGE: THE GREAT RECYCLER
                            render_left_page(ui, &state);
                        });
                        
                        ui.add_space(40.0);
                        
                        ui.vertical(|ui| {
                            ui.set_max_width(500.0);
                            // RIGHT PAGE: THE PLAYER HANDBOOK
                            render_right_page(ui, &mut state);
                        });
                    });
                });
        });
}

fn render_left_page(ui: &mut egui::Ui, state: &LitRpgState) {
    ui.vertical(|ui| {
        // Aesthetic grouping
        ui.heading(egui::RichText::new(&state.current_phase).color(egui::Color32::from_rgb(0, 255, 255)).size(24.0)); // Cyan phase title
        ui.add_space(20.0);
        ui.separator();
        ui.add_space(20.0);

        egui::ScrollArea::vertical().id_salt("left_page_scroll").max_height(450.0).show(ui, |ui| {
            for (speaker, text) in &state.socratic_prose {
                if speaker == "AI" {
                    ui.label(egui::RichText::new(text).color(egui::Color32::from_rgb(200, 230, 255)).size(18.0)); // Bright holo text
                } else {
                    ui.label(egui::RichText::new(format!("> {}", text))
                        .color(egui::Color32::from_rgb(207, 185, 145)) // Gold user text
                        .size(16.0)
                        .background_color(egui::Color32::from_rgba_premultiplied(207, 185, 145, 20)));
                }
                ui.add_space(15.0);
            }
        });
    });
}

fn render_right_page(ui: &mut egui::Ui, state: &mut LitRpgState) {
    ui.add_space(5.0);

    ui.vertical(|ui| {
        // Style tabs for hologram
        ui.horizontal(|ui| {
            let active_color = egui::Color32::from_rgb(0, 255, 255);
            let inactive_color = egui::Color32::from_rgb(100, 150, 200);
            
            if ui.add(egui::SelectableLabel::new(state.active_tab == 0, egui::RichText::new("Stats & Cargo").color(if state.active_tab == 0 { active_color } else { inactive_color }))).clicked() { state.active_tab = 0; }
            if ui.add(egui::SelectableLabel::new(state.active_tab == 1, egui::RichText::new("The Grimoire").color(if state.active_tab == 1 { active_color } else { inactive_color }))).clicked() { state.active_tab = 1; }
            if ui.add(egui::SelectableLabel::new(state.active_tab == 2, egui::RichText::new("Bestiary").color(if state.active_tab == 2 { active_color } else { inactive_color }))).clicked() { state.active_tab = 2; }
        });
        ui.add_space(20.0);

        // Content Area
        egui::ScrollArea::vertical().id_salt("right_page_scroll").max_height(350.0).show(ui, |ui| {
            match state.active_tab {
                0 => {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("Coal: {}", state.coal)).color(egui::Color32::from_rgb(0, 255, 255)).size(22.0).strong());
                        ui.add_space(40.0);
                        ui.label(egui::RichText::new(format!("Steam: {}", state.steam)).color(egui::Color32::from_rgb(255, 100, 100)).size(22.0).strong());
                    });
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new("Cargo Manifest (Objectives)").size(18.0).color(egui::Color32::from_rgb(207, 185, 145)));
                    ui.separator();
                    let mut fake_check = false;
                    ui.checkbox(&mut fake_check, "Identify Target Audience");
                    ui.checkbox(&mut fake_check, "Define Pedagogy constraints");
                },
                1 => {
                    ui.label(egui::RichText::new("Hook Deck").size(18.0).color(egui::Color32::from_rgb(207, 185, 145)));
                    ui.separator();
                    ui.label(egui::RichText::new("🪄 Focus Lens (Lv. 2) - Costs 10 Steam").color(egui::Color32::from_rgb(200, 230, 255)));
                    ui.label(egui::RichText::new("🛡️ Warding Sigil (Lv. 1) - Costs 5 Coal").color(egui::Color32::from_rgb(200, 230, 255)));
                },
                2 => {
                    ui.label(egui::RichText::new("The Bestiary (VAAM)").size(18.0).color(egui::Color32::from_rgb(207, 185, 145)));
                    ui.separator();
                    ui.label(egui::RichText::new("1. Extrinsic Load (Lv. 2)").color(egui::Color32::from_rgb(200, 230, 255)));
                    ui.label(egui::RichText::new("2. Intrinsic Load (Lv. 1)").color(egui::Color32::from_rgb(200, 230, 255)));
                },
                _ => {}
            }
        });
        
        ui.add_space(20.0); // push down fixed amount instead of available_height

        // The Scribe Inkwell input
        ui.separator();
        ui.horizontal(|ui| {
            let text_edit = egui::TextEdit::multiline(&mut state.input_text)
                .hint_text("Dip your quill in the inkwell...")
                .desired_width(400.0)
                .desired_rows(2);
            ui.add(text_edit);
            
            if ui.button("Inscribe").clicked() && !state.input_text.trim().is_empty() {
                let msg = state.input_text.clone();
                state.socratic_prose.push(("User".to_string(), msg));
                state.input_text.clear();
                state.coal = state.coal.saturating_sub(2);
            }
        });
    });
}
