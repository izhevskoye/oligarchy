use bevy::prelude::*;
use bevy_egui::egui::Ui;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Resource {
    Coal,
    Coke,
    Limestone,
    IronOre,
    Iron,
    Steel,
}

pub struct Quarry {
    pub resource: Resource,
}

pub struct Storage {
    pub resource: Resource,
    pub amount: i64,
    pub capacity: i64,
}

#[derive(Default)]
pub struct StorageConsolidator {
    pub connected_storage: Vec<Entity>,
}

pub struct CokeFurnace;

pub struct BlastFurnace;

pub struct OxygenConverter;

pub struct RequiresUpdate {
    pub position: UVec2,
}

pub struct ExportStation {
    pub goods: Vec<Resource>,
}

pub struct Street;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    North,
    South,
    West,
    East,
    None,
}

#[derive(Default)]
pub struct CurrentlySelected {
    pub entity: Option<Entity>,
    pub locked: bool,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Tool {
    None,
    Bulldoze,
    Street,
    Storage(Resource),
    Quarry(Resource),
    CokeFurnace,
    BlastFurnace,
    OxygenConverter,
    ExportStation,
    Car(Resource),
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

pub struct Name(String);

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

impl InfoUI for Quarry {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("{:?} Quarry", self.resource));
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
