use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{
        Building, CanDriveOver, Editable, Occupied, Position, ProductionBuilding, RequiresUpdate,
        StorageConsolidator,
    },
    building_specifications::BuildingSpecifications,
    setup::{BUILDING_LAYER_ID, MAP_ID},
    state_manager::{
        BuildingEntity, GameEntity, GameEntityType, GameState, LoadGameEvent, Vehicle,
    },
};

pub fn load_game(
    mut commands: Commands,
    mut map_query: MapQuery,
    buildings: Res<BuildingSpecifications>,
    mut load_game: EventReader<LoadGameEvent>,
) {
    for event in load_game.iter() {
        load_state(&mut commands, &mut map_query, &event.state, &buildings);
    }
}

fn load_state(
    commands: &mut Commands,
    map_query: &mut MapQuery,
    state: &GameState,
    buildings: &Res<BuildingSpecifications>,
) {
    for game_entity in &state.entities {
        match &game_entity.entity {
            GameEntityType::Vehicle(vehicle) => {
                insert_car(commands, vehicle, game_entity);
            }
            GameEntityType::Building(building) => {
                insert_building(commands, building, game_entity, map_query, buildings);
            }
        }
    }
}

fn insert_car(commands: &mut Commands, vehicle: &Vehicle, game_entity: &GameEntity) {
    let entity = commands
        .spawn()
        .insert(RequiresUpdate)
        .insert(Position {
            position: game_entity.pos,
        })
        .insert(vehicle.car.clone())
        .insert(vehicle.storage.clone())
        .insert(Editable)
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
    buildings: &Res<BuildingSpecifications>,
) {
    let tile = Tile {
        visible: false,
        ..Default::default()
    };

    match map_query.set_tile(commands, game_entity.pos, tile, MAP_ID, BUILDING_LAYER_ID) {
        Err(why) => log::error!("Failed to set tile: {:?}", why),
        Ok(entity) => {
            // TODO: stat tracker!
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

            match building {
                BuildingEntity::Building(c) => {
                    let building = buildings.get(&c.id).unwrap();
                    commands
                        .entity(entity)
                        .insert(Building { id: c.id.clone() });

                    if !building.products.is_empty() {
                        commands
                            .entity(entity)
                            .insert(StorageConsolidator::default())
                            .insert(ProductionBuilding {
                                products: building.products.clone(),
                                active_product: c.active_product,
                            });

                        if building.products.len() > 1 {
                            commands.entity(entity).insert(Editable);
                        }
                    }
                }
                BuildingEntity::Street(c) => {
                    commands.entity(entity).insert(c.clone());
                }
                BuildingEntity::Storage(c) => {
                    commands.entity(entity).insert(c.clone());
                }
                BuildingEntity::DeliveryStation(c) => {
                    commands
                        .entity(entity)
                        .insert(c.clone())
                        .insert(CanDriveOver)
                        .insert(StorageConsolidator::default());
                }
                BuildingEntity::ExportStation(c) => {
                    commands
                        .entity(entity)
                        .insert(c.clone())
                        .insert(Editable)
                        .insert(StorageConsolidator::default());
                }
            }
        }
    }
}
