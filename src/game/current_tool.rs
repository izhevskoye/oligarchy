use bevy::{prelude::*, render::camera::Camera};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiContext;

use super::{
    assets::{RequiresUpdate, SelectedTool, Street, Tool},
    constants::{MapTile, CHUNK_SIZE, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

pub fn current_tool(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    query: Query<&Transform, With<Camera>>,
    street_query: Query<&Street>,
    egui_context: ResMut<EguiContext>,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
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

        if selected_tool.tool == Tool::Street {
            let entity =
                if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, BUILDING_LAYER_ID) {
                    entity
                } else {
                    map_query
                        .set_tile(
                            &mut commands,
                            pos,
                            Tile {
                                texture_index: MapTile::StreetNorthEastSouthWest as u16,
                                ..Default::default()
                            },
                            MAP_ID,
                            BUILDING_LAYER_ID,
                        )
                        .unwrap()
                };

            commands
                .entity(entity)
                .insert(Street)
                .insert(RequiresUpdate { position: pos });

            let neighbors = map_query.get_tile_neighbors(pos, MAP_ID, BUILDING_LAYER_ID);
            for i in 0..4 {
                let (pos, neighbor) = neighbors[i];
                if let Some(neighbor) = neighbor {
                    if street_query.get(neighbor).is_ok() {
                        commands.entity(neighbor).insert(RequiresUpdate {
                            position: pos.as_u32(),
                        });
                    }
                }
            }
        }
    }
}
