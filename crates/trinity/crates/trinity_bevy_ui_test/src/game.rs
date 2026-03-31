// Bevy UI Test Project — Basic UI scaffolding for demonstration
use bevy::prelude::*;

pub struct BevyUIStuffPlugin;
impl Plugin for BevyUIStuffPlugin {
    fn build(&self, app: &mut App) {
        // Main game state
        app.init_state::<GameState>()
            .add_systems(Startup, setup_ui)
            .add_systems(Update, handle_interaction.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
}

// UI Components
#[derive(Component)]
pub struct MainMenu;
#[derive(Component)]
pub struct SettingsPanel;
#[derive(Component)]
pub struct ButtonUI;
#[derive(Component)]
pub struct DialogBox;

// Setup the UI on startup
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a simple 2D camera for UI rendering
    commands.spawn(Camera2dBundle::default());

    // Main menu root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|parent| {
            // Main menu container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        ..default()
                },
                background_color: BackgroundColor(Color::DARK_GRAY),
                border_radius: BorderRadius::all(Val::Px(15.0)),
                ..default()
            )
                .insert(MainMenu)
                .with_children(|parent| {
                    // Title text
                    parent.spawn(TextBundle {
                        style: Style { width: Val::Auto, height: Val::Px(80.0), ..default() },
                        text: Text::from_section(
                            "Bevy UI Test",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE.into(),
                                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            }
                        ),
                        ..default()
                    });

                    // Settings button (interactive)
                    parent.spawn(ButtonBundle {
                        style: Style { width: Val::Px(250.0), height: Val::Px(60.0), margin: UiRect::all(Val::Px(10.0)), ..default()
                    })
                        .insert(SettingsPanel)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                TextStyle { font_size: 24.0, color: Color::WHITE.into(), font: asset_server.load("fonts/FiraSans-Regular.ttf") },
                            ));
                        });

                    // Back button (interactive)
                    parent.spawn(ButtonBundle {
                        style: Style { width: Val::Px(250.0), height: Val::Px(60.0), margin: UiRect::all(Val::Px(10.0)), ..default() }
                        .insert(ButtonUI)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Back",
                                TextStyle { font_size: 24.0, color: Color::WHITE.into(), font: asset_server.load("fonts/FiraSans-Regular.ttf") },
                            ));
                        });
                    
                    // Dialog box (non-interactive)
                    parent.spawn(NodeBundle {
                        style: Style { width: Val::Percent(80.0), height: Val::Auto, ..default() },
                        background_color: BackgroundColor(Color::DARK_GRAY),
                        border_radius: BorderRadius::all(Val::Px(15.0)),
                        ..default()
                    })
                        .insert(DialogBox)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Press 'Settings' to open the settings panel",
                                TextStyle { font_size: 16.0, color: Color::WHITE.into(), font: asset_server.load("fonts/FiraSans-Regular.ttf") },
                            ));
                        });
                });
        });
}

// Interaction system for the Bevy UI test
fn handle_interaction(interaction_query: Query<&Interaction, With<ButtonUI>>, mut state: ResMut<NextState<GameState>>) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            // State transition to "Settings" when button is pressed
            state.set(GameState::Playing);
        }
    }
}