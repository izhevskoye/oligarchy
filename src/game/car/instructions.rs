use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::Storage,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

use super::{Car, CarInstructions, Destination};

pub fn car_instruction(
    mut commands: Commands,
    mut car_query: Query<(Entity, &mut Car), Without<Destination>>,
    mut storage_query: Query<&mut Storage>,
    map_query: MapQuery,
) {
    for (car_entity, mut car) in car_query.iter_mut() {
        if car.instructions.is_empty() {
            return;
        }
        if car.current_instruction >= car.instructions.len() {
            car.current_instruction = 0;
        }

        match car.instructions[car.current_instruction] {
            CarInstructions::Nop => {}
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
                        if let Ok(mut map_storage) = storage_query.get_mut(entity) {
                            if resource == map_storage.resource && map_storage.amount > 0 {
                                map_storage.amount -= 1;
                                true
                            } else {
                                false
                            }
                        } else {
                            log::warn!("Car waiting at location that is not a storage");
                            car.current_instruction += 1;
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
                        if let Ok(mut map_storage) = storage_query.get_mut(entity) {
                            if resource == map_storage.resource
                                && map_storage.amount < map_storage.capacity
                            {
                                map_storage.amount += 1;
                                true
                            } else {
                                false
                            }
                        } else {
                            log::warn!("Car wants to unload at a location that is not a storage");
                            car.current_instruction += 1;
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
    }
}
