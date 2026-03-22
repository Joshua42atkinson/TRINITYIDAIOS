```
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

fn spawn_professor_npc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("model.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        AudioPlayer::new(asset_server.load("theme.wav")),
        Dialogue {
            lines: vec![
                "Ah, my dear, you've arrived just in time to witness the marvel of my latest creation—the 'Chronovore,' a device capable of bending time itself through the power of compressed steam and clockwork precision.".to_string(),
                "I must admit, I find your lack of a mechanical augment rather... quaint. Though, I suppose even flesh-and-blood has its charms.".to_string(),
                "You wouldn’t believe the bureaucratic nightmare I endured to get this airship docked in the upper districts. The paperwork alone was enough to power a small village for a decade.".to_string(),
            ],
        },
        PersonalityTraits {
            traits: vec![
                "Brilliant but eccentric".to_string(),
                "Charismatic with a dry wit".to_string(),
                "Morally ambiguous but charming".to_string(),
                "Obsessed with technological progress".to_string(),
                "Secretly sentimental about old-world craftsmanship".to_string(),
            ],
        },
    ));
}
```