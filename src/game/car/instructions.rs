use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{prelude::SliceRandom, thread_rng};

use crate::game::{
    assets::Position,
    production::{DeliveryStation, Depot},
    setup::{BUILDING_LAYER_ID, MAP_ID},
    storage::{amount_in_storage, Storage, StorageConsolidator},
    storage::{distribute_to_storage, fetch_from_storage, has_space_in_storage},
};

use super::{Car, CarController, CarInstructions, Destination, Waypoints};

const AMOUNT: f64 = 4.0;

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
                    let amount =
                        amount_in_storage(consolidator, &mut storage_query, &car_event.resource)
                            .min(AMOUNT);

                    if fetch_from_storage(
                        consolidator,
                        &mut storage_query,
                        &car_event.resource,
                        amount,
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
                    let amount = {
                        let storage = storage_query.get_mut(car_event.car).unwrap();
                        storage.amount.min(AMOUNT)
                    };

                    if has_space_in_storage(
                        consolidator,
                        &mut storage_query,
                        &car_event.resource,
                        amount,
                    ) {
                        distribute_to_storage(
                            consolidator,
                            &mut storage_query,
                            &car_event.resource,
                            amount,
                        );

                        let mut storage = storage_query.get_mut(car_event.car).unwrap();
                        storage.amount -= amount;
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

#[derive(Default)]
pub struct WaitTime {
    pub ticks: i64,
}

const MAX_TICKS: i64 = 10;
const PERCENTAGE_TO_FILL: f64 = 0.4;

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn car_instruction(
    mut commands: Commands,
    mut car_query: Query<
        (Entity, &mut Car, &Position, Option<&mut WaitTime>),
        (Without<Destination>, Without<Waypoints>),
    >,
    depot_query: Query<&Depot>,
    mut storage_query: Query<&mut Storage>,
    mut load_events: EventWriter<CarLoadInstructionEvent>,
    mut goto_events: EventWriter<CarGoToInstructionEvent>,
    mut unload_events: EventWriter<CarUnloadInstructionEvent>,
    map_query: MapQuery,
) {
    for (car_entity, mut car, position, mut wait) in car_query.iter_mut() {
        let storage = storage_query.get_mut(car_entity).unwrap().clone();

        match &mut car.controller {
            CarController::DepotControlled(depot_controller) => {
                if let Ok(depot) = depot_query.get(depot_controller.depot) {
                    let car_pos = position.position / 2;
                    let mut random = thread_rng();

                    let mut should_load = depot.pickups.contains(&car_pos) && !storage.is_full();
                    let mut should_unload =
                        depot.deliveries.contains(&car_pos) && !storage.is_empty();

                    if should_load || should_unload {
                        if let Some(wait) = &mut wait {
                            wait.ticks += 1;

                            if wait.ticks > MAX_TICKS {
                                should_load = false;
                                should_unload = false;
                                commands.entity(car_entity).remove::<WaitTime>();
                            }
                        } else {
                            commands.entity(car_entity).insert(WaitTime::default());
                        }
                    }

                    if should_load {
                        load_events.send(CarLoadInstructionEvent {
                            car: car_entity,
                            resource: storage.resource.clone(),
                        });
                    } else if should_unload {
                        unload_events.send(CarUnloadInstructionEvent {
                            car: car_entity,
                            resource: storage.resource.clone(),
                        });
                    } else if storage.percentage() <= PERCENTAGE_TO_FILL {
                        // go to pickup
                        let mut places: Vec<&UVec2> = depot.pickups.iter().collect();
                        places.shuffle(&mut random);

                        if let Some(place) = places.get(0) {
                            goto_events.send(CarGoToInstructionEvent {
                                car: car_entity,
                                position: **place,
                            });
                        }
                    } else if !storage.is_empty() {
                        // go to delivery
                        let mut places: Vec<&UVec2> = depot.deliveries.iter().collect();
                        places.shuffle(&mut random);

                        if let Some(place) = places.get(0) {
                            goto_events.send(CarGoToInstructionEvent {
                                car: car_entity,
                                position: **place,
                            });
                        }
                    }
                } else {
                    log::error!("Car assigned to a depot which is not found");
                }
            }
            CarController::UserControlled(user_controller) => {
                if wait.is_some() {
                    commands.entity(car_entity).remove::<WaitTime>();
                }

                if user_controller.instructions.is_empty() || !user_controller.active {
                    continue;
                }

                // needed in case something is deleted?
                if user_controller.current_instruction >= user_controller.instructions.len() {
                    user_controller.current_instruction = 0;
                }

                let current_position_storage = {
                    let tile_pos = position.position / 2;
                    if let Ok(entity) =
                        map_query.get_tile_entity(tile_pos, MAP_ID, BUILDING_LAYER_ID)
                    {
                        if let Ok(storage) = storage_query.get_mut(entity) {
                            Some(storage.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                let skip = match &user_controller.instructions[user_controller.current_instruction]
                {
                    CarInstructions::Nop => true,
                    CarInstructions::GoTo(destination) => position.position == *destination,
                    CarInstructions::Load(_resource) => {
                        if let Some(current_position_storage) = current_position_storage {
                            storage.is_full() || current_position_storage.is_empty()
                        } else {
                            true
                        }
                    }
                    CarInstructions::WaitForLoad(_resource) => {
                        storage.is_full() || current_position_storage.is_none()
                    }
                    CarInstructions::Unload(_resource) => {
                        if let Some(current_position_storage) = current_position_storage {
                            storage.is_empty() || current_position_storage.is_full()
                        } else {
                            true
                        }
                    }
                    CarInstructions::WaitForUnload(_resource) => {
                        storage.is_empty() || current_position_storage.is_none()
                    }
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
                    CarInstructions::WaitForLoad(resource) | CarInstructions::Load(resource) => {
                        load_events.send(CarLoadInstructionEvent {
                            car: car_entity,
                            resource: resource.clone(),
                        });
                    }
                    CarInstructions::Unload(resource)
                    | CarInstructions::WaitForUnload(resource) => {
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
