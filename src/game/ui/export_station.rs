use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::{
    assets::{ExportStation, Resource},
    current_selection::CurrentlySelected,
};

pub fn edit_ui(
    egui_context: ResMut<EguiContext>,
    mut export_query: Query<&mut ExportStation>,
    currently_selected: Res<CurrentlySelected>,
) {
    if !currently_selected.editing {
        return;
    }

    if let Some(entity) = currently_selected.entity {
        if let Ok(mut export) = export_query.get_mut(entity) {
            egui::Window::new("ExportStation").show(egui_context.ctx(), |ui| {
                ui.heading("Export Station");

                for resource in &[
                    Resource::Coal,
                    Resource::Coke,
                    Resource::Limestone,
                    Resource::IronOre,
                    Resource::Iron,
                    Resource::Steel,
                ] {
                    if ui
                        .button(format!(
                            "{:?}: {}",
                            resource,
                            if export.goods.contains(&resource) {
                                "Yes"
                            } else {
                                "No"
                            }
                        ))
                        .clicked()
                    {
                        if export.goods.contains(&resource) {
                            export.goods = export
                                .goods
                                .iter()
                                .copied()
                                .filter(|r| r != resource)
                                .collect();
                        } else {
                            export.goods.push(*resource);
                        }
                    }
                }
            });
        }
    }
}
