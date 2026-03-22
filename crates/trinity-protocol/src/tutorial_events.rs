//! Tutorial Events - Shared events for tutorial workflow
//!
//! These events connect the tutorial system to the ADDIE workflow pipeline.

#[cfg(feature = "bevy")]
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Trait for message serialization
pub trait Message: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de> + 'static {}

impl<T> Message for T where T: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de> + 'static {}

/// Event fired when user submits their first topic
#[cfg_attr(feature = "bevy", derive(Event, bevy::prelude::Message))]
#[derive(Clone, Serialize, Deserialize)]
pub struct TopicSubmitted {
    pub topic: String,
    pub user_class: String,
    pub user_alias: String,
}

/// Event fired when blueprint is generated
#[cfg_attr(feature = "bevy", derive(Event, bevy::prelude::Message))]
#[derive(Clone, Serialize, Deserialize)]
pub struct BlueprintGenerated {
    pub blueprint: GameBlueprint,
}

/// Event fired when assets are generated
#[cfg_attr(feature = "bevy", derive(Event, bevy::prelude::Message))]
#[derive(Clone, Serialize, Deserialize)]
pub struct AssetsGenerated {
    pub sprites: Vec<String>,
    pub music: Vec<String>,
}

/// Event fired when game code is generated
#[cfg_attr(feature = "bevy", derive(Event, bevy::prelude::Message))]
#[derive(Clone, Serialize, Deserialize)]
pub struct GameCodeGenerated {
    pub code_files: Vec<CodeFile>,
}

/// Event fired when game is built and ready
#[cfg_attr(feature = "bevy", derive(Event, bevy::prelude::Message))]
#[derive(Clone, Serialize, Deserialize)]
pub struct GameReady {
    pub executable_path: String,
}

/// Game blueprint from ADDIE Analysis phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBlueprint {
    pub title: String,
    pub topic: String,
    pub learning_objectives: Vec<String>,
    pub game_mechanics: Vec<String>,
    pub required_assets: Vec<AssetRequirement>,
    pub music_themes: Vec<MusicTheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRequirement {
    pub name: String,
    pub description: String,
    pub asset_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicTheme {
    pub name: String,
    pub mood: String,
    pub duration_secs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFile {
    pub path: String,
    pub content: String,
    pub language: String,
}
