mod assets;
mod blast_furnace;
mod camera;
mod coke_furnace;
mod constants;
mod oxygen_converter;
mod quarry;
mod setup;
mod storage;
mod texture;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;

#[derive(Default)]
pub struct Game {}

impl Game {
    pub fn run(&self) {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();

        App::build()
            .add_plugins(DefaultPlugins)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(TilemapPlugin)
            .add_startup_system(setup::setup_map.system())
            .add_system(camera::movement.system())
            .add_system(texture::set_texture_filters_to_nearest.system())
            .add_system(quarry::quarry.system())
            .add_system(coke_furnace::coke_furnace.system())
            .add_system(blast_furnace::blast_furnace.system())
            .add_system(oxygen_converter::oxygen_converter.system())
            .add_system(storage::update_consolidators.system())
            .run();
    }
}
