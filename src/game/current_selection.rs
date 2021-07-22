use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::{ClickedTile, CurrentlySelected},
    car::Car,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

pub fn current_selection(
    clicked_tile: Res<ClickedTile>,
    map_query: MapQuery,
    car_query: Query<(Entity, &Car)>,
    mut currently_selected: ResMut<CurrentlySelected>,
) {
    if currently_selected.locked {
        return;
    }

    if clicked_tile.occupied_vehicle {
        if let Some(pos) = clicked_tile.vehicle_pos {
            // TODO: improve
            for (entity, car) in car_query.iter() {
                if car.position == pos {
                    currently_selected.entity = Some(entity);
                    return;
                }
            }
        }
    }

    if clicked_tile.occupied_building {
        if let Some(pos) = clicked_tile.pos {
            if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, BUILDING_LAYER_ID) {
                currently_selected.entity = Some(entity);
            }
        }
    }
}
