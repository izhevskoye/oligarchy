#[cfg(test)]
mod tests;

use bevy::prelude::*;

use crate::game::{
    assets::{ExportStation, Storage, StorageConsolidator},
    statistics::Statistics,
    storage::fetch_from_storage,
};

pub fn export_station(
    mut export_station: Query<(&ExportStation, &StorageConsolidator, &mut Statistics)>,
    mut storage_query: Query<&mut Storage>,
) {
    for (export, consolidator, mut statistics) in export_station.iter_mut() {
        for resource in &export.goods {
            let amount = 1.0;

            if fetch_from_storage(&consolidator, &mut storage_query, &resource, amount) {
                log::info!("Exporting {:?}", resource);
                // TODO: add test!
                statistics.export.track(resource, amount);
            }
        }
    }
}
