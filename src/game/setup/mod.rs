pub mod game;
pub mod title;

use bevy::prelude::*;

pub const MAP_ID: u16 = 0;
pub const GROUND_LAYER_ID: u16 = 0;
pub const BUILDING_LAYER_ID: u16 = 1;

pub fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.far = 1000.0 / 0.1;
    commands.spawn_bundle(camera);
}
