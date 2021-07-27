use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::RequiresUpdate,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

pub fn remove_update(
    mut commands: Commands,
    query: Query<(Entity, &RequiresUpdate, Option<&Tile>)>,
    mut map_query: MapQuery,
) {
    for (entity, update, tile) in query.iter() {
        commands.entity(entity).remove::<RequiresUpdate>();

        // TODO: is it always a building?
        if tile.is_some() {
            map_query.notify_chunk_for_tile(update.position, MAP_ID, BUILDING_LAYER_ID);
        }
    }
}
