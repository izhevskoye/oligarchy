#[cfg(test)]
mod tests;

use bevy::prelude::*;

const MAX_AMOUNT: f64 = 10.0;

use crate::game::{
    account::AccountTransaction,
    assets::resource_specifications::ResourceSpecifications,
    production::ImportExportStation,
    statistics::Statistics,
    storage::fetch_from_storage,
    storage::{
        amount_in_storage, distribute_to_storage, space_in_storage, Storage, StorageConsolidator,
    },
};

use super::ImportExportDirection;

pub fn import_export_station(
    mut station_query: Query<(&ImportExportStation, &StorageConsolidator, &mut Statistics)>,
    mut storage_query: Query<&mut Storage>,
    resources: Res<ResourceSpecifications>,
    mut events: EventWriter<AccountTransaction>,
) {
    for (station, consolidator, mut statistics) in station_query.iter_mut() {
        for resource in &station.goods {
            if station.direction == ImportExportDirection::Export {
                let amount =
                    amount_in_storage(consolidator, &mut storage_query, resource).min(MAX_AMOUNT);

                if amount > 0.0
                    && fetch_from_storage(&consolidator, &mut storage_query, &resource, amount)
                {
                    statistics.export.track(resource, amount);
                    let resource = resources.get(resource).unwrap();

                    log::info!("Exporting {} {:?}", amount, resource.name);

                    events.send(AccountTransaction {
                        amount: (amount * resource.cost) as i64,
                    });

                    break;
                }
            }

            if station.direction == ImportExportDirection::Import {
                let amount =
                    space_in_storage(consolidator, &mut storage_query, resource).min(MAX_AMOUNT);

                if amount > 0.0 {
                    distribute_to_storage(&consolidator, &mut storage_query, &resource, amount);

                    statistics.import.track(resource, amount);
                    let resource = resources.get(resource).unwrap();

                    log::info!("Importing {} {:?}", amount, resource.name);

                    events.send(AccountTransaction {
                        amount: (amount * -resource.cost) as i64,
                    });

                    break;
                }
            }
        }
    }
}
