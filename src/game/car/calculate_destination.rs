use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use hierarchical_pathfinding::prelude::*;

use crate::game::{
    assets::{Occupied, Position, RemovedBuildingEvent, RequiresUpdate, Street},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

use super::{Car, Destination, Waypoints};

const STREET_COST: isize = 1;
const GRASS_COST: isize = 3;
const BUILDING_COST: isize = 5;

fn cost_fn<'a, 'b: 'a>(
    map_query: &'b MapQuery,
    street_query: &'a Query<(), With<Street>>,
    occupied_query: &'a Query<(), With<Occupied>>,
) -> impl 'a + Fn((usize, usize)) -> isize {
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

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn calculate_destination(
    mut commands: Commands,
    mut car_query: Query<(Entity, &Destination, &Position), With<Car>>,
    street_query: Query<(), With<Street>>,
    occupied_query: Query<(), With<Occupied>>,
    update_query: Query<&Position, (With<Tile>, With<RequiresUpdate>)>,
    map_query: MapQuery,
    mut pathfinding: Local<Option<PathCache<ManhattanNeighborhood>>>,
    mut removed_events: EventReader<RemovedBuildingEvent>,
) {
    let mut updated = false;
    if pathfinding.is_none() {
        log::info!("Building pathfinding cache");
        let (_entity, layer) = map_query.get_layer(MAP_ID, BUILDING_LAYER_ID).unwrap();
        let size = layer.get_layer_size_in_tiles();

        let cache = PathCache::new(
            (size.x as usize, size.y as usize),
            cost_fn(&map_query, &street_query, &occupied_query),
            ManhattanNeighborhood::new(size.x as usize, size.y as usize),
            PathCacheConfig {
                chunk_size: 2,
                ..Default::default()
            },
        );

        *pathfinding = Some(cache);

        updated = true;
    } else {
        let mut changes: Vec<(usize, usize)> = update_query
            .iter()
            .map(|position| (position.position.x as usize, position.position.y as usize))
            .collect();

        for event in removed_events.iter() {
            changes.push((event.position.x as usize, event.position.y as usize));
        }

        if !changes.is_empty() {
            // safe unwrap due because it is always created above
            log::info!("Updating pathfinding cache: {:?}", changes);
            let pathfinding = pathfinding.as_mut().unwrap();
            pathfinding.tiles_changed(
                &changes,
                cost_fn(&map_query, &street_query, &occupied_query),
            );
            updated = true;
        }
    }

    if updated || !car_query.iter().any(|_| true) {
        return;
    }

    // safe unwrap due because it is always created above
    let pathfinding = pathfinding.as_ref().unwrap();

    for (car_entity, destination, position) in car_query.iter_mut() {
        log::info!("Calculating pathfinding");
        let path = pathfinding.find_path(
            (
                position.position.x as usize / 2,
                position.position.y as usize / 2,
            ),
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
                .insert(Waypoints::new(waypoints))
                .remove::<Destination>();
        } else {
            log::error!("No path found for car!");
            commands.entity(car_entity).remove::<Destination>();
        }
    }
}
