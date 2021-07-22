mod assets;
mod camera;
mod car;
mod constants;
mod current_selection;
mod current_tool;
mod production;
mod setup;
mod storage;
mod street;
mod texture;
mod ui;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;

use self::assets::{ClickedTile, CurrentlySelected, SelectedTool};

#[derive(Default)]
pub struct Game {}

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
            .add_system(storage::update_consolidators.system())
            .add_system(street::update_streets.system())
            .add_system(
                current_selection::current_selection
                    .system()
                    .label("current_selection"),
            )
            .add_system_set(current_tool::current_tool_system().after("current_selection"))
            .add_system_set(production::production_system())
            .add_system_set(car::calculate_system())
            .add_system_set(car::instruction_system())
            .add_system_set(car::drive_system())
            .run();
    }
}
