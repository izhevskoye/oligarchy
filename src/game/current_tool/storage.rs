use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    account::{Account, AccountTransaction, MaintenanceCost, PurchaseCost},
    assets::{
        resource_specifications::ResourceSpecifications, ClickedTile, Occupied, Position,
        RequiresUpdate, SelectedTool, Tool,
    },
    setup::{BUILDING_LAYER_ID, MAP_ID},
    storage::{Storage, StorageConsolidator},
};

use super::get_entity;

#[allow(clippy::too_many_arguments)]
pub fn storage_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    consolidator_query: Query<Entity, With<StorageConsolidator>>,
    clicked_tile: Res<ClickedTile>,
    resources: Res<ResourceSpecifications>,
    mut events: EventWriter<AccountTransaction>,
    account: Res<Account>,
) {
    if clicked_tile.dragging {
        return;
    }

    if let Tool::Storage(resource) = &selected_tool.tool {
        if !clicked_tile.occupied_building {
            if let Some(pos) = clicked_tile.pos {
                let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

                let storage = Storage {
                    resource: resource.clone(),
                    ..Default::default()
                };

                let price = storage.price(&resources);
                if account.value < price {
                    return;
                }

                events.send(AccountTransaction { amount: -price });

                commands
                    .entity(entity)
                    .insert(storage)
                    .insert(RequiresUpdate)
                    .insert(Position { position: pos })
                    .insert(MaintenanceCost::new_from_cost(price))
                    .insert(Occupied);

                let neighbors = map_query.get_tile_neighbors(pos, MAP_ID, BUILDING_LAYER_ID);
                for (_pos, neighbor) in neighbors.iter() {
                    if let Some(neighbor) = neighbor {
                        if let Ok(entity) = consolidator_query.get(*neighbor) {
                            commands.entity(entity).insert(RequiresUpdate);
                        }
                    }
                }
            }
        }
    }
}
