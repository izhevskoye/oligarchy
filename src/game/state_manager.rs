use std::{fs::File, io::prelude::*, path::Path};

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};
use serde::{Deserialize, Serialize};

use super::{
    assets::{
        BlastFurnace, CokeFurnace, ExportStation, Name, Occupied, OxygenConverter, Quarry,
        RequiresUpdate, Storage, StorageConsolidator, Street,
    },
    car::Car,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

#[derive(Serialize, Deserialize)]
enum Building {
    Quarry(Quarry),
    Storage(Storage),
    CokeFurnace(CokeFurnace),
    BlastFurnace(BlastFurnace),
    ExportStation(ExportStation),
    OxygenConverter(OxygenConverter),
    Street(Street),
}

#[derive(Serialize, Deserialize)]
struct Vehicle {
    car: Car,
    storage: Storage,
}

#[derive(Serialize, Deserialize)]
enum GameEntityType {
    Building(Building),
    Vehicle(Vehicle),
}

#[derive(Serialize, Deserialize)]
struct GameEntity {
    pub pos: UVec2,
    pub entity: GameEntityType,
    pub name: Option<Name>,
}

#[derive(Default, Serialize, Deserialize)]
struct GameState {
    pub entities: Vec<GameEntity>,
}

fn save_game(
    queries: (
        Query<&Name>,
        Query<&Quarry>,
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
        quarry_query,
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

                if let Ok(building) = quarry_query.get(entity) {
                    state.entities.push(GameEntity {
                        pos,
                        name: name.clone(),
                        entity: GameEntityType::Building(Building::Quarry(building.clone())),
                    });
                }

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

fn load_game(commands: &mut Commands, map_query: &mut MapQuery) {
    let path = Path::new("world.yaml");
    let mut file = File::open(&path).unwrap();

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let state: Result<GameState, serde_yaml::Error> = serde_yaml::from_str(&content);

    match state {
        Ok(state) => load_state(commands, map_query, state),
        Err(why) => log::error!("Could not load state: {}", why),
    }
}

fn load_state(commands: &mut Commands, map_query: &mut MapQuery, state: GameState) {
    for game_entity in state.entities {
        let tile = Tile {
            visible: false,
            ..Default::default()
        };

        match game_entity.entity {
            GameEntityType::Vehicle(vehicle) => {
                commands
                    .spawn()
                    .insert(RequiresUpdate {
                        position: game_entity.pos,
                    })
                    .insert(vehicle.car)
                    .insert(vehicle.storage);
            }
            GameEntityType::Building(building) => {
                if let Ok(entity) =
                    map_query.set_tile(commands, game_entity.pos, tile, MAP_ID, BUILDING_LAYER_ID)
                {
                    commands
                        .entity(entity)
                        .insert(RequiresUpdate {
                            position: game_entity.pos,
                        })
                        .insert(Occupied);

                    match building {
                        Building::Quarry(c) => {
                            commands
                                .entity(entity)
                                .insert(c)
                                .insert(StorageConsolidator::default());
                        }
                        Building::Street(c) => {
                            commands.entity(entity).insert(c);
                        }
                        Building::OxygenConverter(c) => {
                            commands
                                .entity(entity)
                                .insert(c)
                                .insert(StorageConsolidator::default());
                        }
                        Building::Storage(c) => {
                            commands.entity(entity).insert(c);
                        }
                        Building::BlastFurnace(c) => {
                            commands
                                .entity(entity)
                                .insert(c)
                                .insert(StorageConsolidator::default());
                        }
                        Building::CokeFurnace(c) => {
                            commands
                                .entity(entity)
                                .insert(c)
                                .insert(StorageConsolidator::default());
                        }
                        Building::ExportStation(c) => {
                            commands
                                .entity(entity)
                                .insert(c)
                                .insert(StorageConsolidator::default());
                        }
                    }
                }
            }
        }
    }
}

pub fn save_ui(
    mut commands: Commands,
    queries: (
        Query<&Name>,
        Query<&Quarry>,
        Query<&Storage>,
        Query<&CokeFurnace>,
        Query<&BlastFurnace>,
        Query<&ExportStation>,
        Query<&OxygenConverter>,
        Query<&Street>,
        Query<(Entity, &Car)>,
    ),
    egui_context: ResMut<EguiContext>,
    mut map_query: MapQuery,
) {
    egui::Window::new("Game")
        .anchor(Align2::RIGHT_BOTTOM, [-10.0, -10.0])
        .show(egui_context.ctx(), |ui| {
            if ui.button("save").clicked() {
                save_game(queries, &mut map_query);
            }
            if ui.button("load").clicked() {
                load_game(&mut commands, &mut map_query);
            }
        });
}
