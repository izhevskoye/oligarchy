use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{DeliveryStation, Position, Storage, StorageConsolidator},
    setup::{BUILDING_LAYER_ID, MAP_ID},
    storage::{distribute_to_storage, fetch_from_storage, has_space_in_storage},
};

use super::{Car, CarInstructions, Destination, Waypoints};

const AMOUNT: f64 = 1.0;

fn load(
    car_entity: Entity,
    car: &mut Mut<Car>,
    position: &Position,
    resource: &str,
    storage_query: &mut Query<&mut Storage>,
    consolidator_query: &Query<&StorageConsolidator, With<DeliveryStation>>,
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
        return;
    } else if let Ok(entity) =
        map_query.get_tile_entity(position.position / 2, MAP_ID, BUILDING_LAYER_ID)
    {
        if let Ok(consolidator) = consolidator_query.get(entity) {
            if fetch_from_storage(consolidator, storage_query, resource, AMOUNT) {
                let mut storage = storage_query.get_mut(car_entity).unwrap();
                storage.amount += AMOUNT;
                return;
            } else if !wait_for_load {
                car.current_instruction += 1;
                return;
            }
        }
    }

    log::warn!("Car waiting at location that is not a delivery station");
    car.current_instruction += 1;
}

fn unload(
    car_entity: Entity,
    car: &mut Mut<Car>,
    position: &Position,
    resource: &str,
    storage_query: &mut Query<&mut Storage>,
    consolidator_query: &Query<&StorageConsolidator, With<DeliveryStation>>,
    map_query: &MapQuery,
    wait_for_unload: bool,
) {
    let empty = {
        match storage_query.get_mut(car_entity) {
            Ok(storage) => storage.amount == 0.0,
            _ => {
                log::warn!("Car has no storage but should wait for unloading");
                car.current_instruction += 1;
                return;
            }
        }
    };

    if empty {
        car.current_instruction += 1;
        return;
    } else if let Ok(entity) =
        map_query.get_tile_entity(position.position / 2, MAP_ID, BUILDING_LAYER_ID)
    {
        if let Ok(consolidator) = consolidator_query.get(entity) {
            if has_space_in_storage(consolidator, storage_query, resource, AMOUNT) {
                distribute_to_storage(consolidator, storage_query, resource, AMOUNT);

                let mut storage = storage_query.get_mut(car_entity).unwrap();
                storage.amount -= AMOUNT;
                return;
            } else if !wait_for_unload {
                car.current_instruction += 1;
                return;
            }
        }
    }

    log::warn!("Car waiting at location that is not a delivery station");
    car.current_instruction += 1;
}

#[allow(clippy::type_complexity)]
pub fn car_instruction(
    mut commands: Commands,
    mut car_query: Query<(Entity, &mut Car, &Position), (Without<Destination>, Without<Waypoints>)>,
    mut storage_query: Query<&mut Storage>,
    consolidator_query: Query<&StorageConsolidator, With<DeliveryStation>>,
    map_query: MapQuery,
) {
    for (car_entity, mut car, position) in car_query.iter_mut() {
        if car.instructions.is_empty() || !car.active {
            continue;
        }

        if car.current_instruction >= car.instructions.len() {
            car.current_instruction = 0;
        }

        match car.instructions[car.current_instruction].clone() {
            CarInstructions::Nop => {}
            CarInstructions::GoTo(destination) => {
                let car_pos = position.position / 2;
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
                    position,
                    &resource,
                    &mut storage_query,
                    &consolidator_query,
                    &map_query,
                    false,
                );
            }
            CarInstructions::WaitForLoad(resource) => {
                load(
                    car_entity,
                    &mut car,
                    position,
                    &resource,
                    &mut storage_query,
                    &consolidator_query,
                    &map_query,
                    true,
                );
            }
            CarInstructions::Unload(resource) => {
                unload(
                    car_entity,
                    &mut car,
                    position,
                    &resource,
                    &mut storage_query,
                    &consolidator_query,
                    &map_query,
                    false,
                );
            }
            CarInstructions::WaitForUnload(resource) => {
                unload(
                    car_entity,
                    &mut car,
                    position,
                    &resource,
                    &mut storage_query,
                    &consolidator_query,
                    &map_query,
                    true,
                );
            }
        }
    }
}
