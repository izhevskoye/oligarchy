pub mod helper;
pub mod load_game;
pub mod save_game;

use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::{
    account::Account,
    assets::{Direction, MapSettings, Name, StateName},
    car::UserController,
    construction::UnderConstruction,
    goals::Goal,
    production::{DeliveryStation, Depot, ExportStation},
    statistics::Statistics,
    storage::Storage,
    street::Street,
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SerializedBuilding {
    pub id: String,
    pub active_products: Vec<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum BuildingEntity {
    Storage(Storage),
    ExportStation(ExportStation),
    DeliveryStation(DeliveryStation),
    Depot(Depot),
    Street(Street),
    Building(SerializedBuilding),
}

#[derive(Serialize, Deserialize)]
pub enum VehicleController {
    UserControlled(UserController),
    DepotControlled(String),
}

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    direction: Direction,
    controller: VehicleController,
    storage: Storage,
}

#[derive(Serialize, Deserialize)]
pub enum GameEntityType {
    Building(BuildingEntity),
    Vehicle(Vehicle),
}

#[derive(Serialize, Deserialize)]
pub struct GameEntity {
    pub uuid: String,
    pub pos: UVec2,
    pub entity: GameEntityType,
    pub name: Option<Name>,
    pub statistics: Option<Statistics>,
    pub under_construction: Option<UnderConstruction>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct GameState {
    pub state_name: StateName,
    pub settings: MapSettings,
    pub entities: Vec<GameEntity>,
    pub goals: HashMap<String, Goal>,
    pub account: Account,
}

pub struct LoadGameEvent {
    pub state: GameState,
}

pub struct SaveGameEvent {
    pub file_name: String,
}
