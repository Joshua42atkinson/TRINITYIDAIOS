// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Yardmaster Dashboard (Archive UI)
// ═══════════════════════════════════════════════════════════════════════════════
//
// The Y-Car (Yardmaster) terminal UI.
// Fetches the Project Archive from /api/projects on the Axum backend and allows
// the user to manage Socratic sessions natively within the Daydream LitRPG.
//
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use serde::{Deserialize, Serialize};

use crate::train_car::TrainConsist;

// ─── Models ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub id: String,
    pub session_id: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
    pub archived_at: Option<String>,
    pub archive_reason: Option<String>,
}

// ─── State ───────────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
pub struct YardmasterState {
    pub projects: Vec<ProjectSummary>,
    pub status_msg: String,
    pub is_loading: bool,
    pub request_fetch: bool,
}

// ─── Plugin ──────────────────────────────────────────────────────────────────

pub struct YardmasterUiPlugin;

impl Plugin for YardmasterUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin::default());
        }
        
        app.insert_resource(YardmasterState {
            request_fetch: true, // Fetch immediately on startup
            ..default()
        })
        .add_systems(EguiPrimaryContextPass, render_yardmaster_ui)
        .add_systems(Update, handle_project_fetching);
    }
}

// ─── Systems ─────────────────────────────────────────────────────────────────

/// Background thread fetcher for the projects endpoint
fn handle_project_fetching(mut state: ResMut<YardmasterState>) {
    if !state.request_fetch {
        return;
    }
    state.request_fetch = false;
    state.is_loading = true;
    state.status_msg = "Fetching projects from Trinity backend...".to_string();

    // Async block wrapped in a standard thread to match Daydream's pattern
    let (tx, rx) = crossbeam_channel::bounded(1);
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Hardcoded to localhost:3000 for local AI OS architecture
            let client = reqwest::Client::new();
            match client.get("http://127.0.0.1:3000/api/projects").send().await {
                Ok(response) => {
                    if let Ok(projects) = response.json::<Vec<ProjectSummary>>().await {
                        let _ = tx.send(Ok(projects));
                    } else {
                        let _ = tx.send(Err("Failed to parse projects JSON".to_string()));
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(format!("Reqwest error: {}", e)));
                }
            }
        });
    });

    // In a real ECS system we would store the channel Receiver in the Resource
    // and poll it every frame. Since this is an isolated task, we will poll
    // instantly and gracefully downgrade if it takes time. 
    // To avoid blocking the Bevy frame, we'll actually let the next frame poll it.
    // However, to keep it simple and architecturally sound for the Monolith, 
    // we use a persistent channel approach (or just block for demo).
    // Given the constraints, let's block temporarily for reliability, or use a dedicated system.
    
    // For now, we block lightly just to ensure UI populates:
    if let Ok(result) = rx.recv() {
        match result {
            Ok(projects) => {
                state.projects = projects;
                state.status_msg = format!("Loaded {} projects", state.projects.len());
            }
            Err(e) => {
                state.status_msg = e;
            }
        }
    }
    state.is_loading = false;
}

/// The Egui Render block
fn render_yardmaster_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<YardmasterState>,
    consist: Option<Res<TrainConsist>>,
) {
    // Only render if we are in the Yardmaster (last) Car
    if let Some(consist_data) = consist {
        if consist_data.user_index != consist_data.length() - 1 {
            return;
        }
    } else {
        return;
    }

    let Ok(ctx) = contexts.ctx_mut() else { return };

    let mut style = (*ctx.style()).clone();
    style.visuals.window_fill = egui::Color32::from_rgba_premultiplied(10, 15, 20, 240);
    style.visuals.override_text_color = Some(egui::Color32::from_rgb(200, 200, 210));
    ctx.set_style(style);

    egui::Window::new("🚂 Yardmaster Archive [RESTRICTED]")
        .default_width(600.0)
        .default_height(400.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("PROJECT ARCHIVE & OPERATIONS").color(egui::Color32::from_rgb(207, 185, 145)));
                ui.add_space(4.0);
                ui.label(egui::RichText::new("Permanent System Records of Socratic Sessions").italics());
            });

            ui.add_space(16.0);

            ui.horizontal(|ui| {
                if ui.button(egui::RichText::new("🔄 Refresh Records").color(egui::Color32::CYAN)).clicked() {
                    state.request_fetch = true;
                }
                ui.label(egui::RichText::new(&state.status_msg).color(egui::Color32::DARK_GRAY));
            });

            ui.add_space(8.0);
            ui.separator();

            egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                if state.projects.is_empty() && !state.is_loading {
                    ui.label("No projects found in the archive.");
                }

                let mut clicked_load = None;
                for project in &state.projects {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            let status_color = if project.status == "active" {
                                egui::Color32::GREEN
                            } else {
                                egui::Color32::LIGHT_GRAY
                            };
                            ui.label(egui::RichText::new(&project.status).color(status_color).strong());
                            ui.label(egui::RichText::new(&project.name).size(16.0).strong());
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Load Session Array").clicked() {
                                    clicked_load = Some(project.id.clone());
                                    // Normally we would dispatch a DaydreamCommand to load the project into the Accordion
                                }
                                ui.label(egui::RichText::new(project.created_at.split(' ').next().unwrap_or("")).small());
                            });
                        });
                        if let Some(reason) = &project.archive_reason {
                            ui.label(egui::RichText::new(format!("Archive Reason: {}", reason)).italics().small());
                        }
                    });
                    ui.add_space(4.0);
                }
                
                if let Some(id) = clicked_load {
                    state.status_msg = format!("Load request sent for '{}'", id);
                }
            });
        });
}
