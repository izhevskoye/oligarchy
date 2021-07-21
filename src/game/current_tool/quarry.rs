use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{
        ClickedTile, Occupied, Quarry, RequiresUpdate, Resource, SelectedTool, StorageConsolidator,
        Tool,
    },
    constants::MapTile,
};

use super::get_entity;

pub fn quarry_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if let Tool::Quarry(resource) = selected_tool.tool {
        if !clicked_tile.occupied_building {
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
                    .insert(RequiresUpdate { position: pos })
                    .insert(Occupied);
            }
        }
    }
}
