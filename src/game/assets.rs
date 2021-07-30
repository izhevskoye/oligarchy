use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy_egui::egui::Ui;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildingSpecification {
    pub name: String,
    pub tile: u16,
    pub products: Vec<Product>,
    pub group: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceSpecification {
    pub name: String,
    pub storage_tile: Option<u16>,
    pub group: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Building {
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Product {
    pub resource: String,
    // TODO:
    // pub rate: i64,
    #[serde(default)]
    pub requisites: Vec<String>,
}

// TODO: implicit through spec instead?
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProductionBuilding {
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Storage {
    pub resource: String,
    pub amount: i64,
    pub capacity: i64,
}

#[derive(Default)]
pub struct StorageConsolidator {
    pub connected_storage: Vec<Entity>,
}

pub struct RequiresUpdate {
    pub position: UVec2,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ExportStation {
    pub goods: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Street;

#[derive(PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    West,
    East,
    None,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Tool {
    None,
    Bulldoze,
    Street,
    Storage(String),
    ExportStation,
    Car(String),
    Building(String),
}

pub struct SelectedTool {
    pub tool: Tool,
}

impl Default for SelectedTool {
    fn default() -> Self {
        Self { tool: Tool::None }
    }
}

#[derive(Default)]
pub struct ClickedTile {
    pub dragging: bool,
    pub pos: Option<UVec2>,
    pub vehicle_pos: Option<UVec2>,
    pub occupied_building: bool,
    pub occupied_vehicle: bool,
}

pub struct Occupied;

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Name(String);

pub struct Editable;

pub trait InfoUI {
    fn ui(&self, _ui: &mut Ui) {}
}

impl InfoUI for Name {
    fn ui(&self, ui: &mut Ui) {
        ui.heading(&self.0);
    }
}

impl InfoUI for Storage {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(format!(
                "{:?} {} / {}",
                self.resource, self.amount, self.capacity
            ));
        });
    }
}

impl InfoUI for ExportStation {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("Export Station for {:?}", self.goods));
        });
    }
}

impl InfoUI for BuildingSpecification {
    fn ui(&self, ui: &mut Ui) {
        ui.heading(&self.name);
    }
}
