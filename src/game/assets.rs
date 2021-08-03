use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::game::{car::Car, resource_specifications::ResourceSpecifications};

pub struct RemovedBuildingEvent {
    pub position: UVec2,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct MapSettings {
    pub width: u32,
    pub height: u32,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            width: 3,
            height: 3,
        }
    }
}

pub struct Position {
    pub position: UVec2,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildingSpecification {
    pub name: String,
    pub tile: u16,
    pub products: Vec<Product>,
    pub group: String,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CarTileDefinition {
    pub horizontal: u16,
    pub vertical: u16,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceSpecification {
    pub name: String,
    pub storage_tile: Option<u16>,
    pub group: String,
    pub car_tile: Option<CarTileDefinition>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Building {
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProductDependency {
    pub resource: String,
    pub rate: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProductEnhancer {
    pub resource: String,
    pub rate: f64,
    pub modifier: f64,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Product {
    pub resource: String,
    pub rate: f64,
    #[serde(default)]
    pub requisites: Vec<ProductDependency>,
    #[serde(default)]
    pub byproducts: Vec<ProductDependency>,
    #[serde(default)]
    pub enhancers: Vec<ProductEnhancer>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProductionBuilding {
    pub products: Vec<Product>,
    pub active_product: usize,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Storage {
    pub resource: String,
    pub amount: f64,
    pub capacity: f64,
}

#[derive(Default)]
pub struct StorageConsolidator {
    pub connected_storage: Vec<Entity>,
}

pub struct RequiresUpdate;

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
pub struct Name {
    pub name: String,
}

pub struct Editable;

pub trait InfoUI {
    fn ui(&self, _ui: &mut Ui, _resources: &ResourceSpecifications) {}
}

impl InfoUI for Name {
    fn ui(&self, ui: &mut Ui, _resources: &ResourceSpecifications) {
        ui.heading(&self.name);
    }
}

impl InfoUI for Storage {
    fn ui(&self, ui: &mut Ui, resources: &ResourceSpecifications) {
        ui.horizontal(|ui| {
            let resource = resources.get(&self.resource).unwrap();

            ui.label(format!(
                "{} {:.2} / {:.2}",
                resource.name, self.amount, self.capacity
            ));
        });
    }
}

impl InfoUI for ExportStation {
    fn ui(&self, ui: &mut Ui, resources: &ResourceSpecifications) {
        ui.horizontal(|ui| {
            ui.label("Export Station for:");
        });

        for resource in self.goods.iter() {
            let resource = resources.get(resource).unwrap();

            ui.horizontal(|ui| {
                ui.label(&resource.name);
            });
        }
    }
}

impl InfoUI for BuildingSpecification {
    fn ui(&self, ui: &mut Ui, _resources: &ResourceSpecifications) {
        ui.heading(&self.name);
    }
}

impl InfoUI for Car {
    fn ui(&self, ui: &mut Ui, _resources: &ResourceSpecifications) {
        ui.horizontal(|ui| {
            ui.label("Car");
        });
    }
}
