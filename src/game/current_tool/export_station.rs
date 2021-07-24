use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{
        ClickedTile, ExportStation, Occupied, RequiresUpdate, Resource, SelectedTool,
        StorageConsolidator, Tool,
    },
    setup::BUILDING_LAYER_ID,
};

use super::get_entity;

pub fn export_station_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if Tool::ExportStation == selected_tool.tool && !clicked_tile.occupied_building {
        if let Some(pos) = clicked_tile.pos {
            let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

            commands
                .entity(entity)
                .insert(ExportStation {
                    goods: vec![
                        Resource::Steel,
                        Resource::Coal,
                        Resource::Coke,
                        Resource::Limestone,
                        Resource::Iron,
                        Resource::IronOre,
                    ],
                })
                .insert(StorageConsolidator::default())
                .insert(RequiresUpdate { position: pos })
                .insert(Occupied);
        }
    }
}
