use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::menu::{ButtonId, SelectedButton, ShowDetail};
#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component, PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum MainSelect {
    Items,
    Spells,
    Map,
    Save,
}
pub fn toggle_main_menu(
    mut commands: Commands,
    kb_input: Res<ButtonInput<KeyCode>>,
    assets: Res<AssetServer>,
    mut reader: EventReader<ShowDetail>,
    menu_query: Query<Entity, With<MainMenuRoot>>,
    selected: Res<SelectedButton>,
) {
    for _ in reader.read() {
        if selected.0 == ButtonId::Main && kb_input.just_pressed(KeyCode::KeyF) {
            if let Ok(entity) = menu_query.get_single() {
                commands.entity(entity).despawn_recursive();
            } else {
                commands
                    .spawn((
                        MainMenuRoot,
                        Node {
                            display: Display::Grid,
                            position_type: PositionType::Absolute,
                            top: Val::Px(510.),
                            left: Val::Px(1200.),
                            width: Val::Px(700.),
                            height: Val::Px(450.),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        BorderRadius::all(Val::Percent(15.)),
                        BackgroundColor(WHITE.into()),
                        Outline {
                            width: Val::Px(5.0),
                            offset: Val::Px(0.0),
                            color: BLACK.into(),
                        },
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Text::new("hello main menu!"),
                            TextFont {
                                font: assets.load("fonts/hutoi.ttf"),
                                font_size: 45.,
                                ..default()
                            },
                            TextColor(Color::srgb(0., 0., 0.)),
                            Node {
                                position_type: PositionType::Relative,
                                top: Val::Px(15.0),
                                left: Val::Px(65.0),
                                ..default()
                            },
                        ));
                    });
            }
        }
    }
}
