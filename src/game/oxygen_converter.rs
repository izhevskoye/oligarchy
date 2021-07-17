use bevy::prelude::*;

use super::{
    assets::{OxygenConverter, Resource, Storage, StorageConsolidator},
    storage::{distribute_to_storage, fetch_from_storage, has_space_in_storage},
};

pub fn oxygen_converter(
    converter_query: Query<(&OxygenConverter, &StorageConsolidator)>,
    mut storage_query: Query<&mut Storage>,
) {
    for (_converter, consolidator) in converter_query.iter() {
        if has_space_in_storage(&consolidator, &mut storage_query, Resource::Steel)
            && fetch_from_storage(&consolidator, &mut storage_query, Resource::Iron)
        {
            println!("Oxygen Converter Working");
            distribute_to_storage(&consolidator, &mut storage_query, Resource::Steel);
        }
    }
}
