use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use collecting_hashmap::CollectingHashMap;

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

                let mut groups = CollectingHashMap::new();
                for (id, resource) in resources.iter() {
                    groups.insert(resource.group.clone(), (id, resource));
                }

                let mut group_names: Vec<String> = groups.keys().cloned().collect();
                group_names.sort_by_key(|a| a.to_lowercase());

                for group in group_names.iter() {
                    let mut resources = groups.remove_all(group).unwrap();
                    resources.sort_by_key(|(_id, r)| r.name.to_lowercase());

                    let count = resources
                        .iter()
                        .filter(|(id, _resource)| export.goods.contains(id))
                        .count();

                    egui::CollapsingHeader::new(format!("{} ({})", group, count))
                        .id_source(group)
                        .show(ui, |ui| {
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
                                        export.goods = export
                                            .goods
                                            .iter()
                                            .cloned()
                                            .filter(|r| r != *id)
                                            .collect();
                                    } else {
                                        export.goods.push(id.to_string());
                                    }
                                }
                            }
                        });
                }
            });
        }
    }
}
