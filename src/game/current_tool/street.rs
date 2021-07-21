use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{ClickedTile, Occupied, RequiresUpdate, SelectedTool, Street, Tool},
    constants::MapTile,
    setup::BUILDING_LAYER_ID,
};

use super::{get_entity, update_neighbor_streets};

pub fn street_placement(
    mut commands: Commands,
    street_query: Query<&Street>,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if selected_tool.tool != Tool::Street || clicked_tile.occupied_building {
        return;
    }

    if let Some(pos) = clicked_tile.pos {
        let entity = get_entity(
            &mut commands,
            &mut map_query,
            pos,
            MapTile::StreetNorthEastSouthWest,
            BUILDING_LAYER_ID,
        );

        commands
            .entity(entity)
            .insert(Street)
            .insert(RequiresUpdate { position: pos })
            .insert(Occupied);

        update_neighbor_streets(&mut commands, &mut map_query, pos, street_query);
    }
}
