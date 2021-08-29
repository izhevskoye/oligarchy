pub mod game;
pub mod ground_tiles;
pub mod street;
pub mod title;

use bevy::prelude::*;

use super::{
    state_manager::NewGameEvent, GenerateGroundTilesEvent, GenerateStreetEvent, NewGameSetup,
};

pub const MAP_ID: u16 = 0;
pub const GROUND_LAYER_ID: u16 = 0;
pub const BUILDING_LAYER_ID: u16 = 1;

pub fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.far = 1000.0 / 0.1;
    commands.spawn_bundle(camera);
}

pub fn new_game_setup(
    mut events: EventReader<NewGameEvent>,
    mut generate_street: EventWriter<GenerateStreetEvent>,
    mut generate_tiles: EventWriter<GenerateGroundTilesEvent>,
    mut setup: ResMut<Option<NewGameSetup>>,
) {
    for _ in events.iter() {
        *setup = Some(NewGameSetup::default());
        return;
    }

    if let Some(mut setup) = setup.as_mut() {
        if !setup.ground_tiles {
            generate_tiles.send(GenerateGroundTilesEvent);
            setup.ground_tiles = true;
            return;
        }

        if !setup.street {
            generate_street.send(GenerateStreetEvent);
            return;
        }
    }
}
