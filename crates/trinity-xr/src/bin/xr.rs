// bin/xr.rs — Trinity XR Native OpenXR Entry Point
// Run with: cargo run -p trinity-xr --bin trinity-xr --features xr
//
// Target device: XREAL Aura (Android XR, optical see-through)
// Also works with Monado runtime on Linux.

use bevy::prelude::*;
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
use bevy_mod_openxr::{add_xr_plugins, resources::OxrSessionConfig, types::EnvironmentBlendMode};
use trinity_xr::{TrinityXrPlugin, XrShellPlugin};

fn main() {
    tracing_subscriber::fmt().init();

    info!("Trinity XR — Launching for XREAL Aura (optical see-through)");

    App::new()
        .add_plugins(add_xr_plugins(
            DefaultPlugins.build().disable::<PipelinedRenderingPlugin>(),
        ))
        .insert_resource(OxrSessionConfig {
            blend_mode_preference: vec![
                EnvironmentBlendMode::ALPHA_BLEND,
            ],
            ..default()
        })
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(TrinityXrPlugin)
        .add_plugins(XrShellPlugin)
        .run();
}
