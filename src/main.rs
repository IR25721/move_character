mod animation;
mod camera;
mod character;
mod cpu_talk;
mod dialog;
mod field;
mod menu;
use animation::animate_sprite;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ui_anchor::AnchorUiPlugin;
use camera::{CameraMarker, move_camera_with_player, setup_camera};
use character::{
    PlayerCollisionState, Walking, animate_player, handle_keyboard_input,
    handle_player_collision_end, handle_player_collision_events, keep_entity_upright,
    setup_character,
};
use cpu_talk::{FlowingTextTimer, HukidashiToggleState, TalkingState, toggle_hukidashi};
use dialog::flowing_text;
use field::{MyCustomAvianPhysicsBackend, startup};
use menu::{
    ButtonAnimation, SelectedButton, animate_button_position, moveup_button_input, setup_menu,
    setup_selected_button, trigger_button_action, update_button_outline, update_selected_button,
};
fn main() {
    App::new()
        .add_event::<CollisionStarted>()
        .add_event::<CollisionEnded>()
        .insert_resource(PlayerCollisionState::default())
        .insert_resource(HukidashiToggleState::default())
        .insert_resource(FlowingTextTimer::default())
        .insert_resource(TalkingState::default())
        .insert_resource(ButtonAnimation::default())
        .insert_resource(SelectedButton::default())
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
        .add_plugins(AnchorUiPlugin::<CameraMarker>::new())
        .add_systems(
            Startup,
            (
                setup_character,
                setup_camera,
                startup,
                setup_menu,
                setup_selected_button,
            ),
        )
        .add_event::<Walking>()
        .add_systems(
            Update,
            (
                handle_player_collision_events,
                handle_player_collision_end,
                animate_player,
                handle_keyboard_input,
                move_camera_with_player,
                animate_sprite,
                keep_entity_upright,
                toggle_hukidashi,
                flowing_text,
                moveup_button_input,
                animate_button_position,
                update_selected_button,
                trigger_button_action,
                update_button_outline,
            )
                .chain(),
        )
        .run();
}
