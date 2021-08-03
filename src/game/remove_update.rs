use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::{Position, RequiresUpdate},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

pub fn remove_update(
    mut commands: Commands,
    query: Query<(Entity, &Position, Option<&Tile>), With<RequiresUpdate>>,
    mut map_query: MapQuery,
) {
    for (entity, position, tile) in query.iter() {
        commands.entity(entity).remove::<RequiresUpdate>();

        // TODO: is it always a building?
        if tile.is_some() {
            map_query.notify_chunk_for_tile(position.position, MAP_ID, BUILDING_LAYER_ID);
        }
    }
}
