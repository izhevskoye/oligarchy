use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{
        ClickedTile, Occupied, RequiresUpdate, SelectedTool, Storage, StorageConsolidator, Tool,
    },
    constants::MapTile,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

use super::get_entity;

pub fn storage_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    consolidator_query: Query<Entity, With<StorageConsolidator>>,
    clicked_tile: Res<ClickedTile>,
) {
    if let Tool::Storage(resource) = selected_tool.tool {
        if !clicked_tile.occupied_building {
            if let Some(pos) = clicked_tile.pos {
                let entity = get_entity(
                    &mut commands,
                    &mut map_query,
                    pos,
                    MapTile::Storage,
                    BUILDING_LAYER_ID,
                );

                commands
                    .entity(entity)
                    .insert(Storage {
                        resource,
                        amount: 0,
                        capacity: 20,
                    })
                    .insert(Occupied);

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
}
