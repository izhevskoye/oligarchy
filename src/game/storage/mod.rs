#[cfg(test)]
mod tests;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{prelude::SliceRandom, thread_rng};

use super::{
    assets::{Position, RequiresUpdate, Storage, StorageConsolidator},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

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

pub fn has_in_storage(
    consolidator: &StorageConsolidator,
    storage_query: &mut Query<&mut Storage>,
    resource: &str,
    amount: f64,
) -> bool {
    assert!(amount > 0.0);
    let mut amount_needed = amount;

    for storage in consolidator.connected_storage.iter() {
        if let Ok(storage) = storage_query.get_mut(*storage) {
            if storage.resource == resource && storage.amount > 0.0 {
                amount_needed -= storage.amount
            }
        }

        if amount_needed <= 0.0 {
            return true;
        }
    }

    false
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
