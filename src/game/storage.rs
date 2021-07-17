use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::{RequiresUpdate, Resource, Storage, StorageConsolidator},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

pub fn distribute_to_storage(
    consolidator: &StorageConsolidator,
    storage_query: &mut Query<&mut Storage>,
    resource: Resource,
) {
    for storage in consolidator.connected_storage.iter() {
        let ref mut storage = storage_query.get_mut(*storage).unwrap();

        if storage.resource == resource && storage.amount < storage.capacity {
            storage.amount += 1;
            println!("Put {:?} into storage! {}", resource, storage.amount);
            return;
        }
    }

    panic!("Expected some storage to accept resource");
}

pub fn has_in_storage(
    consolidator: &StorageConsolidator,
    storage_query: &mut Query<&mut Storage>,
    resource: Resource,
) -> bool {
    for storage in consolidator.connected_storage.iter() {
        let ref mut storage = storage_query.get_mut(*storage).unwrap();

        if storage.resource == resource && storage.amount > 0 {
            return true;
        }
    }

    println!("No {:?} in storage!", resource);
    false
}

pub fn has_space_in_storage(
    consolidator: &StorageConsolidator,
    storage_query: &mut Query<&mut Storage>,
    resource: Resource,
) -> bool {
    for storage in consolidator.connected_storage.iter() {
        let ref mut storage = storage_query.get_mut(*storage).unwrap();

        if storage.resource == resource && storage.amount < storage.capacity {
            return true;
        }
    }

    false
}

pub fn fetch_from_storage(
    consolidator: &StorageConsolidator,
    storage_query: &mut Query<&mut Storage>,
    resource: Resource,
) -> bool {
    for storage in consolidator.connected_storage.iter() {
        let ref mut storage = storage_query.get_mut(*storage).unwrap();

        if storage.resource == resource && storage.amount > 0 {
            storage.amount -= 1;
            println!("Get {:?} from storage! {}", resource, storage.amount);
            return true;
        }
    }

    false
}

pub fn update_consolidators(
    map_query: MapQuery,
    storage_query: Query<(Entity, &Storage)>,
    mut consolidator_query: Query<(&mut StorageConsolidator, &RequiresUpdate)>,
) {
    for (mut consolidator, update) in consolidator_query.iter_mut() {
        let neighbors = map_query.get_tile_neighbors(update.position, MAP_ID, BUILDING_LAYER_ID);

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
