#[cfg(test)]
mod tests;

use bevy::prelude::*;

const MAX_EXPORT_AMOUNT: f64 = 10.0;

use crate::game::{
    account::AccountTransaction,
    assets::resource_specifications::ResourceSpecifications,
    production::ExportStation,
    statistics::Statistics,
    storage::fetch_from_storage,
    storage::{amount_in_storage, Storage, StorageConsolidator},
};

pub fn export_station(
    mut export_station: Query<(&ExportStation, &StorageConsolidator, &mut Statistics)>,
    mut storage_query: Query<&mut Storage>,
    resources: Res<ResourceSpecifications>,
    mut events: EventWriter<AccountTransaction>,
) {
    for (export, consolidator, mut statistics) in export_station.iter_mut() {
        for resource in &export.goods {
            let amount = amount_in_storage(consolidator, &mut storage_query, resource)
                .min(MAX_EXPORT_AMOUNT);

            if amount > 0.0
                && fetch_from_storage(&consolidator, &mut storage_query, &resource, amount)
            {
                statistics.export.track(resource, amount);
                let resource = resources.get(resource).unwrap();

                log::info!("Exporting {} {:?}", amount, resource.name);

                events.send(AccountTransaction {
                    amount: (amount * resource.cost) as i64,
                });
            }
        }
    }
}
