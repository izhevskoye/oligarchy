mod assets;
mod blast_furnace;
mod camera;
mod car;
mod coke_furnace;
mod constants;
mod construction_ui;
mod current_selection;
mod current_tool;
mod export_station;
mod helper;
mod info_ui;
mod oxygen_converter;
mod quarry;
mod setup;
mod storage;
mod street;
mod texture;

use bevy::{core::FixedTimestep, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;

use self::assets::{ClickedTile, CurrentlySelected, SelectedTool, Tool};

#[derive(Default)]
pub struct Game {}

impl Game {
    pub fn run(&self) {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();

        App::build()
            .insert_resource(CurrentlySelected { entity: None })
            .insert_resource(SelectedTool { tool: Tool::None })
            .init_resource::<ClickedTile>()
            .add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(TilemapPlugin)
            .add_startup_system(setup::setup_map.system())
            .add_system(camera::movement.system())
            .add_system(texture::set_texture_filters_to_nearest.system())
            .add_system(construction_ui::construction_ui.system())
            .add_system(info_ui::info_ui.system())
            .add_system(helper::mouse_pos_to_tile.system())
            .add_system(storage::update_consolidators.system())
            .add_system(street::update_streets.system())
            .add_system(car::calculate_destination.system())
            .add_system(current_selection::current_selection.system())
            .add_system(current_tool::street_placement.system())
            .add_system(current_tool::storage_placement.system())
            .add_system(current_tool::quarry_placement.system())
            .add_system(current_tool::coke_furnace_placement.system())
            .add_system(current_tool::blast_furnace_placement.system())
            .add_system(current_tool::oxygen_converter_placement.system())
            .add_system(current_tool::export_station_placement.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(export_station::export_station.system())
                    .with_system(oxygen_converter::oxygen_converter.system())
                    .with_system(blast_furnace::blast_furnace.system())
                    .with_system(quarry::quarry.system())
                    .with_system(coke_furnace::coke_furnace.system())
                    .with_system(car::car_instruction.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.2))
                    .with_system(car::drive_to_destination.system()),
            )
            .run();
    }
}
