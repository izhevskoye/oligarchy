use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::{
    assets::{ClickedTile, Storage},
    car::{Car, CarInstructions},
    current_selection::CurrentlySelected,
    resource_specifications::ResourceSpecifications,
};

#[derive(Clone, Default)]
pub struct EditInstruction {
    pub entity: Option<Entity>,
    pub index: Option<usize>,
    pub resource: Option<String>,
    pub select_mode: bool,
}

impl EditInstruction {
    fn confirm_selection(&mut self) {
        self.select_mode = false;
        self.index = None;
    }
}

pub fn program_ui(
    egui_context: ResMut<EguiContext>,
    mut car_query: Query<(&mut Car, &Storage)>,
    mut currently_selected: ResMut<CurrentlySelected>,
    mut edit_instruction: Local<EditInstruction>,
    clicked_tile: Res<ClickedTile>,
    resources: Res<ResourceSpecifications>,
) {
    let mut open = false;

    if !edit_instruction.select_mode && currently_selected.locked {
        return;
    }

    if !currently_selected.editing {
        return;
    }

    if let Some(entity) = currently_selected.entity {
        if let Ok((mut car, storage)) = car_query.get_mut(entity) {
            open = true;

            if Some(entity) != edit_instruction.entity {
                edit_instruction.entity = Some(entity);
                edit_instruction.resource = Some(storage.resource.clone());
            };

            if let Some(selected_index) = edit_instruction.index {
                let instruction = &car.instructions[selected_index].clone();

                if let Some(pos) = clicked_tile.pos {
                    if edit_instruction.select_mode {
                        car.instructions[selected_index] = CarInstructions::GoTo(pos);
                        currently_selected.locked = false;
                        edit_instruction.confirm_selection();
                    }
                }

                egui::Window::new("Instruction").show(egui_context.ctx(), |ui| {
                    ui.heading(format!("Current: {}", instruction.format(&resources)));

                    if ui.button("Idle").clicked() {
                        car.instructions[selected_index] = CarInstructions::Nop;
                        currently_selected.locked = false;
                        edit_instruction.confirm_selection();
                    }

                    if ui.button("Unload").clicked() {
                        if let Some(resource) = &edit_instruction.resource {
                            car.instructions[selected_index] =
                                CarInstructions::Unload(resource.clone());
                            currently_selected.locked = false;
                            edit_instruction.confirm_selection();
                        }
                    }

                    if ui.button("Wait For Unload").clicked() {
                        if let Some(resource) = &edit_instruction.resource {
                            car.instructions[selected_index] =
                                CarInstructions::WaitForUnload(resource.clone());
                            currently_selected.locked = false;
                            edit_instruction.confirm_selection();
                        }
                    }

                    if ui.button("Load").clicked() {
                        if let Some(resource) = &edit_instruction.resource {
                            car.instructions[selected_index] =
                                CarInstructions::Load(resource.clone());
                            currently_selected.locked = false;
                            edit_instruction.confirm_selection();
                        }
                    }

                    if ui.button("Wait for Load").clicked() {
                        if let Some(resource) = &edit_instruction.resource {
                            car.instructions[selected_index] =
                                CarInstructions::WaitForLoad(resource.clone());
                            currently_selected.locked = false;
                            edit_instruction.confirm_selection();
                        }
                    }

                    if ui.button("Go to").clicked() {
                        edit_instruction.select_mode = true;
                        currently_selected.locked = true;
                    }

                    egui::CollapsingHeader::new("Load / Unload Resource Configuration").show(
                        ui,
                        |ui| {
                            for (id, resource) in resources.iter() {
                                ui.radio_value(
                                    &mut edit_instruction.resource,
                                    Some(id.to_owned()),
                                    resource.name.clone(),
                                );
                            }
                        },
                    );

                    if ui.button("Abort").clicked() {
                        edit_instruction.confirm_selection();
                        currently_selected.locked = false;
                    }
                });
            }

            egui::Window::new("Instructions").show(egui_context.ctx(), |ui| {
                ui.horizontal(|ui| {
                    if car.active {
                        if ui.button("Deactivate").clicked() {
                            car.active = false;
                        }
                    } else if ui.button("Activate").clicked() {
                        car.active = true;
                    }
                });

                let instructions = car.instructions.clone();
                for (index, instruction) in instructions.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(instruction.format(&resources));
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
