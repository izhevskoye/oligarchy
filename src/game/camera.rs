use bevy::{
    ecs::schedule::ShouldRun,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::Camera,
};
use bevy_egui::EguiContext;

use super::{
    assets::MapSettings,
    constants::{CHUNK_SIZE, TILE_SIZE},
    current_tool::{SelectedTool, Tool},
    NewGameSetup,
};

const MAX_ZOOM_OUT: f32 = 0.1;
const MAX_ZOOM_IN: f32 = 1.5;

pub fn movement_allowed(
    egui_context: ResMut<EguiContext>,
    setup: Res<Option<NewGameSetup>>,
) -> ShouldRun {
    if egui_context.ctx().wants_pointer_input()
        || egui_context.ctx().wants_keyboard_input()
        || setup.is_some()
    {
        return ShouldRun::No;
    }

    ShouldRun::Yes
}

// A simple camera system for moving and zooming the camera.
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
    selected_tool: Res<SelectedTool>,
    mut real_transform: Local<Option<Transform>>,
    map_settings: Res<MapSettings>,
) {
    let win = windows.get_primary().expect("no primary window");
    if real_transform.is_none() {
        let transform = *query.single_mut().unwrap();
        *real_transform = Some(transform);
    }

    let mut transform = (*real_transform).unwrap();

    // scroll

    let mut scroll = 0.0;
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    let mut scale = transform.scale.x + scroll * time.delta_seconds() * 0.4;

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

    // disable if tool

    if selected_tool.tool != Tool::None {
        return;
    }

    // move by mouse

    let mut direction = Vec3::ZERO;

    let mut delta_since = Vec2::ZERO;
    for MouseMotion { delta } in mouse_motion_events.iter() {
        delta_since += *delta;
    }

    if mouse_input.pressed(MouseButton::Left) {
        direction += Vec3::new(-delta_since.x, delta_since.y, 0.0) * transform.scale;
    }

    transform.translation += time.delta_seconds() * direction * 50.;

    // move by keys

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

    transform.translation += time.delta_seconds() * direction * 500.;

    if transform.translation.x < 0.0 {
        transform.translation.x = 0.0;
    }

    let max_width = TILE_SIZE * (map_settings.width * CHUNK_SIZE - 1) as f32;
    if transform.translation.x > max_width {
        transform.translation.x = max_width;
    }

    if transform.translation.y < 0.0 {
        transform.translation.y = 0.0;
    }

    let max_height = TILE_SIZE * (map_settings.height * CHUNK_SIZE - 1) as f32;
    if transform.translation.y > max_height {
        transform.translation.y = max_height;
    }

    *real_transform = Some(transform);

    // fix scan lines in render when position is not rounded
    transform.translation = transform.translation.as_i32().as_f32();
    transform.scale = (transform.scale * 20.0).as_i32().as_f32() / 20.0;

    let mut world_transform = query.single_mut().unwrap();
    world_transform.scale = transform.scale;
    world_transform.translation = transform.translation;
}
