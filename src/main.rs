mod animation;
mod camera;
mod character;
mod field;
use animation::animate_sprite;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use camera::{move_camera_with_player, setup_camera};
use character::{
    PlayerCollisionState, Walking, handle_player_collision_end, handle_player_collision_events,
    keep_entity_upright, move_player, setup_character,
};
use field::{MyCustomAvianPhysicsBackend, startup};
fn main() {
    App::new()
        .add_event::<CollisionStarted>()
        .add_event::<CollisionEnded>()
        .insert_resource(PlayerCollisionState::default())
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Tiled Map Editor Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().with_length_unit(100.),
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(Gravity::ZERO)
        .add_plugins(TiledMapPlugin::default())
        .add_plugins(TiledPhysicsPlugin::<MyCustomAvianPhysicsBackend>::default())
        .add_systems(Startup, (setup_character, setup_camera, startup))
        .add_event::<Walking>()
        .add_systems(
            Update,
            (
                handle_player_collision_events,
                handle_player_collision_end,
                move_player,
                move_camera_with_player,
                animate_sprite,
                keep_entity_upright,
            )
                .chain(),
        )
        .run();
}
