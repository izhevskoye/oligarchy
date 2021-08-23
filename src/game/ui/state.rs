use crate::game::{
    assets::{MapSettings, MapSize, StateName},
    state_manager::{
        helper::generate_save_game_path, GameState, LoadGameEvent, NewGameEvent, SaveGameEvent,
    },
    AppState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};
use glob::glob;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::{
    collections::HashMap,
    fs::{remove_file, File},
    io::prelude::*,
    path::Path,
};

#[derive(PartialEq, Eq)]
pub enum SubMenuState {
    None,
    NewGameMenu,
    LoadGameMenu,
    SaveGameMenu,
}

pub enum ConfirmDialog {
    DeleteFile(String),
    ExitGame,
    ExitProgram,
}

pub struct MenuState {
    pub sub_menu_state: SubMenuState,
    pub save_game_path: String,
    pub confirm_dialog: Option<ConfirmDialog>,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            sub_menu_state: SubMenuState::None,
            save_game_path: generate_save_game_path(),
            confirm_dialog: None,
        }
    }
}

pub fn emit_load_game(
    commands: &mut Commands,
    load_game: &mut EventWriter<LoadGameEvent>,
    file_name: &str,
) {
    let path = Path::new(file_name);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            log::error!("Could not read file: {}", why);
            return;
        }
    };

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let state: Result<GameState, serde_yaml::Error> = serde_yaml::from_str(&content);

    match state {
        Ok(state) => {
            commands.insert_resource(state.settings.clone());
            load_game.send(LoadGameEvent { state })
        }
        Err(why) => log::error!("Could not load state: {}", why),
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct SaveGameState {
    pub state_name: StateName,
}

pub fn get_state_name(file_name: &str) -> Option<String> {
    let path = Path::new(file_name);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            log::error!("Could not read file: {}", why);
            return None;
        }
    };

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let state: Result<SaveGameState, serde_yaml::Error> = serde_yaml::from_str(&content);

    match state {
        Ok(state) => Some(state.state_name.name),
        Err(why) => {
            log::error!("Could not load state: {}", why);
            None
        }
    }
}

pub fn save_ui(
    mut commands: Commands,
    egui_context: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
    mut new_game: EventWriter<NewGameEvent>,
    mut load_game: EventWriter<LoadGameEvent>,
    mut save_game: EventWriter<SaveGameEvent>,
    mut state: Local<MenuState>,
    mut state_name: ResMut<StateName>,
    mut files: Local<Option<HashMap<String, String>>>,
) {
    if let Some(confirm_dialog) = &state.confirm_dialog {
        let mut close_confirm = false;

        egui::Window::new("Game")
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(150.0)
            .resizable(false)
            .collapsible(false)
            .show(egui_context.ctx(), |ui| {
                ui.label("Are you sure?");

                ui.horizontal(|ui| {
                    if ui.small_button("No").clicked() {
                        close_confirm = true;
                    }
                    if ui.small_button("Yes").clicked() {
                        close_confirm = true;

                        match confirm_dialog {
                            ConfirmDialog::DeleteFile(file_name) => {
                                let _ = remove_file(&file_name);
                            }
                            ConfirmDialog::ExitGame => {
                                if let AppState::InGame = app_state.current() {
                                    let _ = app_state.pop();
                                }
                            }
                            ConfirmDialog::ExitProgram => {
                                std::process::exit(0);
                            }
                        }
                    }
                });
            });

        if close_confirm {
            state.confirm_dialog = None;
        } else {
            return;
        }
    }

    if SubMenuState::None == state.sub_menu_state {
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
                            state.sub_menu_state = SubMenuState::NewGameMenu;
                        }

                        if ui.button("Load Game").clicked() {
                            state.sub_menu_state = SubMenuState::LoadGameMenu;
                        }

                        if ui.button("Exit Game").clicked() {
                            state.confirm_dialog = Some(ConfirmDialog::ExitProgram);
                        }
                    }

                    if let AppState::InGame = app_state.current() {
                        if ui.button("Save Game").clicked() {
                            state.sub_menu_state = SubMenuState::SaveGameMenu;
                        }

                        if ui.button("Back to Menu").clicked() {
                            state.confirm_dialog = Some(ConfirmDialog::ExitGame);
                        }
                    }
                });
            });
    }

    if SubMenuState::LoadGameMenu == state.sub_menu_state
        || SubMenuState::SaveGameMenu == state.sub_menu_state
    {
        if files.is_none() {
            let mut list = HashMap::new();

            for file in
                glob(&format!("{}/*.yml", state.save_game_path)).expect("Failed to read files")
            {
                let file = file
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let file_name = format!("{}/{}", state.save_game_path, file);

                if let Some(name) = get_state_name(&file_name) {
                    list.insert(name, file_name);
                }
            }
            *files = Some(list);
        }

        let (title, button_title) = match state.sub_menu_state {
            SubMenuState::LoadGameMenu => ("Load Game", "Load"),
            SubMenuState::SaveGameMenu => ("Save Game", "Save"),
            _ => unreachable!("cannot happen"),
        };

        let mut invalidate_files = false;

        egui::Window::new(title)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .resizable(false)
            .collapsible(false)
            .show(egui_context.ctx(), |ui| {
                if SubMenuState::SaveGameMenu == state.sub_menu_state {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut state_name.name);
                    });

                    ui.separator();

                    ui.vertical_centered_justified(|ui| {
                        if ui.button("New Save").clicked() {
                            let file_name = format!(
                                "{}/{}.yml",
                                state.save_game_path,
                                Uuid::new_v4().to_string()
                            );

                            if !state_name.name.is_empty() {
                                save_game.send(SaveGameEvent { file_name });
                                state.sub_menu_state = SubMenuState::None;
                                invalidate_files = true;
                            }
                        }
                    });
                }

                if let Some(list) = files.as_ref() {
                    if list.is_empty() {
                        ui.label("No save games yet");
                    } else {
                        egui::Grid::new("file list").show(ui, |ui| {
                            let mut list = list.iter().collect::<Vec<(&String, &String)>>();
                            list.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

                            for (name, file_name) in list {
                                ui.label(name);

                                if ui.button(&button_title).clicked() {
                                    if SubMenuState::LoadGameMenu == state.sub_menu_state {
                                        app_state.push(AppState::InGame).unwrap();
                                        emit_load_game(&mut commands, &mut load_game, &file_name);
                                    }
                                    if SubMenuState::SaveGameMenu == state.sub_menu_state
                                        && !state_name.name.is_empty()
                                    {
                                        save_game.send(SaveGameEvent {
                                            file_name: file_name.to_owned(),
                                        });
                                        invalidate_files = true;
                                    }

                                    state.sub_menu_state = SubMenuState::None;
                                }

                                if ui.button("Delete").clicked() {
                                    state.confirm_dialog =
                                        Some(ConfirmDialog::DeleteFile(file_name.to_owned()));
                                    invalidate_files = true;
                                }

                                ui.end_row();
                            }
                        });
                    }
                }

                ui.separator();

                if ui.button("Abort").clicked() {
                    state.sub_menu_state = SubMenuState::None;
                }
            });

        if invalidate_files {
            *files = None;
        }
    }

    if SubMenuState::NewGameMenu == state.sub_menu_state {
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
                        state.sub_menu_state = SubMenuState::None;
                    }

                    if ui.button("Size: Medium").clicked() {
                        commands.insert_resource(MapSettings {
                            width: 5,
                            height: 5,
                            size: MapSize::Medium,
                        });

                        new_game.send(NewGameEvent);
                        app_state.push(AppState::InGame).unwrap();
                        state.sub_menu_state = SubMenuState::None;
                    }

                    if ui.button("Size Large").clicked() {
                        commands.insert_resource(MapSettings {
                            width: 8,
                            height: 8,
                            size: MapSize::Large,
                        });

                        new_game.send(NewGameEvent);
                        app_state.push(AppState::InGame).unwrap();
                        state.sub_menu_state = SubMenuState::None;
                    }

                    ui.separator();

                    if ui.button("Abort").clicked() {
                        state.sub_menu_state = SubMenuState::None;
                    }
                });
            });
    }
}
