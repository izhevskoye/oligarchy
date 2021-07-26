use std::{fs::File, io::prelude::*, path::Path};

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{Editable, Occupied, RequiresUpdate, StorageConsolidator},
    car::Car,
    constants::{CHUNK_SIZE, MAP_HEIGHT, MAP_WIDTH},
    setup::{BUILDING_LAYER_ID, MAP_ID, VEHICLE_LAYER_ID},
    state_manager::{Building, GameEntityType, GameState},
};

pub fn load_game(
    commands: &mut Commands,
    map_query: &mut MapQuery,
    car_query: &Query<(Entity, &Car)>,
) {
    let path = Path::new("world.yaml");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            log::error!("Could not read file: {}", why);
            return;
        }
    };

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let state: Result<GameState, serde_yaml::Error> = serde_yaml::from_str(&content);

    match state {
        Ok(state) => {
            reset_state(commands, map_query, car_query);
            load_state(commands, map_query, state);
        }
        Err(why) => log::error!("Could not load state: {}", why),
    }
}

fn reset_state(
    commands: &mut Commands,
    map_query: &mut MapQuery,
    car_query: &Query<(Entity, &Car)>,
) {
    map_query.despawn_layer_tiles(commands, MAP_ID, BUILDING_LAYER_ID);
    map_query.despawn_layer_tiles(commands, MAP_ID, VEHICLE_LAYER_ID);

    for (entity, _car) in car_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            map_query.notify_chunk_for_tile(
                UVec2::new(x * CHUNK_SIZE, y * CHUNK_SIZE),
                MAP_ID,
                BUILDING_LAYER_ID,
            );

            for i in 1..2 {
                map_query.notify_chunk_for_tile(
                    UVec2::new(x * CHUNK_SIZE * i, y * CHUNK_SIZE * i),
                    MAP_ID,
                    VEHICLE_LAYER_ID,
                );
            }
        }
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
                    .insert(vehicle.storage)
                    .insert(Editable);
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
                        // TODO: Building
                        Building::Street(c) => {
                            commands.entity(entity).insert(c);
                        }
                        Building::Storage(c) => {
                            commands.entity(entity).insert(c);
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
