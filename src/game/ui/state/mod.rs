mod emit_load_game;
mod get_state_name;

use crate::game::{
    assets::{MapSettings, MapSize, StateName},
    state_manager::{helper::generate_save_game_path, LoadGameEvent, NewGameEvent, SaveGameEvent},
    AppState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};
use glob::glob;
use uuid::Uuid;

use std::fs::remove_file;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ConfirmDialogState {
    DeleteFile(String),
    ExitGame,
    ExitProgram,
}

impl Default for ConfirmDialogState {
    fn default() -> Self {
        Self::ExitProgram
    }
}

pub struct SaveGameList {
    save_game_path: String,
    files: Vec<(String, String)>,
}

impl Default for SaveGameList {
    fn default() -> Self {
        let save_game_path = generate_save_game_path();

        Self {
            files: load_file_list(&save_game_path),
            save_game_path,
        }
    }
}

impl SaveGameList {
    pub fn update_list(&mut self) {
        self.files = load_file_list(&self.save_game_path);
    }
}

fn load_file_list(save_game_path: &str) -> Vec<(String, String)> {
    let mut list = vec![];

    for file in glob(&format!("{}/*.yml", save_game_path)).expect("Failed to read files") {
        let file = file
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let file_name = format!("{}/{}", save_game_path, file);

        if let Some(name) = get_state_name::get_state_name(&file_name) {
            list.push((name, file_name));
        }
    }
    list.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    list
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MainMenuState {
    Main,
    New,
    Load,
    Save,
    ConfirmDialog,
}

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

pub fn new_game_menu(
    mut commands: Commands,
    mut new_game: EventWriter<NewGameEvent>,
    mut app_state: ResMut<State<AppState>>,
    mut menu_state: ResMut<State<MainMenuState>>,
    egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("New Game")
        .default_width(100.0)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .show(egui_context.ctx(), |ui| {
            ui.vertical_centered_justified(|ui| {
                if ui.button("Size: Small").clicked() {
                    commands.insert_resource(MapSettings {
                        width: 3,
                        height: 3,
                        size: MapSize::Small,
                    });

                    new_game.send(NewGameEvent);
                    app_state.push(AppState::InGame).unwrap();
                    menu_state.pop().unwrap();
                }

                if ui.button("Size: Medium").clicked() {
                    commands.insert_resource(MapSettings {
                        width: 5,
                        height: 5,
                        size: MapSize::Medium,
                    });

                    new_game.send(NewGameEvent);
                    app_state.push(AppState::InGame).unwrap();
                    menu_state.pop().unwrap();
                }

                if ui.button("Size Large").clicked() {
                    commands.insert_resource(MapSettings {
                        width: 8,
                        height: 8,
                        size: MapSize::Large,
                    });

                    new_game.send(NewGameEvent);
                    app_state.push(AppState::InGame).unwrap();
                    menu_state.pop().unwrap();
                }

                ui.separator();

                if ui.button("Abort").clicked() {
                    menu_state.pop().unwrap();
                }
            });
        });
}

pub struct SaveGamePath(String);

impl Default for SaveGamePath {
    fn default() -> Self {
        Self(generate_save_game_path())
    }
}

pub fn load_save_game_menu(
    mut commands: Commands,
    egui_context: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
    mut menu_state: ResMut<State<MainMenuState>>,
    mut load_game: EventWriter<LoadGameEvent>,
    mut save_game: EventWriter<SaveGameEvent>,
    mut state_name: ResMut<StateName>,
    mut save_game_list: ResMut<SaveGameList>,
    save_game_path: Local<SaveGamePath>,
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
                        let file_name =
                            format!("{}/{}.yml", save_game_path.0, Uuid::new_v4().to_string());

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
                                emit_load_game::emit_load_game(
                                    &mut commands,
                                    &mut load_game,
                                    &file_name,
                                );
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
                            if let AppState::InGame = app_state.current() {
                                let _ = app_state.pop();
                            }
                        }
                        ConfirmDialogState::ExitProgram => {
                            std::process::exit(0);
                        }
                    }
                }
            });
        });
}
