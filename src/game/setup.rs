use crate::game::assets::{BlastFurnace, CokeFurnace, OxygenConverter};

use super::assets::{Quarry, Resource, Storage, StorageConsolidator};
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    let coal_storage = commands
        .spawn()
        .insert(Storage {
            resource: Resource::Coal,
            amount: 0,
            capacity: 20,
        })
        .id();

    commands
        .spawn()
        .insert(Quarry {
            resource: Resource::Coal,
        })
        .insert(StorageConsolidator {
            connected_storage: vec![coal_storage],
        });

    let coke_storage = commands
        .spawn()
        .insert(Storage {
            resource: Resource::Coke,
            amount: 0,
            capacity: 20,
        })
        .id();

    commands
        .spawn()
        .insert(CokeFurnace)
        .insert(StorageConsolidator {
            connected_storage: vec![coal_storage, coke_storage],
        });

    let limestone_storage = commands
        .spawn()
        .insert(Storage {
            resource: Resource::Limestone,
            amount: 0,
            capacity: 20,
        })
        .id();

    commands
        .spawn()
        .insert(Quarry {
            resource: Resource::Limestone,
        })
        .insert(StorageConsolidator {
            connected_storage: vec![limestone_storage],
        });

    let iron_ore_storage = commands
        .spawn()
        .insert(Storage {
            resource: Resource::IronOre,
            amount: 0,
            capacity: 20,
        })
        .id();

    commands
        .spawn()
        .insert(Quarry {
            resource: Resource::IronOre,
        })
        .insert(StorageConsolidator {
            connected_storage: vec![iron_ore_storage],
        });

    let iron_storage = commands
        .spawn()
        .insert(Storage {
            resource: Resource::Iron,
            amount: 0,
            capacity: 20,
        })
        .id();

    commands
        .spawn()
        .insert(BlastFurnace)
        .insert(StorageConsolidator {
            connected_storage: vec![
                iron_storage,
                limestone_storage,
                coke_storage,
                iron_ore_storage,
            ],
        });

    let steel_storage = commands
        .spawn()
        .insert(Storage {
            resource: Resource::Steel,
            amount: 0,
            capacity: 20,
        })
        .id();

    commands
        .spawn()
        .insert(OxygenConverter)
        .insert(StorageConsolidator {
            connected_storage: vec![iron_storage, steel_storage],
        });
}
