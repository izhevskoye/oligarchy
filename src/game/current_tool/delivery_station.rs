use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    account::{Account, AccountTransaction, PurchaseCost},
    assets::{
        resource_specifications::ResourceSpecifications, CanDriveOver, ClickedTile,
        DeliveryStation, MaintenanceCost, Occupied, Position, RequiresUpdate, SelectedTool, Tool,
    },
    setup::BUILDING_LAYER_ID,
    storage::StorageConsolidator,
};

use super::get_entity;

#[allow(clippy::too_many_arguments)]
pub fn delivery_station_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    resources: Res<ResourceSpecifications>,
    mut events: EventWriter<AccountTransaction>,
    account: Res<Account>,
) {
    if clicked_tile.dragging {
        return;
    }

    if Tool::DeliveryStation == selected_tool.tool && !clicked_tile.occupied_building {
        if let Some(pos) = clicked_tile.pos {
            let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

            let price = DeliveryStation::default().price(&resources);
            if account.value < price {
                return;
            }

            events.send(AccountTransaction { amount: -price });

            commands
                .entity(entity)
                .insert(DeliveryStation)
                .insert(StorageConsolidator::default())
                .insert(MaintenanceCost::new_from_cost(price))
                .insert(RequiresUpdate)
                .insert(Position { position: pos })
                .insert(CanDriveOver)
                .insert(Occupied);
        }
    }
}
