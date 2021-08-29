pub mod building_specifications;
pub mod integrity;
pub mod resource_specifications;

use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::game::{
    constants::MapTile, helper::neighbor_structure::LayerIndex, setup::GROUND_LAYER_ID,
};
use resource_specifications::ResourceSpecifications;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct StateName {
    pub name: String,
}

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

#[derive(Debug)]
pub struct Position {
    pub position: UVec2,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Building {
    pub id: String,
}

pub struct RequiresUpdate;

#[derive(PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    West,
    East,
    None,
}

#[derive(Default)]
pub struct ClickedTile {
    pub dragging: bool,
    pub pos: Option<UVec2>,
    pub vehicle_pos: Option<UVec2>,
    pub occupied_building: bool,
    pub occupied_vehicle: bool,
    pub can_build: bool,
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
        ui.label(&self.name);
    }
}

pub struct BlockedForBuilding;

#[derive(Default)]
pub struct Forrest;

impl From<Forrest> for MapTile {
    fn from(_: Forrest) -> MapTile {
        MapTile::ForrestTilesOffset
    }
}

impl From<Forrest> for LayerIndex {
    fn from(_: Forrest) -> LayerIndex {
        LayerIndex::new(GROUND_LAYER_ID)
    }
}

#[derive(Default)]
pub struct Water;

impl From<Water> for MapTile {
    fn from(_: Water) -> MapTile {
        MapTile::WaterTilesOffset
    }
}

impl From<Water> for LayerIndex {
    fn from(_: Water) -> LayerIndex {
        LayerIndex::new(GROUND_LAYER_ID)
    }
}
