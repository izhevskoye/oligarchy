use std::{fs::File, io::prelude::*, path::Path};

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{Building, GameEntity, GameEntityType, GameState, Vehicle};
use crate::game::{
    assets::{BlastFurnace, CokeFurnace, ExportStation, Name, OxygenConverter, Storage, Street},
    car::Car,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

#[allow(clippy::type_complexity)]
pub fn save_game(
    queries: &(
        Query<&Name>,
        Query<&Storage>,
        Query<&CokeFurnace>,
        Query<&BlastFurnace>,
        Query<&ExportStation>,
        Query<&OxygenConverter>,
        Query<&Street>,
        Query<(Entity, &Car)>,
    ),
    map_query: &mut MapQuery,
) {
    let (
        name_query,
        storage_query,
        coke_furnace_query,
        blast_furnace_query,
        export_station_query,
        oxygen_converter_query,
        street_query,
        car_query,
    ) = queries;

    let mut state = GameState::default();

    for (entity, car) in car_query.iter() {
        let pos = car.position;

        let name = if let Ok(name) = name_query.get(entity) {
            Some(name.clone())
        } else {
            None
        };

        let storage = storage_query.get(entity).unwrap();

        state.entities.push(GameEntity {
            pos,
            name: name.clone(),
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

                if let Ok(building) = storage_query.get(entity) {
                    state.entities.push(GameEntity {
                        pos,
                        name: name.clone(),
                        entity: GameEntityType::Building(Building::Storage(building.clone())),
                    });
                }

                if let Ok(building) = coke_furnace_query.get(entity) {
                    state.entities.push(GameEntity {
                        pos,
                        name: name.clone(),
                        entity: GameEntityType::Building(Building::CokeFurnace(building.clone())),
                    });
                }

                if let Ok(building) = blast_furnace_query.get(entity) {
                    state.entities.push(GameEntity {
                        pos,
                        name: name.clone(),
                        entity: GameEntityType::Building(Building::BlastFurnace(building.clone())),
                    });
                }

                if let Ok(building) = export_station_query.get(entity) {
                    state.entities.push(GameEntity {
                        pos,
                        name: name.clone(),
                        entity: GameEntityType::Building(Building::ExportStation(building.clone())),
                    });
                }

                if let Ok(building) = oxygen_converter_query.get(entity) {
                    state.entities.push(GameEntity {
                        pos,
                        name: name.clone(),
                        entity: GameEntityType::Building(Building::OxygenConverter(
                            building.clone(),
                        )),
                    });
                }

                if let Ok(building) = street_query.get(entity) {
                    state.entities.push(GameEntity {
                        pos,
                        name: name.clone(),
                        entity: GameEntityType::Building(Building::Street(building.clone())),
                    });
                }
            }
        }
    }

    let path = Path::new("world.yaml");
    let mut file = File::create(&path).unwrap();

    let _ = file.write_all(serde_yaml::to_string(&state).unwrap().as_bytes());
}
