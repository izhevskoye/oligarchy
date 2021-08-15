use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::{
    assets::{
        building_specifications::BuildingSpecifications,
        resource_specifications::ResourceSpecifications, Building,
    },
    constants::UNIT,
    current_selection::CurrentlySelected,
    production::ProductionBuilding,
};

fn resource_name(resource: &str, resources: &ResourceSpecifications) -> String {
    let resource = resources.get(resource).unwrap();

    if !resource.substitute.is_empty() {
        let items = resource
            .substitute
            .iter()
            .map(|(substitute, efficiency)| {
                let resource = resources.get(substitute).unwrap();
                format!("{} {}x", resource.name, efficiency)
            })
            .collect::<Vec<String>>()
            .join(", ");

        format!("{} ({})", resource.name, items)
    } else {
        resource.name.to_owned()
    }
}

pub fn edit_ui(
    egui_context: ResMut<EguiContext>,
    mut building_query: Query<(&mut ProductionBuilding, &Building)>,
    currently_selected: Res<CurrentlySelected>,
    resources: Res<ResourceSpecifications>,
    buildings: Res<BuildingSpecifications>,
) {
    if !currently_selected.editing {
        return;
    }

    if let Some(entity) = currently_selected.entity {
        if let Ok((mut production, building)) = building_query.get_mut(entity) {
            let building = buildings.get(&building.id).unwrap();

            egui::Window::new(&building.name).show(egui_context.ctx(), |ui| {
                for (product, active) in &mut production.products {
                    let resource = resources.get(&product.resource).unwrap();

                    ui.checkbox(
                        active,
                        format!("Produce {}{} {}", product.rate, UNIT, resource.name),
                    );

                    if !product.byproducts.is_empty() {
                        ui.label("It will also optionally produce:");
                        for byproduct in product.byproducts.iter() {
                            ui.label(format!(
                                "{}{} {}",
                                byproduct.rate,
                                UNIT,
                                resource_name(&byproduct.resource, &resources)
                            ));
                        }
                    }

                    if !product.requisites.is_empty() {
                        ui.label("This will consume:");
                        for requisite in product.requisites.iter() {
                            ui.label(format!(
                                "{}{} {}",
                                requisite.rate,
                                UNIT,
                                resource_name(&requisite.resource, &resources)
                            ));
                        }
                    }

                    if !product.enhancers.is_empty() {
                        ui.label("If the following is provided, it increases output:");
                        for enhancer in product.enhancers.iter() {
                            ui.label(format!(
                                "{}{} {} by {}x",
                                enhancer.rate,
                                UNIT,
                                resource_name(&enhancer.resource, &resources),
                                enhancer.modifier,
                            ));
                        }
                    }
                }
            });
        }
    }
}
