mod camera;
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
            .add_system(camera::movement.system())
            .add_system(texture::set_texture_filters_to_nearest.system())
            .run();
    }
}
