mod blast_furnace;
mod car;
mod coke_furnace;
mod export_station;
mod oxygen_converter;
mod quarry;
mod storage;
mod street;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use num_traits::ToPrimitive;

use super::setup::MAP_ID;

fn get_entity<T: ToPrimitive>(
    commands: &mut Commands,
    map_query: &mut MapQuery,
    pos: UVec2,
    map_tile: T,
    layer_id: u16,
) -> Entity {
    if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, layer_id) {
        entity
    } else {
        map_query.notify_chunk_for_tile(pos, MAP_ID, layer_id);
        map_query
            .set_tile(
                commands,
                pos,
                Tile {
                    texture_index: map_tile.to_u16().unwrap(),
                    ..Default::default()
                },
                MAP_ID,
                layer_id,
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
        .with_system(car::car_placement.system())
}
