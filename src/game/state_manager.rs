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
struct GameEntity {
    pub pos: UVec2,
    pub building: Building,
    pub name: Option<Name>,
}

#[derive(Default, Serialize, Deserialize)]
struct GameState {
    pub buildings: Vec<GameEntity>,
    // TODO: vehicles!
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
    ),
    map_query: &mut MapQuery,
) {
    let (_, layer) = map_query.get_layer(MAP_ID, BUILDING_LAYER_ID).unwrap();
    let size = layer.get_layer_size_in_tiles();

    let (
        name_query,
        quarry_query,
        storage_query,
        coke_furnace_query,
        blast_furnace_query,
        export_station_query,
        oxygen_converter_query,
        street_query,
    ) = queries;

    let mut state = GameState::default();

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
                    state.buildings.push(GameEntity {
                        pos,
                        name: name.clone(),
                        building: Building::Quarry(building.clone()),
                    });
                }

                if let Ok(building) = storage_query.get(entity) {
                    state.buildings.push(GameEntity {
                        pos,
                        name: name.clone(),
                        building: Building::Storage(building.clone()),
                    });
                }

                if let Ok(building) = coke_furnace_query.get(entity) {
                    state.buildings.push(GameEntity {
                        pos,
                        name: name.clone(),
                        building: Building::CokeFurnace(building.clone()),
                    });
                }

                if let Ok(building) = blast_furnace_query.get(entity) {
                    state.buildings.push(GameEntity {
                        pos,
                        name: name.clone(),
                        building: Building::BlastFurnace(building.clone()),
                    });
                }

                if let Ok(building) = export_station_query.get(entity) {
                    state.buildings.push(GameEntity {
                        pos,
                        name: name.clone(),
                        building: Building::ExportStation(building.clone()),
                    });
                }

                if let Ok(building) = oxygen_converter_query.get(entity) {
                    state.buildings.push(GameEntity {
                        pos,
                        name: name.clone(),
                        building: Building::OxygenConverter(building.clone()),
                    });
                }

                if let Ok(building) = street_query.get(entity) {
                    state.buildings.push(GameEntity {
                        pos,
                        name: name.clone(),
                        building: Building::Street(building.clone()),
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

    let state: GameState = serde_yaml::from_str(&content).unwrap();

    for game_entity in state.buildings {
        let tile = Tile {
            visible: false,
            ..Default::default()
        };

        // TODO: not needed?
        // map_query.notify_chunk_for_tile(game_entity.pos, MAP_ID, BUILDING_LAYER_ID);

        if let Ok(entity) =
            map_query.set_tile(commands, game_entity.pos, tile, MAP_ID, BUILDING_LAYER_ID)
        {
            commands
                .entity(entity)
                .insert(RequiresUpdate {
                    position: game_entity.pos,
                })
                .insert(Occupied);

            match game_entity.building {
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
