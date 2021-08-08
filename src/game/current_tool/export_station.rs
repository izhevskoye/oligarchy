use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    account::{Account, AccountTransaction, PurchaseCost},
    assets::{
        ClickedTile, Editable, ExportStation, Occupied, Position, RequiresUpdate, SelectedTool,
        Tool,
    },
    resource_specifications::ResourceSpecifications,
    setup::BUILDING_LAYER_ID,
    statistics::Statistics,
    storage::StorageConsolidator,
};

use super::get_entity;

pub fn export_station_placement(
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

    if Tool::ExportStation == selected_tool.tool && !clicked_tile.occupied_building {
        if let Some(pos) = clicked_tile.pos {
            let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

            let price = ExportStation::default().price(&resources);
            if account.value < price {
                return;
            }

            events.send(AccountTransaction { amount: -price });

            commands
                .entity(entity)
                .insert(ExportStation { goods: vec![] })
                .insert(StorageConsolidator::default())
                .insert(Statistics::default())
                .insert(RequiresUpdate)
                .insert(Position { position: pos })
                .insert(Editable)
                .insert(Occupied);
        }
    }
}
