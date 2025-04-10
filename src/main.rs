mod camera;
mod character;
use bevy::prelude::*;
use camera::*;
use character::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_character, setup_camera))
        .add_systems(Update, move_player)
        .run();
}
