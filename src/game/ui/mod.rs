mod car_instructions;
mod construction;
mod export_station;
mod info;
mod mouse_pos_to_tile;
mod name;

use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    InfoUI,
    UIEnd,
}

pub fn ui_system() -> SystemSet {
    SystemSet::new()
        .with_system(
            info::info_ui
                .system()
                .before(Label::UIEnd)
                .label(Label::InfoUI),
        )
        .with_system(export_station::edit_ui.system().after(Label::InfoUI))
        .with_system(car_instructions::program_ui.system().after(Label::InfoUI))
        .with_system(construction::construction_ui.system().before(Label::UIEnd))
        .with_system(name::name_ui.system().before(Label::UIEnd))
        .with_system(
            mouse_pos_to_tile::mouse_pos_to_tile
                .system()
                .label(Label::UIEnd),
        )
}
