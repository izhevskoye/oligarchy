use bevy::prelude::*;

use crate::game::storage::has_space_in_storage;

use super::{
    assets::{BlastFurnace, Resource, Storage, StorageConsolidator},
    storage::{distribute_to_storage, fetch_from_storage, has_in_storage},
};

pub fn blast_furnace(
    furnace_query: Query<(&BlastFurnace, &StorageConsolidator)>,
    mut storage_query: Query<&mut Storage>,
) {
    for (_furnace, consolidator) in furnace_query.iter() {
        if !has_in_storage(&consolidator, &mut storage_query, Resource::Coke)
            || !has_in_storage(&consolidator, &mut storage_query, Resource::IronOre)
            || !has_in_storage(&consolidator, &mut storage_query, Resource::Limestone)
        {
            continue;
        }

        if !has_space_in_storage(&consolidator, &mut storage_query, Resource::Iron) {
            continue;
        }

        fetch_from_storage(&consolidator, &mut storage_query, Resource::Coke);
        fetch_from_storage(&consolidator, &mut storage_query, Resource::IronOre);
        fetch_from_storage(&consolidator, &mut storage_query, Resource::Limestone);

        println!("Blast Furnace Cooking");
        distribute_to_storage(&consolidator, &mut storage_query, Resource::Iron);
    }
}
