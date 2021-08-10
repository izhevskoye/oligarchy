use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    account::{Account, AccountTransaction, PurchaseCost},
    assets::{
        building_specifications::BuildingSpecifications,
        resource_specifications::ResourceSpecifications, Building, ClickedTile, Editable,
        MaintenanceCost, Occupied, Position, ProductionBuilding, RequiresUpdate, SelectedTool,
        Tool,
    },
    setup::BUILDING_LAYER_ID,
    statistics::Statistics,
    storage::StorageConsolidator,
};

use super::get_entity;

#[allow(clippy::too_many_arguments)]
pub fn building_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    buildings: Res<BuildingSpecifications>,
    resources: Res<ResourceSpecifications>,
    mut events: EventWriter<AccountTransaction>,
    account: Res<Account>,
) {
    if clicked_tile.dragging {
        return;
    }

    if let Tool::Building(id) = &selected_tool.tool {
        if !clicked_tile.occupied_building {
            if let Some(pos) = clicked_tile.pos {
                let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

                let building = buildings.get(id).unwrap();

                let price = building.price(&resources);
                if account.value < price {
                    return;
                }

                events.send(AccountTransaction { amount: -price });

                commands
                    .entity(entity)
                    .insert(Building { id: id.clone() })
                    .insert(Position { position: pos })
                    .insert(MaintenanceCost::new_from_cost(price))
                    .insert(RequiresUpdate)
                    .insert(Occupied);

                if !building.products.is_empty() {
                    commands
                        .entity(entity)
                        .insert(Statistics::default())
                        .insert(StorageConsolidator::default())
                        .insert(ProductionBuilding {
                            products: building.products.clone(),
                            active_product: 0,
                        })
                        .insert(Editable);
                }
            }
        }
    }
}
