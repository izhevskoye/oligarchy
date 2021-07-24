mod asset_tiles;
mod assets;
mod camera;
mod car;
mod constants;
mod current_selection;
mod current_tool;
mod production;
mod setup;
mod state_manager;
mod storage;
mod street;
mod texture;
mod ui;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;

use self::{
    assets::{ClickedTile, CurrentlySelected, RequiresUpdate, SelectedTool},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

fn remove_update(
    mut commands: Commands,
    query: Query<(Entity, &RequiresUpdate)>,
    mut map_query: MapQuery,
) {
    for (entity, update) in query.iter() {
        commands.entity(entity).remove::<RequiresUpdate>();
        // TODO: is it always a building?
        map_query.notify_chunk_for_tile(update.position, MAP_ID, BUILDING_LAYER_ID);
    }
}

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
            .add_system_set(car::calculate_system())
            .add_system_set(car::instruction_system())
            .add_system_set(car::drive_system().before(Label::Update))
            .add_system(state_manager::save_ui.system().before(Label::Update))
            .add_system_set(
                SystemSet::new()
                    .label(Label::Update)
                    .before(Label::UpdateEnd)
                    .with_system(asset_tiles::quarry_update.system())
                    .with_system(asset_tiles::storage_update.system())
                    .with_system(asset_tiles::coke_furnace_update.system())
                    .with_system(asset_tiles::blast_furnace_update.system())
                    .with_system(asset_tiles::export_station_update.system())
                    .with_system(asset_tiles::oxygen_converter_update.system())
                    .with_system(street::update_streets.system())
                    .with_system(storage::update_consolidators.system())
                    .with_system(car::update_car.system()),
            )
            .add_system(remove_update.system().label(Label::UpdateEnd))
            .run();
    }
}
