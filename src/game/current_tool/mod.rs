mod blast_furnace;
mod coke_furnace;
mod export_station;
mod oxygen_converter;
mod quarry;
mod storage;
mod street;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    constants::MapTile,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

fn get_entity(
    commands: &mut Commands,
    map_query: &mut MapQuery,
    pos: UVec2,
    map_tile: MapTile,
) -> Entity {
    if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, BUILDING_LAYER_ID) {
        entity
    } else {
        map_query.notify_chunk_for_tile(pos, MAP_ID, BUILDING_LAYER_ID);
        map_query
            .set_tile(
                commands,
                pos,
                Tile {
                    texture_index: map_tile as u16,
                    ..Default::default()
                },
                MAP_ID,
                BUILDING_LAYER_ID,
            )
            .unwrap()
    }
}

pub fn current_tool_system() -> SystemSet {
    SystemSet::new()
        .with_system(street::street_placement.system())
        .with_system(storage::storage_placement.system())
        .with_system(quarry::quarry_placement.system())
        .with_system(coke_furnace::coke_furnace_placement.system())
        .with_system(blast_furnace::blast_furnace_placement.system())
        .with_system(oxygen_converter::oxygen_converter_placement.system())
        .with_system(export_station::export_station_placement.system())
}
