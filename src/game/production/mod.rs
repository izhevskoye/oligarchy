pub mod export_station;
pub mod idle;
pub mod production_building;

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
    pub requisites: Vec<ProductDependency>,
    #[serde(default)]
    pub byproducts: Vec<ProductDependency>,
    #[serde(default)]
    pub enhancers: Vec<ProductEnhancer>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct ExportStation {
    pub goods: Vec<String>,
}

impl PurchaseCost for ExportStation {
    fn price(&self, _resources: &ResourceSpecifications) -> i64 {
        1200
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

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct DeliveryStation;

impl PurchaseCost for DeliveryStation {
    fn price(&self, _resources: &ResourceSpecifications) -> i64 {
        250
    }
}
