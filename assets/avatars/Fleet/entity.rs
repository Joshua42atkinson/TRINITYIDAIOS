```
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

fn spawn_fleet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let model_handle: Handle<Mesh> = asset_server.load("model.glb#Mesh0");
    let theme_handle: Handle<AudioSource> = asset_server.load("theme.wav");

    commands.spawn((
        Name::new("Fleet"),
        SceneBundle {
            scene: asset_server.load("model.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        AudioPlayer::new(theme_handle),
        Dialogue {
            lines: vec![
                "Greetings, traveler.".to_string(),
                "The road is long.".to_string(),
                "Stay sharp.".to_string(),
            ],
        },
        PersonalityTraits::default(),
    ));
}
```