use std::{collections::HashMap, fs::File, io::prelude::*, path::Path};

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use uuid::Uuid;

use crate::game::{
    account::Account,
    assets::{Building, MapSettings, Name, Position, StateName},
    car::{Car, CarController},
    construction::UnderConstruction,
    goals::GoalManager,
    ground_tiles::{Forrest, Water},
    production::{DeliveryStation, Depot, ExportStation, ProductionBuilding, StorageManagement},
    setup::{BUILDING_LAYER_ID, GROUND_LAYER_ID, MAP_ID},
    state_manager::{
        BuildingEntity, GameEntity, GameEntityType, GameState, SaveGameEvent, SerializedBuilding,
        Vehicle,
    },
    statistics::Statistics,
    storage::Storage,
    street::Street,
};

use super::VehicleController;

#[derive(Default)]
struct UuidCollection {
    uuids: HashMap<Entity, String>,
}

impl UuidCollection {
    fn get(&mut self, entity: Entity) -> String {
        self.uuids
            .entry(entity)
            .or_insert_with(|| Uuid::new_v4().to_string())
            .to_owned()
    }
}

#[allow(clippy::type_complexity)]
pub fn save_game(
    queries: (
        Query<&Name>,
        Query<(Entity, &Car, &Position)>,
        Query<&Statistics>,
        Query<&UnderConstruction>,
        Query<&Storage>,
        Query<&ExportStation>,
        Query<&DeliveryStation>,
        Query<&StorageManagement>,
        Query<&Depot>,
        Query<&Street>,
        Query<(), With<Water>>,
        Query<(), With<Forrest>>,
        Query<(&Building, Option<&ProductionBuilding>)>,
    ),
    map_query: MapQuery,
    mut save_game: EventReader<SaveGameEvent>,
    map_settings: Res<MapSettings>,
    goals: Res<GoalManager>,
    account: Res<Account>,
    state_name: Res<StateName>,
) {
    let (
        name_query,
        car_query,
        statistics_query,
        under_construction_query,
        storage_query,
        export_station_query,
        delivery_station_query,
        storage_management_query,
        depot_query,
        street_query,
        water_query,
        forrest_query,
        building_query,
    ) = queries;

    for event in save_game.iter() {
        let mut state = GameState {
            settings: map_settings.clone(),
            goals: goals.goals.clone(),
            account: account.clone(),
            state_name: state_name.clone(),
            ..Default::default()
        };

        let mut uuids = UuidCollection::default();

        let (_, layer) = map_query.get_layer(MAP_ID, BUILDING_LAYER_ID).unwrap();
        let size = layer.get_layer_size_in_tiles();
        for y in 0..size.y {
            for x in 0..size.x {
                let pos = UVec2::new(x, y);

                if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, GROUND_LAYER_ID) {
                    if water_query.get(entity).is_ok() {
                        state.entities.push(GameEntity {
                            uuid: uuids.get(entity),
                            pos,
                            name: None,
                            entity: GameEntityType::Water,
                            statistics: None,
                            under_construction: None,
                        });
                    }

                    if forrest_query.get(entity).is_ok() {
                        state.entities.push(GameEntity {
                            uuid: uuids.get(entity),
                            pos,
                            name: None,
                            entity: GameEntityType::Forrest,
                            statistics: None,
                            under_construction: None,
                        });
                    }
                }

                if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, BUILDING_LAYER_ID) {
                    let name = if let Ok(name) = name_query.get(entity) {
                        Some(name.clone())
                    } else {
                        None
                    };

                    let statistics = if let Ok(statistics) = statistics_query.get(entity) {
                        Some(statistics.clone())
                    } else {
                        None
                    };

                    let under_construction =
                        if let Ok(under_construction) = under_construction_query.get(entity) {
                            Some(under_construction.clone())
                        } else {
                            None
                        };

                    if let Ok((building, production_building)) = building_query.get(entity) {
                        let active_products = if let Some(pb) = production_building {
                            pb.products.iter().map(|(_, active)| *active).collect()
                        } else {
                            vec![]
                        };

                        state.entities.push(GameEntity {
                            uuid: uuids.get(entity),
                            pos,
                            name: name.clone(),
                            entity: GameEntityType::Building(BuildingEntity::Building(
                                SerializedBuilding {
                                    id: building.id.clone(),
                                    active_products,
                                },
                            )),
                            statistics: statistics.clone(),
                            under_construction: under_construction.clone(),
                        });
                    }

                    if let Ok(building) = storage_query.get(entity) {
                        state.entities.push(GameEntity {
                            uuid: uuids.get(entity),
                            pos,
                            name: name.clone(),
                            entity: GameEntityType::Building(BuildingEntity::Storage(
                                building.clone(),
                            )),
                            statistics: statistics.clone(),
                            under_construction: under_construction.clone(),
                        });
                    }

                    if let Ok(building) = export_station_query.get(entity) {
                        state.entities.push(GameEntity {
                            uuid: uuids.get(entity),
                            pos,
                            name: name.clone(),
                            entity: GameEntityType::Building(BuildingEntity::ExportStation(
                                building.clone(),
                            )),
                            statistics: statistics.clone(),
                            under_construction: under_construction.clone(),
                        });
                    }

                    if let Ok(building) = delivery_station_query.get(entity) {
                        state.entities.push(GameEntity {
                            uuid: uuids.get(entity),
                            pos,
                            name: name.clone(),
                            entity: GameEntityType::Building(BuildingEntity::DeliveryStation(
                                building.clone(),
                            )),
                            statistics: statistics.clone(),
                            under_construction: under_construction.clone(),
                        });
                    }

                    if let Ok(building) = storage_management_query.get(entity) {
                        state.entities.push(GameEntity {
                            uuid: uuids.get(entity),
                            pos,
                            name: name.clone(),
                            entity: GameEntityType::Building(BuildingEntity::StorageManagement(
                                building.clone(),
                            )),
                            statistics: statistics.clone(),
                            under_construction: under_construction.clone(),
                        });
                    }

                    if let Ok(depot) = depot_query.get(entity) {
                        state.entities.push(GameEntity {
                            uuid: uuids.get(entity),
                            pos,
                            name: name.clone(),
                            entity: GameEntityType::Building(BuildingEntity::Depot(depot.clone())),
                            statistics: statistics.clone(),
                            under_construction: under_construction.clone(),
                        });
                    }

                    if let Ok(building) = street_query.get(entity) {
                        state.entities.push(GameEntity {
                            uuid: uuids.get(entity),
                            pos,
                            name: name.clone(),
                            entity: GameEntityType::Building(BuildingEntity::Street(
                                building.clone(),
                            )),
                            statistics: statistics.clone(),
                            under_construction: under_construction.clone(),
                        });
                    }
                }
            }
        }

        for (entity, car, position) in car_query.iter() {
            let pos = position.position;

            let name = if let Ok(name) = name_query.get(entity) {
                Some(name.clone())
            } else {
                None
            };

            let storage = storage_query.get(entity).unwrap();

            let controller = match &car.controller {
                CarController::UserControlled(controller) => {
                    VehicleController::UserControlled(controller.clone())
                }
                CarController::DepotControlled(depot) => {
                    VehicleController::DepotControlled(uuids.get(depot.depot))
                }
            };

            state.entities.push(GameEntity {
                uuid: uuids.get(entity),
                pos,
                name: name.clone(),
                statistics: None,
                under_construction: None,
                entity: GameEntityType::Vehicle(Vehicle {
                    direction: car.direction,
                    controller,
                    storage: storage.clone(),
                }),
            });
        }

        let path = Path::new(&event.file_name);
        let mut file = File::create(&path).unwrap();

        let _ = file.write_all(serde_yaml::to_string(&state).unwrap().as_bytes());
    }
}
