use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    account::{MaintenanceCost, PurchaseCost},
    assets::{
        resource_specifications::ResourceSpecifications, ClickedTile, Occupied, Position,
        RequiresUpdate, SelectedTool, Tool,
    },
    construction::UnderConstruction,
    helper::get_entity::get_entity,
    production::StorageManagement,
    setup::BUILDING_LAYER_ID,
    storage::StorageConsolidator,
};

pub fn storage_management_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    resources: Res<ResourceSpecifications>,
) {
    if clicked_tile.dragging {
        return;
    }

    if Tool::StorageManagement == selected_tool.tool
        && !clicked_tile.occupied_building
        && clicked_tile.can_build
    {
        if let Some(pos) = clicked_tile.pos {
            let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

            let price = StorageManagement::default().price(&resources);

            commands
                .entity(entity)
                .insert(StorageManagement)
                .insert(StorageConsolidator::default())
                .insert(MaintenanceCost::new_from_cost(price))
                .insert(UnderConstruction::from_fixed_cost(price))
                .insert(RequiresUpdate)
                .insert(Position { position: pos })
                .insert(Occupied);
        }
    }
}
