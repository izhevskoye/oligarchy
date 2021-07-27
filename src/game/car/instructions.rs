use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{Resource, Storage},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

use super::{Car, CarInstructions, Destination, Waypoints};

fn load(
    car_entity: Entity,
    car: &mut Mut<Car>,
    resource: Resource,
    storage_query: &mut Query<&mut Storage>,
    map_query: &MapQuery,
    wait_for_load: bool,
) {
    let full = {
        match storage_query.get_mut(car_entity) {
            Ok(storage) => storage.amount >= storage.capacity,
            _ => {
                log::warn!("Car has no storage but should wait for loading");
                car.current_instruction += 1;
                return;
            }
        }
    };

    if full {
        car.current_instruction += 1;
    } else {
        let has_item = {
            let mut result = None;

            if let Ok(entity) =
                map_query.get_tile_entity(car.position / 2, MAP_ID, BUILDING_LAYER_ID)
            {
                if let Ok(mut map_storage) = storage_query.get_mut(entity) {
                    if resource == map_storage.resource && map_storage.amount > 0 {
                        map_storage.amount -= 1;
                        result = Some(true)
                    } else {
                        result = Some(false)
                    }
                }
            }

            if let Some(result) = result {
                result
            } else {
                log::warn!("Car waiting at location that is not a storage");
                car.current_instruction += 1;
                return;
            }
        };

        if has_item {
            // save unwrap here because we checked above and has_item is false if
            // it does not exist
            let mut storage = storage_query.get_mut(car_entity).unwrap();
            storage.amount += 1;
        } else if !wait_for_load {
            car.current_instruction += 1;
        }
    }
}

fn unload(
    car_entity: Entity,
    car: &mut Mut<Car>,
    resource: Resource,
    storage_query: &mut Query<&mut Storage>,
    map_query: &MapQuery,
    wait_for_unload: bool,
) {
    let empty = {
        match storage_query.get_mut(car_entity) {
            Ok(storage) => storage.amount == 0,
            _ => {
                log::warn!("Car has no storage but should wait for unloading");
                car.current_instruction += 1;
                return;
            }
        }
    };

    if empty {
        car.current_instruction += 1;
    } else {
        let transfer_item = {
            let mut result = None;

            if let Ok(entity) =
                map_query.get_tile_entity(car.position / 2, MAP_ID, BUILDING_LAYER_ID)
            {
                if let Ok(mut map_storage) = storage_query.get_mut(entity) {
                    if resource == map_storage.resource && map_storage.amount < map_storage.capacity
                    {
                        map_storage.amount += 1;
                        result = Some(true)
                    } else {
                        result = Some(false)
                    }
                }
            }

            if let Some(result) = result {
                result
            } else {
                log::warn!("Car wants to unload at a location that is not a storage");
                car.current_instruction += 1;
                return;
            }
        };

        if transfer_item {
            // save unwrap here because we checked above and transfer_item is false if
            // it does not exist
            let mut storage = storage_query.get_mut(car_entity).unwrap();
            storage.amount -= 1;
        } else if !wait_for_unload {
            car.current_instruction += 1;
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn car_instruction(
    mut commands: Commands,
    mut car_query: Query<(Entity, &mut Car), (Without<Destination>, Without<Waypoints>)>,
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
            CarInstructions::Load(resource) => {
                load(
                    car_entity,
                    &mut car,
                    resource,
                    &mut storage_query,
                    &map_query,
                    false,
                );
            }
            CarInstructions::WaitForLoad(resource) => {
                load(
                    car_entity,
                    &mut car,
                    resource,
                    &mut storage_query,
                    &map_query,
                    true,
                );
            }
            CarInstructions::Unload(resource) => {
                unload(
                    car_entity,
                    &mut car,
                    resource,
                    &mut storage_query,
                    &map_query,
                    false,
                );
            }
            CarInstructions::WaitForUnload(resource) => {
                unload(
                    car_entity,
                    &mut car,
                    resource,
                    &mut storage_query,
                    &map_query,
                    true,
                );
            }
        }
    }
}
