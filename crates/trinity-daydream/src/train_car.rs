use bevy::prelude::*;

// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — The Isomorphic Train Array (Accordion Topology)
// ═══════════════════════════════════════════════════════════════════════════════
//
// The Train is the literal OS spatial wrapper.
// - The Front  (Index 0): The P-Car (Programmer Pete / Recycler). Permanent.
// - The Middle (Index N): The ART Cars. Dynamically expands "like an accordion".
//      Each car encapsulates a dedicated Session/Product (DnD Literacy game, etc).
// - The Back   (End):     The Y-Car (Yardmaster). Permanent OS Configuration terminal.
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deliverable {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtCar {
    pub session_id: String,
    pub product_name: String,
    pub deliverables: Vec<Deliverable>,
}

impl ArtCar {
    /// Writes the ArtCar into a portable JSON manifest that can be decoupled from Trinity
    /// and uploaded to the Conscious Framework economy.
    pub fn uncouple_and_export(&self) -> Result<std::path::PathBuf, std::io::Error> {
        let export_dir = std::path::PathBuf::from("workspace/exported_cars");
        std::fs::create_dir_all(&export_dir)?;
        
        let path = export_dir.join(format!("{}.json", self.session_id));
        let manifest = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            
        std::fs::write(&path, manifest)?;
        Ok(path)
    }
}

/// The entire physical composition of the OS Train.
#[derive(Resource)]
pub struct TrainConsist {
    pub user_index: usize, // Which car the user is currently standing in
    pub art_cars: Vec<ArtCar>,
}

impl Default for TrainConsist {
    fn default() -> Self {
        Self {
            user_index: 0, // Start in the P-Car
            art_cars: vec![
                ArtCar {
                    session_id: "init_session_001".to_string(),
                    product_name: "Session: Default Workspace".to_string(),
                    deliverables: vec![],
                }
            ],
        }
    }
}

impl TrainConsist {
    /// Total length of the train (P-Car + all ART cars + Y-Car)
    pub fn length(&self) -> usize {
        2 + self.art_cars.len()
    }

    /// Appends a new product session into the train array.
    pub fn couple_new_art_car(&mut self, product_name: String) {
        let new_car = ArtCar {
            session_id: uuid::Uuid::new_v4().to_string(),
            product_name,
            deliverables: vec![
                Deliverable { title: "Draft Proposal".to_string(), content: "Pending AI generation...".to_string() }
            ],
        };
        self.art_cars.push(new_car);
        bevy::log::info!("🚂 COUPLED NEW ART CAR: {}", self.art_cars.last().unwrap().product_name);
    }
}

pub struct IsomorphicTrainPlugin;

impl Plugin for IsomorphicTrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TrainConsist>()
           .add_systems(Update, handle_train_navigation)
           .add_systems(Update, render_universal_nav_bar);
    }
}

/// Allows the user to physically traverse the Train Consist.
fn handle_train_navigation(
    keys: Res<ButtonInput<KeyCode>>,
    mut consist: ResMut<TrainConsist>,
) {
    if keys.just_pressed(KeyCode::ArrowLeft) && consist.user_index > 0 {
        consist.user_index -= 1;
        info!("🚂 Trudging forward. Current Car Index: {}", consist.user_index);
    }
    
    if keys.just_pressed(KeyCode::ArrowRight) && consist.user_index < consist.length() - 1 {
        consist.user_index += 1;
        info!("🚂 Trudging backward. Current Car Index: {}", consist.user_index);
    }

    // OS Utility hotkey to quickly spawn a new product session
    if keys.just_pressed(KeyCode::KeyN) {
        let count = consist.art_cars.len() + 1;
        consist.couple_new_art_car(format!("Session: Generated Product 00{}", count));
    }

    // OS Utility hotkey to DECOUPLE the car into the economy pipeline
    if keys.just_pressed(KeyCode::KeyU) {
        if consist.user_index > 0 && consist.user_index < consist.length() - 1 {
            let art_car = &consist.art_cars[consist.user_index - 1];
            match art_car.uncouple_and_export() {
                Ok(path) => bevy::log::info!("✅ CAR UNCOUPLED TO DISK: {:?}", path),
                Err(e) => bevy::log::error!("❌ FAILED TO UNCOUPLE CAR: {}", e),
            }
        } else {
            bevy::log::warn!("⚠ Cannot uncouple Permanent System Cars (Pete and Yardmaster). Move to an ART Car to export.");
        }
    }
}

/// An overlay across all cars showing what State OS we are in.
fn render_universal_nav_bar(
    mut contexts: bevy_egui::EguiContexts,
    consist: Res<TrainConsist>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return };
    
    bevy_egui::egui::TopBottomPanel::top("train_car_nav")
        .frame(bevy_egui::egui::Frame::new().fill(bevy_egui::egui::Color32::from_rgba_premultiplied(10, 10, 15, 250)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(bevy_egui::egui::RichText::new("🚂 THE TRINITY").color(bevy_egui::egui::Color32::from_rgb(207, 185, 145)).strong());
                ui.add_space(20.0);
                
                // Construct the visual array of the train cars
                let mut visual_train = Vec::new();

                // 1. P-CAR
                let p_label = if consist.user_index == 0 { ">> [P] <<".to_string() } else { "[P]".to_string() };
                visual_train.push(p_label);

                // 2. ART CARS (The Accordion)
                for (i, art_car) in consist.art_cars.iter().enumerate() {
                    let true_index = i + 1;
                    let art_idx = i + 1;
                    let label = if consist.user_index == true_index { 
                        format!(">> [ART {}] <<", art_idx) 
                    } else { 
                        format!("[ART {}]", art_idx) 
                    };
                    visual_train.push(label);
                }

                // 3. Y-CAR
                let y_index = consist.length() - 1;
                let y_label = if consist.user_index == y_index { ">> [Y] <<".to_string() } else { "[Y]".to_string() };
                visual_train.push(y_label);

                ui.label(bevy_egui::egui::RichText::new(visual_train.join("  -  ")).color(bevy_egui::egui::Color32::GRAY));
                
                // Show product name if standing in an ART car
                if consist.user_index > 0 && consist.user_index < consist.length() - 1 {
                    let current_art = &consist.art_cars[consist.user_index - 1];
                    ui.with_layout(bevy_egui::egui::Layout::right_to_left(bevy_egui::egui::Align::Center), |ui| {
                        ui.label(bevy_egui::egui::RichText::new(format!("Payload: {}", current_art.product_name)).color(bevy_egui::egui::Color32::from_rgb(0, 255, 255)).italics());
                    });
                }
            });
        });
}
