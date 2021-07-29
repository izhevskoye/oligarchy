mod asset_tiles;
mod assets;
mod building_specifications;
mod camera;
mod car;
mod constants;
mod current_selection;
mod current_tool;
mod production;
mod remove_update;
mod resource_specifications;
mod setup;
mod state_manager;
mod storage;
mod street;
mod texture;
mod ui;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;

use crate::game::{
    assets::{ClickedTile, SelectedTool},
    current_selection::CurrentlySelected,
};

#[derive(Default)]
pub struct Game {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    Update,
    UpdateEnd,
    CurrentSelection,
}

impl Game {
    pub fn run(&self) {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();

        App::build()
            .init_resource::<CurrentlySelected>()
            .init_resource::<SelectedTool>()
            .init_resource::<ClickedTile>()
            .insert_resource(building_specifications::load_specifications())
            .insert_resource(resource_specifications::load_specifications())
            .add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(TilemapPlugin)
            .add_startup_system(setup::setup_map.system())
            .add_system(camera::movement.system())
            .add_system(texture::set_texture_filters_to_nearest.system())
            .add_system_set(ui::ui_system())
            .add_system(
                current_selection::current_selection
                    .system()
                    .label(Label::CurrentSelection),
            )
            .add_system_set(
                current_tool::current_tool_system()
                    .after(Label::CurrentSelection)
                    .before(Label::Update),
            )
            .add_system_set(production::production_system())
            .add_system(car::calculate_destination.system().before(Label::UpdateEnd))
            .add_system_set(car::instruction_system())
            .add_system_set(car::drive_system().before(Label::Update))
            .add_system(state_manager::save_ui.system().before(Label::Update))
            .add_system_set(
                SystemSet::new()
                    .label(Label::Update)
                    .before(Label::UpdateEnd)
                    .with_system(asset_tiles::building_update.system())
                    .with_system(asset_tiles::storage_update.system())
                    .with_system(asset_tiles::export_station_update.system())
                    .with_system(asset_tiles::ground_update.system())
                    .with_system(street::update_streets.system())
                    .with_system(storage::update_consolidators.system())
                    .with_system(car::update_car.system()),
            )
            .add_system(
                remove_update::remove_update
                    .system()
                    .label(Label::UpdateEnd),
            )
            .run();
    }
}
