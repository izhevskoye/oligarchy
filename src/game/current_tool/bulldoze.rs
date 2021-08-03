use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{ClickedTile, RemovedBuildingEvent, SelectedTool, Street, Tool},
    car::Car,
    constants::MapTile,
    setup::{BUILDING_LAYER_ID, GROUND_LAYER_ID, MAP_ID},
};

use super::update_neighbor_streets;

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn bulldoze(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    street_query: Query<&Street>,
    car_query: Query<(Entity, &Car)>,
    mut tile_query: Query<&mut Tile>,
    mut removed_events: EventWriter<RemovedBuildingEvent>,
) {
    if clicked_tile.dragging {
        return;
    }

    if Tool::Bulldoze == selected_tool.tool {
        if clicked_tile.occupied_vehicle {
            if let Some(pos) = clicked_tile.vehicle_pos {
                for (entity, car) in car_query.iter() {
                    if car.position == pos {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        } else if clicked_tile.occupied_building {
            if let Some(pos) = clicked_tile.pos {
                let _ = map_query.despawn_tile(&mut commands, pos, MAP_ID, BUILDING_LAYER_ID);
                map_query.notify_chunk_for_tile(pos, MAP_ID, BUILDING_LAYER_ID);

                removed_events.send(RemovedBuildingEvent { position: pos });

                update_neighbor_streets(&mut commands, &mut map_query, pos, street_query);

                let entity = map_query
                    .get_tile_entity(pos, MAP_ID, GROUND_LAYER_ID)
                    .unwrap();

                let mut tile = tile_query.get_mut(entity).unwrap();
                tile.texture_index = MapTile::Ground as u16;

                map_query.notify_chunk_for_tile(pos, MAP_ID, GROUND_LAYER_ID);
            }
        }
    }
}
