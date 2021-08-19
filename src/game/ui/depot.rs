use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::{assets::ClickedTile, current_selection::CurrentlySelected, production::Depot};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum EditMode {
    AddPickup,
    AddDelivery,
    None,
}

impl Default for EditMode {
    fn default() -> Self {
        Self::None
    }
}

pub fn edit_ui(
    egui_context: ResMut<EguiContext>,
    mut depot_query: Query<&mut Depot>,
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
        if let Ok(mut depot) = depot_query.get_mut(entity) {
            if let Some(pos) = clicked_tile.pos {
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

            egui::Window::new("Depot").show(egui_context.ctx(), |ui| {
                ui.heading("Depot");

                egui::CollapsingHeader::new("Deliveries").show(ui, |ui| {
                    for point in &depot.deliveries {
                        ui.label(format!("- {}", point));
                    }

                    if ui.button("Add").clicked() {
                        *edit_mode = EditMode::AddDelivery;
                        currently_selected.locked = true;
                    }
                });

                egui::CollapsingHeader::new("Pickups").show(ui, |ui| {
                    for point in &depot.pickups {
                        ui.label(format!("- {}", point));
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
