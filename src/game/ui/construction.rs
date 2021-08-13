use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Response, Ui},
    EguiContext,
};
use collecting_hashmap::CollectingHashMap;
use num_format::{Locale, ToFormattedString};

use crate::game::{
    account::{Account, PurchaseCost},
    assets::{
        building_specifications::BuildingSpecifications,
        resource_specifications::ResourceSpecifications, SelectedTool, Tool,
    },
    car::Car,
    production::{DeliveryStation, ExportStation},
    storage::Storage,
    street::Street,
};

fn button(
    ui: &mut Ui,
    name: &str,
    item: &dyn PurchaseCost,
    resources: &ResourceSpecifications,
    account: &Account,
) -> Response {
    let price = item.price(&resources);
    let name = format!("{} ({})", name, price.to_formatted_string(&Locale::en));

    let mut button = ui.small_button(name);

    if price >= account.value {
        button = button.on_hover_text("You cannot afford this");
    }

    button
}

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

                if button(ui, "Street", &Street, &resources, &account).clicked() {
                    selected_tool.tool = Tool::Street;
                }
                if button(
                    ui,
                    "Export Station",
                    &ExportStation::default(),
                    &resources,
                    &account,
                )
                .clicked()
                {
                    selected_tool.tool = Tool::ExportStation;
                }
                ui.end_row();
                if button(
                    ui,
                    "Delivery Station",
                    &DeliveryStation,
                    &resources,
                    &account,
                )
                .clicked()
                {
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
                let resource_list = groups.get_all_mut(group).unwrap();
                resource_list.sort_by_key(|(_id, r)| r.name.to_lowercase());
            }

            for group in group_names.iter() {
                let resource_list = groups.get_all(group).unwrap().clone();

                egui::CollapsingHeader::new(format!("Transport: {}", group)).show(ui, |ui| {
                    egui::Grid::new(group).show(ui, |ui| {
                        for (index, (id, resource)) in resource_list.iter().enumerate() {
                            if button(
                                ui,
                                &format!("{} Truck", resource.name),
                                &(
                                    Car::default(),
                                    Storage {
                                        resource: id.to_string(),
                                        ..Default::default()
                                    },
                                ),
                                &resources,
                                &account,
                            )
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
                let resource_list = groups.get_all(group).unwrap().clone();

                egui::CollapsingHeader::new(format!("Storage: {}", group)).show(ui, |ui| {
                    egui::Grid::new(group).show(ui, |ui| {
                        for (index, (id, resource)) in resource_list.iter().enumerate() {
                            if button(
                                ui,
                                &format!("{} Storage", resource.name),
                                &Storage {
                                    resource: id.to_string(),
                                    ..Default::default()
                                },
                                &resources,
                                &account,
                            )
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
                            if button(ui, &building.name, *building, &resources, &account).clicked()
                            {
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
