use bevy::{prelude::*, render::camera::Camera};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiContext;

use super::{
    assets::CurrentlySelected,
    constants::{CHUNK_SIZE, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

pub fn current_selection(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    query: Query<&Transform, With<Camera>>,
    egui_context: ResMut<EguiContext>,
    map_query: MapQuery,
    mut currently_selected: ResMut<CurrentlySelected>,
) {
    for transform in query.iter() {
        if egui_context.ctx().wants_pointer_input() {
            return;
        }

        if !mouse_input.pressed(MouseButton::Left) {
            return;
        }
        let win = windows.get_primary().expect("no primary window");

        let pos = win.cursor_position().unwrap();
        let x = (pos.x - (win.width() / 2.0)) * transform.scale.x + transform.translation.x;
        let y = (pos.y - (win.height() / 2.0)) * transform.scale.y + transform.translation.y;

        let x = (x / TILE_SIZE).floor() as i32;
        let y = (y / TILE_SIZE).floor() as i32;

        if x < 0
            || x >= (MAP_WIDTH * CHUNK_SIZE - 1) as i32
            || y < 0
            || y >= (MAP_HEIGHT * CHUNK_SIZE - 1) as i32
        {
            return;
        }
        let pos = UVec2::new(x as u32, y as u32);

        let entity = map_query.get_tile_entity(pos, MAP_ID, BUILDING_LAYER_ID);

        if let Ok(entity) = entity {
            currently_selected.entity = Some(entity);
        } else {
            currently_selected.entity = None;
        }
    }
}
