use std::collections::HashMap;

use bevy::prelude::*;

const MAX_AMOUNT: f64 = 5.0;

use crate::game::{
    construction::UnderConstruction,
    storage::{Storage, StorageConsolidator},
};
use collecting_hashmap::CollectingHashMap;
use combinations::Combinations;

use super::StorageManagement;

pub fn storage_management(
    mut storage_management: Query<
        &StorageConsolidator,
        (Without<UnderConstruction>, With<StorageManagement>),
    >,
    mut storage_query: Query<(Entity, &mut Storage)>,
) {
    for consolidator in storage_management.iter_mut() {
        if consolidator.connected_storage.len() < 2 {
            continue;
        }

        let mut storages = HashMap::new();
        let mut storages_by_resource = CollectingHashMap::new();
        for entity in consolidator.connected_storage.iter() {
            let (entity, storage) = storage_query.get_mut(*entity).unwrap();
            let storage = storage.clone();

            storages_by_resource.insert(storage.resource.clone(), entity);
            storages.insert(entity, storage);
        }

        let mut unequal_distributions = vec![];

        for (_resource, entities) in storages_by_resource.iter() {
            if entities.len() < 2 {
                continue;
            }

            let combinations: Vec<_> = if entities.len() == 2 {
                vec![entities.clone()]
            } else {
                Combinations::new(entities.clone(), 2).collect()
            };

            for ab in combinations {
                let a = storages.get(&ab[0]).unwrap();
                let b = storages.get(&ab[1]).unwrap();

                let difference = (a.amount - b.amount).abs();

                if difference > 0.0 {
                    let lesser = if a.amount < b.amount { ab[0] } else { ab[1] };
                    let larger = if a.amount >= b.amount { ab[0] } else { ab[1] };

                    unequal_distributions.push((difference, lesser, larger));
                }
            }
        }

        unequal_distributions.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        if unequal_distributions.len() >= 1 {
            let (diff, lesser, larger) = unequal_distributions[0];
            let diff = (diff / 2.0).min(MAX_AMOUNT);

            {
                let mut lesser = storage_query.get_mut(lesser).unwrap().1;
                lesser.amount += diff;
            }

            {
                let mut larger = storage_query.get_mut(larger).unwrap().1;
                larger.amount -= diff;
            }
        }
    }
}
