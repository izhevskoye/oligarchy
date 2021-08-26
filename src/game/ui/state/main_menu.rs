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
    let (align, offset, width, collapsible) = match app_state.current() {
        AppState::InGame => (Align2::RIGHT_BOTTOM, [-10.0, -10.0], 50.0, true),
        _ => (Align2::CENTER_CENTER, [0.0, 0.0], 100.0, false),
    };

    egui::Window::new("Game")
        .anchor(align, offset)
        .default_width(width)
        .resizable(false)
        .collapsible(collapsible)
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

                if let AppState::InGame = app_state.current() {
                    if ui.button("Save Game").clicked() {
                        menu_state.push(MainMenuState::Save).unwrap();
                    }

                    if ui.button("Back to Menu").clicked() {
                        *confirm_dialog = ConfirmDialogState::ExitGame;
                        menu_state.push(MainMenuState::ConfirmDialog).unwrap();
                    }
                }
            });
        });
}
