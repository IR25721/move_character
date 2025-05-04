use crate::character::{Npc, NpcId, PlayerCollisionState};
use crate::dialog::FlowingText;
use bevy::input::{ButtonInput, keyboard::KeyCode};
use bevy::prelude::*;
use bevy_ui_anchor::{AnchorTarget, AnchorUiNode, HorizontalAnchor, VerticalAnchor};

#[derive(Component)]
pub struct Hukidashi;
#[derive(Resource, Default)]
pub struct FlowingTextTimer(pub Timer);

#[derive(Resource, Default)]
pub struct HukidashiToggleState {
    cooldown: Timer,
}
#[derive(Resource, Default)]
pub struct TalkingState {
    pub is_talking: bool,
}

pub fn toggle_hukidashi(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    assets: Res<AssetServer>,
    query_npc: Query<(Entity, &NpcId), With<Npc>>,
    query: Query<Entity, With<Hukidashi>>,
    mut state: ResMut<HukidashiToggleState>,
    collider: Res<PlayerCollisionState>,
    mut talkingstate: ResMut<TalkingState>,
    time: Res<Time>,
    timer: Res<FlowingTextTimer>,
) {
    state.cooldown.tick(time.delta());
    for (npc_entity, id) in query_npc.iter() {
        if *id == NpcId::Cpu01 {
            if collider.is_colliding.get(id).copied().unwrap_or(false) {
                if input.just_pressed(KeyCode::Enter) && state.cooldown.finished() {
                    talkingstate.is_talking = false;
                    if let Some(entity) = query.iter().next() {
                        commands.entity(entity).despawn_recursive();
                    } else {
                        talkingstate.is_talking = true;
                        commands
                    .spawn((
                        ImageNode::new(assets.load("hukidashi.png")),
                        Node {
                            width: Val::Px(800.),
                            height: Val::Px(300.),
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
                            Text::new(""),
                            TextFont {
                                font: assets.load("fonts/hutoi.ttf"),
                                font_size: 25.,
                                ..default()
                            },
                            Node {
                                position_type: PositionType::Relative,
                                bottom: Val::Px(-40.0),
                                right: Val::Px(-60.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0., 0., 0.)),
                            FlowingText {
                                content: "こんにちは．これはテスト文章です．もし何か問題が\nありましたら連絡してください．".to_owned(),
                                index: 0,
                                timer: timer.0.clone(),
                            },
                        ));
                    });
                    }
                    state.cooldown = Timer::from_seconds(0.1, TimerMode::Once);
                }
            }
        } else if *id == NpcId::Woman2 {
            if collider.is_colliding.get(id).copied().unwrap_or(false) {
                if input.just_pressed(KeyCode::Enter) && state.cooldown.finished() {
                    talkingstate.is_talking = false;
                    if let Some(entity) = query.iter().next() {
                        commands.entity(entity).despawn_recursive();
                    } else {
                        talkingstate.is_talking = true;
                        commands
                    .spawn((
                        ImageNode::new(assets.load("hukidashi.png")),
                        Node {
                            width: Val::Px(800.),
                            height: Val::Px(300.),
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
                            Text::new(""),
                            TextFont {
                                font: assets.load("fonts/hutoi.ttf"),
                                font_size: 25.,
                                ..default()
                            },
                            Node {
                                position_type: PositionType::Relative,
                                bottom: Val::Px(-40.0),
                                right: Val::Px(-60.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0., 0., 0.)),
                            FlowingText {
                                content: "こんにちは．これはテスト文章です．もし何か問題が\nありましたら連絡してください．\nこっちが表示されたら，NPCの識別問題は\n解決したということになります．".to_owned(),
                                index: 0,
                                timer: timer.0.clone(),
                            },
                        ));
                    });
                    }
                    state.cooldown = Timer::from_seconds(0.1, TimerMode::Once);
                }
            } else {
                return;
            }
        }
    }
}
