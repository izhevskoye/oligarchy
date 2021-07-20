use bevy::prelude::*;

use crate::game::{
    assets::{Quarry, Storage, StorageConsolidator},
    storage::{distribute_to_storage, has_space_in_storage},
};

pub fn quarry(
    quarry_query: Query<(&Quarry, &StorageConsolidator)>,
    mut storage_query: Query<&mut Storage>,
) {
    for (quarry, consolidator) in quarry_query.iter() {
        if has_space_in_storage(&consolidator, &mut storage_query, quarry.resource) {
            distribute_to_storage(&consolidator, &mut storage_query, quarry.resource);
        }
    }
}
