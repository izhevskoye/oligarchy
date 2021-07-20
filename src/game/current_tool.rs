use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::assets::Resource;

use super::{
    assets::{
        BlastFurnace, ClickedTile, CokeFurnace, ExportStation, OxygenConverter, Quarry,
        RequiresUpdate, SelectedTool, Storage, StorageConsolidator, Street, Tool,
    },
    constants::MapTile,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

fn get_entity(
    commands: &mut Commands,
    map_query: &mut MapQuery,
    pos: UVec2,
    map_tile: MapTile,
) -> Entity {
    if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, BUILDING_LAYER_ID) {
        entity
    } else {
        map_query.notify_chunk_for_tile(pos, MAP_ID, BUILDING_LAYER_ID);
        map_query
            .set_tile(
                commands,
                pos,
                Tile {
                    texture_index: map_tile as u16,
                    ..Default::default()
                },
                MAP_ID,
                BUILDING_LAYER_ID,
            )
            .unwrap()
    }
}

pub fn street_placement(
    mut commands: Commands,
    street_query: Query<&Street>,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if selected_tool.tool != Tool::Street {
        return;
    }

    if let Some(pos) = clicked_tile.pos {
        let entity = get_entity(
            &mut commands,
            &mut map_query,
            pos,
            MapTile::StreetNorthEastSouthWest,
        );

        commands
            .entity(entity)
            .insert(Street)
            .insert(RequiresUpdate { position: pos });

        let neighbors = map_query.get_tile_neighbors(pos, MAP_ID, BUILDING_LAYER_ID);
        for (pos, neighbor) in neighbors[0..4].iter() {
            if let Some(neighbor) = neighbor {
                if street_query.get(*neighbor).is_ok() {
                    commands.entity(*neighbor).insert(RequiresUpdate {
                        position: pos.as_u32(),
                    });
                }
            }
        }
    }
}

pub fn storage_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    consolidator_query: Query<Entity, With<StorageConsolidator>>,
    clicked_tile: Res<ClickedTile>,
) {
    if let Tool::Storage(resource) = selected_tool.tool {
        if let Some(pos) = clicked_tile.pos {
            let entity = get_entity(&mut commands, &mut map_query, pos, MapTile::Storage);

            commands.entity(entity).insert(Storage {
                resource,
                amount: 0,
                capacity: 20,
            });

            let neighbors = map_query.get_tile_neighbors(pos, MAP_ID, BUILDING_LAYER_ID);
            for (pos, neighbor) in neighbors.iter() {
                if let Some(neighbor) = neighbor {
                    if let Ok(entity) = consolidator_query.get(*neighbor) {
                        commands.entity(entity).insert(RequiresUpdate {
                            position: pos.as_u32(),
                        });
                    }
                }
            }
        }
    }
}

pub fn quarry_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if let Tool::Quarry(resource) = selected_tool.tool {
        if let Some(pos) = clicked_tile.pos {
            let tile = match resource {
                Resource::Coal => MapTile::CoalQuarry,
                Resource::Limestone => MapTile::LimestoneQuarry,
                Resource::IronOre => MapTile::IronOreQuarry,
                _ => panic!("Invalid Quarry type"),
            };
            let entity = get_entity(&mut commands, &mut map_query, pos, tile);

            commands
                .entity(entity)
                .insert(Quarry { resource })
                .insert(StorageConsolidator::default())
                .insert(RequiresUpdate { position: pos });
        }
    }
}

pub fn coke_furnace_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if let Tool::CokeFurnace = selected_tool.tool {
        if let Some(pos) = clicked_tile.pos {
            let entity = get_entity(&mut commands, &mut map_query, pos, MapTile::CokeFurnace);

            commands
                .entity(entity)
                .insert(CokeFurnace)
                .insert(StorageConsolidator::default())
                .insert(RequiresUpdate { position: pos });
        }
    }
}

pub fn blast_furnace_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if let Tool::BlastFurnace = selected_tool.tool {
        if let Some(pos) = clicked_tile.pos {
            let entity = get_entity(&mut commands, &mut map_query, pos, MapTile::BlastFurnace);

            commands
                .entity(entity)
                .insert(BlastFurnace)
                .insert(StorageConsolidator::default())
                .insert(RequiresUpdate { position: pos });
        }
    }
}

pub fn oxygen_converter_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if let Tool::OxygenConverter = selected_tool.tool {
        if let Some(pos) = clicked_tile.pos {
            let entity = get_entity(&mut commands, &mut map_query, pos, MapTile::OxygenConverter);

            commands
                .entity(entity)
                .insert(OxygenConverter)
                .insert(StorageConsolidator::default())
                .insert(RequiresUpdate { position: pos });
        }
    }
}

pub fn export_station_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if let Tool::ExportStation = selected_tool.tool {
        if let Some(pos) = clicked_tile.pos {
            let entity = get_entity(&mut commands, &mut map_query, pos, MapTile::ExportStation);

            commands
                .entity(entity)
                .insert(ExportStation {
                    goods: vec![Resource::Steel],
                })
                .insert(StorageConsolidator::default())
                .insert(RequiresUpdate { position: pos });
        }
    }
}
