pub mod load_game;
pub mod save_game;

use std::{fs::File, io::prelude::*, path::Path};

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};
use serde::{Deserialize, Serialize};

use crate::game::{
    assets::{ExportStation, MapSettings, Name, Storage, Street},
    car::Car,
};

use super::{assets::DeliveryStation, AppState};

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SerializedBuilding {
    pub id: String,
    pub active_product: usize,
}

#[derive(Serialize, Deserialize)]
pub enum BuildingEntity {
    Storage(Storage),
    ExportStation(ExportStation),
    DeliveryStation(DeliveryStation),
    Street(Street),
    Building(SerializedBuilding),
}

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    car: Car,
    storage: Storage,
}

#[derive(Serialize, Deserialize)]
pub enum GameEntityType {
    Building(BuildingEntity),
    Vehicle(Vehicle),
}

#[derive(Serialize, Deserialize)]
pub struct GameEntity {
    pub pos: UVec2,
    pub entity: GameEntityType,
    pub name: Option<Name>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct GameState {
    #[serde(default)]
    pub settings: MapSettings,
    pub entities: Vec<GameEntity>,
}

pub struct LoadGameEvent {
    state: GameState,
}
pub struct SaveGameEvent;

pub fn emit_load_game(commands: &mut Commands, load_game: &mut EventWriter<LoadGameEvent>) {
    let path = Path::new("world.yaml");
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

#[derive(PartialEq, Eq)]
pub enum MenuState {
    None,
    OpenMenu,
}

impl Default for MenuState {
    fn default() -> Self {
        Self::None
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
                    *state = MenuState::OpenMenu;
                }

                if ui.button("load").clicked() {
                    app_state.push(AppState::InGame).unwrap();
                    emit_load_game(&mut commands, &mut load_game);
                }

                if ui.button("exit").clicked() {
                    std::process::exit(0);
                }
            }

            if let AppState::InGame = app_state.current() {
                if ui.button("save").clicked() {
                    save_game.send(SaveGameEvent);
                }

                if ui.button("exit").clicked() {
                    if let AppState::InGame = app_state.current() {
                        let _ = app_state.pop();
                    }
                }
            }
        });

    if MenuState::None != *state {
        if let AppState::InGame = app_state.current() {
            *state = MenuState::None;
        }
    }

    if MenuState::OpenMenu == *state {
        egui::Window::new("New Game")
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(egui_context.ctx(), |ui| {
                if ui.button("small").clicked() {
                    commands.insert_resource(MapSettings {
                        width: 3,
                        height: 3,
                    });

                    app_state.push(AppState::InGame).unwrap();
                }

                if ui.button("medium").clicked() {
                    commands.insert_resource(MapSettings {
                        width: 5,
                        height: 5,
                    });

                    app_state.push(AppState::InGame).unwrap();
                }

                if ui.button("large").clicked() {
                    commands.insert_resource(MapSettings {
                        width: 8,
                        height: 8,
                    });

                    app_state.push(AppState::InGame).unwrap();
                }
            });
    }
}
