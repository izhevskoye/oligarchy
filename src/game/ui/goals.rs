use crate::game::{goals::GoalManager, resource_specifications::ResourceSpecifications};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};
use num_format::{Locale, ToFormattedString};

pub fn goals_ui(
    egui_context: ResMut<EguiContext>,
    goals: Res<GoalManager>,
    resources: Res<ResourceSpecifications>,
) {
    if goals.goals.is_empty() {
        return;
    }

    egui::Window::new("Goals")
        .anchor(Align2::LEFT_BOTTOM, [10.0, -10.0])
        .show(egui_context.ctx(), |ui| {
            egui::Grid::new("goals").show(ui, |ui| {
                for (resource, goal) in goals.goals.iter() {
                    let resource = resources.get(resource).unwrap();
                    let resource = &resource.name;
                    ui.label(resource);
                    ui.label(format!(
                        "{} / {}",
                        (goal.current as i64).to_formatted_string(&Locale::en),
                        (goal.amount as i64).to_formatted_string(&Locale::en)
                    ));
                    ui.end_row();
                }
            });
        });
}
