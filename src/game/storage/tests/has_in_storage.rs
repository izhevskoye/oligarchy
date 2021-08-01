use super::*;

fn has_in_storage_test_system(
    consolidator_query: Query<&StorageConsolidator>,
    mut storage_query: Query<&mut Storage>,
    mut result: ResMut<TestResult>,
    params: Res<TestAmount>,
) {
    for consolidator in consolidator_query.iter() {
        result.result = has_in_storage(&consolidator, &mut storage_query, COKE, params.amount);
    }
}

#[test]
fn test_has_in_storage() {
    let mut world = World::default();
    world.insert_resource(TestResult::default());
    world.insert_resource(TestAmount { amount: 1.0 });

    let mut stage = SystemStage::parallel();
    stage.add_system(has_in_storage_test_system.system());

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

    assert!(!world.get_resource::<TestResult>().unwrap().result);

    world
        .get_entity_mut(coke_storage_id)
        .unwrap()
        .insert(Storage {
            resource: COKE.to_owned(),
            amount: 1.0,
            capacity: 10.0,
        });

    stage.run(&mut world);

    assert!(world.get_resource::<TestResult>().unwrap().result);

    world.insert_resource(TestAmount { amount: 2.0 });
    stage.run(&mut world);

    assert!(!world.get_resource::<TestResult>().unwrap().result);

    let second_coke_storage_id = world
        .spawn()
        .insert(Storage {
            resource: COKE.to_owned(),
            amount: 1.0,
            capacity: 10.0,
        })
        .id();

    world
        .get_entity_mut(consolidator_id)
        .unwrap()
        .insert(StorageConsolidator {
            connected_storage: vec![coke_storage_id, second_coke_storage_id],
        });
    stage.run(&mut world);

    assert!(world.get_resource::<TestResult>().unwrap().result);
}

#[test]
fn test_storage_deleted() {
    let mut world = World::default();
    world.insert_resource(TestResult::default());
    world.insert_resource(TestAmount { amount: 1.0 });

    let mut stage = SystemStage::parallel();
    stage.add_system(has_in_storage_test_system.system());

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

    stage.run(&mut world);

    // should not panic

    assert!(!world.get_resource::<TestResult>().unwrap().result);
}

#[test]
#[should_panic]
fn test_negative_amount() {
    let mut world = World::default();
    world.insert_resource(TestResult::default());
    world.insert_resource(TestAmount { amount: -1.0 });

    let mut stage = SystemStage::parallel();
    stage.add_system(has_in_storage_test_system.system());

    world.spawn().insert(StorageConsolidator::default());

    stage.run(&mut world);

    // should panic
}
