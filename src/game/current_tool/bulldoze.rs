use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{ClickedTile, Position, RemovedBuildingEvent, SelectedTool, Tool},
    car::Car,
    constants::MapTile,
    production::Idle,
    setup::{BUILDING_LAYER_ID, GROUND_LAYER_ID, MAP_ID},
    statistics::{StatisticTracker, Statistics},
    street::Street,
};

use super::update_neighbor_streets;

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn bulldoze(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    street_query: Query<&Street>,
    statistics_query: Query<&Statistics>,
    car_query: Query<(Entity, &Position), With<Car>>,
    idle_query: Query<&Idle>,
    mut deleted_export_statistics: ResMut<StatisticTracker>,
    mut tile_query: Query<&mut Tile>,
    mut removed_events: EventWriter<RemovedBuildingEvent>,
) {
    if clicked_tile.dragging {
        return;
    }

    if Tool::Bulldoze == selected_tool.tool {
        if clicked_tile.occupied_vehicle {
            if let Some(pos) = clicked_tile.vehicle_pos {
                for (entity, position) in car_query.iter() {
                    if position.position == pos {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        } else if clicked_tile.occupied_building {
            if let Some(pos) = clicked_tile.pos {
                if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, BUILDING_LAYER_ID) {
                    if let Ok(idle) = idle_query.get(entity) {
                        if let Some(entity) = idle.entity {
                            commands.entity(entity).despawn_recursive();
                        }
                    }

                    if let Ok(statistics) = statistics_query.get(entity) {
                        deleted_export_statistics.merge(&statistics.export);
                    }
                }

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
