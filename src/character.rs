use bevy::prelude::*;

#[derive(Component)]
struct Player;
pub fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("woman_walking.png");
    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(24, 24), 6, 4, None, None);
    let texture_atlas_layout_handle = texture_atlases.add(texture_atlas_layout);
    commands.spawn(
        (Sprite::from_atlas_image(
            texture_handle,
            TextureAtlas {
                layout: texture_atlas_layout_handle,
                index: 0,
            },
        )),
    );
}
