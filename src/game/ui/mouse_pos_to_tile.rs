use bevy::{prelude::*, render::camera::Camera};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiContext;

use crate::game::{
    assets::{BlockedForBuilding, ClickedTile, MapSettings, Occupied, Position},
    car::Car,
    constants::{CHUNK_SIZE, TILE_SIZE},
    setup::{BUILDING_LAYER_ID, GROUND_LAYER_ID, MAP_ID},
};

fn eval_pos(map_settings: &Res<MapSettings>, x: f32, y: f32, modifier: i32) -> Option<UVec2> {
    let tile_size = TILE_SIZE / modifier as f32;
    let x = (x / tile_size).floor() as i32;
    let y = (y / tile_size).floor() as i32;

    if x < 0
        || x >= (map_settings.width * CHUNK_SIZE - 1) as i32 * modifier
        || y < 0
        || y >= (map_settings.height * CHUNK_SIZE - 1) as i32 * modifier
    {
        return None;
    }

    Some(UVec2::new(x as u32, y as u32))
}

pub fn mouse_pos_to_tile(
    egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    mut clicked_tile: ResMut<ClickedTile>,
    queries: (
        Query<&Transform, With<Camera>>,
        Query<&Occupied>,
        Query<&Position, With<Car>>,
        Query<&BlockedForBuilding>,
    ),
    map_query: MapQuery,
    map_settings: Res<MapSettings>,
) {
    let (transform, occupied_query, car_query, blocked_query) = queries;

    clicked_tile.pos = None;
    clicked_tile.occupied_building = false;
    clicked_tile.vehicle_pos = None;
    clicked_tile.occupied_vehicle = false;

    let transform = transform.single().unwrap();
    if egui_context.ctx().wants_pointer_input() {
        return;
    }

    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }

    let win = windows.get_primary().expect("no primary window");

    if let Some(pos) = win.cursor_position() {
        let x = (pos.x - (win.width() / 2.0)) * transform.scale.x + transform.translation.x;
        let y = (pos.y - (win.height() / 2.0)) * transform.scale.y + transform.translation.y;

        clicked_tile.pos = eval_pos(&map_settings, x, y, 1);
        clicked_tile.vehicle_pos = eval_pos(&map_settings, x, y, 2);
        clicked_tile.dragging = !mouse_input.just_pressed(MouseButton::Left);

        if let Some(pos) = clicked_tile.pos {
            clicked_tile.occupied_building =
                if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, BUILDING_LAYER_ID) {
                    occupied_query.get(entity).is_ok()
                } else {
                    false
                };

            clicked_tile.can_build =
                if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, GROUND_LAYER_ID) {
                    blocked_query.get(entity).is_err()
                } else {
                    true
                };
        }

        if let Some(vehicle_pos) = clicked_tile.vehicle_pos {
            clicked_tile.occupied_vehicle = car_query
                .iter()
                .any(|position| position.position == vehicle_pos);
        }
    }
}
