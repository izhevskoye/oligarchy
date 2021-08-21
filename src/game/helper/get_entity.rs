use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::setup::MAP_ID;

pub fn get_entity(
    commands: &mut Commands,
    map_query: &mut MapQuery,
    pos: UVec2,
    layer_id: u16,
) -> Entity {
    if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, layer_id) {
        entity
    } else {
        map_query.notify_chunk_for_tile(pos, MAP_ID, layer_id);
        map_query
            .set_tile(
                commands,
                pos,
                Tile {
                    visible: false,
                    ..Default::default()
                },
                MAP_ID,
                layer_id,
            )
            .unwrap()
    }
}
