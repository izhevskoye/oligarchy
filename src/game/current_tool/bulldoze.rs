use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{ClickedTile, SelectedTool, Street, Tool},
    car::Car,
    setup::{BUILDING_LAYER_ID, MAP_ID, VEHICLE_LAYER_ID},
};

use super::update_neighbor_streets;

pub fn bulldoze(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    street_query: Query<&Street>,
    car_query: Query<(Entity, &Car)>,
) {
    if clicked_tile.dragging {
        return;
    }

    if Tool::Bulldoze == selected_tool.tool {
        if clicked_tile.occupied_vehicle {
            if let Some(pos) = clicked_tile.vehicle_pos {
                let _ = map_query.despawn_tile(&mut commands, pos, MAP_ID, VEHICLE_LAYER_ID);
                map_query.notify_chunk_for_tile(pos, MAP_ID, VEHICLE_LAYER_ID);

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

                update_neighbor_streets(&mut commands, &mut map_query, pos, street_query);
            }
        }
    }
}
