use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{ClickedTile, SelectedTool, Street, Tool},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

use super::update_neighbor_streets;

pub fn bulldoze(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    street_query: Query<&Street>,
) {
    if Tool::Bulldoze == selected_tool.tool && clicked_tile.occupied_building {
        if let Some(pos) = clicked_tile.pos {
            let _ = map_query.despawn_tile(&mut commands, pos, MAP_ID, BUILDING_LAYER_ID);

            update_neighbor_streets(&mut commands, &mut map_query, pos, street_query);
        }
    }
}
