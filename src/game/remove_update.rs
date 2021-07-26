use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::RequiresUpdate,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

pub fn remove_update(
    mut commands: Commands,
    query: Query<(Entity, &RequiresUpdate)>,
    mut map_query: MapQuery,
) {
    for (entity, update) in query.iter() {
        commands.entity(entity).remove::<RequiresUpdate>();
        // TODO: is it always a building?
        map_query.notify_chunk_for_tile(update.position, MAP_ID, BUILDING_LAYER_ID);
    }
}
