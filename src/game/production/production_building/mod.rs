#[cfg(test)]
mod tests;

use bevy::prelude::*;

use crate::game::{
    assets::{ProductionBuilding, Storage, StorageConsolidator},
    storage::{distribute_to_storage, fetch_from_storage, has_in_storage, has_space_in_storage},
};

use super::Idle;

pub fn production_building(
    mut commands: Commands,
    building_query: Query<(
        Entity,
        &ProductionBuilding,
        &StorageConsolidator,
        Option<&Idle>,
    )>,
    mut storage_query: Query<&mut Storage>,
) {
    for (entity, building, consolidator, idle) in building_query.iter() {
        let product = &building.products[building.active_product];

        let has_requisites = product.requisites.iter().all(|requisite| {
            has_in_storage(
                &consolidator,
                &mut storage_query,
                &requisite.resource,
                requisite.rate,
            )
        });

        let mut modifier = 1.0;

        for enhancer in &product.enhancers {
            if has_in_storage(
                consolidator,
                &mut storage_query,
                &enhancer.resource,
                enhancer.rate,
            ) {
                modifier *= enhancer.modifier;
            }
        }

        if has_requisites
            && has_space_in_storage(
                &consolidator,
                &mut storage_query,
                &product.resource,
                product.rate * modifier,
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

            for enhancer in &product.enhancers {
                fetch_from_storage(
                    consolidator,
                    &mut storage_query,
                    &enhancer.resource,
                    enhancer.rate,
                );
            }

            distribute_to_storage(
                &consolidator,
                &mut storage_query,
                &product.resource,
                product.rate * modifier,
            );

            if idle.is_some() {
                commands.entity(entity).remove::<Idle>();
            }

            for byproduct in &product.byproducts {
                if has_space_in_storage(
                    consolidator,
                    &mut storage_query,
                    &byproduct.resource,
                    byproduct.rate * modifier,
                ) {
                    distribute_to_storage(
                        consolidator,
                        &mut storage_query,
                        &byproduct.resource,
                        byproduct.rate * modifier,
                    );
                }
            }
        } else if idle.is_none() {
            // not produced
            commands.entity(entity).insert(Idle::default());
        }
    }
}
