use crate::game::{
    assets::{MapSettings, MapSize},
    state_manager::{helper::generate_save_game_path, GameState, LoadGameEvent, SaveGameEvent},
    AppState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};
use glob::glob;
use uuid::Uuid;

use std::{fs::File, io::prelude::*, path::Path};

#[derive(PartialEq, Eq)]
pub enum SubMenuState {
    None,
    NewGameMenu,
    LoadGameMenu,
    SaveGameMenu,
}

pub struct MenuState {
    pub sub_menu_state: SubMenuState,
    pub save_game_path: String,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            sub_menu_state: SubMenuState::None,
            save_game_path: generate_save_game_path(),
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

#[allow(clippy::type_complexity)]
pub fn save_ui(
    mut commands: Commands,
    egui_context: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
    mut load_game: EventWriter<LoadGameEvent>,
    mut save_game: EventWriter<SaveGameEvent>,
    mut state: Local<MenuState>,
) {
    egui::Window::new("Game")
        .anchor(Align2::RIGHT_BOTTOM, [-10.0, -10.0])
        .show(egui_context.ctx(), |ui| {
            if let AppState::MainMenu = app_state.current() {
                if ui.button("new").clicked() {
                    state.sub_menu_state = SubMenuState::NewGameMenu;
                }

                if ui.button("load").clicked() {
                    state.sub_menu_state = SubMenuState::LoadGameMenu;
                }

                if ui.button("exit").clicked() {
                    std::process::exit(0);
                }
            }

            if let AppState::InGame = app_state.current() {
                if ui.button("save").clicked() {
                    state.sub_menu_state = SubMenuState::SaveGameMenu;
                }

                if ui.button("exit").clicked() {
                    if let AppState::InGame = app_state.current() {
                        let _ = app_state.pop();
                    }
                }
            }
        });

    if SubMenuState::LoadGameMenu == state.sub_menu_state
        || SubMenuState::SaveGameMenu == state.sub_menu_state
    {
        let title = match state.sub_menu_state {
            SubMenuState::LoadGameMenu => "Load Game",
            SubMenuState::SaveGameMenu => "Save Game",
            _ => unreachable!("cannot happen"),
        };

        egui::Window::new(title)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(egui_context.ctx(), |ui| {
                if SubMenuState::SaveGameMenu == state.sub_menu_state {
                    if ui.button("New Save").clicked() {
                        let file_name = format!(
                            "{}/{}.yml",
                            state.save_game_path,
                            Uuid::new_v4().to_string()
                        );

                        save_game.send(SaveGameEvent { file_name });
                        state.sub_menu_state = SubMenuState::None;
                    }
                }

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

                    if ui.button(&file).clicked() {
                        let file_name = format!("{}/{}", state.save_game_path, file);

                        if SubMenuState::LoadGameMenu == state.sub_menu_state {
                            app_state.push(AppState::InGame).unwrap();
                            emit_load_game(&mut commands, &mut load_game, &file_name);
                        }
                        if SubMenuState::SaveGameMenu == state.sub_menu_state {
                            save_game.send(SaveGameEvent { file_name });
                        }

                        state.sub_menu_state = SubMenuState::None;
                    }
                }

                if ui.button("Abort").clicked() {
                    state.sub_menu_state = SubMenuState::None;
                }
            });
    }

    if SubMenuState::NewGameMenu == state.sub_menu_state {
        egui::Window::new("New Game")
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(egui_context.ctx(), |ui| {
                if ui.button("small").clicked() {
                    commands.insert_resource(MapSettings {
                        width: 3,
                        height: 3,
                        size: MapSize::Small,
                    });

                    app_state.push(AppState::InGame).unwrap();
                    state.sub_menu_state = SubMenuState::None;
                }

                if ui.button("medium").clicked() {
                    commands.insert_resource(MapSettings {
                        width: 5,
                        height: 5,
                        size: MapSize::Medium,
                    });

                    app_state.push(AppState::InGame).unwrap();
                    state.sub_menu_state = SubMenuState::None;
                }

                if ui.button("large").clicked() {
                    commands.insert_resource(MapSettings {
                        width: 8,
                        height: 8,
                        size: MapSize::Large,
                    });

                    app_state.push(AppState::InGame).unwrap();
                    state.sub_menu_state = SubMenuState::None;
                }

                if ui.button("Abort").clicked() {
                    state.sub_menu_state = SubMenuState::None;
                }
            });
    }
}
