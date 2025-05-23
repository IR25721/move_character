use std::collections::HashMap;

use crate::animation::*;
use crate::cpu_talk::TalkingState;
use avian2d::collision::*;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum NpcId {
    Cpu01,
    Woman2,
}
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Npc;

#[derive(Resource, Default)]
pub struct PlayerCollisionState {
    pub is_colliding: HashMap<NpcId, bool>,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct Walking {
    pub first: usize,
    pub last: usize,
}
pub fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle0: Handle<Image> = asset_server.load("woman_walking.png");
    let texture_atlas_layout0 = TextureAtlasLayout::from_grid(UVec2::new(20, 28), 6, 4, None, None);
    let texture_atlas_layout_handle0 = texture_atlases.add(texture_atlas_layout0);

    let texture_handle3: Handle<Image> = asset_server.load("fellow1.png");
    let texture_atlas_layout3 = TextureAtlasLayout::from_grid(UVec2::new(20, 28), 6, 4, None, None);
    let texture_atlas_layout_handle3 = texture_atlases.add(texture_atlas_layout3);

    let texture_handle1: Handle<Image> = asset_server.load("cpu01.png");
    let texture_atlas_layout1 = TextureAtlasLayout::from_grid(UVec2::new(20, 28), 6, 4, None, None);
    let texture_atlas_layout_handle1 = texture_atlases.add(texture_atlas_layout1);
    commands.spawn((
        Player,
        Sprite::from_atlas_image(
            texture_handle0,
            TextureAtlas {
                layout: texture_atlas_layout_handle0,
                index: 0,
            },
        ),
        Transform {
            translation: Vec3::new(0., 0., -50.),
            scale: Vec3::splat(5.),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(10., 20.),
        LockedAxes::ROTATION_LOCKED,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
    commands.spawn((
        Npc,
        NpcId::Cpu01,
        Sprite::from_atlas_image(
            texture_handle1,
            TextureAtlas {
                layout: texture_atlas_layout_handle1,
                index: 0,
            },
        ),
        Transform {
            translation: Vec3::new(40., -40., -50.),
            scale: Vec3::splat(5.),
            ..Default::default()
        },
        RigidBody::Static,
        Collider::rectangle(10., 20.),
        LockedAxes::ROTATION_LOCKED,
    ));
    let texture_handle2: Handle<Image> = asset_server.load("woman2.png");
    let texture_atlas_layout2 = TextureAtlasLayout::from_grid(UVec2::new(20, 28), 6, 4, None, None);
    let texture_atlas_layout_handle2 = texture_atlases.add(texture_atlas_layout2);
    commands.spawn((
        Npc,
        NpcId::Woman2,
        Sprite::from_atlas_image(
            texture_handle2,
            TextureAtlas {
                layout: texture_atlas_layout_handle2,
                index: 0,
            },
        ),
        Transform {
            translation: Vec3::new(0., 200., -50.),
            scale: Vec3::splat(5.),
            ..Default::default()
        },
        RigidBody::Static,
        Collider::rectangle(10., 20.),
        LockedAxes::ROTATION_LOCKED,
    ));
}

pub fn keep_entity_upright(mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut query {
        transform.rotation = Quat::IDENTITY;
    }
}
pub fn animate_player(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut write: EventWriter<Walking>,
    talkingstate: Res<TalkingState>,
) {
    if talkingstate.is_talking {
    } else {
        let mut walking_animation = None;

        let w = kb_input.pressed(KeyCode::KeyW);
        let a = kb_input.pressed(KeyCode::KeyA);
        let s = kb_input.pressed(KeyCode::KeyS);
        let d = kb_input.pressed(KeyCode::KeyD);
        match (w, a, s, d) {
            (true, false, false, false) => {
                walking_animation = Some((18, 20));
            }
            (false, true, false, false) => {
                walking_animation = Some((6, 8));
            }
            (false, false, true, false) => {
                walking_animation = Some((0, 2));
            }
            (false, false, false, true) => {
                walking_animation = Some((12, 14));
            }
            (true, true, false, false) => {
                walking_animation = Some((15, 17));
            }
            (true, false, false, true) => {
                walking_animation = Some((21, 23));
            }
            (false, true, true, false) => {
                walking_animation = Some((3, 5));
            }
            (false, false, true, true) => {
                walking_animation = Some((9, 11));
            }
            _ => {}
        }
        if let Some((first, last)) = walking_animation {
            write.send(Walking { first, last });
        }
    }
}
pub fn handle_keyboard_input(
    mut query: Query<&mut LinearVelocity, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    talkingstate: Res<TalkingState>,
) {
    let speed = if talkingstate.is_talking { 0. } else { 200. };
    for mut linear_velocity in query.iter_mut() {
        linear_velocity.x = if input.pressed(KeyCode::KeyD) {
            speed
        } else if input.pressed(KeyCode::KeyA) {
            -speed
        } else {
            0.
        };

        linear_velocity.y = if input.pressed(KeyCode::KeyW) {
            speed
        } else if input.pressed(KeyCode::KeyS) {
            -speed
        } else {
            0.
        };
    }
}
pub fn handle_player_collision_events(
    mut events: EventReader<CollisionStarted>,
    mut state: ResMut<PlayerCollisionState>,
    player_query: Query<Entity, With<Player>>,
    npc_query: Query<(Entity, &NpcId), With<Npc>>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    for event in events.read() {
        for (npc_entity, npc_id) in npc_query.iter() {
            if (event.0 == player_entity && event.1 == npc_entity)
                || (event.1 == player_entity && npc_entity == event.0)
            {
                state.is_colliding.insert(*npc_id, true);
                println!("Player collided with {:?}", npc_id);
            }
        }
    }
}
pub fn handle_player_collision_end(
    mut events: EventReader<CollisionEnded>,
    mut state: ResMut<PlayerCollisionState>,
    player_query: Query<Entity, With<Player>>,
    npc_query: Query<(Entity, &NpcId), With<Npc>>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    for event in events.read() {
        for (npc_entity, npc_id) in npc_query.iter() {
            if (event.0 == player_entity && event.1 == npc_entity)
                || (event.1 == player_entity && npc_entity == event.0)
            {
                state.is_colliding.insert(*npc_id, false);
                println!("Player no longer colliding with {:?}", npc_id);
            }
        }
    }
}
