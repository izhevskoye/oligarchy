use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{
        Building, ClickedTile, Occupied, ProductionBuilding, RequiresUpdate, SelectedTool,
        StorageConsolidator, Tool,
    },
    building_specifications::BuildingSpecifications,
    setup::BUILDING_LAYER_ID,
};

use super::get_entity;

pub fn building_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    buildings: Res<BuildingSpecifications>,
) {
    if let Tool::Building(id) = &selected_tool.tool {
        if !clicked_tile.occupied_building {
            if let Some(pos) = clicked_tile.pos {
                let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

                let building = buildings.get(id).unwrap();

                commands
                    .entity(entity)
                    .insert(Building { id: id.clone() })
                    .insert(RequiresUpdate { position: pos })
                    .insert(Occupied);

                if !building.products.is_empty() {
                    commands
                        .entity(entity)
                        .insert(StorageConsolidator::default())
                        .insert(ProductionBuilding {
                            products: building.products.clone(),
                        });
                }
            }
        }
    }
}