use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    account::{Account, MaintenanceCost, PurchaseCost},
    assets::{
        building_specifications::BuildingSpecifications,
        resource_specifications::ResourceSpecifications, Building, CanDriveOver, Editable,
        Occupied, Position, RequiresUpdate, StateName,
    },
    goals::GoalManager,
    production::{Product, ProductionBuilding},
    setup::{BUILDING_LAYER_ID, MAP_ID},
    state_manager::{
        BuildingEntity, GameEntity, GameEntityType, GameState, LoadGameEvent, Vehicle,
    },
    storage::StorageConsolidator,
};

#[allow(clippy::too_many_arguments)]
pub fn load_game(
    mut commands: Commands,
    mut map_query: MapQuery,
    buildings: Res<BuildingSpecifications>,
    mut load_game: EventReader<LoadGameEvent>,
    mut goals: ResMut<GoalManager>,
    mut account: ResMut<Account>,
    mut state_name: ResMut<StateName>,
    resources: Res<ResourceSpecifications>,
) {
    for event in load_game.iter() {
        goals.goals = event.state.goals.clone();
        *account = event.state.account.clone();
        *state_name = event.state.state_name.clone();

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
    for game_entity in &state.entities {
        match &game_entity.entity {
            GameEntityType::Vehicle(vehicle) => {
                insert_car(commands, vehicle, game_entity, resources);
            }
            GameEntityType::Building(building) => {
                insert_building(
                    commands,
                    building,
                    game_entity,
                    map_query,
                    buildings,
                    resources,
                );
            }
        }
    }
}

fn insert_car(
    commands: &mut Commands,
    vehicle: &Vehicle,
    game_entity: &GameEntity,
    resources: &ResourceSpecifications,
) {
    let price = (vehicle.car.clone(), vehicle.storage.clone()).price(resources);

    let entity = commands
        .spawn()
        .insert(RequiresUpdate)
        .insert(Position {
            position: game_entity.pos,
        })
        .insert(vehicle.car.clone())
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
) {
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
                BuildingEntity::DeliveryStation(c) => {
                    commands
                        .entity(entity)
                        .insert(c.clone())
                        .insert(MaintenanceCost::new_from_cost(c.price(resources)))
                        .insert(CanDriveOver)
                        .insert(StorageConsolidator::default());
                }
                BuildingEntity::ExportStation(c) => {
                    commands
                        .entity(entity)
                        .insert(c.clone())
                        .insert(MaintenanceCost::new_from_cost(c.price(resources)))
                        .insert(Editable)
                        .insert(StorageConsolidator::default());
                }
            }
        }
    }
}
