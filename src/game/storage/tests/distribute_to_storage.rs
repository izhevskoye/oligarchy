use super::*;

fn distribute_to_storage_test_system(
    consolidator_query: Query<&StorageConsolidator>,
    mut storage_query: Query<&mut Storage>,
    params: Res<TestAmount>,
) {
    for consolidator in consolidator_query.iter() {
        distribute_to_storage(consolidator, &mut storage_query, COKE, params.amount);
    }
}

#[test]
fn test_distribute_to_storage() {
    let mut world = World::default();
    world.insert_resource(TestAmount { amount: 2.0 });

    let mut stage = SystemStage::parallel();
    stage.add_system(distribute_to_storage_test_system.system());

    let coke_storage_id = world
        .spawn()
        .insert(Storage {
            resource: COKE.to_owned(),
            amount: 0.0,
            capacity: 10.0,
        })
        .id();

    let consolidator_id = world
        .spawn()
        .insert(StorageConsolidator {
            connected_storage: vec![coke_storage_id],
        })
        .id();

    stage.run(&mut world);

    assert!((world.get::<Storage>(coke_storage_id).unwrap().amount - 2.0).abs() < f64::EPSILON);

    let second_coke_storage_id = world
        .spawn()
        .insert(Storage {
            resource: COKE.to_owned(),
            amount: 0.0,
            capacity: 10.0,
        })
        .id();

    world.insert_resource(TestAmount { amount: 10.0 });
    world
        .get_entity_mut(consolidator_id)
        .unwrap()
        .insert(StorageConsolidator {
            connected_storage: vec![coke_storage_id, second_coke_storage_id],
        });

    stage.run(&mut world);

    assert!(
        (world.get::<Storage>(coke_storage_id).unwrap().amount
            + world.get::<Storage>(second_coke_storage_id).unwrap().amount
            - 12.0)
            .abs()
            < f64::EPSILON
    );
}

#[test]
fn test_storage_deleted() {
    let mut world = World::default();
    world.insert_resource(TestAmount { amount: 2.0 });

    let mut stage = SystemStage::parallel();
    stage.add_system(distribute_to_storage_test_system.system());

    let coke_storage_id = world
        .spawn()
        .insert(Storage {
            resource: COKE.to_owned(),
            amount: 0.0,
            capacity: 10.0,
        })
        .id();

    world.despawn(coke_storage_id);

    world.spawn().insert(StorageConsolidator {
        connected_storage: vec![coke_storage_id],
    });

    // should not panic
    stage.run(&mut world);
}

#[test]
fn test_no_connected_storage() {
    let mut world = World::default();
    world.insert_resource(TestAmount { amount: 2.0 });

    let mut stage = SystemStage::parallel();
    stage.add_system(distribute_to_storage_test_system.system());

    world.spawn().insert(StorageConsolidator::default());

    stage.run(&mut world);

    // should not panic
}

#[test]
#[should_panic]
fn test_negative_amount() {
    let mut world = World::default();
    world.insert_resource(TestAmount { amount: -2.0 });

    let mut stage = SystemStage::parallel();
    stage.add_system(distribute_to_storage_test_system.system());

    world.spawn().insert(StorageConsolidator::default());

    stage.run(&mut world);
}
