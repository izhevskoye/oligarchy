pub mod building;
pub mod bulldoze;
pub mod car;
pub mod delivery_station;
pub mod depot;
pub mod export_station;
pub mod storage;
pub mod storage_management;
pub mod street;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::RequiresUpdate,
    setup::{BUILDING_LAYER_ID, MAP_ID},
    street::Street,
};

fn update_neighbor_streets(
    commands: &mut Commands,
    map_query: &mut MapQuery,
    pos: UVec2,
    street_query: Query<&Street>,
) {
    let neighbors = map_query.get_tile_neighbors(pos, MAP_ID, BUILDING_LAYER_ID);
    for (_pos, neighbor) in neighbors[0..4].iter() {
        if let Some(neighbor) = neighbor {
            if street_query.get(*neighbor).is_ok() {
                commands.entity(*neighbor).insert(RequiresUpdate);
            }
        }
    }
}
