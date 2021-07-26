mod blast_furnace;
mod building;
mod bulldoze;
mod car;
mod coke_furnace;
mod export_station;
mod oxygen_converter;
mod storage;
mod street;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::{RequiresUpdate, Street},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

fn get_entity(
    commands: &mut Commands,
    map_query: &mut MapQuery,
    pos: UVec2,
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
                    visible: false,
                    ..Default::default()
                },
                MAP_ID,
                layer_id,
            )
            .unwrap()
    }
}

fn update_neighbor_streets(
    commands: &mut Commands,
    map_query: &mut MapQuery,
    pos: UVec2,
    street_query: Query<&Street>,
) {
    let neighbors = map_query.get_tile_neighbors(pos, MAP_ID, BUILDING_LAYER_ID);
    for (pos, neighbor) in neighbors[0..4].iter() {
        if let Some(neighbor) = neighbor {
            if street_query.get(*neighbor).is_ok() {
                commands.entity(*neighbor).insert(RequiresUpdate {
                    position: pos.as_u32(),
                });
            }
        }
    }
}

pub fn current_tool_system() -> SystemSet {
    SystemSet::new()
        .with_system(street::street_placement.system())
        .with_system(storage::storage_placement.system())
        .with_system(coke_furnace::coke_furnace_placement.system())
        .with_system(blast_furnace::blast_furnace_placement.system())
        .with_system(oxygen_converter::oxygen_converter_placement.system())
        .with_system(export_station::export_station_placement.system())
        .with_system(car::car_placement.system())
        .with_system(building::building_placement.system())
        .with_system(bulldoze::bulldoze.system())
}
