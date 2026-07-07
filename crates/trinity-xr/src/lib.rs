// lib.rs — Trinity XR
// Spatial instructional design workspace (Bevy 0.18 OpenXR)
//
// Three-device architecture:
//   Phone PWA (chat portal) → Trinity :3000 (server) → trinity-xr (VR client)
//
// Active plugins:
//   XrShell, EnvironmentManager, HandTracking, SpatialAudio,
//   SpatialUi, Interaction, Widgets, SystemMenu,
//   ChatPanel, AssetViewer, Ipc (WebSocket to Trinity)

pub mod environment_manager;
pub mod hand_tracking;
pub mod interaction;
pub mod spatial_audio;
pub mod spatial_ui;
pub mod widgets;
pub mod xr_shell;

pub mod asset_viewer;
pub mod chat_panel;
pub mod ipc;
pub mod system_menu;

pub use environment_manager::EnvironmentManagerPlugin;
pub use hand_tracking::HandTrackingPlugin;
pub use interaction::InteractionPlugin;
pub use spatial_audio::SpatialAudioPlugin;
pub use spatial_ui::SpatialUiPlugin;
pub use xr_shell::XrShellPlugin;

pub use asset_viewer::AssetViewerPlugin;
pub use chat_panel::ChatPanelPlugin;
pub use ipc::IpcPlugin;
pub use system_menu::SystemMenuPlugin;

use bevy::prelude::*;

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum TrinityXrState {
    #[default]
    Idle,
    ChatMode,
    BuildMode,
    PreviewMode,
}

pub struct TrinityXrPlugin;

impl Plugin for TrinityXrPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TrinityXrState>()
            .add_plugins(EnvironmentManagerPlugin)
            .add_plugins(HandTrackingPlugin)
            .add_plugins(SpatialAudioPlugin)
            .add_plugins(SpatialUiPlugin)
            .add_plugins(InteractionPlugin)
            .add_plugins(XrShellPlugin)
            .add_plugins(SystemMenuPlugin)
            .add_plugins(ChatPanelPlugin)
            .add_plugins(AssetViewerPlugin)
            .add_plugins(IpcPlugin);
    }
}

// ── Android Entry Point (cdylib) ─────────────────────────────

#[cfg(target_os = "android")]
#[bevy_main]
fn main() {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    #[cfg(feature = "xr")]
    {
        use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
        use bevy_mod_openxr::{add_xr_plugins, resources::OxrSessionConfig, types::EnvironmentBlendMode};

        App::new()
            .add_plugins(add_xr_plugins(
                DefaultPlugins.build().disable::<PipelinedRenderingPlugin>(),
            ))
            .insert_resource(OxrSessionConfig {
                blend_mode_preference: vec![
                    EnvironmentBlendMode::ALPHA_BLEND,
                    EnvironmentBlendMode::OPAQUE,
                ],
                ..default()
            })
            .insert_resource(ClearColor(Color::NONE))
            .add_plugins(TrinityXrPlugin)
            .run();
    }

    #[cfg(not(feature = "xr"))]
    {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(TrinityXrPlugin)
            .run();
    }
}
