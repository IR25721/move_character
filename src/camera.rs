use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
pub fn move_camera_with_player(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };
    let Ok(player) = player.get_sigle_mut() else {
        return;
    };

    Vec3 { x, y, .. } = player.transform;
    let direction = Vec3::new(x, y, camera.translation.z);
    camera
        .translation
        .smooth_nudge(&direction, 2., time.delta_secs());
}
