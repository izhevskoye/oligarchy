use bevy::{ecs::query::QueryEntityError, prelude::*};
use bevy_egui::{egui, EguiContext};

use crate::game::{
    assets::{
        BlastFurnace, CokeFurnace, CurrentlySelected, ExportStation, InfoUI, Name, OxygenConverter,
        Quarry, Storage,
    },
    car::Car,
};

fn query_resolve<'a, T>(items: &mut Vec<&'a dyn InfoUI>, item: Result<&'a T, QueryEntityError>)
where
    T: 'a + InfoUI,
{
    if let Ok(item) = item {
        items.push(item);
    }
}

#[allow(clippy::type_complexity)]
pub fn info_ui(
    egui_context: ResMut<EguiContext>,
    queries: (
        Query<&Name>,
        Query<&Car>,
        Query<&Storage>,
        Query<&ExportStation>,
        Query<&Quarry>,
        Query<&CokeFurnace>,
        Query<&BlastFurnace>,
        Query<&OxygenConverter>,
    ),
    currently_selected: Res<CurrentlySelected>,
) {
    if let Some(entity) = currently_selected.entity {
        let mut items: Vec<&dyn InfoUI> = vec![];

        query_resolve(&mut items, queries.0.get(entity));
        query_resolve(&mut items, queries.1.get(entity));
        query_resolve(&mut items, queries.2.get(entity));
        query_resolve(&mut items, queries.3.get(entity));
        query_resolve(&mut items, queries.4.get(entity));
        query_resolve(&mut items, queries.5.get(entity));
        query_resolve(&mut items, queries.6.get(entity));
        query_resolve(&mut items, queries.7.get(entity));

        if !items.is_empty() {
            egui::SidePanel::left("side_panel")
                .default_width(200.0)
                .show(egui_context.ctx(), |ui| {
                    ui.heading("Info");

                    for item in items {
                        item.ui(ui);
                    }
                });
        }
    }
}
