use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{
        ClickedTile, CokeFurnace, Occupied, RequiresUpdate, SelectedTool, StorageConsolidator, Tool,
    },
    constants::MapTile,
};

use super::get_entity;

pub fn coke_furnace_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if Tool::CokeFurnace == selected_tool.tool && !clicked_tile.occupied_building {
        if let Some(pos) = clicked_tile.pos {
            let entity = get_entity(&mut commands, &mut map_query, pos, MapTile::CokeFurnace);

            commands
                .entity(entity)
                .insert(CokeFurnace)
                .insert(StorageConsolidator::default())
                .insert(RequiresUpdate { position: pos })
                .insert(Occupied);
        }
    }
}
