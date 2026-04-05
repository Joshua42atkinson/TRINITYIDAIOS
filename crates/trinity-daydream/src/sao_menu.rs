// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Sword Art Online (SAO) Style Menu & Hook Deck
// ═══════════════════════════════════════════════════════════════════════════════
//
// Replaces floating UI elements with a native, collapsible side/top menu.
// Manages "Dream Mode", which dictates the visibility of HUD telemetry, chat, 
// and the ART rails across the LitRPG world.
//
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

// ─── Colors ──────────────────────────────────────────────────────────────────
const MENU_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(15, 15, 20, 240);
const MENU_BORDER: egui::Color32 = egui::Color32::from_rgba_premultiplied(100, 100, 110, 200);
const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(180, 180, 195);
const GOLD_ACCENT: egui::Color32 = egui::Color32::from_rgb(207, 185, 145);
const CYAN_ACCENT: egui::Color32 = egui::Color32::from_rgb(0, 255, 255);

// ─── State ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaoTab {
    CharacterSheet,
    HookDeck, // Inventory of modifiers
    SystemLogs,
}

#[derive(Resource)]
pub struct SaoMenuState {
    pub is_open: bool,
    pub active_tab: SaoTab,
    pub is_dream_mode: bool, // If TRUE, hide all HUDs, chat, and ART rails
}

impl Default for SaoMenuState {
    fn default() -> Self {
        Self {
            is_open: false,
            active_tab: SaoTab::HookDeck,
            is_dream_mode: false,
        }
    }
}

// ─── Plugin ──────────────────────────────────────────────────────────────────

pub struct SaoMenuPlugin;

impl Plugin for SaoMenuPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin::default());
        }

        app.init_resource::<SaoMenuState>()
           .add_systems(Update, toggle_sao_menu_hotkey)
           .add_systems(EguiPrimaryContextPass, render_sao_menu);
    }
}

// ─── Systems ─────────────────────────────────────────────────────────────────

/// Allows the user to toggle the SAO Menu with the Escape or Tab key
fn toggle_sao_menu_hotkey(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<SaoMenuState>,
) {
    if keys.just_pressed(KeyCode::Tab) || keys.just_pressed(KeyCode::Escape) {
        state.is_open = !state.is_open;
    }
}

/// Renders the SAO Menu if open, and handles the static overlay toggle button
fn render_sao_menu(
    mut contexts: EguiContexts,
    mut state: ResMut<SaoMenuState>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return };

    // ── 1. The Floating Toggle Button ──
    // Always visible (even in Dream Mode) so the user can escape Dream Mode
    egui::Window::new("SAO_Toggle_Btn")
        .title_bar(false)
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::LEFT_CENTER, egui::vec2(10.0, 0.0))
        .frame(egui::Frame::none())
        .show(ctx, |ui| {
            let label = if state.is_open { "« CLOSE MENU" } else { "» MENU" };
            let dream_text = if state.is_dream_mode { " [DREAM]" } else { "" };
            
            let btn = ui.button(
                egui::RichText::new(format!("{}{}", label, dream_text))
                    .strong()
                    .color(if state.is_dream_mode { GOLD_ACCENT } else { TEXT_MUTED })
            );
            if btn.clicked() {
                state.is_open = !state.is_open;
            }
        });

    if !state.is_open {
        return;
    }

    // ── 2. The SAO Slide-Out Menu ──
    let frame = egui::Frame {
        fill: MENU_BG,
        stroke: egui::Stroke::new(1.0, MENU_BORDER),
        inner_margin: egui::Margin::same(16),
        corner_radius: egui::CornerRadius::same(6),
        ..Default::default()
    };

    egui::Window::new("SAO Main Menu")
        .title_bar(false)
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::LEFT_CENTER, egui::vec2(120.0, 0.0))
        .fixed_size(egui::vec2(350.0, 500.0))
        .frame(frame)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Header
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("SYSTEM MENU").color(GOLD_ACCENT).strong().size(18.0));
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // Dream Mode Toggle
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("DREAM MODE").color(TEXT_MUTED).size(14.0));
                    let dream_label = if state.is_dream_mode { "ON" } else { "OFF" };
                    let btn_color = if state.is_dream_mode { GOLD_ACCENT } else { egui::Color32::DARK_GRAY };
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(egui::RichText::new(dream_label).strong().color(btn_color)).clicked() {
                            state.is_dream_mode = !state.is_dream_mode;
                        }
                    });
                });
                if state.is_dream_mode {
                    ui.label(egui::RichText::new("HUD, Chat, and Rails are hidden.").italics().size(11.0).color(CYAN_ACCENT));
                }

                ui.add_space(15.0);

                // Tab Selector
                ui.horizontal(|ui| {
                    if ui.selectable_value(&mut state.active_tab, SaoTab::HookDeck, "Hook Deck").clicked() {}
                    if ui.selectable_value(&mut state.active_tab, SaoTab::CharacterSheet, "Character Sheet").clicked() {}
                    if ui.selectable_value(&mut state.active_tab, SaoTab::SystemLogs, "System Logs").clicked() {}
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // Tab Content
                egui::ScrollArea::vertical().max_height(350.0).show(ui, |ui| {
                    match state.active_tab {
                        SaoTab::HookDeck => {
                            render_hook_deck_inventory(ui);
                        }
                        SaoTab::CharacterSheet => {
                            ui.label("Name: Pioneer");
                            ui.label("Role: Instructional Designer");
                            ui.add_space(10.0);
                            ui.label("Aesthetic Drive: Steampunk");
                            ui.label("Focus: Gamified Learning");
                        }
                        SaoTab::SystemLogs => {
                            ui.label(egui::RichText::new("No recent critical errors.").color(egui::Color32::GREEN));
                        }
                    }
                });
            });
        });
}

/// Renders the Hook Deck (TCG items) inside the SAO Menu, replacing the old floating cards
fn render_hook_deck_inventory(ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("INVENTORY: GLOBAL MODIFIERS").strong().color(CYAN_ACCENT));
    ui.add_space(8.0);
    
    // Placeholder TCG hooks
    let hooks = vec![
        ("Socratic Shift", "Forces Pete to answer with a question.", true),
        ("Clarity Burst", "Simplifies the current jargon to a 5th-grade reading level.", false),
        ("Lore Pivot", "Shifts the literal LitRPG lore into the instructional design.", false),
    ];

    for (name, desc, active) in hooks {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                let color = if active { GOLD_ACCENT } else { TEXT_MUTED };
                ui.label(egui::RichText::new(name).strong().color(color));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let btn_text = if active { "DEACTIVATE" } else { "ACTIVATE" };
                    if ui.button(btn_text).clicked() {
                        // Normally fires a DaydreamCommand to the backend
                    }
                });
            });
            ui.add_space(4.0);
            ui.label(egui::RichText::new(desc).size(11.0).italics().color(egui::Color32::DARK_GRAY));
        });
        ui.add_space(6.0);
    }
}
