use crate::game::{
    assets::StateName,
    state_manager::{LoadGameEvent, SaveGameEvent},
    ui::state::{emit_load_game::emit_load_game, ConfirmDialogState, MainMenuState, SaveGameList},
    AppState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};
use uuid::Uuid;

pub fn load_save_game_menu(
    mut commands: Commands,
    egui_context: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
    mut menu_state: ResMut<State<MainMenuState>>,
    mut load_game: EventWriter<LoadGameEvent>,
    mut save_game: EventWriter<SaveGameEvent>,
    mut state_name: ResMut<StateName>,
    mut save_game_list: ResMut<SaveGameList>,
    mut confirm_dialog: ResMut<ConfirmDialogState>,
) {
    let (title, button_title) = match menu_state.current() {
        MainMenuState::Load => ("Load Game", "Load"),
        MainMenuState::Save => ("Save Game", "Save"),
        _ => unreachable!("cannot happen"),
    };

    let mut invalidate_files = false;

    egui::Window::new(title)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .show(egui_context.ctx(), |ui| {
            if MainMenuState::Save == *menu_state.current() {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut state_name.name);
                });

                ui.separator();

                ui.vertical_centered_justified(|ui| {
                    if ui.button("New Save").clicked() {
                        let file_name = format!(
                            "{}/{}.yml",
                            save_game_list.save_game_path,
                            Uuid::new_v4().to_string()
                        );

                        if !state_name.name.is_empty() {
                            save_game.send(SaveGameEvent { file_name });
                            menu_state.pop().unwrap();
                            invalidate_files = true;
                        }
                    }
                });
            }

            if save_game_list.files.is_empty() {
                ui.label("No save games yet");
            } else {
                egui::Grid::new("file list").show(ui, |ui| {
                    for (name, file_name) in &save_game_list.files {
                        ui.label(name);

                        if ui.button(&button_title).clicked() {
                            if MainMenuState::Load == *menu_state.current() {
                                app_state.push(AppState::InGame).unwrap();
                                emit_load_game(&mut commands, &mut load_game, &file_name);
                            }

                            if MainMenuState::Save == *menu_state.current()
                                && !state_name.name.is_empty()
                            {
                                save_game.send(SaveGameEvent {
                                    file_name: file_name.to_owned(),
                                });
                                invalidate_files = true;
                            }

                            menu_state.pop().unwrap();
                        }

                        if ui.button("Delete").clicked() {
                            *confirm_dialog = ConfirmDialogState::DeleteFile(file_name.to_owned());
                            menu_state.push(MainMenuState::ConfirmDialog).unwrap();
                            invalidate_files = true;
                        }

                        ui.end_row();
                    }
                });
            }

            ui.separator();

            if ui.button("Abort").clicked() {
                menu_state.pop().unwrap();
            }
        });

    if invalidate_files {
        save_game_list.update_list();
    }
}
