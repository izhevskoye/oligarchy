use bevy::prelude::*;

use crate::game::{
    assets::{ClickedTile, Direction, Editable, SelectedTool, Storage, Tool},
    car::{Car, CarInstructions},
};

pub fn car_placement(
    mut commands: Commands,
    mut selected_tool: ResMut<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
) {
    if clicked_tile.dragging {
        return;
    }

    if let Tool::Car(resource) = &selected_tool.tool {
        if !clicked_tile.occupied_vehicle {
            if let Some(pos) = clicked_tile.vehicle_pos {
                commands
                    .spawn()
                    .insert(Car {
                        position: pos,
                        direction: Direction::North,
                        instructions: vec![CarInstructions::Nop],
                        current_instruction: 0,
                        active: false,
                    })
                    .insert(Storage {
                        resource: resource.clone(),
                        amount: 0.0,
                        capacity: 4.0,
                    })
                    .insert(Editable);

                selected_tool.tool = Tool::None;
            }
        }
    }
}
