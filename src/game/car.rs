use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::{Car, CarInstructions, Destination, Storage, Waypoints},
    constants::VehicleTile,
    setup::{BUILDING_LAYER_ID, MAP_ID, VEHICLE_LAYER_ID},
};

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    None,
    North,
    East,
    South,
    West,
}

pub fn car_instruction(
    mut commands: Commands,
    mut car_query: Query<(Entity, &mut Car), Without<Destination>>,
    mut storage_query: Query<&mut Storage>,
    map_query: MapQuery,
) {
    for (car_entity, mut car) in car_query.iter_mut() {
        match car.instructions[car.current_instruction] {
            CarInstructions::GoTo(destination) => {
                let car_pos = car.position / 2;
                if car_pos == destination {
                    car.current_instruction += 1;
                } else {
                    commands
                        .entity(car_entity)
                        .insert(Destination { destination });
                }
            }
            CarInstructions::WaitForLoad(resource) => {
                let full = {
                    let storage = storage_query.get_mut(car_entity).unwrap();

                    storage.amount >= storage.capacity
                };

                if full {
                    car.current_instruction += 1;
                } else {
                    let entity = map_query
                        .get_tile_entity(car.position / 2, MAP_ID, BUILDING_LAYER_ID)
                        .unwrap();

                    let has_item = {
                        let mut map_storage = storage_query.get_mut(entity).unwrap();

                        if resource == map_storage.resource && map_storage.amount > 0 {
                            map_storage.amount -= 1;
                            true
                        } else {
                            false
                        }
                    };

                    if has_item {
                        let mut storage = storage_query.get_mut(car_entity).unwrap();
                        storage.amount += 1;
                    }
                }
            }
            CarInstructions::WaitForUnload(resource) => {
                let empty = {
                    let storage = storage_query.get_mut(car_entity).unwrap();

                    storage.amount == 0
                };

                if empty {
                    car.current_instruction += 1;
                } else {
                    let entity = map_query
                        .get_tile_entity(car.position / 2, MAP_ID, BUILDING_LAYER_ID)
                        .unwrap();

                    let transfer_item = {
                        let mut map_storage = storage_query.get_mut(entity).unwrap();

                        if resource == map_storage.resource
                            && map_storage.amount < map_storage.capacity
                        {
                            map_storage.amount += 1;
                            true
                        } else {
                            false
                        }
                    };

                    if transfer_item {
                        let mut storage = storage_query.get_mut(car_entity).unwrap();
                        storage.amount -= 1;
                    }
                }
            }
        }

        if car.current_instruction >= car.instructions.len() {
            car.current_instruction = 0;
        }
    }
}

pub fn calculate_destination(
    mut commands: Commands,
    mut car_query: Query<(Entity, &Destination), With<Car>>,
) {
    for (car_entity, destination) in car_query.iter_mut() {
        let waypoints = vec![destination.destination];

        // TODO: better :)

        commands
            .entity(car_entity)
            .insert(Waypoints { waypoints })
            .remove::<Destination>();
    }
}

pub fn drive_to_destination(
    mut commands: Commands,
    mut car_query: Query<(Entity, &mut Car, &mut Waypoints)>,
    mut tile_query: Query<&mut Tile>,
    mut map_query: MapQuery,
) {
    for (car_entity, mut car, mut waypoint) in car_query.iter_mut() {
        let direction = waypoint.waypoints[0];
        let c_pos = car.position / 2;

        let mut direction = if direction.x < c_pos.x {
            Direction::West
        } else if direction.x > c_pos.x {
            Direction::East
        } else if direction.y < c_pos.y {
            Direction::South
        } else if direction.y > c_pos.y {
            Direction::North
        } else {
            Direction::None
        };

        // TODO: better :)
        if direction == Direction::North && car.position.x % 2 == 0 {
            direction = Direction::East;
        } else if direction == Direction::South && car.position.x % 2 == 1 {
            direction = Direction::West;
        } else if direction == Direction::East && car.position.y % 2 == 1 {
            direction = Direction::South;
        } else if direction == Direction::West && car.position.y % 2 == 0 {
            direction = Direction::North;
        }

        let entity = map_query
            .get_tile_entity(car.position, MAP_ID, VEHICLE_LAYER_ID)
            .unwrap();

        if let Ok(mut tile) = tile_query.get_mut(entity) {
            tile.texture_index = VehicleTile::Empty as u16;
            map_query.notify_chunk_for_tile(car.position, MAP_ID, VEHICLE_LAYER_ID);

            if direction == Direction::West {
                car.position.x -= 1;
            } else if direction == Direction::East {
                car.position.x += 1;
            } else if direction == Direction::South {
                car.position.y -= 1;
            } else if direction == Direction::North {
                car.position.y += 1;
            } else {
                // we are on correct tile
                waypoint.waypoints = waypoint.waypoints[1..]
                    .iter()
                    .map(|v| v.clone())
                    .collect::<Vec<UVec2>>();

                if waypoint.waypoints.len() == 0 {
                    commands.entity(car_entity).remove::<Waypoints>();
                }
            }

            let entity = map_query
                .get_tile_entity(car.position, MAP_ID, VEHICLE_LAYER_ID)
                .unwrap();

            let mut tile = tile_query.get_mut(entity).unwrap();
            tile.texture_index = VehicleTile::BlueVertical as u16;

            map_query.notify_chunk_for_tile(car.position, MAP_ID, VEHICLE_LAYER_ID);
        }
    }
}
