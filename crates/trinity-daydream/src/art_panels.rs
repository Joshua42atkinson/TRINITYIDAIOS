// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — ART Control Rail (bevy_egui)
// ═══════════════════════════════════════════════════════════════════════════════
//
// MINIMAL egui overlay for the ART Canvas. Takes ≤10% of screen height.
// The canvas behind this is the star — this rail is just the steering wheel.
//
// Layout:
//   ┌──────────────────────────────────────────────────────────────┐
//   │  [🖼️ Image] [🎵 Tempo] [🎲 3D] [🎬 Video] [🗣️ Voice]       │
//   │  Prompt: [________________________________] [✨ Generate]    │
//   │  Style: [Steampunk ▼]    Status: 🟢 LLM  ⚪ ComfyUI        │
//   └──────────────────────────────────────────────────────────────┘
//
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

use crate::creative_bridge::{request_image_generation, request_tempo_generation, ArtSidecarState, CreativeMailbox, ImageGenRequest, TempoGenRequest, SidecarStatus};

// ─── Plugin ──────────────────────────────────────────────────────────────────

/// Minimal ART control rail — egui overlay at bottom of canvas.
pub struct ArtPanelsPlugin;

impl Plugin for ArtPanelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .insert_resource(CanvasState::default())
            .add_systems(EguiPrimaryContextPass, render_control_rail);
    }
}

// ─── State ───────────────────────────────────────────────────────────────────

/// Which creative lane is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CreativeLane {
    #[default]
    Image,
    Tempo,
    Mesh3D,
    Video,
    Voice,
}

impl CreativeLane {
    fn label(&self) -> &'static str {
        match self {
            CreativeLane::Image => "🖼️ Image",
            CreativeLane::Tempo => "🎵 Tempo",
            CreativeLane::Mesh3D => "🎲 3D",
            CreativeLane::Video => "🎬 Video",
            CreativeLane::Voice => "🗣️ Voice",
        }
    }

    fn all() -> &'static [CreativeLane] {
        &[
            CreativeLane::Image,
            CreativeLane::Tempo,
            CreativeLane::Mesh3D,
            CreativeLane::Video,
            CreativeLane::Voice,
        ]
    }
}

/// Visual style presets (from CharacterSheet.creative_config.visual_style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StylePreset {
    #[default]
    Steampunk,
    Cyberpunk,
    Fantasy,
    Minimalist,
    Retro,
    Noir,
}

impl StylePreset {
    fn label(&self) -> &'static str {
        match self {
            StylePreset::Steampunk => "⚙️ Steampunk",
            StylePreset::Cyberpunk => "🌃 Cyberpunk",
            StylePreset::Fantasy => "🐉 Fantasy",
            StylePreset::Minimalist => "⬜ Minimalist",
            StylePreset::Retro => "👾 Retro",
            StylePreset::Noir => "🎩 Noir",
        }
    }

    fn prompt_suffix(&self) -> &'static str {
        match self {
            StylePreset::Steampunk => "steampunk aesthetic, brass gears, steam pipes, warm amber lighting",
            StylePreset::Cyberpunk => "cyberpunk aesthetic, neon lights, holographic, blue and pink lighting",
            StylePreset::Fantasy => "fantasy aesthetic, magical atmosphere, ethereal glow",
            StylePreset::Minimalist => "minimalist aesthetic, clean lines, simple shapes",
            StylePreset::Retro => "retro pixel art, 8-bit graphics, nostalgic",
            StylePreset::Noir => "noir aesthetic, dramatic shadows, film noir lighting",
        }
    }

    fn all() -> &'static [StylePreset] {
        &[
            StylePreset::Steampunk,
            StylePreset::Cyberpunk,
            StylePreset::Fantasy,
            StylePreset::Minimalist,
            StylePreset::Retro,
            StylePreset::Noir,
        ]
    }
}

/// Persistent UI state for the control rail.
#[derive(Resource)]
pub struct CanvasState {
    pub active_lane: CreativeLane,
    pub prompt: String,
    pub style: StylePreset,
    pub last_status: String,
    pub generating: bool,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            active_lane: CreativeLane::Image,
            prompt: String::new(),
            style: StylePreset::Steampunk,
            last_status: "Ready".to_string(),
            generating: false,
        }
    }
}

// ─── Colors ──────────────────────────────────────────────────────────────────

const RAIL_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(15, 15, 25, 230);
const OLD_GOLD: egui::Color32 = egui::Color32::from_rgb(207, 185, 145);
const CYAN: egui::Color32 = egui::Color32::from_rgb(0, 255, 255);
const DARK_TEXT: egui::Color32 = egui::Color32::from_rgb(180, 180, 195);
const ACTIVE_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(207, 185, 145, 40);

// ─── Render ──────────────────────────────────────────────────────────────────

/// The one and only UI system — a bottom control rail.
fn render_control_rail(
    mut contexts: EguiContexts,
    mut state: ResMut<CanvasState>,
    sidecar: Option<Res<ArtSidecarState>>,
    mailbox: Option<Res<CreativeMailbox>>,
    consist: Option<Res<crate::train_car::TrainConsist>>,
) {
    if let Some(consist_data) = consist {
        if consist_data.user_index == 0 || consist_data.user_index == consist_data.length() - 1 {
            return; // Art Control Rail ONLY visible when standing inside an Accordion ART Car
        }
    }

    let Ok(ctx) = contexts.ctx_mut() else { return };

    // ── Style: dark, minimal, transparent ─────────────────────────
    let mut style = (*ctx.style()).clone();
    style.visuals.window_fill = RAIL_BG;
    style.visuals.panel_fill = RAIL_BG;
    style.visuals.override_text_color = Some(DARK_TEXT);
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_premultiplied(30, 30, 45, 200);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_premultiplied(50, 45, 35, 200);
    style.visuals.widgets.active.bg_fill = ACTIVE_BG;
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    ctx.set_style(style);

    // ── Bottom Panel: Control Rail ────────────────────────────────
    egui::TopBottomPanel::bottom("art_control_rail")
        .min_height(90.0)
        .max_height(120.0)
        .frame(egui::Frame::new()
            .fill(RAIL_BG)
            .inner_margin(egui::Margin::symmetric(16, 10))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(207, 185, 145, 60)))
        )
        .show(ctx, |ui| {
            // ── Row 1: Lane Selector ─────────────────────────────
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;

                for lane in CreativeLane::all() {
                    let is_active = state.active_lane == *lane;
                    let text = egui::RichText::new(lane.label())
                        .size(14.0)
                        .color(if is_active { OLD_GOLD } else { DARK_TEXT });

                    let btn = if is_active {
                        egui::Button::new(text)
                            .fill(ACTIVE_BG)
                            .stroke(egui::Stroke::new(1.0, OLD_GOLD))
                    } else {
                        egui::Button::new(text)
                            .fill(egui::Color32::TRANSPARENT)
                    };

                    if ui.add(btn).clicked() {
                        state.active_lane = *lane;
                    }
                }

                // Right-align: status indicators
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Sidecar status
                    let (comfy_status, comfy_color) = match &sidecar {
                        Some(s) if matches!(s.comfyui, SidecarStatus::Healthy) => ("🟢 ComfyUI", OLD_GOLD),
                        _ => ("⚪ ComfyUI", DARK_TEXT),
                    };
                    ui.label(egui::RichText::new(comfy_status).size(11.0).color(comfy_color));

                    let (llm_status, llm_color) = match &sidecar {
                        Some(s) if matches!(s.llm, SidecarStatus::Healthy) => ("🟢 LLM", OLD_GOLD),
                        _ => ("⚪ LLM", DARK_TEXT),
                    };
                    ui.label(egui::RichText::new(llm_status).size(11.0).color(llm_color));
                });
            });

            ui.add_space(4.0);

            // ── Row 2: Prompt + Generate ─────────────────────────
            ui.horizontal(|ui| {
                // Style selector
                egui::ComboBox::from_id_salt("style_preset")
                    .selected_text(egui::RichText::new(state.style.label()).size(12.0).color(OLD_GOLD))
                    .width(130.0)
                    .show_ui(ui, |ui| {
                        for preset in StylePreset::all() {
                            let text = egui::RichText::new(preset.label()).size(12.0);
                            ui.selectable_value(&mut state.style, *preset, text);
                        }
                    });

                // Prompt input
                let prompt_response = ui.add(
                    egui::TextEdit::singleline(&mut state.prompt)
                        .hint_text("Describe what you want to create...")
                        .desired_width(ui.available_width() - 100.0)
                        .font(egui::TextStyle::Body)
                );

                // Generate button
                let generate_text = if state.generating {
                    egui::RichText::new("⏳ Generating...").size(13.0).color(CYAN)
                } else {
                    egui::RichText::new("✨ Generate").size(13.0).color(OLD_GOLD)
                };

                let can_generate = !state.prompt.is_empty() && !state.generating;
                let generate_btn = egui::Button::new(generate_text)
                    .fill(if can_generate {
                        egui::Color32::from_rgba_premultiplied(207, 185, 145, 30)
                    } else {
                        egui::Color32::TRANSPARENT
                    });

                let enter_pressed = prompt_response.lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if ui.add_enabled(can_generate, generate_btn).clicked() || (enter_pressed && can_generate) {
                    // Dispatch generation based on active lane
                    let full_prompt = format!("{}, {}", state.prompt, state.style.prompt_suffix());

                    match state.active_lane {
                        CreativeLane::Image => {
                            if let Some(ref mb) = mailbox {
                                let request = ImageGenRequest {
                                    prompt: full_prompt.clone(),
                                    negative_prompt: Some("blurry, low quality, text, watermark".to_string()),
                                    width: 1024,
                                    height: 1024,
                                };
                                request_image_generation(mb, request);
                                state.generating = true;
                                state.last_status = format!("Generating: {}", state.prompt);
                            } else {
                                state.last_status = "⚠ ComfyUI not connected".to_string();
                            }
                        }
                        CreativeLane::Tempo => {
                            if let Some(ref mb) = mailbox {
                                let request = TempoGenRequest {
                                    prompt: state.prompt.clone(),
                                    style: Some(state.style.prompt_suffix().to_string()),
                                    duration_secs: 15, // Output 15 second tracks
                                };
                                request_tempo_generation(mb, request);
                                state.generating = true;
                                state.last_status = format!("Procedural Tempo: {}", state.prompt);
                            } else {
                                state.last_status = "⚠ Tempo Engine unavailable".to_string();
                            }
                        }
                        _ => {
                            state.last_status = format!("{} generation coming soon", state.active_lane.label());
                        }
                    }
                }
            });

            // ── Row 3: Status line ───────────────────────────────
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(&state.last_status)
                        .size(11.0)
                        .color(egui::Color32::from_rgb(120, 120, 140))
                        .italics()
                );
            });
        });
}
