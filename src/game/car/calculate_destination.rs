use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use hierarchical_pathfinding::prelude::*;

use crate::game::{
    assets::{Occupied, Street},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

use super::{Car, Destination, Waypoints};

const STREET_COST: isize = 1;
const GRASS_COST: isize = 100;
const BUILDING_COST: isize = 1000;

fn cost_fn<'a, 'b: 'a>(
    map_query: &'b MapQuery,
    street_query: &'a Query<(), With<Street>>,
    occupied_query: &'a Query<(), With<Occupied>>,
) -> impl 'a + FnMut((usize, usize)) -> isize {
    move |(x, y)| match map_query.get_tile_entity(
        UVec2::new(x as u32, y as u32),
        MAP_ID,
        BUILDING_LAYER_ID,
    ) {
        Ok(entity) => {
            if street_query.get(entity).is_ok() {
                STREET_COST
            } else if occupied_query.get(entity).is_ok() {
                BUILDING_COST
            } else {
                GRASS_COST
            }
        }
        Err(_) => GRASS_COST,
    }
}

pub fn calculate_destination(
    mut commands: Commands,
    mut car_query: Query<(Entity, &Car, &Destination)>,
    street_query: Query<(), With<Street>>,
    occupied_query: Query<(), With<Occupied>>,
    map_query: MapQuery,
    // TODO: invalidate when new things are placed
    mut pathfinding: Local<Option<PathCache<ManhattanNeighborhood>>>,
) {
    if car_query.iter().find(|_| true).is_none() {
        return;
    }

    let (_entity, layer) = map_query.get_layer(MAP_ID, BUILDING_LAYER_ID).unwrap();

    let size = layer.get_layer_size_in_tiles();

    if pathfinding.is_none() {
        log::info!("Redo pathfinding cache");
        let cache = PathCache::new(
            (size.x as usize, size.y as usize),
            cost_fn(&map_query, &street_query, &occupied_query),
            ManhattanNeighborhood::new(size.x as usize, size.y as usize),
            PathCacheConfig {
                chunk_size: 3,
                ..Default::default()
            },
        );

        *pathfinding = Some(cache);
    }

    if let Some(pathfinding) = pathfinding.as_ref() {
        for (car_entity, car, destination) in car_query.iter_mut() {
            log::info!("Calculating pathfinding");
            let path = pathfinding.find_path(
                (car.position.x as usize / 2, car.position.y as usize / 2),
                (
                    destination.destination.x as usize,
                    destination.destination.y as usize,
                ),
                cost_fn(&map_query, &street_query, &occupied_query),
            );

            if let Some(path) = path {
                let waypoints = path
                    .into_iter()
                    .map(|(x, y)| UVec2::new(x as u32, y as u32))
                    .collect();

                commands
                    .entity(car_entity)
                    .insert(Waypoints { waypoints })
                    .remove::<Destination>();
            } else {
                // TODO:
            }
        }
    }
}
