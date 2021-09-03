use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    account::{MaintenanceCost, PurchaseCost},
    assets::{
        resource_specifications::ResourceSpecifications, CanDriveOver, ClickedTile, Occupied,
        Position, RequiresUpdate,
    },
    construction::UnderConstruction,
    helper::get_entity::get_entity,
    setup::BUILDING_LAYER_ID,
    street::{Path, Street},
};

use super::{update_neighbor_streets, SelectedTool, Tool};

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

        let street = Street::default();
        let price = street.price(&resources);

        commands
            .entity(entity)
            .insert(street)
            .insert(RequiresUpdate)
            .insert(MaintenanceCost::new_from_cost(price))
            .insert(UnderConstruction::from_fixed_cost(price))
            .insert(Position { position: pos })
            .insert(CanDriveOver)
            .insert(Occupied);

        update_neighbor_streets(&mut commands, &mut map_query, pos, street_query);
    }
}

pub fn path_placement(
    mut commands: Commands,
    street_query: Query<&Path>,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    resources: Res<ResourceSpecifications>,
) {
    if selected_tool.tool != Tool::Path || clicked_tile.occupied_building || !clicked_tile.can_build
    {
        return;
    }

    if let Some(pos) = clicked_tile.pos {
        let entity = get_entity(&mut commands, &mut map_query, pos, BUILDING_LAYER_ID);

        let street = Path::default();
        let price = street.price(&resources);

        commands
            .entity(entity)
            .insert(street)
            .insert(RequiresUpdate)
            .insert(MaintenanceCost::new_from_cost(price))
            .insert(UnderConstruction::from_fixed_cost(price))
            .insert(Position { position: pos })
            .insert(CanDriveOver)
            .insert(Occupied);

        update_neighbor_streets(&mut commands, &mut map_query, pos, street_query);
    }
}
