use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MenuSelectStates {
    pub is_main_select: bool,
    pub is_status_select: bool,
    pub is_setting_select: bool,
    pub is_main_items_select: bool,
    pub is_main_spells_select: bool,
    pub is_main_map_select: bool,
    pub is_main_save_select: bool,
}
