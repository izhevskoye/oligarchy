mod car_instructions;
mod construction;
mod helper;
mod info;

use bevy::prelude::*;

pub fn ui_system() -> SystemSet {
    SystemSet::new()
        .with_system(info::info_ui.system().before("post_ui").label("info_ui"))
        .with_system(car_instructions::program_ui.system().after("info_ui"))
        .with_system(construction::construction_ui.system().before("post_ui"))
        .with_system(helper::mouse_pos_to_tile.system().label("post_ui"))
}
