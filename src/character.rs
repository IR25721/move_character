use bevy::prelude::*;

#[derive(Component)]
struct Player;
fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("woman_walking.png");
    let texture_atlas_layout = TextureAtlasLayout::from_grid(Vec2::new(), columns, rows, padding, offset)
    commands.spawn((Player));
}
