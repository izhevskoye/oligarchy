use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};

use crate::game::{
    ui::state::{ConfirmDialogState, MainMenuState},
    AppState,
};

pub fn pause_menu(
    keyboard_input: Res<Input<KeyCode>>,
    egui_context: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
    mut menu_state: ResMut<State<MainMenuState>>,
    mut confirm_dialog: ResMut<ConfirmDialogState>,
) {
    if let AppState::Paused = app_state.current() {
        if let MainMenuState::Main = menu_state.current() {
            egui::Window::new("Paused")
                .default_width(100.0)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .collapsible(false)
                .show(egui_context.ctx(), |ui| {
                    ui.vertical_centered_justified(|ui| {
                        if ui.button("Continue").clicked() {
                            let _ = app_state.pop().unwrap();
                        }

                        ui.separator();

                        if ui.button("Save Game").clicked() {
                            menu_state.push(MainMenuState::Save).unwrap();
                        }

                        if ui.button("Back to Menu").clicked() {
                            *confirm_dialog = ConfirmDialogState::ExitGame;
                            menu_state.push(MainMenuState::ConfirmDialog).unwrap();
                        }
                    });
                });
        }
    }

    if egui_context.ctx().wants_pointer_input() || egui_context.ctx().wants_keyboard_input() {
        return;
    }

    if keyboard_input.pressed(KeyCode::Escape) {
        if let AppState::InGame = app_state.current() {
            let _ = app_state.push(AppState::Paused);
            let _ = menu_state.set(MainMenuState::Main);
        }
    }
}
