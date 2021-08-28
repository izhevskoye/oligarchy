use crate::game::{
    ui::state::{ConfirmDialogState, MainMenuState},
    AppState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};

pub fn main_menu(
    app_state: Res<State<AppState>>,
    mut menu_state: ResMut<State<MainMenuState>>,
    egui_context: ResMut<EguiContext>,
    mut confirm_dialog: ResMut<ConfirmDialogState>,
) {
    if let AppState::MainMenu = app_state.current() {
        egui::Window::new("Game")
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(100.0)
            .resizable(false)
            .collapsible(false)
            .show(egui_context.ctx(), |ui| {
                ui.vertical_centered_justified(|ui| {
                    if let AppState::MainMenu = app_state.current() {
                        if ui.button("New Game").clicked() {
                            menu_state.push(MainMenuState::New).unwrap();
                        }

                        if ui.button("Load Game").clicked() {
                            menu_state.push(MainMenuState::Load).unwrap();
                        }

                        if ui.button("Exit Game").clicked() {
                            *confirm_dialog = ConfirmDialogState::ExitProgram;
                            menu_state.push(MainMenuState::ConfirmDialog).unwrap();
                        }
                    }
                });
            });
    }
}
