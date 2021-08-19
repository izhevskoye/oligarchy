use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{prelude::SliceRandom, thread_rng};

use crate::game::{
    assets::Position,
    production::{DeliveryStation, Depot},
    setup::{BUILDING_LAYER_ID, MAP_ID},
    storage::{distribute_to_storage, fetch_from_storage, has_space_in_storage},
    storage::{Storage, StorageConsolidator},
};

use super::{Car, CarController, CarInstructions, Destination, Waypoints};

const AMOUNT: f64 = 1.0;

#[allow(clippy::too_many_arguments)]
pub fn load(
    mut car_query: Query<(&mut Car, &Position)>,
    mut storage_query: Query<&mut Storage>,
    consolidator_query: Query<&StorageConsolidator, With<DeliveryStation>>,
    map_query: MapQuery,
    mut car_events: EventReader<CarLoadInstructionEvent>,
) {
    for car_event in car_events.iter() {
        let (_car, position) = car_query.get_mut(car_event.car).unwrap();

        let full = {
            match storage_query.get_mut(car_event.car) {
                Ok(storage) => storage.is_full(),
                _ => {
                    log::warn!("Car has no storage but should wait for loading");
                    continue;
                }
            }
        };

        if !full {
            if let Ok(entity) =
                map_query.get_tile_entity(position.position / 2, MAP_ID, BUILDING_LAYER_ID)
            {
                if let Ok(consolidator) = consolidator_query.get(entity) {
                    if fetch_from_storage(
                        consolidator,
                        &mut storage_query,
                        &car_event.resource,
                        AMOUNT,
                    ) {
                        let mut storage = storage_query.get_mut(car_event.car).unwrap();
                        storage.amount += AMOUNT;
                    }

                    continue;
                }
            }
        }

        log::warn!("Car waiting at location that is not a delivery station");
    }
}

#[allow(clippy::too_many_arguments)]
pub fn unload(
    mut car_query: Query<(&mut Car, &Position)>,
    mut storage_query: Query<&mut Storage>,
    consolidator_query: Query<&StorageConsolidator, With<DeliveryStation>>,
    map_query: MapQuery,
    mut car_events: EventReader<CarUnloadInstructionEvent>,
) {
    for car_event in car_events.iter() {
        let (_car, position) = car_query.get_mut(car_event.car).unwrap();

        let empty = {
            match storage_query.get_mut(car_event.car) {
                Ok(storage) => storage.is_empty(),
                _ => {
                    log::warn!("Car has no storage but should wait for unloading");
                    continue;
                }
            }
        };

        if !empty {
            if let Ok(entity) =
                map_query.get_tile_entity(position.position / 2, MAP_ID, BUILDING_LAYER_ID)
            {
                if let Ok(consolidator) = consolidator_query.get(entity) {
                    if has_space_in_storage(
                        consolidator,
                        &mut storage_query,
                        &car_event.resource,
                        AMOUNT,
                    ) {
                        distribute_to_storage(
                            consolidator,
                            &mut storage_query,
                            &car_event.resource,
                            AMOUNT,
                        );

                        let mut storage = storage_query.get_mut(car_event.car).unwrap();
                        storage.amount -= AMOUNT;
                    }

                    continue;
                }
            }
        }
        log::warn!("Car waiting at location that is not a delivery station");
    }
}

#[allow(clippy::too_many_arguments)]
pub fn goto(
    mut commands: Commands,
    mut car_query: Query<(&mut Car, &Position)>,
    mut car_events: EventReader<CarGoToInstructionEvent>,
) {
    for car_event in car_events.iter() {
        let (_car, position) = car_query.get_mut(car_event.car).unwrap();

        let car_pos = position.position / 2;
        if car_pos != car_event.position {
            commands.entity(car_event.car).insert(Destination {
                destination: car_event.position,
            });
        }
    }
}

#[derive(Debug)]
pub struct CarGoToInstructionEvent {
    pub car: Entity,
    pub position: UVec2,
}

#[derive(Debug)]
pub struct CarLoadInstructionEvent {
    pub car: Entity,
    pub resource: String,
}

#[derive(Debug)]
pub struct CarUnloadInstructionEvent {
    pub car: Entity,
    pub resource: String,
}

#[allow(clippy::type_complexity)]
pub fn car_instruction(
    mut car_query: Query<(Entity, &mut Car, &Position), (Without<Destination>, Without<Waypoints>)>,
    depot_query: Query<&Depot>,
    mut storage_query: Query<&mut Storage>,
    mut load_events: EventWriter<CarLoadInstructionEvent>,
    mut goto_events: EventWriter<CarGoToInstructionEvent>,
    mut unload_events: EventWriter<CarUnloadInstructionEvent>,
) {
    for (car_entity, mut car, position) in car_query.iter_mut() {
        let storage = storage_query.get_mut(car_entity).unwrap();

        match &mut car.controller {
            CarController::DepotControlled(depot_controller) => {
                if let Ok(depot) = depot_query.get(depot_controller.depot) {
                    let car_pos = position.position / 2;
                    let mut random = thread_rng();

                    if depot.pickups.contains(&car_pos) && !storage.is_full() {
                        load_events.send(CarLoadInstructionEvent {
                            car: car_entity,
                            resource: storage.resource.clone(),
                        });
                    } else if depot.deliveries.contains(&car_pos) && !storage.is_empty() {
                        unload_events.send(CarUnloadInstructionEvent {
                            car: car_entity,
                            resource: storage.resource.clone(),
                        });
                    } else if storage.is_empty() {
                        // go to pickup
                        let mut places = depot.pickups.clone();
                        places.shuffle(&mut random);

                        if let Some(place) = places.get(0) {
                            goto_events.send(CarGoToInstructionEvent {
                                car: car_entity,
                                position: *place,
                            });
                        }
                    } else if !storage.is_empty() {
                        // go to delivery
                        let mut places = depot.deliveries.clone();
                        places.shuffle(&mut random);

                        if let Some(place) = places.get(0) {
                            goto_events.send(CarGoToInstructionEvent {
                                car: car_entity,
                                position: *place,
                            });
                        }
                    }
                } else {
                    log::error!("Car assigned to a depot which is not found");
                }
            }
            CarController::UserControlled(user_controller) => {
                if user_controller.instructions.is_empty() || !user_controller.active {
                    continue;
                }

                // needed in case something is deleted?
                if user_controller.current_instruction >= user_controller.instructions.len() {
                    user_controller.current_instruction = 0;
                }

                let skip = match &user_controller.instructions[user_controller.current_instruction]
                {
                    CarInstructions::Nop => true,
                    CarInstructions::GoTo(destination) => position.position == *destination,
                    CarInstructions::Load(_resource) => storage.is_full(),
                    CarInstructions::WaitForLoad(_resource) => storage.is_full(),
                    CarInstructions::Unload(_resource) => storage.is_empty(),
                    CarInstructions::WaitForUnload(_resource) => storage.is_empty(),
                };

                if skip {
                    user_controller.current_instruction += 1;
                }

                if user_controller.current_instruction >= user_controller.instructions.len() {
                    user_controller.current_instruction = 0;
                }

                match &user_controller.instructions[user_controller.current_instruction] {
                    CarInstructions::Nop => {}
                    CarInstructions::GoTo(destination) => {
                        goto_events.send(CarGoToInstructionEvent {
                            car: car_entity,
                            position: *destination,
                        });
                    }
                    CarInstructions::Load(resource) => {
                        // TODO: no wait!
                        load_events.send(CarLoadInstructionEvent {
                            car: car_entity,
                            resource: resource.clone(),
                        });
                    }
                    CarInstructions::WaitForLoad(resource) => {
                        load_events.send(CarLoadInstructionEvent {
                            car: car_entity,
                            resource: resource.clone(),
                        });
                    }
                    CarInstructions::Unload(resource) => {
                        unload_events.send(CarUnloadInstructionEvent {
                            car: car_entity,
                            resource: resource.clone(),
                        });
                    }
                    CarInstructions::WaitForUnload(resource) => {
                        unload_events.send(CarUnloadInstructionEvent {
                            car: car_entity,
                            resource: resource.clone(),
                        });
                    }
                }
            }
        }
    }
}
