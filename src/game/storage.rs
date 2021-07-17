use bevy::prelude::*;

use super::assets::{Resource, Storage, StorageConsolidator};

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
