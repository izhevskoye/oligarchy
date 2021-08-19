use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::{
    assets::{resource_specifications::ResourceSpecifications, ClickedTile, Position},
    car::{Car, CarController, CarInstructions, Destination, UserController, Waypoints},
    current_selection::CurrentlySelected,
    storage::Storage,
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
    mut commands: Commands,
    egui_context: ResMut<EguiContext>,
    mut car_query: Query<(&mut Car, &Storage, &Position)>,
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
        if car_query.get_mut(entity).is_err() {
            return;
        }

        let mut car_controller_modified = false;
        let mut car_controller = {
            let (car, _storage, _position) = car_query.get_mut(entity).unwrap();

            if let CarController::UserControlled(controller) = &car.controller {
                controller.clone()
            } else {
                UserController::default()
            }
        };

        if edit_instruction.index.is_none() {
            if let Some(pos) = clicked_tile.vehicle_pos {
                if edit_instruction.select_mode && clicked_tile.occupied_vehicle {
                    // clone instructions
                    let controller =
                        car_query
                            .iter_mut()
                            .find_map(|(other_car, _storage, position)| {
                                if position.position == pos {
                                    Some(other_car.controller.clone())
                                } else {
                                    None
                                }
                            });

                    if let Some(mut controller) = controller {
                        if let CarController::UserControlled(controller) = &mut controller {
                            controller.current_instruction = 0;
                        }

                        if let Ok((mut car, _storage, _position)) = car_query.get_mut(entity) {
                            car.controller = controller;
                        }
                    }

                    edit_instruction.select_mode = false;
                    currently_selected.locked = false;
                }
            }
        }

        open = true;

        if Some(entity) != edit_instruction.entity {
            if let Ok((_car, storage, _position)) = car_query.get_mut(entity) {
                edit_instruction.entity = Some(entity);
                edit_instruction.resource = Some(storage.resource.clone());
            }
        };

        if let Some(selected_index) = edit_instruction.index {
            let instruction = car_controller.instructions[selected_index].clone();

            if let Some(pos) = clicked_tile.pos {
                if edit_instruction.select_mode {
                    car_controller.instructions[selected_index] = CarInstructions::GoTo(pos);
                    currently_selected.locked = false;
                    edit_instruction.confirm_selection();
                    car_controller_modified = true;
                }
            }

            egui::Window::new("Instruction").default_width(100.0).show(
                    egui_context.ctx(),
                    |ui| {
                        ui.heading(format!("Current: {}", instruction.format(&resources)));

                        ui.vertical_centered_justified(|ui| {
                            if ui.button("Idle").clicked() {
                                car_controller.instructions[selected_index] = CarInstructions::Nop;
                                currently_selected.locked = false;
                                edit_instruction.confirm_selection();
                        car_controller_modified = true;
                            }

                            if ui.button("Unload").clicked() {
                                if let Some(resource) = &edit_instruction.resource {
                                    car_controller.instructions[selected_index] =
                                        CarInstructions::Unload(resource.clone());
                                    currently_selected.locked = false;
                                    edit_instruction.confirm_selection();
                        car_controller_modified = true;
                                }
                            }

                            if ui.button("Wait For Unload").clicked() {
                                if let Some(resource) = &edit_instruction.resource {
                                    car_controller.instructions[selected_index] =
                                        CarInstructions::WaitForUnload(resource.clone());
                                    currently_selected.locked = false;
                                    edit_instruction.confirm_selection();
                        car_controller_modified = true;
                                }
                            }

                            if ui.button("Load").clicked() {
                                if let Some(resource) = &edit_instruction.resource {
                                    car_controller.instructions[selected_index] =
                                        CarInstructions::Load(resource.clone());
                                    currently_selected.locked = false;
                                    edit_instruction.confirm_selection();
                        car_controller_modified = true;
                                }
                            }

                            if ui.button("Wait for Load").clicked() {
                                if let Some(resource) = &edit_instruction.resource {
                                    car_controller.instructions[selected_index] =
                                        CarInstructions::WaitForLoad(resource.clone());
                                    currently_selected.locked = false;
                                    edit_instruction.confirm_selection();
                        car_controller_modified = true;
                                }
                            }

                            if ui.button("Go to").clicked() {
                                edit_instruction.select_mode = true;
                                currently_selected.locked = true;
                            }
                        });

                        egui::CollapsingHeader::new("Load / Unload Resource Configuration").show(
                            ui,
                            |ui| {
                                egui::containers::ScrollArea::from_max_height(200.0).show(
                                    ui,
                                    |ui| {
                                    ui.label("Please note this is more for debugging as cars cannot load anything other than what they are designed to load!");
                                        for (id, resource) in resources.iter() {
                                            if ui.radio_value(
                                                &mut edit_instruction.resource,
                                                Some(id.to_owned()),
                                                resource.name.clone(),
                                            ).clicked() {

                        car_controller_modified = true;
                                            }
                                        }
                                    },
                                );
                            },
                        );

                        if ui.button("Abort").clicked() {
                            edit_instruction.confirm_selection();
                            currently_selected.locked = false;
                        }
                    },
                );
        }

        egui::Window::new("Instructions")
            .default_width(100.0)
            .show(egui_context.ctx(), |ui| {
                ui.horizontal(|ui| {
                    if car_controller.active {
                        if ui.button("Deactivate").clicked() {
                            car_controller.active = false;
                            car_controller_modified = true;
                        }
                    } else if ui.button("Activate").clicked() {
                        commands
                            .entity(entity)
                            .remove::<Waypoints>()
                            .remove::<Destination>();
                        car_controller.active = true;
                        car_controller_modified = true;
                    }

                    if ui.button("Clone instructions").clicked() {
                        edit_instruction.select_mode = !edit_instruction.select_mode;
                        edit_instruction.index = None;
                        currently_selected.locked = true;
                    }
                });

                ui.separator();

                let instructions = car_controller.instructions.clone();
                egui::Grid::new("instructions").show(ui, |ui| {
                    for (index, instruction) in instructions.iter().enumerate() {
                        ui.label(instruction.format(&resources));
                        if ui.button("Edit").clicked() {
                            edit_instruction.select_mode = false;
                            edit_instruction.index = Some(index);
                            currently_selected.locked = false;
                        }

                        if ui.button("Delete").clicked() {
                            car_controller.instructions.remove(index);
                            edit_instruction.index = None;
                            edit_instruction.select_mode = false;
                            currently_selected.locked = false;

                            if car_controller.current_instruction
                                > car_controller.instructions.len()
                            {
                                car_controller.current_instruction = 0;
                            }

                            if car_controller.instructions.is_empty() {
                                car_controller.instructions.push(CarInstructions::Nop);
                            }

                            car_controller_modified = true;
                        }
                        ui.end_row();
                    }
                });

                ui.separator();

                ui.vertical_centered_justified(|ui| {
                    if ui.button("[Add new]").clicked() {
                        car_controller.instructions.push(CarInstructions::Nop);
                        car_controller_modified = true;
                        edit_instruction.index = Some(car_controller.instructions.len() - 1);
                        edit_instruction.select_mode = false;
                        currently_selected.locked = false;
                    }
                });
            });

        if car_controller_modified {
            let (mut car, _storage, _position) = car_query.get_mut(entity).unwrap();

            car.controller = CarController::UserControlled(car_controller);
        }
    }

    if !open && edit_instruction.index.is_some() {
        edit_instruction.index = None;
        edit_instruction.select_mode = false;
        currently_selected.locked = false;
    }
}
