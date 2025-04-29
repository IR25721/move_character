use crate::animation::*;
use avian2d::collision::*;
use avian2d::prelude::*;
use bevy::prelude::*;
#[derive(Component)]
pub struct Player;
#[derive(Resource, Default)]
pub struct PlayerCollisionState {
    pub is_colliding: bool,
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
    let texture_handle: Handle<Image> = asset_server.load("woman_walking.png");
    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(20, 28), 6, 4, None, None);
    let texture_atlas_layout_handle = texture_atlases.add(texture_atlas_layout);
    commands.spawn((
        Player,
        Sprite::from_atlas_image(
            texture_handle,
            TextureAtlas {
                layout: texture_atlas_layout_handle,
                index: 0,
            },
        ),
        Transform {
            translation: Vec3::new(0., 0., -50.),
            scale: Vec3::splat(3.),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(10., 20.),
        LockedAxes::ROTATION_LOCKED,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
}

pub fn keep_entity_upright(mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut query {
        transform.rotation = Quat::IDENTITY;
    }
}
pub fn animate_player(
    mut player: Query<&mut LinearVelocity, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut write: EventWriter<Walking>,
    state: Res<PlayerCollisionState>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

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
pub fn handle_keyboard_input(
    mut query: Query<&mut LinearVelocity, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let speed = 500.;
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
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    for event in events.read() {
        if event.0 == player_entity || event.1 == player_entity {
            state.is_colliding = true;
            println!("Player collided!");
        }
    }
}

pub fn handle_player_collision_end(
    mut events: EventReader<CollisionEnded>,
    mut state: ResMut<PlayerCollisionState>,
    player_query: Query<Entity, With<Player>>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    for event in events.read() {
        if event.0 == player_entity || event.1 == player_entity {
            state.is_colliding = false;
            println!("Player no longer colliding");
        }
    }
}
