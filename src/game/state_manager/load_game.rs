use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    account::{Account, MaintenanceCost, PurchaseCost},
    assets::{
        building_specifications::BuildingSpecifications,
        resource_specifications::ResourceSpecifications, BlockedForBuilding, Building,
        CanDriveOver, Editable, Forest, Occupied, Position, RequiresUpdate, StateName, Water,
    },
    car::{Car, CarController, DepotController},
    goals::GoalManager,
    production::{Product, ProductionBuilding},
    setup::{BUILDING_LAYER_ID, GROUND_LAYER_ID, MAP_ID},
    state_manager::{
        BuildingEntity, GameEntity, GameEntityType, GameState, LoadGameEvent, Vehicle,
    },
    statistics::StatisticTracker,
    storage::StorageConsolidator,
};

use super::VehicleController;

pub fn load_game(
    mut commands: Commands,
    mut map_query: MapQuery,
    buildings: Res<BuildingSpecifications>,
    mut load_game: EventReader<LoadGameEvent>,
    mut goals: ResMut<GoalManager>,
    mut account: ResMut<Account>,
    mut state_name: ResMut<StateName>,
    mut deleted_export_statistics: ResMut<StatisticTracker>,
    resources: Res<ResourceSpecifications>,
) {
    for event in load_game.iter() {
        goals.goals = event.state.goals.clone();
        *account = event.state.account.clone();
        *state_name = event.state.state_name.clone();
        *deleted_export_statistics = event.state.deleted_export_statistics.clone();

        load_state(
            &mut commands,
            &mut map_query,
            &event.state,
            &buildings,
            &resources,
        );
    }
}

fn load_state(
    commands: &mut Commands,
    map_query: &mut MapQuery,
    state: &GameState,
    buildings: &BuildingSpecifications,
    resources: &ResourceSpecifications,
) {
    let mut uuids = HashMap::new();

    for game_entity in &state.entities {
        match &game_entity.entity {
            GameEntityType::Vehicle(vehicle) => {
                insert_car(commands, vehicle, game_entity, resources, &uuids);
            }
            GameEntityType::Building(building) => {
                let entity = insert_building(
                    commands,
                    building,
                    game_entity,
                    map_query,
                    buildings,
                    resources,
                );

                if let Some(entity) = entity {
                    let uuid = game_entity.uuid.to_owned();
                    uuids.insert(uuid, entity);
                }
            }
            GameEntityType::Water => {
                if let Some(entity) = insert_ground_tile(commands, game_entity, map_query) {
                    commands.entity(entity).insert(Water);
                }
            }
            GameEntityType::Forest => {
                if let Some(entity) = insert_ground_tile(commands, game_entity, map_query) {
                    commands.entity(entity).insert(Forest);
                }
            }
        }
    }
}

fn insert_ground_tile(
    commands: &mut Commands,
    game_entity: &GameEntity,
    map_query: &mut MapQuery,
) -> Option<Entity> {
    let tile = Tile {
        visible: false,
        ..Default::default()
    };

    match map_query.set_tile(commands, game_entity.pos, tile, MAP_ID, GROUND_LAYER_ID) {
        Err(why) => {
            log::error!("Failed to set tile: {:?}", why);
            None
        }
        Ok(entity) => {
            commands
                .entity(entity)
                .insert(RequiresUpdate)
                .insert(BlockedForBuilding)
                .insert(Position {
                    position: game_entity.pos,
                });

            Some(entity)
        }
    }
}

fn insert_car(
    commands: &mut Commands,
    vehicle: &Vehicle,
    game_entity: &GameEntity,
    resources: &ResourceSpecifications,
    uuids: &HashMap<String, Entity>,
) {
    let controller = match &vehicle.controller {
        VehicleController::UserControlled(controller) => {
            CarController::UserControlled(controller.clone())
        }
        VehicleController::DepotControlled(depot_uuid) => {
            CarController::DepotControlled(DepotController {
                depot: *uuids.get(depot_uuid).unwrap(),
            })
        }
    };

    let car = Car {
        direction: vehicle.direction,
        controller,
    };

    let price = (car.clone(), vehicle.storage.clone()).price(resources);

    let entity = commands
        .spawn()
        .insert(RequiresUpdate)
        .insert(Position {
            position: game_entity.pos,
        })
        .insert(car)
        .insert(vehicle.storage.clone())
        .insert(Editable)
        .insert(MaintenanceCost::new_from_cost(price))
        .id();

    if let Some(name) = &game_entity.name {
        commands.entity(entity).insert(name.clone());
    }
}

fn insert_building(
    commands: &mut Commands,
    building: &BuildingEntity,
    game_entity: &GameEntity,
    map_query: &mut MapQuery,
    buildings: &BuildingSpecifications,
    resources: &ResourceSpecifications,
) -> Option<Entity> {
    let tile = Tile {
        visible: false,
        ..Default::default()
    };

    match map_query.set_tile(commands, game_entity.pos, tile, MAP_ID, BUILDING_LAYER_ID) {
        Err(why) => log::error!("Failed to set tile: {:?}", why),
        Ok(entity) => {
            let entity = commands
                .entity(entity)
                .insert(RequiresUpdate)
                .insert(Position {
                    position: game_entity.pos,
                })
                .insert(Occupied)
                .id();

            if let Some(name) = &game_entity.name {
                commands.entity(entity).insert(name.clone());
            }

            if let Some(statistics) = &game_entity.statistics {
                commands.entity(entity).insert(statistics.clone());
            }

            if let Some(under_construction) = &game_entity.under_construction {
                commands.entity(entity).insert(under_construction.clone());
            }

            match building {
                BuildingEntity::Building(c) => {
                    let building = buildings.get(&c.id).unwrap();
                    commands
                        .entity(entity)
                        .insert(Building { id: c.id.clone() });

                    if !building.products.is_empty() {
                        let products = building
                            .products
                            .iter()
                            .enumerate()
                            .map(|(index, product)| {
                                (
                                    product.clone(),
                                    *c.active_products.get(index).unwrap_or(&false),
                                )
                            })
                            .collect::<Vec<(Product, bool)>>();

                        commands
                            .entity(entity)
                            .insert(StorageConsolidator::default())
                            .insert(ProductionBuilding { products })
                            .insert(MaintenanceCost::new_from_cost(building.price(resources)))
                            .insert(Editable);
                    }
                }
                BuildingEntity::Street(c) => {
                    commands
                        .entity(entity)
                        .insert(c.clone())
                        .insert(CanDriveOver)
                        .insert(MaintenanceCost::new_from_cost(c.price(resources)));
                }
                BuildingEntity::Storage(c) => {
                    commands
                        .entity(entity)
                        .insert(c.clone())
                        .insert(MaintenanceCost::new_from_cost(c.price(resources)));
                }
                BuildingEntity::Depot(c) => {
                    commands
                        .entity(entity)
                        .insert(c.clone())
                        .insert(Editable)
                        .insert(MaintenanceCost::new_from_cost(c.price(resources)));
                }
                BuildingEntity::DeliveryStation(c) => {
                    commands
                        .entity(entity)
                        .insert(c.clone())
                        .insert(MaintenanceCost::new_from_cost(c.price(resources)))
                        .insert(CanDriveOver)
                        .insert(StorageConsolidator::default());
                }
                BuildingEntity::StorageManagement(c) => {
                    commands
                        .entity(entity)
                        .insert(c.clone())
                        .insert(MaintenanceCost::new_from_cost(c.price(resources)))
                        .insert(StorageConsolidator::default());
                }
                BuildingEntity::ImportExportStation(c) => {
                    commands
                        .entity(entity)
                        .insert(c.clone())
                        .insert(MaintenanceCost::new_from_cost(c.price(resources)))
                        .insert(Editable)
                        .insert(StorageConsolidator::default());
                }
            }

            return Some(entity);
        }
    }

    None
}
