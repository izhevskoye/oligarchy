use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    account::{MaintenanceCost, PurchaseCost},
    assets::{
        resource_specifications::ResourceSpecifications, CanDriveOver, ClickedTile, Occupied,
        Position, RequiresUpdate, SelectedTool, Tool,
    },
    construction::UnderConstruction,
    helper::get_entity::get_entity,
    setup::BUILDING_LAYER_ID,
    street::Street,
};

use super::update_neighbor_streets;

pub fn street_placement(
    mut commands: Commands,
    street_query: Query<&Street>,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    resources: Res<ResourceSpecifications>,
) {
    if selected_tool.tool != Tool::Street
        || clicked_tile.occupied_building
        || !clicked_tile.can_build
    {
        return;
    }

    if let Some(pos) = clicked_tile.pos {
        let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

        let price = Street::default().price(&resources);

        commands
            .entity(entity)
            .insert(Street)
            .insert(RequiresUpdate)
            .insert(MaintenanceCost::new_from_cost(price))
            .insert(UnderConstruction::from_fixed_cost(price))
            .insert(Position { position: pos })
            .insert(CanDriveOver)
            .insert(Occupied);

        update_neighbor_streets(&mut commands, &mut map_query, pos, street_query);
    }
}
