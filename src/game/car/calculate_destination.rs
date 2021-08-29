use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{BlockedForBuilding, CanDriveOver, Occupied, Position},
    pathfinder::{cost_fn, Pathfinding},
    setup::{BUILDING_LAYER_ID, MAP_ID},
    street::Street,
};

use super::{Car, Destination, Waypoints};

pub fn calculate_destination(
    mut commands: Commands,
    mut car_query: Query<(Entity, &Destination, &Position), With<Car>>,
    street_query: Query<(), With<Street>>,
    occupied_query: Query<(), (With<Occupied>, Without<CanDriveOver>)>,
    blocked_query: Query<(), With<BlockedForBuilding>>,
    map_query: MapQuery,
    pathfinding: Res<Pathfinding>,
) {
    if let Some(pathfinding) = &pathfinding.cache {
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
                cost_fn(&map_query, &street_query, &occupied_query, &blocked_query),
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

                // if car is on building somehow
                if let Ok(entity) =
                    map_query.get_tile_entity(position.position / 2, MAP_ID, BUILDING_LAYER_ID)
                {
                    if occupied_query.get(entity).is_ok() {
                        log::error!("Car is on building!");

                        let waypoints = vec![destination.destination];

                        commands
                            .entity(car_entity)
                            .insert(Waypoints::new(waypoints));
                    }
                }
            }
        }
    }
}
