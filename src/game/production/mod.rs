pub mod idle;
pub mod import_export_station;
pub mod production_building;
pub mod storage_management;

use std::collections::HashSet;

use bevy::prelude::*;
use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};

use super::{
    account::PurchaseCost,
    assets::{resource_specifications::ResourceSpecifications, InfoUI},
};

#[derive(Default)]
pub struct Idle {
    pub entity: Option<Entity>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProductionBuilding {
    pub products: Vec<(Product, bool)>,
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
    pub cost: f64,
    #[serde(default)]
    pub requisites: Vec<ProductDependency>,
    #[serde(default)]
    pub byproducts: Vec<ProductDependency>,
    #[serde(default)]
    pub enhancers: Vec<ProductEnhancer>,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum ImportExportDirection {
    Import,
    Export,
}

impl Default for ImportExportDirection {
    fn default() -> Self {
        ImportExportDirection::Export
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct ImportExportStation {
    pub direction: ImportExportDirection,
    pub goods: Vec<String>,
}

impl PurchaseCost for ImportExportStation {
    fn price(&self, _resources: &ResourceSpecifications) -> i64 {
        1200
    }
}

impl InfoUI for ImportExportStation {
    fn ui(&self, ui: &mut Ui, resources: &ResourceSpecifications) {
        ui.horizontal(|ui| {
            ui.label(match self.direction {
                ImportExportDirection::Export => "Export Station for:",
                ImportExportDirection::Import => "Import Station for:",
            });
        });

        for resource in self.goods.iter() {
            let resource = resources.get(resource).unwrap();

            ui.horizontal(|ui| {
                ui.label(&resource.name);
            });
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Depot {
    pub deliveries: HashSet<UVec2>,
    pub pickups: HashSet<UVec2>,
}

impl PurchaseCost for Depot {
    fn price(&self, _resources: &ResourceSpecifications) -> i64 {
        1200
    }
}

impl InfoUI for Depot {
    fn ui(&self, ui: &mut Ui, _resources: &ResourceSpecifications) {
        ui.horizontal(|ui| {
            ui.label("Depot");
        });
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct DeliveryStation;

impl PurchaseCost for DeliveryStation {
    fn price(&self, _resources: &ResourceSpecifications) -> i64 {
        250
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct StorageManagement;

impl PurchaseCost for StorageManagement {
    fn price(&self, _resources: &ResourceSpecifications) -> i64 {
        1200
    }
}

impl InfoUI for StorageManagement {
    fn ui(&self, ui: &mut Ui, _resources: &ResourceSpecifications) {
        ui.horizontal(|ui| {
            ui.label("Storage Management");
        });
    }
}
