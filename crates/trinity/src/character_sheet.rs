// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        character_sheet.rs
// PURPOSE:     Character Sheet persistence — user preferences and IBSTPI progress
//
// 🪟 THE LIVING CODE TEXTBOOK (Identity & Motivation):
// This file is the permanent agreement between YOU and the OS. It is designed 
// to be read, modified, and authored by YOU. If you want to add new skills to  
// track, or change how Trinity remembers your workflow preferences, edit this.
// ACTION: Check how `load_character_sheet()` pulls your profile from `~/.local`.
//
// 📖 THE HOOK BOOK CONNECTION:
// This file defines the 'Character Sheet' Hook from the School of Identity. 
// It is the foundational data structure that tells Pete who he is talking to.
// For a full catalogue of system capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
//
// ARCHITECTURE:
//   • Loads/saves CharacterSheet to ~/.trinity/character_sheet.json
//   • Single source of truth for user progress and creative settings
//   • IBSTPI competency domains stored (Pedigree, Perception, Craft, Validation)
//   • Dreyfus skill levels tracked per domain (Novice → Expert)
//   • Created on first run if not exists
//
// DEPENDENCIES:
//   - std::path — File system path handling
//   - serde — CharacterSheet JSON serialization
//   - tracing — Persistence operation logging
//   - trinity_protocol — CharacterSheet type definition
//
// CHANGES:
//   2026-03-16  Cascade  Migrated to §17 comment standard
//
// ═══════════════════════════════════════════════════════════════════════════════

use std::path::PathBuf;
use tracing::{info, warn};
use trinity_protocol::CharacterSheet;

/// Get the path to the character sheet file
fn character_sheet_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| {
        PathBuf::from(&home)
            .join(".local")
            .join("share")
            .join("trinity")
            .join("character_sheet.json")
    })
}

/// Load character sheet from disk, or return default if not found
pub fn load_character_sheet() -> CharacterSheet {
    let path = match character_sheet_path() {
        Some(p) => p,
        None => {
            warn!("Could not determine home directory, using default character sheet");
            return CharacterSheet::default();
        }
    };

    if !path.exists() {
        info!("No character sheet found at {:?}, creating default", path);
        return CharacterSheet::default();
    }

    match std::fs::read_to_string(&path) {
        Ok(json) => match serde_json::from_str::<CharacterSheet>(&json) {
            Ok(sheet) => {
                info!("✅ Loaded character sheet for: {}", sheet.alias);
                sheet
            }
            Err(e) => {
                warn!(
                    "Failed to parse character sheet at {:?}: {}. Using default.",
                    path, e
                );
                CharacterSheet::default()
            }
        },
        Err(e) => {
            warn!(
                "Failed to read character sheet at {:?}: {}. Using default.",
                path, e
            );
            CharacterSheet::default()
        }
    }
}

/// Save character sheet to disk
pub fn save_character_sheet(sheet: &CharacterSheet) -> Result<(), String> {
    let path = match character_sheet_path() {
        Some(p) => p,
        None => return Err("Could not determine home directory".to_string()),
    };

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return Err(format!("Failed to create directory {:?}: {}", parent, e));
        }
    }

    let json = serde_json::to_string_pretty(sheet)
        .map_err(|e| format!("Failed to serialize character sheet: {}", e))?;

    std::fs::write(&path, json)
        .map_err(|e| format!("Failed to write character sheet to {:?}: {}", path, e))?;

    info!("💾 Saved character sheet for: {}", sheet.alias);
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Bestiary Persistence — same pattern as CharacterSheet
// ═══════════════════════════════════════════════════════════════════════════════

use trinity_iron_road::game_loop::CreepBestiary;

fn bestiary_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| {
        PathBuf::from(&home)
            .join(".local")
            .join("share")
            .join("trinity")
            .join("bestiary.json")
    })
}

/// Load bestiary from disk, or return empty if not found
pub fn load_bestiary() -> CreepBestiary {
    let path = match bestiary_path() {
        Some(p) => p,
        None => {
            warn!("Could not determine home directory, using empty bestiary");
            return CreepBestiary::new();
        }
    };

    if !path.exists() {
        info!("No bestiary found at {:?}, starting fresh", path);
        return CreepBestiary::new();
    }

    match std::fs::read_to_string(&path) {
        Ok(json) => match trinity_iron_road::game_loop::load_bestiary_json(&json) {
            Ok(bestiary) => {
                info!(
                    "✅ Loaded bestiary: {} creeps ({} tamed)",
                    bestiary.creeps.len(),
                    bestiary.creeps_tamed
                );
                bestiary
            }
            Err(e) => {
                warn!(
                    "Failed to parse bestiary at {:?}: {}. Starting fresh.",
                    path, e
                );
                CreepBestiary::new()
            }
        },
        Err(e) => {
            warn!(
                "Failed to read bestiary at {:?}: {}. Starting fresh.",
                path, e
            );
            CreepBestiary::new()
        }
    }
}

/// Save bestiary to disk
pub fn save_bestiary(bestiary: &CreepBestiary) -> Result<(), String> {
    let path = match bestiary_path() {
        Some(p) => p,
        None => return Err("Could not determine home directory".to_string()),
    };

    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return Err(format!("Failed to create directory {:?}: {}", parent, e));
        }
    }

    let profile = trinity_protocol::VaamProfile::new();
    let json = trinity_iron_road::game_loop::save_state_json(bestiary, &profile)
        .map_err(|e| format!("Failed to serialize bestiary: {}", e))?;

    std::fs::write(&path, json)
        .map_err(|e| format!("Failed to write bestiary to {:?}: {}", path, e))?;

    info!("💾 Saved bestiary: {} creeps", bestiary.creeps.len());
    Ok(())
}
