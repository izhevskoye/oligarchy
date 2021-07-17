use bevy::prelude::*;

use super::{
    assets::{Quarry, Storage, StorageConsolidator},
    storage::distribute_to_storage,
};

pub fn quarry(
    quarry_query: Query<(&Quarry, &StorageConsolidator)>,
    mut storage_query: Query<&mut Storage>,
) {
    for (quarry, consolidator) in quarry_query.iter() {
        println!("Quarry Prodution");

        distribute_to_storage(&consolidator, &mut storage_query, quarry.resource);
    }
}
