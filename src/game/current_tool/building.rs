use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    account::{MaintenanceCost, PurchaseCost},
    assets::{
        building_specifications::BuildingSpecifications,
        resource_specifications::ResourceSpecifications, Building, ClickedTile, Editable, Occupied,
        Position, RequiresUpdate, SelectedTool, Tool,
    },
    construction::UnderConstruction,
    helper::get_entity::get_entity,
    production::ProductionBuilding,
    setup::BUILDING_LAYER_ID,
    statistics::Statistics,
    storage::StorageConsolidator,
};

pub fn building_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    buildings: Res<BuildingSpecifications>,
    resources: Res<ResourceSpecifications>,
) {
    if clicked_tile.dragging {
        return;
    }

    if let Tool::Building(id) = &selected_tool.tool {
        if !clicked_tile.occupied_building && clicked_tile.can_build {
            if let Some(pos) = clicked_tile.pos {
                let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

                let building = buildings.get(id).unwrap();

                let price = building.price(&resources);

                commands
                    .entity(entity)
                    .insert(Building { id: id.clone() })
                    .insert(Position { position: pos })
                    .insert(MaintenanceCost::new_from_cost(price))
                    .insert(RequiresUpdate)
                    .insert(UnderConstruction::from_building_specification(building))
                    .insert(Occupied);

                if !building.products.is_empty() {
                    commands
                        .entity(entity)
                        .insert(Statistics::default())
                        .insert(StorageConsolidator::default())
                        .insert(ProductionBuilding {
                            products: building
                                .products
                                .iter()
                                .map(|product| (product.clone(), true))
                                .collect(),
                        })
                        .insert(Editable);
                }
            }
        }
    }
}
