use crate::game::{
    ui::state::{ConfirmDialogState, MainMenuState, SaveGameList},
    AppState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};

use std::fs::remove_file;

pub fn confirm_dialog(
    egui_context: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
    mut menu_state: ResMut<State<MainMenuState>>,
    mut save_game_list: ResMut<SaveGameList>,
    confirm_dialog: Res<ConfirmDialogState>,
) {
    egui::Window::new("Confirmation")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .default_width(150.0)
        .resizable(false)
        .collapsible(false)
        .show(egui_context.ctx(), |ui| {
            ui.label("Are you sure?");

            ui.horizontal(|ui| {
                if ui.small_button("No").clicked() {
                    menu_state.pop().unwrap();
                }
                if ui.small_button("Yes").clicked() {
                    menu_state.pop().unwrap();

                    match confirm_dialog.clone() {
                        ConfirmDialogState::DeleteFile(file_name) => {
                            let _ = remove_file(&file_name);

                            save_game_list.update_list();
                        }
                        ConfirmDialogState::ExitGame => {
                            let _ = app_state.overwrite_replace(AppState::MainMenu);
                        }
                        ConfirmDialogState::ExitProgram => {
                            std::process::exit(0);
                        }
                    }
                }
            });
        });
}
