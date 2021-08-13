use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    account::{Account, AccountTransaction, MaintenanceCost, PurchaseCost},
    assets::{
        resource_specifications::ResourceSpecifications, CanDriveOver, ClickedTile, Occupied,
        Position, RequiresUpdate, SelectedTool, Tool,
    },
    setup::BUILDING_LAYER_ID,
    street::Street,
};

use super::{get_entity, update_neighbor_streets};

#[allow(clippy::too_many_arguments)]
pub fn street_placement(
    mut commands: Commands,
    street_query: Query<&Street>,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    resources: Res<ResourceSpecifications>,
    mut events: EventWriter<AccountTransaction>,
    account: Res<Account>,
) {
    if selected_tool.tool != Tool::Street || clicked_tile.occupied_building {
        return;
    }

    if let Some(pos) = clicked_tile.pos {
        let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

        let price = Street::default().price(&resources);
        if account.value < price {
            return;
        }

        events.send(AccountTransaction { amount: -price });

        commands
            .entity(entity)
            .insert(Street)
            .insert(RequiresUpdate)
            .insert(MaintenanceCost::new_from_cost(price))
            .insert(Position { position: pos })
            .insert(CanDriveOver)
            .insert(Occupied);

        update_neighbor_streets(&mut commands, &mut map_query, pos, street_query);
    }
}
