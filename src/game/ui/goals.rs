use crate::game::goals::GoalManager;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};

pub fn goals_ui(egui_context: ResMut<EguiContext>, goals: Res<GoalManager>) {
    egui::Window::new("Goals")
        .anchor(Align2::LEFT_BOTTOM, [10.0, -10.0])
        .show(egui_context.ctx(), |ui| {
            for (resource, goal) in goals.goals.iter() {
                ui.label(format!("{}: {} / {}", resource, goal.current, goal.amount));
            }
        });
}
