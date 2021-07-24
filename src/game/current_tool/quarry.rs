use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{
        ClickedTile, Occupied, Quarry, RequiresUpdate, SelectedTool, StorageConsolidator, Tool,
    },
    setup::BUILDING_LAYER_ID,
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
                let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

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
