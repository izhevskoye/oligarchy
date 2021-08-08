pub mod load_game;
pub mod save_game;

use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::{
    account::Account,
    assets::{DeliveryStation, ExportStation, MapSettings, Name, Storage, Street},
    car::Car,
    goals::Goal,
    statistics::Statistics,
};

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
    pub statistics: Option<Statistics>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct GameState {
    pub settings: MapSettings,
    pub entities: Vec<GameEntity>,
    pub goals: HashMap<String, Goal>,
    pub account: Account,
}

pub struct LoadGameEvent {
    pub state: GameState,
}
pub struct SaveGameEvent;
