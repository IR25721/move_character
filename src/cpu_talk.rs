use bevy::input::{ButtonInput, keyboard::KeyCode};
use bevy::prelude::*;

use crate::character::Cpu;

#[derive(Component)]
pub struct Hukidashi;

#[derive(Resource, Default)]
pub struct HukidashiToggleState {
    cooldown: Timer,
}

pub fn toggle_hukidashi(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    assets: Res<AssetServer>,
    query: Query<Entity, With<Hukidashi>>,
    mut state: ResMut<HukidashiToggleState>,
    time: Res<Time>,
) {
    state.cooldown.tick(time.delta());

    if input.just_pressed(KeyCode::Enter) && state.cooldown.finished() {
        if let Some(entity) = query.iter().next() {
            commands.entity(entity).despawn_recursive();
        } else {
            commands
                .spawn((
                    ImageNode::new(assets.load("hukidashi.png")),
                    Node {
                        width: Val::Px(200.),
                        height: Val::Px(100.),
                        ..default()
                    },
                    Hukidashi,
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new("hello Bevy !"),
                        TextFont {
                            font_size: 35.,
                            ..default()
                        },
                        TextColor(Color::srgb(0., 0., 0.)),
                    ));
                });
        }
        state.cooldown = Timer::from_seconds(0.1, TimerMode::Once);
    }
}
