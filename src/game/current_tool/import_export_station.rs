use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    account::{MaintenanceCost, PurchaseCost},
    assets::{
        resource_specifications::ResourceSpecifications, ClickedTile, Editable, Occupied, Position,
        RequiresUpdate,
    },
    construction::UnderConstruction,
    helper::get_entity::get_entity,
    production::ImportExportStation,
    setup::BUILDING_LAYER_ID,
    statistics::Statistics,
    storage::StorageConsolidator,
};

use super::{SelectedTool, Tool};

pub fn import_export_station_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    resources: Res<ResourceSpecifications>,
) {
    if clicked_tile.dragging {
        return;
    }

    if let Tool::ImportExportStation(direction) = selected_tool.tool {
        if !clicked_tile.occupied_building && clicked_tile.can_build {
            if let Some(pos) = clicked_tile.pos {
                let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

                let station = ImportExportStation {
                    direction,
                    ..Default::default()
                };
                let price = station.price(&resources);

                commands
                    .entity(entity)
                    .insert(station)
                    .insert(StorageConsolidator::default())
                    .insert(Statistics::default())
                    .insert(MaintenanceCost::new_from_cost(price))
                    .insert(UnderConstruction::from_fixed_cost(price))
                    .insert(RequiresUpdate)
                    .insert(Position { position: pos })
                    .insert(Editable)
                    .insert(Occupied);
            }
        }
    }
}
