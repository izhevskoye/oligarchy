use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::Camera,
};
use bevy_egui::EguiContext;

// A simple camera system for moving and zooming the camera.
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
    egui_context: ResMut<EguiContext>,
) {
    if egui_context.ctx().wants_pointer_input() {
        return;
    }

    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let scale = transform.scale.x;

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

        if keyboard_input.pressed(KeyCode::Z) {
            let scale = scale + 0.05;
            transform.scale = Vec3::splat(scale);
        }

        if keyboard_input.pressed(KeyCode::X) {
            let scale = scale - 0.05;
            transform.scale = Vec3::splat(scale);
        }

        let mut scroll = 0.0;
        for ev in ev_scroll.iter() {
            scroll += ev.y;
        }
        let scale = scale + scroll * time.delta_seconds() * 0.4;
        transform.scale = Vec3::splat(scale);

        if transform.scale.x > 1.5 {
            transform.scale = Vec3::splat(1.5)
        }

        if transform.scale.x < 0.1 {
            transform.scale = Vec3::splat(0.1)
        }

        transform.translation += time.delta_seconds() * direction * 500.;
    }
}
