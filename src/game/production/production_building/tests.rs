use super::*;
use crate::game::assets::{Product, ProductRequisite, Storage};
use bevy::prelude::*;

#[test]
fn produces_resource() {
    let mut world = World::default();

    let mut stage = SystemStage::parallel();
    stage.add_system(production_building.system());

    let coke = "coke";
    let coal = "coal";

    let coke_storage_id = world
        .spawn()
        .insert(Storage {
            resource: coke.to_owned(),
            amount: 0.0,
            capacity: 10.0,
        })
        .id();

    let coal_storage_id = world
        .spawn()
        .insert(Storage {
            resource: coal.to_owned(),
            amount: 10.0,
            capacity: 10.0,
        })
        .id();

    world
        .spawn()
        .insert(ProductionBuilding {
            products: vec![Product {
                resource: coke.to_owned(),
                rate: 1.0,
                requisites: vec![ProductRequisite {
                    resource: coal.to_owned(),
                    rate: 2.0,
                }],
            }],
        })
        .insert(StorageConsolidator {
            connected_storage: vec![coal_storage_id, coke_storage_id],
        })
        .id();

    stage.run(&mut world);

    assert!((world.get::<Storage>(coke_storage_id).unwrap().amount - 1.0).abs() < f64::EPSILON);
    assert!((world.get::<Storage>(coal_storage_id).unwrap().amount - 8.0).abs() < f64::EPSILON);

    // if already full
    world.get_mut::<Storage>(coke_storage_id).unwrap().amount = 10.0;

    stage.run(&mut world);

    // no overflow
    assert!((world.get::<Storage>(coke_storage_id).unwrap().amount - 10.0).abs() < f64::EPSILON);
    assert!((world.get::<Storage>(coal_storage_id).unwrap().amount - 8.0).abs() < f64::EPSILON);

    // no requisites left
    world.get_mut::<Storage>(coal_storage_id).unwrap().amount = 0.0;
    world.get_mut::<Storage>(coke_storage_id).unwrap().amount = 0.0;

    stage.run(&mut world);

    // no production
    assert!(world.get::<Storage>(coke_storage_id).unwrap().amount < f64::EPSILON);
    assert!(world.get::<Storage>(coal_storage_id).unwrap().amount < f64::EPSILON);
}