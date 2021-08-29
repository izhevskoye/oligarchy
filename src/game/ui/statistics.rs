use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Ui},
    EguiContext,
};
use num_format::{Locale, ToFormattedString};

use crate::game::{
    assets::resource_specifications::ResourceSpecifications,
    constants::UNIT,
    current_selection::CurrentlySelected,
    statistics::{StatisticTracker, Statistics},
};

fn group(
    title: &str,
    tracker: &StatisticTracker,
    resources: &ResourceSpecifications,
    ui: &mut Ui,
) -> bool {
    if !tracker.is_empty() {
        egui::CollapsingHeader::new(title).show(ui, |ui| {
            egui::Grid::new(title).show(ui, |ui| {
                for (resource, amount) in tracker.get_all() {
                    let resource = resources.get(resource).unwrap();
                    ui.label(&resource.name);
                    ui.label(format!(
                        "{} {}",
                        (*amount as i64).to_formatted_string(&Locale::en),
                        UNIT
                    ));

                    ui.end_row();
                }
            });
        });

        true
    } else {
        false
    }
}

pub fn statistics_ui(
    egui_context: ResMut<EguiContext>,
    statistics_query: Query<&Statistics>,
    currently_selected: Res<CurrentlySelected>,
    resources: Res<ResourceSpecifications>,
) {
    if !currently_selected.statistics {
        return;
    }

    if let Some(entity) = currently_selected.entity {
        if let Ok(statistics) = statistics_query.get(entity) {
            egui::Window::new("Statistics").show(egui_context.ctx(), |ui| {
                if !group("Exported", &statistics.export, &resources, ui)
                    && !group("Imported", &statistics.import, &resources, ui)
                    && !group("Production", &statistics.production, &resources, ui)
                    && !group("Consumption", &statistics.consumption, &resources, ui)
                {
                    ui.label("No statistics");
                }
            });
        }
    }
}
