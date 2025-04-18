use crate::animation::*;
use avian2d::prelude::*;
use bevy::prelude::*;
#[derive(Component)]
pub struct Player;

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
            scale: Vec3::splat(6.),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::circle(50.),
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
}
pub fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut write: EventWriter<Walking>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };
    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
        write.send(Walking {
            first: 18,
            last: 20,
        });
    }
    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
        write.send(Walking { first: 0, last: 2 });
    }
    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
        write.send(Walking { first: 6, last: 8 });
    }
    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
        write.send(Walking {
            first: 12,
            last: 14,
        });
    }
    let player_speed = if kb_input.pressed(KeyCode::ShiftLeft) {
        1000.
    } else {
        500.
    };
    let move_delta = direction.normalize_or_zero() * player_speed * time.delta_secs();
    player.translation += move_delta.extend(0.);
}
