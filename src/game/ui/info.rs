use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::assets::{CurrentlySelected, Storage};

pub fn info_ui(
    egui_context: ResMut<EguiContext>,
    storage_query: Query<&Storage>,
    currently_selected: Res<CurrentlySelected>,
) {
    if let Some(entity) = currently_selected.entity {
        if let Ok(storage) = storage_query.get(entity) {
            egui::SidePanel::left("side_panel")
                .default_width(200.0)
                .show(egui_context.ctx(), |ui| {
                    ui.heading("Side Panel");

                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{:?} {} / {}",
                            storage.resource, storage.amount, storage.capacity
                        ));
                    });
                });
        }
    }
}
