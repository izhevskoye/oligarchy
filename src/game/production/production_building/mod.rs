#[cfg(test)]
mod tests;

use bevy::prelude::*;
use rand::{prelude::SliceRandom, thread_rng};

use crate::game::{
    account::AccountTransaction,
    assets::resource_specifications::ResourceSpecifications,
    construction::UnderConstruction,
    production::{Idle, ProductionBuilding},
    statistics::Statistics,
    storage::{distribute_to_storage, fetch_from_storage, has_in_storage, has_space_in_storage},
    storage::{Storage, StorageConsolidator},
};

pub fn production_building(
    mut commands: Commands,
    mut building_query: Query<
        (
            Entity,
            &ProductionBuilding,
            &StorageConsolidator,
            &mut Statistics,
            Option<&Idle>,
        ),
        Without<UnderConstruction>,
    >,
    mut storage_query: Query<&mut Storage>,
    resources: Res<ResourceSpecifications>,
    mut events: EventWriter<AccountTransaction>,
) {
    for (entity, building, consolidator, mut statistics, idle) in building_query.iter_mut() {
        let mut available_products = vec![];

        for (index, (product, active)) in building.products.iter().enumerate() {
            if !active {
                continue;
            };

            let mut modifier = 1.0;
            let mut consumed_resources = vec![];

            let mut has_requisites = true;
            for requisite in product.requisites.iter() {
                if !has_in_storage(
                    consolidator,
                    &mut storage_query,
                    &requisite.resource,
                    requisite.rate,
                ) {
                    has_requisites = false;
                }
                consumed_resources.push((&requisite.resource, requisite.rate));
            }
            if !has_requisites {
                continue;
            }

            for enhancer in &product.enhancers {
                let enhancer_present = has_in_storage(
                    consolidator,
                    &mut storage_query,
                    &enhancer.resource,
                    enhancer.rate,
                );

                if enhancer_present {
                    modifier *= enhancer.modifier;
                    consumed_resources.push((&enhancer.resource, enhancer.rate));
                    continue;
                }

                let enhancer_resource = resources.get(&enhancer.resource).unwrap();
                for (substitute, rate) in enhancer_resource.substitute.iter() {
                    if has_in_storage(
                        consolidator,
                        &mut storage_query,
                        substitute,
                        enhancer.rate / rate,
                    ) {
                        modifier *= enhancer.modifier;
                        consumed_resources.push((substitute, enhancer.rate / rate));
                        continue;
                    }
                }
            }

            if has_space_in_storage(
                consolidator,
                &mut storage_query,
                &product.resource,
                product.rate * modifier,
            ) {
                available_products.push((index, modifier, consumed_resources));
            }
        }

        let mut random = thread_rng();
        available_products.shuffle(&mut random);

        if available_products.is_empty() {
            if idle.is_none() {
                // not produced
                commands.entity(entity).insert(Idle::default());
            }

            continue;
        }

        let product = &building.products[available_products[0].0].0;
        let modifier = available_products[0].1;
        let consumed_resources = &available_products[0].2;

        for (resource, amount) in consumed_resources {
            fetch_from_storage(consolidator, &mut storage_query, resource, *amount);
            statistics.consumption.track(resource, *amount);
        }

        distribute_to_storage(
            consolidator,
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

        if product.cost > 0.0 {
            events.send(AccountTransaction {
                amount: -product.cost as i64,
            });
        }
    }
}
