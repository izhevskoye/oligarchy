#[cfg(test)]
mod tests;

use bevy::prelude::*;

use crate::game::{
    account::AccountTransaction,
    assets::{ExportStation, Storage, StorageConsolidator},
    resource_specifications::ResourceSpecifications,
    statistics::Statistics,
    storage::fetch_from_storage,
};

pub fn export_station(
    mut export_station: Query<(&ExportStation, &StorageConsolidator, &mut Statistics)>,
    mut storage_query: Query<&mut Storage>,
    resources: Res<ResourceSpecifications>,
    mut events: EventWriter<AccountTransaction>,
) {
    for (export, consolidator, mut statistics) in export_station.iter_mut() {
        for resource in &export.goods {
            let amount = 1.0;

            if fetch_from_storage(&consolidator, &mut storage_query, &resource, amount) {
                log::info!("Exporting {:?}", resource);
                statistics.export.track(resource, amount);

                let resource = resources.get(resource).unwrap();

                if let Some(amount) = resource.cost {
                    events.send(AccountTransaction {
                        amount: amount as i64,
                    });
                }
            }
        }
    }
}
