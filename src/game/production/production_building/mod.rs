#[cfg(test)]
mod tests;

use bevy::prelude::*;

use crate::game::{
    assets::{ProductionBuilding, Storage, StorageConsolidator},
    storage::{distribute_to_storage, fetch_from_storage, has_in_storage, has_space_in_storage},
};

pub fn production_building(
    building_query: Query<(&ProductionBuilding, &StorageConsolidator)>,
    mut storage_query: Query<&mut Storage>,
) {
    for (building, consolidator) in building_query.iter() {
        for product in &building.products {
            let has_requisites = product.requisites.iter().all(|requisite| {
                has_in_storage(
                    &consolidator,
                    &mut storage_query,
                    &requisite.resource,
                    requisite.rate,
                )
            });

            if has_requisites
                && has_space_in_storage(
                    &consolidator,
                    &mut storage_query,
                    &product.resource,
                    product.rate,
                )
            {
                for requisite in &product.requisites {
                    fetch_from_storage(
                        consolidator,
                        &mut storage_query,
                        &requisite.resource,
                        requisite.rate,
                    );
                }

                distribute_to_storage(
                    &consolidator,
                    &mut storage_query,
                    &product.resource,
                    product.rate,
                );
            }
        }
    }
}
