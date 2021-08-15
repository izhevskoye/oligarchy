use crate::game::{
    account::Account,
    assets::resource_specifications::ResourceSpecifications,
    constants::{CURRENCY, UNIT},
    goals::GoalManager,
};
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
    account: Res<Account>,
) {
    egui::Window::new(format!(
        "{} {}",
        account.value.to_formatted_string(&Locale::en),
        CURRENCY,
    ))
    .anchor(Align2::LEFT_BOTTOM, [10.0, -10.0])
    .id(egui::Id::new("account_goals"))
    .show(egui_context.ctx(), |ui| {
        egui::Grid::new("goals").show(ui, |ui| {
            if goals.goals.is_empty() {
                ui.label("You have reached all goals.");
            } else {
                for (resource, goal) in goals.goals.iter() {
                    let resource = resources.get(resource).unwrap();
                    let resource = &resource.name;
                    ui.label(format!("Export {}", resource));
                    ui.label(format!(
                        "{}{} / {}{}",
                        (goal.current as i64).to_formatted_string(&Locale::en),
                        UNIT,
                        (goal.amount as i64).to_formatted_string(&Locale::en),
                        UNIT,
                    ));
                    ui.end_row();
                }
            }
        });
    });
}
