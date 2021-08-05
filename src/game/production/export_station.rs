use bevy::prelude::*;

use crate::game::{
    assets::{ExportStation, Storage, StorageConsolidator},
    statistics::Statistics,
    storage::fetch_from_storage,
};

pub fn export_station(
    export_station: Query<(&ExportStation, &StorageConsolidator, &Statistics)>,
    mut storage_query: Query<&mut Storage>,
) {
    for (export, consolidator, statistics) in export_station.iter() {
        for resource in &export.goods {
            let amount = 1.0;

            if fetch_from_storage(&consolidator, &mut storage_query, &resource, amount) {
                log::info!("Exporting {:?}", resource);
                statistics.export.track(resource, amount);
            }
        }
    }
}
