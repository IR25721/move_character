use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::{
    menu::{ButtonId, SelectedButton, ShowDetail},
    menu_windows::MenuSelectStates,
};
const MAIN_MENU_ITEMS: [MainSelect; 4] = [
    MainSelect::Items,
    MainSelect::Spells,
    MainSelect::Map,
    MainSelect::Save,
];

#[derive(Resource, Default)]
pub struct MainMenuCursor {
    pub index: usize,
}

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component, PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum MainSelect {
    SelectButton,
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
    mut select_main_menu: ResMut<MenuSelectStates>,
    selected: Res<SelectedButton>,
) {
    for _ in reader.read() {
        if selected.0 == ButtonId::Main && kb_input.just_pressed(KeyCode::KeyF) {
            if !select_main_menu.is_main_select {
                let root_entity = commands
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
                    .id();
                commands.spawn((
                    MainMenuRoot,
                    MainSelect::SelectButton,
                    Node {
                        top: Val::Px(550.0),
                        left: Val::Px(1230.),
                        width: Val::Px(45.),
                        height: Val::Px(45.),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BorderRadius::all(Val::Percent(50.)),
                    BackgroundColor(BLACK.into()),
                    ZIndex(1),
                ));
                commands.entity(root_entity).with_children(|p| {
                    for (label, variant) in [
                        ("Items", MainSelect::Items),
                        ("Spells", MainSelect::Spells),
                        ("Map", MainSelect::Map),
                        ("Save", MainSelect::Save),
                    ] {
                        p.spawn((
                            variant,
                            Text::new(label),
                            TextFont {
                                font: assets.load("fonts/hutoi.ttf"),
                                font_size: 45.,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                            Node {
                                position_type: PositionType::Relative,
                                top: Val::Px(35.0),
                                left: Val::Px(85.0),
                                ..default()
                            },
                        ));
                    }
                });

                select_main_menu.is_main_select = true;
            } else {
                for entity in menu_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                select_main_menu.is_main_select = false;
            }
        }
    }
}

pub fn navigate_main_menu(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut cursor: ResMut<MainMenuCursor>,
    mut query: Query<(&MainSelect, &mut Node)>,
) {
    let mut moved = false;
    if kb_input.just_pressed(KeyCode::ArrowDown) && cursor.index + 1 < MAIN_MENU_ITEMS.len() {
        cursor.index += 1;
        moved = true;
    }
    if kb_input.just_pressed(KeyCode::ArrowUp) && cursor.index > 0 {
        cursor.index -= 1;
        moved = true;
    }

    if moved {
        let offset_y = 35.0 + cursor.index as f32 * 110.0;

        for (select, mut style) in query.iter_mut() {
            if *select == MainSelect::SelectButton {
                style.top = Val::Px(510.0 + offset_y);
            }
        }
    }
    if kb_input.just_pressed(KeyCode::KeyE) {
        if let Some(selected) = MAIN_MENU_ITEMS.get(cursor.index) {
            println!("選択された項目: {:?}", selected);
        }
    }
}
