use crate::game::account::Account;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};
use num_format::{Locale, ToFormattedString};

pub fn account_ui(egui_context: ResMut<EguiContext>, account: Res<Account>) {
    egui::Window::new("Account")
        .anchor(Align2::LEFT_TOP, [10.0, 10.0])
        .show(egui_context.ctx(), |ui| {
            ui.label(account.value.to_formatted_string(&Locale::en));
        });
}
