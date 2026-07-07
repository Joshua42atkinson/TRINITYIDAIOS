// bin/desktop.rs — Trinity XR Desktop Emulator
// Run with: cargo run -p trinity-xr --bin trinity-xr-desktop --features desktop
// Provides a mouse-controlled 3D preview without requiring an XR headset.

use bevy::prelude::*;
#[cfg(feature = "desktop")]
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use trinity_xr::TrinityXrPlugin;

fn main() {
    tracing_subscriber::fmt().init();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Trinity XR — Desktop Emulator".to_string(),
            resolution: (1280, 800).into(),
            ..default()
        }),
        ..default()
    }));

    #[cfg(feature = "desktop")]
    {
        app.add_plugins(PanOrbitCameraPlugin);
    }

    app.add_plugins(TrinityXrPlugin);
    app.add_systems(Startup, setup_desktop_camera);
    app.run();
}

#[cfg(feature = "desktop")]
fn setup_desktop_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
        Transform::from_xyz(0.0, 1.2, 2.5).looking_at(Vec3::new(0.0, 0.9, 0.0), Vec3::Y),
        PanOrbitCamera {
            focus: Vec3::new(0.0, 0.9, 0.0),
            radius: Some(2.5),
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Middle,
            ..default()
        },
    ));
}

#[cfg(not(feature = "desktop"))]
fn setup_desktop_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
        Transform::from_xyz(0.0, 1.5, 4.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
}
