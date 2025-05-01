use crate::character::{Npc, PlayerCollisionState};
use bevy::input::{ButtonInput, keyboard::KeyCode};
use bevy::prelude::*;
use bevy_ui_anchor::{AnchorTarget, AnchorUiNode, HorizontalAnchor, VerticalAnchor};

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
    query_npc: Query<Entity, With<Npc>>,
    mut state: ResMut<HukidashiToggleState>,
    collider: Res<PlayerCollisionState>,
    time: Res<Time>,
) {
    state.cooldown.tick(time.delta());
    let npc_entity = query_npc.get_single().expect("not found");
    if collider.is_colliding {
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
                        AnchorUiNode {
                            target: AnchorTarget::Entity(npc_entity),
                            offset: None,
                            anchorwidth: HorizontalAnchor::Mid,
                            anchorheight: VerticalAnchor::Bottom,
                        },
                        Hukidashi,
                    ))
                    .with_children(|builder| {
                        builder.spawn((
                            Text::new("hello bevy!"),
                            TextFont {
                                font_size: 25.,
                                ..default()
                            },
                            Node {
                                position_type: PositionType::Relative,
                                bottom: Val::Px(-15.0),
                                right: Val::Px(-15.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0., 0., 0.)),
                        ));
                    });
            }
            state.cooldown = Timer::from_seconds(0.1, TimerMode::Once);
        }
    } else {
    }
}
