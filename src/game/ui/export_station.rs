use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::{
    assets::ExportStation, current_selection::CurrentlySelected,
    resource_specifications::ResourceSpecifications,
};

pub fn edit_ui(
    egui_context: ResMut<EguiContext>,
    mut export_query: Query<&mut ExportStation>,
    currently_selected: Res<CurrentlySelected>,
    resources: Res<ResourceSpecifications>,
) {
    if !currently_selected.editing {
        return;
    }

    if let Some(entity) = currently_selected.entity {
        if let Ok(mut export) = export_query.get_mut(entity) {
            egui::Window::new("ExportStation").show(egui_context.ctx(), |ui| {
                ui.heading("Export Station");

                for (id, resource) in resources.iter() {
                    if ui
                        .button(format!(
                            "{}: {}",
                            resource.name,
                            if export.goods.contains(id) {
                                "Yes"
                            } else {
                                "No"
                            }
                        ))
                        .clicked()
                    {
                        if export.goods.contains(&id) {
                            export.goods =
                                export.goods.iter().cloned().filter(|r| r != id).collect();
                        } else {
                            export.goods.push(id.clone());
                        }
                    }
                }
            });
        }
    }
}
