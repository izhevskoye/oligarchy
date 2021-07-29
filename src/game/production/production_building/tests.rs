use super::*;
use crate::game::assets::{Product, Storage};
use bevy::prelude::*;

#[test]
fn produces_resource() {
    // Setup world
    let mut world = World::default();

    // Setup stage with our two systems
    let mut stage = SystemStage::parallel();
    stage.add_system(production_building.system());

    let coke = "coke".to_owned();
    let coal = "coal".to_owned();

    // Setup test entities
    let coke_storage_id = world
        .spawn()
        .insert(Storage {
            resource: coke.clone(),
            amount: 0,
            capacity: 10,
        })
        .id();

    // Setup test entities
    let coal_storage_id = world
        .spawn()
        .insert(Storage {
            resource: coal.clone(),
            amount: 10,
            capacity: 10,
        })
        .id();

    world
        .spawn()
        .insert(ProductionBuilding {
            products: vec![Product {
                resource: coke.clone(),
                requisites: vec![coal.clone()],
            }],
        })
        .insert(StorageConsolidator {
            connected_storage: vec![coal_storage_id, coke_storage_id],
        })
        .id();

    stage.run(&mut world);

    assert_eq!(world.get::<Storage>(coke_storage_id).unwrap().amount, 1);
    assert_eq!(world.get::<Storage>(coal_storage_id).unwrap().amount, 9);

    // if already full
    world.get_mut::<Storage>(coke_storage_id).unwrap().amount = 10;

    stage.run(&mut world);

    // no overflow
    assert_eq!(world.get::<Storage>(coke_storage_id).unwrap().amount, 10);
    assert_eq!(world.get::<Storage>(coal_storage_id).unwrap().amount, 9);

    // no requisites left
    world.get_mut::<Storage>(coal_storage_id).unwrap().amount = 0;
    world.get_mut::<Storage>(coke_storage_id).unwrap().amount = 0;

    stage.run(&mut world);

    // no production
    assert_eq!(world.get::<Storage>(coke_storage_id).unwrap().amount, 0);
    assert_eq!(world.get::<Storage>(coal_storage_id).unwrap().amount, 0);
}
