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
    assets::{Building, ExportStation, Name, Storage, Street},
    building_specifications::BuildingSpecifications,
    car::Car,
};

#[derive(Serialize, Deserialize)]
pub enum BuildingEntity {
    Storage(Storage),
    ExportStation(ExportStation),
    Street(Street),
    Building(Building),
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
    pub entities: Vec<GameEntity>,
}

#[allow(clippy::type_complexity)]
pub fn save_ui(
    mut commands: Commands,
    queries: (
        Query<&Name>,
        Query<(Entity, &Car)>,
        Query<&Storage>,
        Query<&ExportStation>,
        Query<&Street>,
        Query<&Building>,
    ),
    egui_context: ResMut<EguiContext>,
    mut map_query: MapQuery,
    buildings: Res<BuildingSpecifications>,
) {
    egui::Window::new("Game")
        .anchor(Align2::RIGHT_BOTTOM, [-10.0, -10.0])
        .show(egui_context.ctx(), |ui| {
            if ui.button("save").clicked() {
                save_game::save_game(&queries, &mut map_query);
            }
            if ui.button("load").clicked() {
                load_game::load_game(&mut commands, &mut map_query, &queries.1, &buildings);
            }
        });
}
