use bevy::prelude::*;

use crate::game::{
    assets::{CokeFurnace, Resource, Storage, StorageConsolidator},
    storage::{distribute_to_storage, fetch_from_storage, has_space_in_storage},
};

pub fn coke_furnace(
    furnace_query: Query<&StorageConsolidator, With<CokeFurnace>>,
    mut storage_query: Query<&mut Storage>,
) {
    for consolidator in furnace_query.iter() {
        if has_space_in_storage(&consolidator, &mut storage_query, Resource::Coke)
            && fetch_from_storage(&consolidator, &mut storage_query, Resource::Coal)
        {
            distribute_to_storage(&consolidator, &mut storage_query, Resource::Coke);
        }
    }
}
