mod camera;
mod character;
use bevy::prelude::*;
use camera::setup_camera;
use character::setup_character;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_character, setup_camera))
        .run();
}
