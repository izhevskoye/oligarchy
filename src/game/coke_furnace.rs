use bevy::prelude::*;

use super::{
    assets::{CokeFurnace, Resource, Storage, StorageConsolidator},
    storage::{distribute_to_storage, fetch_from_storage},
};

pub fn coke_furnace(
    furnace_query: Query<(&CokeFurnace, &StorageConsolidator)>,
    mut storage_query: Query<&mut Storage>,
) {
    for (_furnace, consolidator) in furnace_query.iter() {
        if fetch_from_storage(&consolidator, &mut storage_query, Resource::Coal) {
            println!("Furnace Cooking");
            distribute_to_storage(&consolidator, &mut storage_query, Resource::Coke);
        }
    }
}
