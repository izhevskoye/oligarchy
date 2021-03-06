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
        building_specifications::{BuildingSpecification, BuildingSpecifications},
        resource_specifications::{ResourceSpecification, ResourceSpecifications},
    },
    car::Car,
    constants::CURRENCY,
    current_tool::{SelectedTool, Tool},
    production::{
        DeliveryStation, Depot, ImportExportDirection, ImportExportStation, StorageManagement,
    },
    storage::Storage,
    street::{Street, StreetType},
};

fn button(
    ui: &mut Ui,
    name: &str,
    item: &dyn PurchaseCost,
    resources: &ResourceSpecifications,
    account: &Account,
) -> Response {
    let price = item.price(resources);

    let button = ui.small_button(name);

    let mut hover_text = format!(
        "{}\n---\nTotal Cost: {} {}",
        item.price_description(resources),
        price.to_formatted_string(&Locale::en),
        CURRENCY
    );

    if price >= account.value {
        hover_text += "\nYou cannot afford it"
    }

    button.on_hover_text(hover_text)
}

pub struct Filter(String);

impl Filter {
    fn match_name(&self, name: &str) -> bool {
        self.0.is_empty() || name.to_lowercase().contains(&self.0.to_lowercase())
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self("".to_owned())
    }
}

pub fn construction_ui(
    egui_context: ResMut<EguiContext>,
    mut selected_tool: ResMut<SelectedTool>,
    buildings: Res<BuildingSpecifications>,
    resources: Res<ResourceSpecifications>,
    account: Res<Account>,
    windows: Res<Windows>,
    mut filter: Local<Filter>,
) {
    let win = windows.get_primary().expect("no primary window");
    let max_height = win.height() * 0.75;

    let open_groups = !filter.0.is_empty();

    egui::Window::new("Construction")
        .anchor(Align2::RIGHT_TOP, [-10.0, 10.0])
        .default_width(150.0)
        .show(egui_context.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Filter");
                ui.text_edit_singleline(&mut filter.0);
                if ui.small_button("X").clicked() {
                    filter.0 = "".to_owned();
                }
            });

            ui.separator();

            ui.vertical_centered_justified(|ui| {
                if ui.small_button("None").clicked() {
                    selected_tool.tool = Tool::None;
                }
                if ui.small_button("Bulldoze").clicked() {
                    selected_tool.tool = Tool::Bulldoze;
                }
            });

            ui.separator();

            egui::containers::ScrollArea::from_max_height(max_height).show(ui, |ui| {
                if vec![
                    "Street (Road)",
                    "Street (Dirt)",
                    "Export Station",
                    "Import Station",
                    "Delivery Station",
                    "Depot",
                    "Storage Management",
                ]
                .into_iter()
                .any(|item| filter.match_name(item))
                {
                    let group_title = "Building: General";
                    let items: Box<dyn FnOnce(&mut Ui)> = Box::new(|ui| {
                        ui.vertical_centered_justified(|ui| {
                            if filter.match_name("Street (Road)")
                                && button(
                                    ui,
                                    "Street (Road)",
                                    &Street {
                                        street_type: StreetType::Asphalt,
                                    },
                                    &resources,
                                    &account,
                                )
                                .clicked()
                            {
                                selected_tool.tool = Tool::Street;
                            }

                            if filter.match_name("Street (Dirt)")
                                && button(
                                    ui,
                                    "Street (Dirt)",
                                    &Street {
                                        street_type: StreetType::Dirt,
                                    },
                                    &resources,
                                    &account,
                                )
                                .clicked()
                            {
                                selected_tool.tool = Tool::Path;
                            }

                            if filter.match_name("Export Station")
                                && button(
                                    ui,
                                    "Export Station",
                                    &ImportExportStation::default(),
                                    &resources,
                                    &account,
                                )
                                .clicked()
                            {
                                selected_tool.tool =
                                    Tool::ImportExportStation(ImportExportDirection::Export);
                            }

                            if filter.match_name("Import Station")
                                && button(
                                    ui,
                                    "Import Station",
                                    &ImportExportStation::default(),
                                    &resources,
                                    &account,
                                )
                                .clicked()
                            {
                                selected_tool.tool =
                                    Tool::ImportExportStation(ImportExportDirection::Import);
                            }

                            if filter.match_name("Depot")
                                && button(ui, "Depot", &Depot::default(), &resources, &account)
                                    .clicked()
                            {
                                selected_tool.tool = Tool::Depot;
                            }

                            if filter.match_name("Delivery Station")
                                && button(
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

                            if filter.match_name("Storage Management")
                                && button(
                                    ui,
                                    "Storage Management",
                                    &StorageManagement,
                                    &resources,
                                    &account,
                                )
                                .clicked()
                            {
                                selected_tool.tool = Tool::StorageManagement;
                            }
                        });
                    });

                    if open_groups {
                        ui.label(group_title);
                        items(ui);
                    } else {
                        egui::CollapsingHeader::new(group_title)
                            .default_open(open_groups)
                            .show(ui, |ui| items(ui));
                    }
                }

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
                    let buildings: Vec<&(&String, &BuildingSpecification)> = groups
                        .get_all(group)
                        .unwrap()
                        .iter()
                        .filter(|(_id, building)| filter.match_name(&building.name))
                        .collect();

                    if !buildings.is_empty() {
                        let group_title = format!("Building: {}", group);

                        let items: Box<dyn FnOnce(&mut Ui)> = Box::new(|ui| {
                            ui.vertical_centered_justified(|ui| {
                                for (id, building) in buildings.iter() {
                                    if button(ui, &building.name, *building, &resources, &account)
                                        .clicked()
                                    {
                                        selected_tool.tool = Tool::Building(id.to_string());
                                    }
                                }
                            });
                        });

                        if open_groups {
                            ui.label(group_title);
                            items(ui);
                        } else {
                            egui::CollapsingHeader::new(group_title)
                                .default_open(open_groups)
                                .show(ui, |ui| items(ui));
                        }
                    }
                }

                let mut groups = CollectingHashMap::new();
                for (id, resource) in resources.iter() {
                    if resource.virtual_resource {
                        continue;
                    }
                    groups.insert(resource.group.clone(), (id, resource));
                }

                let mut group_names: Vec<String> = groups.keys().cloned().collect();
                group_names.sort_by_key(|a| a.to_lowercase());

                for group in group_names.iter() {
                    let resource_list = groups.get_all_mut(group).unwrap();
                    resource_list.sort_by_key(|(_id, r)| r.name.to_lowercase());
                }

                for group in group_names.iter() {
                    let resource_list: Vec<&(&String, &ResourceSpecification)> = groups
                        .get_all(group)
                        .unwrap()
                        .iter()
                        .filter(|(_id, resource)| {
                            let name = format!("{} Storage", resource.name);
                            filter.match_name(&name)
                        })
                        .collect();

                    if !resource_list.is_empty() {
                        let group_title = format!("Storage: {}", group);

                        let items: Box<dyn FnOnce(&mut Ui)> = Box::new(|ui| {
                            ui.vertical_centered_justified(|ui| {
                                for (id, resource) in resource_list.iter() {
                                    let name = format!("{} Storage", resource.name);

                                    if button(
                                        ui,
                                        &name,
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
                                }
                            });
                        });

                        if open_groups {
                            ui.label(group_title);
                            items(ui);
                        } else {
                            egui::CollapsingHeader::new(group_title)
                                .default_open(open_groups)
                                .show(ui, |ui| items(ui));
                        }
                    }
                }

                for group in group_names.iter() {
                    let resource_list: Vec<&(&String, &ResourceSpecification)> = groups
                        .get_all(group)
                        .unwrap()
                        .iter()
                        .filter(|(_id, resource)| {
                            let name = format!("{} Truck", resource.name);
                            filter.match_name(&name)
                        })
                        .collect();

                    if !resource_list.is_empty() {
                        let group_title = format!("Transport: {}", group);

                        let items: Box<dyn FnOnce(&mut Ui)> = Box::new(|ui| {
                            ui.vertical_centered_justified(|ui| {
                                for (id, resource) in resource_list.iter() {
                                    let name = format!("{} Truck", resource.name);

                                    if button(
                                        ui,
                                        &name,
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
                                }
                            });
                        });

                        if open_groups {
                            ui.label(group_title);
                            items(ui);
                        } else {
                            egui::CollapsingHeader::new(group_title)
                                .default_open(open_groups)
                                .show(ui, |ui| items(ui));
                        }
                    }
                }
            });
        });
}
