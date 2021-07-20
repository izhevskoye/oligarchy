use bevy::{prelude::*, render::camera::Camera};
use bevy_egui::EguiContext;

use crate::game::{
    assets::ClickedTile,
    constants::{CHUNK_SIZE, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE},
};

pub fn mouse_pos_to_tile(
    egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    query: Query<&Transform, With<Camera>>,
    mouse_input: Res<Input<MouseButton>>,
    mut clicked_tile: ResMut<ClickedTile>,
) {
    let transform = query.single().unwrap();
    if egui_context.ctx().wants_pointer_input() {
        clicked_tile.pos = None;
        return;
    }

    if !mouse_input.pressed(MouseButton::Left) {
        clicked_tile.pos = None;
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
        clicked_tile.pos = None;
        return;
    }

    clicked_tile.pos = Some(UVec2::new(x as u32, y as u32));
}
