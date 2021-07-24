use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{ClickedTile, Direction, SelectedTool, Storage, Tool},
    car::{Car, CarInstructions},
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
    if let Tool::Car(resource) = selected_tool.tool {
        if !clicked_tile.occupied_vehicle {
            if let Some(pos) = clicked_tile.vehicle_pos {
                // make sure tile is set
                let entity = get_entity(&mut commands, &mut map_query, pos, VEHICLE_LAYER_ID);

                commands.entity(entity).insert(Tile {
                    texture_index: VehicleTile::BlueVertical as u16,
                    ..Default::default()
                });

                commands
                    .spawn()
                    .insert(Car {
                        position: pos,
                        direction: Direction::North,
                        instructions: vec![CarInstructions::Nop],
                        current_instruction: 0,
                    })
                    .insert(Storage {
                        resource,
                        amount: 0,
                        capacity: 4,
                    });
            }
        }
    }
}
