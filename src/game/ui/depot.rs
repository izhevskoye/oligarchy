use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::{
    assets::{ClickedTile, Position},
    car::{Car, CarController, DepotController},
    current_selection::CurrentlySelected,
    production::Depot,
};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum EditMode {
    AddPickup,
    AddDelivery,
    AddCar,
    None,
}

impl Default for EditMode {
    fn default() -> Self {
        Self::None
    }
}

pub fn edit_ui(
    egui_context: ResMut<EguiContext>,
    mut depot_query: Query<(Entity, &mut Depot)>,
    mut car_query: Query<(&mut Car, &Position)>,
    mut currently_selected: ResMut<CurrentlySelected>,
    clicked_tile: Res<ClickedTile>,
    mut edit_mode: Local<EditMode>,
) {
    if !currently_selected.editing {
        return;
    }

    if EditMode::None == *edit_mode && currently_selected.locked {
        return;
    }

    if let Some(entity) = currently_selected.entity {
        if let Ok((entity, mut depot)) = depot_query.get_mut(entity) {
            if clicked_tile.occupied_building {
                if let Some(pos) = clicked_tile.pos {
                    // TODO: only select delivery

                    if EditMode::AddDelivery == *edit_mode {
                        currently_selected.locked = false;
                        *edit_mode = EditMode::None;
                        depot.deliveries.push(pos);
                    }

                    if EditMode::AddPickup == *edit_mode {
                        currently_selected.locked = false;
                        *edit_mode = EditMode::None;
                        depot.pickups.push(pos);
                    }
                }
            }

            if clicked_tile.occupied_vehicle {
                if let Some(pos) = clicked_tile.vehicle_pos {
                    if EditMode::AddCar == *edit_mode {
                        currently_selected.locked = false;
                        *edit_mode = EditMode::None;

                        for (mut car, position) in car_query.iter_mut() {
                            if position.position != pos {
                                continue;
                            }

                            car.controller =
                                CarController::DepotControlled(DepotController { depot: entity });
                        }
                    }
                }
            }

            egui::Window::new("Depot").show(egui_context.ctx(), |ui| {
                ui.heading("Depot");

                if EditMode::None != *edit_mode && ui.button("Abort selection").clicked() {
                    *edit_mode = EditMode::None;
                    currently_selected.locked = false;
                }

                if ui.button("Add car").clicked() {
                    *edit_mode = EditMode::AddCar;
                    currently_selected.locked = true;
                }

                egui::CollapsingHeader::new("Deliveries").show(ui, |ui| {
                    for (index, point) in depot.deliveries.clone().iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}", point));

                            if ui.button("Delete").clicked() {
                                depot.deliveries.remove(index);
                            }
                        });
                    }

                    if ui.button("Add").clicked() {
                        *edit_mode = EditMode::AddDelivery;
                        currently_selected.locked = true;
                    }
                });

                egui::CollapsingHeader::new("Pickups").show(ui, |ui| {
                    for (index, point) in depot.pickups.clone().iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}", point));

                            if ui.button("Delete").clicked() {
                                depot.pickups.remove(index);
                            }
                        });
                    }

                    if ui.button("Add").clicked() {
                        *edit_mode = EditMode::AddPickup;
                        currently_selected.locked = true;
                    }
                });
            });
        }
    }
}
