#[cfg(test)]
mod tests;

use bevy::prelude::*;

use crate::game::{
    assets::ProductionBuilding,
    statistics::Statistics,
    storage::{distribute_to_storage, fetch_from_storage, has_in_storage, has_space_in_storage},
    storage::{Storage, StorageConsolidator},
};

use super::Idle;

pub fn production_building(
    mut commands: Commands,
    mut building_query: Query<(
        Entity,
        &ProductionBuilding,
        &StorageConsolidator,
        &mut Statistics,
        Option<&Idle>,
    )>,
    mut storage_query: Query<&mut Storage>,
) {
    for (entity, building, consolidator, mut statistics, idle) in building_query.iter_mut() {
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

                statistics
                    .consumption
                    .track(&requisite.resource, requisite.rate);
            }

            for enhancer in &product.enhancers {
                if fetch_from_storage(
                    consolidator,
                    &mut storage_query,
                    &enhancer.resource,
                    enhancer.rate,
                ) {
                    statistics
                        .consumption
                        .track(&enhancer.resource, enhancer.rate);
                }
            }

            distribute_to_storage(
                &consolidator,
                &mut storage_query,
                &product.resource,
                product.rate * modifier,
            );

            statistics
                .production
                .track(&product.resource, product.rate * modifier);

            if let Some(idle) = idle {
                if let Some(entity) = idle.entity {
                    commands.entity(entity).despawn_recursive();
                }
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

                    statistics
                        .production
                        .track(&byproduct.resource, byproduct.rate * modifier);
                }
            }
        } else if idle.is_none() {
            // not produced
            commands.entity(entity).insert(Idle::default());
        }
    }
}
