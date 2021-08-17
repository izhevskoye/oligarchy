#[cfg(test)]
mod tests;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::egui::Ui;
use rand::{prelude::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::game::constants::UNIT;

use super::{
    account::PurchaseCost,
    assets::{resource_specifications::ResourceSpecifications, InfoUI, Position, RequiresUpdate},
    constants::STORAGE_SIZE,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Storage {
    pub resource: String,
    pub amount: f64,
    pub capacity: f64,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            resource: "".to_owned(),
            amount: 0.0,
            capacity: STORAGE_SIZE,
        }
    }
}

impl PurchaseCost for Storage {
    fn price(&self, resources: &ResourceSpecifications) -> i64 {
        let resource = resources
            .get(&self.resource)
            .unwrap_or_else(|| panic!("expected to find resource {} in spec", self.resource));

        ((resource.cost * self.capacity) / 25.0) as i64 + 1000
    }
}

impl InfoUI for Storage {
    fn ui(&self, ui: &mut Ui, resources: &ResourceSpecifications) {
        ui.horizontal(|ui| {
            let resource = resources.get(&self.resource).unwrap();

            ui.label(format!(
                "{} {:.2}{} / {:.2}{}",
                resource.name, self.amount, UNIT, self.capacity, UNIT,
            ));
        });
    }
}

#[derive(Default)]
pub struct StorageConsolidator {
    pub connected_storage: Vec<Entity>,
}

pub fn distribute_to_storage(
    consolidator: &StorageConsolidator,
    storage_query: &mut Query<&mut Storage>,
    resource: &str,
    amount: f64,
) {
    assert!(amount > 0.0);
    let mut amount_left = amount;

    let mut entities = consolidator.connected_storage.clone();
    let mut random = thread_rng();
    entities.shuffle(&mut random);

    for storage in entities.iter() {
        if let Ok(mut storage) = storage_query.get_mut(*storage) {
            if storage.resource == resource && storage.amount < storage.capacity {
                let space_left = storage.capacity - storage.amount;
                let amount_added = if space_left < amount_left {
                    space_left
                } else {
                    amount_left
                };

                storage.amount += amount_added;
                amount_left -= amount_added;
            }
        }

        if amount_left <= 0.0 {
            return;
        }
    }

    log::error!("Expected some storage to accept resource");
}

pub fn amount_in_storage(
    consolidator: &StorageConsolidator,
    storage_query: &mut Query<&mut Storage>,
    resource: &str,
) -> f64 {
    let mut amount = 0.0;

    for storage in consolidator.connected_storage.iter() {
        if let Ok(storage) = storage_query.get_mut(*storage) {
            if storage.resource == resource {
                amount += storage.amount
            }
        }
    }

    amount
}

pub fn has_in_storage(
    consolidator: &StorageConsolidator,
    storage_query: &mut Query<&mut Storage>,
    resource: &str,
    amount: f64,
) -> bool {
    assert!(amount >= 0.0);
    amount_in_storage(consolidator, storage_query, resource) >= amount
}

pub fn has_space_in_storage(
    consolidator: &StorageConsolidator,
    storage_query: &mut Query<&mut Storage>,
    resource: &str,
    amount: f64,
) -> bool {
    assert!(amount > 0.0);
    let mut amount_found = 0.0;
    for storage in consolidator.connected_storage.iter() {
        if let Ok(storage) = storage_query.get_mut(*storage) {
            if storage.resource == resource && storage.amount < storage.capacity {
                let space_left = storage.capacity - storage.amount;
                amount_found += space_left;
            }
        }

        if amount_found >= amount {
            return true;
        }
    }

    false
}

pub fn fetch_from_storage(
    consolidator: &StorageConsolidator,
    storage_query: &mut Query<&mut Storage>,
    resource: &str,
    amount: f64,
) -> bool {
    if !has_in_storage(consolidator, storage_query, resource, amount) {
        return false;
    }
    let mut amount_left = amount;

    let mut entities = consolidator.connected_storage.clone();
    let mut random = thread_rng();
    entities.shuffle(&mut random);

    for storage in entities.iter() {
        if let Ok(mut storage) = storage_query.get_mut(*storage) {
            if storage.resource == resource && storage.amount > 0.0 {
                let amount_taken = if storage.amount < amount_left {
                    storage.amount
                } else {
                    amount_left
                };
                storage.amount -= amount_taken;
                amount_left -= amount_taken;
            }
        }

        if amount_left <= 0.0 {
            return true;
        }
    }

    false
}

pub fn update_consolidators(
    map_query: MapQuery,
    storage_query: Query<(Entity, &Storage)>,
    mut consolidator_query: Query<(&mut StorageConsolidator, &Position), With<RequiresUpdate>>,
) {
    for (mut consolidator, position) in consolidator_query.iter_mut() {
        let neighbors = map_query.get_tile_neighbors(position.position, MAP_ID, BUILDING_LAYER_ID);

        let mut connected_storage = vec![];
        for (_, neighbor) in neighbors.iter() {
            if let Some(neighbor) = neighbor {
                if storage_query.get(*neighbor).is_ok() {
                    connected_storage.push(*neighbor);
                }
            }
        }

        consolidator.connected_storage = connected_storage;
    }
}
