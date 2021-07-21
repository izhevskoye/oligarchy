use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{Car, CarInstructions, ClickedTile, Direction, Resource, SelectedTool, Storage, Tool},
    constants::VehicleTile,
    setup::VEHICLE_LAYER_ID,
};

use super::get_entity;

pub fn car_placement(
    mut commands: Commands,
    mut map_query: MapQuery,
    selected_tool: Res<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if Tool::Car == selected_tool.tool && !clicked_tile.occupied_vehicle {
        if let Some(pos) = clicked_tile.vehicle_pos {
            // make sure tile is set
            get_entity(
                &mut commands,
                &mut map_query,
                pos,
                VehicleTile::BlueVertical,
                VEHICLE_LAYER_ID,
            );

            commands
                .spawn()
                .insert(Car {
                    position: pos,
                    direction: Direction::North,
                    instructions: vec![
                        CarInstructions::GoTo(UVec2::new(14, 7)),
                        CarInstructions::WaitForLoad(Resource::Steel),
                        CarInstructions::GoTo(UVec2::new(10, 1)),
                        CarInstructions::WaitForUnload(Resource::Steel),
                    ],
                    current_instruction: 0,
                })
                .insert(Storage {
                    resource: Resource::Steel,
                    amount: 0,
                    capacity: 4,
                });
        }
    }
}
