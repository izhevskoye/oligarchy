use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::{ClickedTile, Position},
    car::Car,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

#[derive(Default)]
pub struct CurrentlySelected {
    pub entity: Option<Entity>,
    pub locked: bool,
    pub editing: bool,
    pub renaming: bool,
}

pub fn current_selection(
    clicked_tile: Res<ClickedTile>,
    map_query: MapQuery,
    car_query: Query<(Entity, &Position), With<Car>>,
    mut currently_selected: ResMut<CurrentlySelected>,
) {
    if currently_selected.locked || currently_selected.editing {
        return;
    }

    if clicked_tile.occupied_vehicle {
        if let Some(pos) = clicked_tile.vehicle_pos {
            // TODO: improve
            for (entity, position) in car_query.iter() {
                if position.position == pos {
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
