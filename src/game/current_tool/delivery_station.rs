use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{
        CanDriveOver, ClickedTile, DeliveryStation, Occupied, Position, RequiresUpdate,
        SelectedTool, Tool,
    },
    setup::BUILDING_LAYER_ID,
    storage::StorageConsolidator,
};

use super::get_entity;

pub fn delivery_station_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if clicked_tile.dragging {
        return;
    }

    if Tool::DeliveryStation == selected_tool.tool && !clicked_tile.occupied_building {
        if let Some(pos) = clicked_tile.pos {
            let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

            commands
                .entity(entity)
                .insert(DeliveryStation)
                .insert(StorageConsolidator::default())
                .insert(RequiresUpdate)
                .insert(Position { position: pos })
                .insert(CanDriveOver)
                .insert(Occupied);
        }
    }
}
