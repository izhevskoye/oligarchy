#[cfg(test)]
mod tests;

use bevy_egui::egui::Ui;
use glob::glob;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::prelude::*, path::Path};

use crate::game::{
    account::PurchaseCost,
    assets::{resource_specifications::ResourceSpecifications, InfoUI},
    constants::CURRENCY,
    production::Product,
};

pub type BuildingSpecifications = HashMap<String, BuildingSpecification>;

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct BuildingSpecificationCost {
    #[serde(default)]
    pub resources: HashMap<String, f64>,
    pub base: f64,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct BuildingSpecification {
    pub name: String,
    pub tile: u16,
    pub products: Vec<Product>,
    pub group: String,
    pub cost: BuildingSpecificationCost,
}

impl PurchaseCost for BuildingSpecification {
    fn price(&self, resources: &ResourceSpecifications) -> i64 {
        (self.cost.base
            + self.cost.resources.iter().fold(0.0, |acc, (r, a)| {
                let resource = resources.get(r).unwrap();
                acc + resource.cost * a
            })) as i64
    }

    fn price_description(&self, resources: &ResourceSpecifications) -> String {
        let mut items = vec![];

        if self.cost.base > f64::EPSILON {
            items.push(format!("Labor worth {} {}", self.cost.base, CURRENCY));
        }

        for (resource, amount) in self.cost.resources.iter() {
            let resource = resources.get(resource).unwrap();
            items.push(format!(
                "{} {} worth {} {}",
                amount,
                resource.name,
                resource.cost * amount,
                CURRENCY
            ));
        }

        items.join("\n")
    }
}

impl InfoUI for BuildingSpecification {
    fn ui(&self, ui: &mut Ui, _resources: &ResourceSpecifications) {
        ui.label(&self.name);
    }
}

pub fn load_file(buildings: &mut BuildingSpecifications, file_name: &str) {
    let path = Path::new(file_name);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            log::error!("Could not read file: {}", why);
            return;
        }
    };

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let state: Result<BuildingSpecifications, serde_yaml::Error> = serde_yaml::from_str(&content);

    match state {
        Ok(state) => {
            for (id, building) in state.into_iter() {
                log::info!("load building spec {}", id);
                buildings.insert(id, building);
            }
        }
        Err(why) => log::error!("Could not load state: {}", why),
    }
}

pub fn load_specifications() -> BuildingSpecifications {
    let mut buildings = HashMap::new();
    for file in glob("assets/buildings/**/*.yml").expect("Failed to read files") {
        load_file(&mut buildings, &format!("{}", file.unwrap().display()));
    }
    buildings
}
