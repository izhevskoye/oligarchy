mod load_game;
mod save_game;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};
use serde::{Deserialize, Serialize};

use crate::game::{
    assets::{
        BlastFurnace, CokeFurnace, ExportStation, Name, OxygenConverter, Quarry, Storage, Street,
    },
    car::Car,
};

#[derive(Serialize, Deserialize)]
pub enum Building {
    Quarry(Quarry),
    Storage(Storage),
    CokeFurnace(CokeFurnace),
    BlastFurnace(BlastFurnace),
    ExportStation(ExportStation),
    OxygenConverter(OxygenConverter),
    Street(Street),
}

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    car: Car,
    storage: Storage,
}

#[derive(Serialize, Deserialize)]
pub enum GameEntityType {
    Building(Building),
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
    pub entities: Vec<GameEntity>,
}

#[allow(clippy::type_complexity)]
pub fn save_ui(
    mut commands: Commands,
    queries: (
        Query<&Name>,
        Query<&Quarry>,
        Query<&Storage>,
        Query<&CokeFurnace>,
        Query<&BlastFurnace>,
        Query<&ExportStation>,
        Query<&OxygenConverter>,
        Query<&Street>,
        Query<(Entity, &Car)>,
    ),
    egui_context: ResMut<EguiContext>,
    mut map_query: MapQuery,
) {
    egui::Window::new("Game")
        .anchor(Align2::RIGHT_BOTTOM, [-10.0, -10.0])
        .show(egui_context.ctx(), |ui| {
            if ui.button("save").clicked() {
                save_game::save_game(&queries, &mut map_query);
            }
            if ui.button("load").clicked() {
                load_game::load_game(&mut commands, &mut map_query, &queries.8);
            }
        });
}
