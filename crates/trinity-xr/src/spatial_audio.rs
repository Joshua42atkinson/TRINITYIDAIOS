// spatial_audio.rs — Spatial Audio for XR
// Plays audio cues at 3D positions in the lesson environment.
// In XR mode, the listener position is the XR camera.

use bevy::prelude::*;
use bevy::audio::SpatialListener;

pub struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_spatial_listener);
    }
}

#[derive(Component)]
pub struct SpatialListenerMarker;

fn setup_spatial_listener(mut commands: Commands) {
    commands.spawn((
        SpatialListener::new(0.4),
        SpatialListenerMarker,
        Transform::from_xyz(0.0, 1.5, 4.0),
    ));
}
