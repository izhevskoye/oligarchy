use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use collecting_hashmap::CollectingHashMap;

use crate::game::{
    assets::resource_specifications::ResourceSpecifications,
    current_selection::CurrentlySelected,
    production::{ImportExportDirection, ImportExportStation},
};

pub fn edit_ui(
    egui_context: ResMut<EguiContext>,
    mut query: Query<&mut ImportExportStation>,
    currently_selected: Res<CurrentlySelected>,
    resources: Res<ResourceSpecifications>,
) {
    if !currently_selected.editing {
        return;
    }

    if let Some(entity) = currently_selected.entity {
        if let Ok(mut station) = query.get_mut(entity) {
            egui::Window::new(match station.direction {
                ImportExportDirection::Export => "Export Station",
                ImportExportDirection::Import => "Import Station",
            })
            .show(egui_context.ctx(), |ui| {
                let mut groups = CollectingHashMap::new();
                for (id, resource) in resources.iter() {
                    if resource.cost > f64::EPSILON && !resource.virtual_resource {
                        groups.insert(resource.group.clone(), (id, resource));
                    }
                }

                let mut group_names: Vec<String> = groups.keys().cloned().collect();
                group_names.sort_by_key(|a| a.to_lowercase());

                for group in group_names.iter() {
                    let mut resources = groups.remove_all(group).unwrap();
                    resources.sort_by_key(|(_id, r)| r.name.to_lowercase());

                    let count = resources
                        .iter()
                        .filter(|(id, _resource)| station.goods.contains(id))
                        .count();

                    egui::CollapsingHeader::new(format!("{} ({})", group, count))
                        .id_source(group)
                        .show(ui, |ui| {
                            for (id, resource) in resources.iter() {
                                if ui
                                    .button(format!(
                                        "{}: {}",
                                        resource.name,
                                        if station.goods.contains(id) {
                                            "Yes"
                                        } else {
                                            "No"
                                        }
                                    ))
                                    .clicked()
                                {
                                    if station.goods.contains(&id) {
                                        station.goods = station
                                            .goods
                                            .iter()
                                            .cloned()
                                            .filter(|r| r != *id)
                                            .collect();
                                    } else {
                                        station.goods.push(id.to_string());
                                    }
                                }
                            }
                        });
                }
            });
        }
    }
}
