use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};
use collecting_hashmap::CollectingHashMap;
use num_format::{Locale, ToFormattedString};

use crate::game::{
    account::Account,
    assets::{SelectedTool, Tool},
    building_specifications::BuildingSpecifications,
    resource_specifications::ResourceSpecifications,
};

pub fn construction_ui(
    egui_context: ResMut<EguiContext>,
    mut selected_tool: ResMut<SelectedTool>,
    buildings: Res<BuildingSpecifications>,
    resources: Res<ResourceSpecifications>,
    account: Res<Account>,
) {
    egui::Window::new("Construction")
        .anchor(Align2::RIGHT_TOP, [-10.0, 10.0])
        .show(egui_context.ctx(), |ui| {
            egui::Grid::new("items").show(ui, |ui| {
                if ui.small_button("None").clicked() {
                    selected_tool.tool = Tool::None;
                }
                if ui.small_button("Bulldoze").clicked() {
                    selected_tool.tool = Tool::Bulldoze;
                }
                ui.end_row();

                if ui.small_button("Street").clicked() {
                    selected_tool.tool = Tool::Street;
                }
                if ui.small_button("Export Station").clicked() {
                    selected_tool.tool = Tool::ExportStation;
                }
                ui.end_row();
                if ui.small_button("Delivery Station").clicked() {
                    selected_tool.tool = Tool::DeliveryStation;
                }
                ui.end_row();
            });

            ui.heading("Cars");

            let mut groups = CollectingHashMap::new();
            for (id, resource) in resources.iter() {
                groups.insert(resource.group.clone(), (id, resource));
            }

            let mut group_names: Vec<String> = groups.keys().cloned().collect();
            group_names.sort_by_key(|a| a.to_lowercase());

            for group in group_names.iter() {
                let resources = groups.get_all_mut(group).unwrap();
                resources.sort_by_key(|(_id, r)| r.name.to_lowercase());
            }

            for group in group_names.iter() {
                let resources = groups.get_all(group).unwrap().clone();

                egui::CollapsingHeader::new(format!("Transport: {}", group)).show(ui, |ui| {
                    egui::Grid::new(group).show(ui, |ui| {
                        for (index, (id, resource)) in resources.iter().enumerate() {
                            if ui
                                .small_button(format!("{} Truck", resource.name))
                                .clicked()
                            {
                                selected_tool.tool = Tool::Car(id.to_string());
                            }

                            if (index + 1) % 2 == 0 {
                                ui.end_row();
                            }
                        }
                    });
                });
            }

            ui.heading("Storage");

            for group in group_names.iter() {
                let resources = groups.get_all(group).unwrap().clone();

                egui::CollapsingHeader::new(format!("Storage: {}", group)).show(ui, |ui| {
                    egui::Grid::new(group).show(ui, |ui| {
                        for (index, (id, resource)) in resources.iter().enumerate() {
                            if ui
                                .small_button(format!("{} Storage", resource.name))
                                .clicked()
                            {
                                selected_tool.tool = Tool::Storage(id.to_string());
                            }

                            if (index + 1) % 2 == 0 {
                                ui.end_row();
                            }
                        }
                    });
                });
            }

            ui.heading("Buildings");

            let mut groups = CollectingHashMap::new();
            for (id, building) in buildings.iter() {
                groups.insert(building.group.clone(), (id, building));
            }

            let mut group_names: Vec<String> = groups.keys().cloned().collect();
            group_names.sort_by_key(|a| a.to_lowercase());

            for group in group_names.iter() {
                let buildings = groups.get_all_mut(group).unwrap();
                buildings.sort_by_key(|(_id, r)| r.name.to_lowercase());
            }

            for group in group_names.iter() {
                let buildings = groups.get_all(group).unwrap();

                egui::CollapsingHeader::new(format!("Building: {}", group)).show(ui, |ui| {
                    egui::Grid::new(group).show(ui, |ui| {
                        for (index, (id, building)) in buildings.iter().enumerate() {
                            let name = if building.cost.is_some() {
                                format!(
                                    "{} ({})",
                                    building.name,
                                    building.price(&resources).to_formatted_string(&Locale::en)
                                )
                            } else {
                                building.name.to_owned()
                            };

                            let mut button = ui.small_button(name);

                            if building.price(&resources) >= account.value {
                                button = button.on_hover_text("You cannot afford this");
                            }

                            if button.clicked() {
                                selected_tool.tool = Tool::Building(id.to_string());
                            }

                            if (index + 1) % 2 == 0 {
                                ui.end_row();
                            }
                        }
                    });
                });
            }
        });
}
