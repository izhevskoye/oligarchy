use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::Camera,
};
use bevy_egui::EguiContext;

use crate::game::assets::{SelectedTool, Tool};

const MAX_ZOOM_OUT: f32 = 0.1;
const MAX_ZOOM_IN: f32 = 1.5;

// A simple camera system for moving and zooming the camera.
#[allow(clippy::too_many_arguments)]
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
    egui_context: ResMut<EguiContext>,
    selected_tool: Res<SelectedTool>,
) {
    if egui_context.ctx().wants_pointer_input() || egui_context.ctx().wants_keyboard_input() {
        return;
    }

    if selected_tool.tool != Tool::None {
        return;
    }

    let mut transform = query.single_mut().unwrap();

    let mut direction = Vec3::ZERO;

    let mut delta_since = Vec2::ZERO;
    for MouseMotion { delta } in mouse_motion_events.iter() {
        delta_since += *delta;
    }

    if mouse_input.pressed(MouseButton::Left) {
        direction += Vec3::new(-delta_since.x, delta_since.y, 0.0) * transform.scale;
    }

    transform.translation += time.delta_seconds() * direction * 50.;

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction -= Vec3::new(0.0, 1.0, 0.0);
    }

    let mut scroll = 0.0;
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    let mut scale = transform.scale.x + scroll * time.delta_seconds() * 0.4;

    let win = windows.get_primary().expect("no primary window");

    if scale > MAX_ZOOM_IN {
        scale = MAX_ZOOM_IN;
    }

    if scale < MAX_ZOOM_OUT {
        scale = MAX_ZOOM_OUT;
    }

    if scroll != 0.0 {
        if let Some(cursor) = win.cursor_position() {
            let mut cursor = cursor;
            cursor.x += win.width() / 2.0;
            cursor.y += win.height() / 2.0;
            let mouse_in_world = Vec2::new(
                ((cursor.x - win.width()) * transform.scale.x) + transform.translation.x,
                ((cursor.y - win.height()) * transform.scale.x) + transform.translation.y,
            );

            let new_x = mouse_in_world.x - ((cursor.x - win.width()) * scale);
            let new_y = mouse_in_world.y - ((cursor.y - win.height()) * scale);

            transform.translation.x = new_x;
            transform.translation.y = new_y;

            transform.scale = Vec3::splat(scale);
        }
    }

    transform.translation += time.delta_seconds() * direction * 500.;
}
