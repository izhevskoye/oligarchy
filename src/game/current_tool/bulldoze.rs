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
            // make sure tile is set
            if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, BUILDING_LAYER_ID) {
                commands.entity(entity).despawn_recursive();
                commands.spawn().insert(Tile::default());
                map_query.notify_chunk_for_tile(pos, MAP_ID, BUILDING_LAYER_ID);

                update_neighbor_streets(&mut commands, &mut map_query, pos, street_query);
            }
        }
    }
}
