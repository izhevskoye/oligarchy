use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy_egui::egui::Ui;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum Resource {
    Coal,
    Coke,
    Limestone,
    IronOre,
    Iron,
    Steel,
}

#[derive(Deserialize)]
pub struct BuildingSpecification {
    pub name: String,
    pub tile: u16,
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Building {
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub resource: Resource,
    // TODO:
    // pub rate: i64,
}

// TODO: implicit through spec instead?
#[derive(Serialize, Deserialize, Clone)]
pub struct ProductionBuilding {
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Storage {
    pub resource: Resource,
    pub amount: i64,
    pub capacity: i64,
}

#[derive(Default)]
pub struct StorageConsolidator {
    pub connected_storage: Vec<Entity>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CokeFurnace;

#[derive(Serialize, Deserialize, Clone)]
pub struct BlastFurnace;

#[derive(Serialize, Deserialize, Clone)]
pub struct OxygenConverter;

pub struct RequiresUpdate {
    pub position: UVec2,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExportStation {
    pub goods: Vec<Resource>,
}

#[derive(Clone, Serialize, Deserialize)]
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
    Storage(Resource),
    CokeFurnace,
    BlastFurnace,
    OxygenConverter,
    ExportStation,
    Car(Resource),
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
    pub pos: Option<UVec2>,
    pub vehicle_pos: Option<UVec2>,
    pub occupied_building: bool,
    pub occupied_vehicle: bool,
}

pub struct Occupied;

#[derive(Clone, Serialize, Deserialize)]
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

impl InfoUI for Building {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("{:?}", self.id));
        });
    }
}

impl InfoUI for CokeFurnace {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Coke Furnace");
        });
    }
}

impl InfoUI for BlastFurnace {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Blast Furnace");
        });
    }
}

impl InfoUI for OxygenConverter {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Oxygen Converter");
        });
    }
}
