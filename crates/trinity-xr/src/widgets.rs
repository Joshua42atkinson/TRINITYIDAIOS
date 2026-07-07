// widgets.rs — Reusable Holographic UI Widgets
// Glassmorphism panels and holographic buttons with hover/press states.
// Trinity color palette: blue-cyan accents on dark glass.

use bevy::prelude::*;

pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, _app: &mut App) {}
}

pub const COLOR_GLASS_PANEL: Color = Color::srgba(0.03, 0.05, 0.12, 0.85);
pub const COLOR_GLASS_BORDER: Color = Color::srgba(0.2, 0.5, 0.9, 0.5);
pub const COLOR_BTN_NORMAL: Color = Color::srgba(0.08, 0.12, 0.2, 0.7);
pub const COLOR_BTN_HOVER: Color = Color::srgba(0.1, 0.2, 0.35, 0.9);
pub const COLOR_BTN_PRESSED: Color = Color::srgba(0.2, 0.5, 0.9, 0.9);
pub const COLOR_TEXT_PRIMARY: Color = Color::srgb(0.9, 0.92, 0.95);
pub const COLOR_TEXT_HIGHLIGHT: Color = Color::srgb(0.3, 0.6, 1.0);

#[derive(Component)]
pub struct HolographicButton;

pub fn spawn_glass_panel(
    parent: &mut ChildSpawnerCommands,
    size: Vec2,
    padding: f32,
    f: impl FnOnce(&mut ChildSpawnerCommands),
) {
    parent
        .spawn((
            Node {
                width: Val::Px(size.x),
                height: Val::Px(size.y),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(padding)),
                overflow: Overflow::clip(),
                border_radius: BorderRadius::all(Val::Px(24.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(COLOR_GLASS_PANEL),
            BorderColor::all(COLOR_GLASS_BORDER),
        ))
        .with_children(f);
}

pub fn spawn_holographic_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    width: f32,
    height: f32,
    action: impl Component,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(COLOR_BTN_NORMAL),
            HolographicButton,
            action,
        ))
        .observe(
            |over: On<Pointer<Over>>, mut colors: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = colors.get_mut(over.entity) {
                    color.0 = COLOR_BTN_HOVER;
                }
            },
        )
        .observe(
            |out: On<Pointer<Out>>, mut colors: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = colors.get_mut(out.entity) {
                    color.0 = COLOR_BTN_NORMAL;
                }
            },
        )
        .observe(
            |down: On<Pointer<Press>>, mut colors: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = colors.get_mut(down.entity) {
                    color.0 = COLOR_BTN_PRESSED;
                }
            },
        )
        .observe(
            |up: On<Pointer<Release>>, mut colors: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = colors.get_mut(up.entity) {
                    color.0 = COLOR_BTN_HOVER;
                }
            },
        )
        .with_children(|btn| {
            btn.spawn((
                Text::new(label.to_string()),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(COLOR_TEXT_PRIMARY),
                Node::default(),
            ));
        });
}
