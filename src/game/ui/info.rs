use bevy::{ecs::query::QueryEntityError, prelude::*};
use bevy_egui::{egui, EguiContext};

use crate::game::{
    assets::{Editable, ExportStation, InfoUI, Name, Storage},
    car::Car,
    current_selection::CurrentlySelected,
};

fn query_resolve<'a, T>(items: &mut Vec<&'a dyn InfoUI>, item: Result<&'a T, QueryEntityError>)
where
    T: 'a + InfoUI,
{
    if let Ok(item) = item {
        items.push(item);
    }
}

// TODO: Building info needed!

#[allow(clippy::type_complexity)]
pub fn info_ui(
    egui_context: ResMut<EguiContext>,
    queries: (
        Query<&Editable>,
        Query<&Name>,
        Query<&Car>,
        Query<&Storage>,
        Query<&ExportStation>,
    ),
    mut currently_selected: ResMut<CurrentlySelected>,
) {
    if let Some(entity) = currently_selected.entity {
        let mut items: Vec<&dyn InfoUI> = vec![];

        query_resolve(&mut items, queries.1.get(entity));
        query_resolve(&mut items, queries.2.get(entity));
        query_resolve(&mut items, queries.3.get(entity));
        query_resolve(&mut items, queries.4.get(entity));

        if !items.is_empty() {
            egui::SidePanel::left("side_panel")
                .default_width(200.0)
                .show(egui_context.ctx(), |ui| {
                    ui.heading("Info");

                    for item in items {
                        item.ui(ui);
                    }

                    if queries.0.get(entity).is_ok() {
                        let label = if currently_selected.editing {
                            "close edit"
                        } else {
                            "edit"
                        };
                        if ui.button(label).clicked() {
                            currently_selected.editing = !currently_selected.editing;
                        }
                    }
                });
        }
    }
}
