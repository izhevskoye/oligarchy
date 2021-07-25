use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::{
    assets::{ClickedTile, Resource},
    car::{Car, CarInstructions},
    current_selection::CurrentlySelected,
};

#[derive(Copy, Clone)]
pub struct EditInstruction {
    pub index: Option<usize>,
    pub resource: Resource,
    pub select_mode: bool,
}

impl Default for EditInstruction {
    fn default() -> Self {
        Self {
            index: None,
            resource: Resource::Coal,
            select_mode: false,
        }
    }
}

impl EditInstruction {
    fn confirm_selection(&mut self) {
        self.select_mode = false;
        self.index = None;
    }
}

pub fn program_ui(
    egui_context: ResMut<EguiContext>,
    mut car_query: Query<&mut Car>,
    mut currently_selected: ResMut<CurrentlySelected>,
    mut edit_instruction: Local<EditInstruction>,
    clicked_tile: Res<ClickedTile>,
) {
    let mut open = false;

    if !edit_instruction.select_mode && currently_selected.locked {
        return;
    }

    if !currently_selected.editing {
        return;
    }

    if let Some(entity) = currently_selected.entity {
        if let Ok(mut car) = car_query.get_mut(entity) {
            open = true;

            if let Some(selected_index) = edit_instruction.index {
                let instruction = &car.instructions[selected_index].clone();

                if let Some(pos) = clicked_tile.pos {
                    car.instructions[selected_index] = CarInstructions::GoTo(pos);
                    currently_selected.locked = false;
                    edit_instruction.confirm_selection();
                }

                egui::Window::new("Instruction").show(egui_context.ctx(), |ui| {
                    ui.heading(format!("Current: {}", instruction));

                    if ui.button("Idle").clicked() {
                        car.instructions[selected_index] = CarInstructions::Nop;
                        currently_selected.locked = false;
                        edit_instruction.confirm_selection();
                    }

                    if ui.button("Wait For Unload").clicked() {
                        car.instructions[selected_index] =
                            CarInstructions::WaitForUnload(edit_instruction.resource);
                        currently_selected.locked = false;
                        edit_instruction.confirm_selection();
                    }

                    if ui.button("Wait for Load").clicked() {
                        car.instructions[selected_index] =
                            CarInstructions::WaitForLoad(edit_instruction.resource);
                        currently_selected.locked = false;
                        edit_instruction.confirm_selection();
                    }

                    if ui.button("Go to").clicked() {
                        edit_instruction.select_mode = true;
                        currently_selected.locked = true;
                    }

                    ui.radio_value(&mut edit_instruction.resource, Resource::Coal, "Coal");
                    ui.radio_value(&mut edit_instruction.resource, Resource::Coke, "Coke");
                    ui.radio_value(
                        &mut edit_instruction.resource,
                        Resource::Limestone,
                        "Limestone",
                    );
                    ui.radio_value(
                        &mut edit_instruction.resource,
                        Resource::IronOre,
                        "Iron Ore",
                    );
                    ui.radio_value(&mut edit_instruction.resource, Resource::Iron, "Iron");
                    ui.radio_value(&mut edit_instruction.resource, Resource::Steel, "Steel");

                    if ui.button("Abort").clicked() {
                        edit_instruction.confirm_selection();
                        currently_selected.locked = false;
                    }
                });
            }

            egui::Window::new("Instructions").show(egui_context.ctx(), |ui| {
                let instructions = car.instructions.clone();

                for (index, instruction) in instructions.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}", instruction));
                        if ui.button("Edit").clicked() {
                            edit_instruction.index = Some(index);
                        }

                        if ui.button("Delete").clicked() {
                            car.instructions.remove(index);
                            edit_instruction.index = None;
                            edit_instruction.select_mode = false;
                            currently_selected.locked = false;

                            if car.current_instruction > car.instructions.len() {
                                car.current_instruction = 0;
                            }

                            if car.instructions.is_empty() {
                                car.instructions.push(CarInstructions::Nop);
                            }
                        }
                    });
                }

                if ui.button("[Add new]").clicked() {
                    car.instructions.push(CarInstructions::Nop);
                    edit_instruction.index = Some(car.instructions.len() - 1);
                    edit_instruction.select_mode = false;
                    currently_selected.locked = false;
                }
            });
        }
    }

    if !open && edit_instruction.index.is_some() {
        edit_instruction.index = None;
        edit_instruction.select_mode = false;
        currently_selected.locked = false;
    }
}
