use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::game::resource_specifications::ResourceSpecifications;

pub struct RemovedBuildingEvent {
    pub position: UVec2,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MapSize {
    Small,
    Medium,
    Large,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct MapSettings {
    pub width: u32,
    pub height: u32,
    pub size: MapSize,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            width: 3,
            height: 3,
            size: MapSize::Small,
        }
    }
}

pub struct Position {
    pub position: UVec2,
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

pub struct RequiresUpdate;

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ExportStation {
    pub goods: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeliveryStation;

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
    DeliveryStation,
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
pub struct CanDriveOver;

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
