use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::{
    assets::{Building, ProductionBuilding},
    building_specifications::BuildingSpecifications,
    current_selection::CurrentlySelected,
    resource_specifications::ResourceSpecifications,
};

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
            egui::Window::new("ProductionBuilding").show(egui_context.ctx(), |ui| {
                let building = buildings.get(&building.id).unwrap();
                ui.heading(&building.name);

                for (index, product) in production.products.clone().iter().enumerate() {
                    let resource = resources.get(&product.resource).unwrap();

                    ui.radio_value(
                        &mut production.active_product,
                        index,
                        format!("Produce {} {}", product.rate, resource.name),
                    );

                    if !product.byproducts.is_empty() {
                        ui.label("It will also optionally produce:");
                        for byproduct in product.byproducts.iter() {
                            let resource = resources.get(&byproduct.resource).unwrap();
                            ui.label(format!("{} {}", byproduct.rate, resource.name));
                        }
                    }

                    if !product.requisites.is_empty() {
                        ui.label("This will consume:");
                        for requisite in product.requisites.iter() {
                            let resource = resources.get(&requisite.resource).unwrap();
                            ui.label(format!("{} {}", requisite.rate, resource.name));
                        }
                    }

                    if !product.enhancers.is_empty() {
                        ui.label("If the following is provided, it increases output:");
                        for enhancer in product.enhancers.iter() {
                            let resource = resources.get(&enhancer.resource).unwrap();
                            ui.label(format!(
                                "{} {} by {}x",
                                enhancer.rate, resource.name, enhancer.modifier,
                            ));
                        }
                    }
                }
            });
        }
    }
}
