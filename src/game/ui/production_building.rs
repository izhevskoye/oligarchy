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
                    let name = if product.requisites.is_empty() {
                        format!("{} ({:.2})", resource.name, product.rate)
                    } else {
                        let requisites = product
                            .requisites
                            .iter()
                            .map(|requisite| {
                                let resource = resources.get(&requisite.resource).unwrap();
                                format!("{} ({:.2})", resource.name, requisite.rate)
                            })
                            .collect::<Vec<String>>()
                            .join(", ");

                        format!(
                            "{} ({:.2}) from {}",
                            resource.name, product.rate, &requisites
                        )
                    };

                    ui.radio_value(&mut production.active_product, index, name);
                }
            });
        }
    }
}