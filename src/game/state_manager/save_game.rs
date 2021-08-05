use std::{fs::File, io::prelude::*, path::Path};

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{
        Building, DeliveryStation, ExportStation, MapSettings, Name, Position, ProductionBuilding,
        Storage, Street,
    },
    car::Car,
    setup::{BUILDING_LAYER_ID, MAP_ID},
    state_manager::{
        BuildingEntity, GameEntity, GameEntityType, GameState, SaveGameEvent, SerializedBuilding,
        Vehicle,
    },
    statistics::Statistics,
};

#[allow(clippy::type_complexity)]
pub fn save_game(
    queries: (
        Query<&Name>,
        Query<(Entity, &Car, &Position)>,
        Query<&Statistics>,
        Query<&Storage>,
        Query<&ExportStation>,
        Query<&DeliveryStation>,
        Query<&Street>,
        Query<(&Building, Option<&ProductionBuilding>)>,
    ),
    map_query: MapQuery,
    mut save_game: EventReader<SaveGameEvent>,
    map_settings: Res<MapSettings>,
) {
    let (
        name_query,
        car_query,
        statistics_query,
        storage_query,
        export_station_query,
        delivery_station_query,
        street_query,
        building_query,
    ) = queries;

    for _ in save_game.iter() {
        let mut state = GameState {
            settings: map_settings.clone(),
            ..Default::default()
        };

        for (entity, car, position) in car_query.iter() {
            let pos = position.position;

            let name = if let Ok(name) = name_query.get(entity) {
                Some(name.clone())
            } else {
                None
            };

            let storage = storage_query.get(entity).unwrap();

            state.entities.push(GameEntity {
                pos,
                name: name.clone(),
                statistics: None,
                entity: GameEntityType::Vehicle(Vehicle {
                    car: car.clone(),
                    storage: storage.clone(),
                }),
            });
        }

        let (_, layer) = map_query.get_layer(MAP_ID, BUILDING_LAYER_ID).unwrap();
        let size = layer.get_layer_size_in_tiles();
        for y in 0..size.y {
            for x in 0..size.x {
                let pos = UVec2::new(x, y);

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

                    if let Ok((building, production_building)) = building_query.get(entity) {
                        let active_product = if let Some(pb) = production_building {
                            pb.active_product
                        } else {
                            0
                        };

                        state.entities.push(GameEntity {
                            pos,
                            name: name.clone(),
                            entity: GameEntityType::Building(BuildingEntity::Building(
                                SerializedBuilding {
                                    id: building.id.clone(),
                                    active_product,
                                },
                            )),
                            statistics: statistics.clone(),
                        });
                    }

                    if let Ok(building) = storage_query.get(entity) {
                        state.entities.push(GameEntity {
                            pos,
                            name: name.clone(),
                            entity: GameEntityType::Building(BuildingEntity::Storage(
                                building.clone(),
                            )),
                            statistics: statistics.clone(),
                        });
                    }

                    if let Ok(building) = export_station_query.get(entity) {
                        state.entities.push(GameEntity {
                            pos,
                            name: name.clone(),
                            entity: GameEntityType::Building(BuildingEntity::ExportStation(
                                building.clone(),
                            )),
                            statistics: statistics.clone(),
                        });
                    }

                    if let Ok(building) = delivery_station_query.get(entity) {
                        state.entities.push(GameEntity {
                            pos,
                            name: name.clone(),
                            entity: GameEntityType::Building(BuildingEntity::DeliveryStation(
                                building.clone(),
                            )),
                            statistics: statistics.clone(),
                        });
                    }

                    if let Ok(building) = street_query.get(entity) {
                        state.entities.push(GameEntity {
                            pos,
                            name: name.clone(),
                            entity: GameEntityType::Building(BuildingEntity::Street(
                                building.clone(),
                            )),
                            statistics: statistics.clone(),
                        });
                    }
                }
            }
        }

        let path = Path::new("world.yaml");
        let mut file = File::create(&path).unwrap();

        let _ = file.write_all(serde_yaml::to_string(&state).unwrap().as_bytes());
    }
}
