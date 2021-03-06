use bevy::{ecs::query::QueryEntityError, prelude::*};
use bevy_egui::{egui, EguiContext};

use crate::game::{
    assets::{
        building_specifications::BuildingSpecifications,
        resource_specifications::ResourceSpecifications, Building, Editable, InfoUI, Name,
    },
    car::Car,
    construction::UnderConstruction,
    current_selection::CurrentlySelected,
    production::{Depot, ImportExportStation},
    statistics::Statistics,
    storage::Storage,
};

fn query_resolve<'a, T>(items: &mut Vec<&'a dyn InfoUI>, item: Result<&'a T, QueryEntityError>)
where
    T: 'a + InfoUI,
{
    if let Ok(item) = item {
        items.push(item);
    }
}

pub fn info_ui(
    buildings: Res<BuildingSpecifications>,
    egui_context: ResMut<EguiContext>,
    queries: (
        Query<&Building>,
        Query<&Editable>,
        Query<&Statistics>,
        Query<&Name>,
        Query<&Car>,
        Query<&Storage>,
        Query<&ImportExportStation>,
        Query<&Depot>,
        Query<&UnderConstruction>,
    ),
    mut currently_selected: ResMut<CurrentlySelected>,
    resources: Res<ResourceSpecifications>,
) {
    if let Some(entity) = currently_selected.entity {
        let mut items: Vec<&dyn InfoUI> = vec![];

        if let Ok(building) = queries.0.get(entity) {
            let building = buildings.get(&building.id).unwrap();

            items.push(building);
        }

        query_resolve(&mut items, queries.3.get(entity));
        query_resolve(&mut items, queries.4.get(entity));
        query_resolve(&mut items, queries.5.get(entity));
        query_resolve(&mut items, queries.6.get(entity));
        query_resolve(&mut items, queries.7.get(entity));
        query_resolve(&mut items, queries.8.get(entity));

        if !items.is_empty() {
            egui::SidePanel::left("side_panel")
                .default_width(200.0)
                .show(egui_context.ctx(), |ui| {
                    ui.heading("Info");

                    for item in items {
                        item.ui(ui, &resources);
                    }

                    if queries.1.get(entity).is_ok() {
                        let label = if currently_selected.editing {
                            "Edit (Close)"
                        } else {
                            "Edit"
                        };
                        if ui.button(label).clicked() {
                            currently_selected.editing = !currently_selected.editing;
                        }
                    }

                    let label = if currently_selected.renaming {
                        "Rename (Close)"
                    } else if queries.3.get(entity).is_err() {
                        "Give Name"
                    } else {
                        "Rename"
                    };
                    if ui.button(label).clicked() {
                        currently_selected.renaming = !currently_selected.renaming;
                        currently_selected.locked = !currently_selected.locked;
                    }

                    if queries.2.get(entity).is_ok() {
                        let label = if currently_selected.statistics {
                            "Statistics (Close)"
                        } else {
                            "Statistics"
                        };
                        if ui.button(label).clicked() {
                            currently_selected.statistics = !currently_selected.statistics;
                        }
                    }
                });
        }
    }
}
