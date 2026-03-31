use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// A plugin that renders the DAYDREAM native HUD (VirtueTopology & CognitiveLoad)
pub struct DaydreamHudPlugin;

impl Plugin for DaydreamHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_hud);
    }
}

// ── Colors ──
const BG_COLOR: egui::Color32 = egui::Color32::from_rgba_premultiplied(15, 15, 20, 240);
const BORDER_COLOR: egui::Color32 = egui::Color32::from_rgba_premultiplied(40, 45, 60, 255);
const THEME_CYAN: egui::Color32 = egui::Color32::from_rgb(0, 255, 255);
const THEME_GOLD: egui::Color32 = egui::Color32::from_rgb(207, 185, 145);
const THEME_TEXT: egui::Color32 = egui::Color32::from_rgb(200, 200, 210);
const DANGER_RED: egui::Color32 = egui::Color32::from_rgb(255, 60, 60);

// Basic state variables for development — these would normally sync with backend state
#[derive(Resource)]
pub struct HudState {
    pub attention: f32, // 0.0 to 100.0
    pub steam: f32,     // 0.0 to 100.0
    pub creeps_slain: u32,
    pub quest_phase: String,
}

impl Default for HudState {
    fn default() -> Self {
        Self {
            attention: 85.0,
            steam: 42.0,
            creeps_slain: 3,
            quest_phase: "Extracting (Analyze)".to_string(),
        }
    }
}

pub fn render_hud(
    mut contexts: EguiContexts,
    // Add Res<HudState> once we inject it, for now mock it if not present
) {
    let Ok(ctx) = contexts.ctx_mut() else { return };
    
    let frame = egui::Frame {
        fill: BG_COLOR,
        stroke: egui::Stroke::new(1.0, BORDER_COLOR),
        inner_margin: egui::Margin::same(12.0),
        rounding: egui::Rounding::same(4.0),
        ..Default::default()
    };

    egui::Window::new("IRON ROAD TELEMETRY")
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(20.0, 20.0))
        .resizable(false)
        .collapsible(false)
        .title_bar(false) // Custom title bar
        .frame(frame)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Title
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("IRON ROAD TELEMETRY").color(THEME_GOLD).strong().size(14.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("● ONLINE").color(THEME_CYAN).size(10.0));
                    });
                });
                
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Virtue Topology
                ui.label(egui::RichText::new("VIRTUE TOPOLOGY").color(THEME_TEXT).size(12.0));
                ui.add_space(4.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("PEARL Phase:").color(THEME_TEXT));
                    ui.label(egui::RichText::new("Extracting (Analyze)").color(THEME_GOLD));
                });
                
                ui.add_space(10.0);

                // Cognitive Load (Bars)
                ui.label(egui::RichText::new("COGNITIVE LOAD").color(THEME_TEXT).size(12.0));
                ui.add_space(4.0);

                // Attention/Coal
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Attention (Coal):").color(THEME_TEXT).size(11.0));
                    let (rect, _response) = ui.allocate_exact_size(egui::vec2(120.0, 10.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 2.0, egui::Color32::from_rgba_premultiplied(50,50,50, 200));
                    let fill_rect = egui::Rect::from_min_size(rect.min, egui::vec2(120.0 * 0.85, 10.0));
                    ui.painter().rect_filled(fill_rect, 2.0, DANGER_RED);
                });

                ui.add_space(4.0);

                // Steam/Momentum
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Steam (Momentum):").color(THEME_TEXT).size(11.0));
                    let (rect, _response) = ui.allocate_exact_size(egui::vec2(120.0, 10.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 2.0, egui::Color32::from_rgba_premultiplied(50,50,50, 200));
                    let fill_rect = egui::Rect::from_min_size(rect.min, egui::vec2(120.0 * 0.42, 10.0));
                    ui.painter().rect_filled(fill_rect, 2.0, THEME_CYAN);
                });
                
                ui.add_space(10.0);

                // Scope Creep Bestiary 
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Bestiary: 3 Slain").color(THEME_GOLD).size(11.0));
                });
            });
        });
}
