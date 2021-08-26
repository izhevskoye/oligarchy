use crate::game::{
    assets::{MapSettings, MapSize},
    state_manager::NewGameEvent,
    ui::state::MainMenuState,
    AppState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};

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
