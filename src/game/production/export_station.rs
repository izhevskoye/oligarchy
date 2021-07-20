use bevy::prelude::*;

use crate::game::{
    assets::{ExportStation, Storage, StorageConsolidator},
    storage::fetch_from_storage,
};

pub fn export_station(
    export_station: Query<(&ExportStation, &StorageConsolidator)>,
    mut storage_query: Query<&mut Storage>,
) {
    for (export, consolidator) in export_station.iter() {
        for resource in &export.goods {
            if fetch_from_storage(&consolidator, &mut storage_query, *resource) {
                log::info!("Exporting {:?}", resource);
            }
        }
    }
}
