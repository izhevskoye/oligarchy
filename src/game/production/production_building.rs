use bevy::prelude::*;

use crate::game::{
    assets::{ProductionBuilding, Storage, StorageConsolidator},
    storage::{distribute_to_storage, has_space_in_storage},
};

pub fn production_building(
    building_query: Query<(&ProductionBuilding, &StorageConsolidator)>,
    mut storage_query: Query<&mut Storage>,
) {
    for (building, consolidator) in building_query.iter() {
        for product in &building.products {
            if has_space_in_storage(&consolidator, &mut storage_query, product.resource) {
                distribute_to_storage(&consolidator, &mut storage_query, product.resource);
            }
        }
    }
}
