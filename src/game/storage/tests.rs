use super::*;

const COKE: &str = "coke";

#[derive(Default)]
struct TestAmount {
    pub amount: f64,
}

#[derive(Default)]
struct TestResult {
    pub result: bool,
}

#[test]
fn test_distribute_to_storage() {
    let mut world = World::default();
    world.insert_resource(TestAmount { amount: 2.0 });

    fn distribute_to_storage_test_system(
        consolidator_query: Query<&StorageConsolidator>,
        mut storage_query: Query<&mut Storage>,
        params: Res<TestAmount>,
    ) {
        for consolidator in consolidator_query.iter() {
            distribute_to_storage(&consolidator, &mut storage_query, COKE, params.amount);
        }
    }

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
fn test_has_in_storage() {
    let mut world = World::default();
    world.insert_resource(TestResult::default());
    world.insert_resource(TestAmount { amount: 1.0 });

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
fn test_has_space_in_storage() {
    let mut world = World::default();
    world.insert_resource(TestResult::default());
    world.insert_resource(TestAmount { amount: 10.0 });

    fn has_space_in_storage_test_system(
        consolidator_query: Query<&StorageConsolidator>,
        mut storage_query: Query<&mut Storage>,
        mut result: ResMut<TestResult>,
        params: Res<TestAmount>,
    ) {
        for consolidator in consolidator_query.iter() {
            result.result =
                has_space_in_storage(&consolidator, &mut storage_query, COKE, params.amount);
        }
    }

    let mut stage = SystemStage::parallel();
    stage.add_system(has_space_in_storage_test_system.system());

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

    assert!(world.get_resource::<TestResult>().unwrap().result);

    world
        .get_entity_mut(coke_storage_id)
        .unwrap()
        .insert(Storage {
            resource: COKE.to_owned(),
            amount: 10.0,
            capacity: 10.0,
        });

    stage.run(&mut world);

    assert!(!world.get_resource::<TestResult>().unwrap().result);

    let second_coke_storage_id = world
        .spawn()
        .insert(Storage {
            resource: COKE.to_owned(),
            amount: 0.0,
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
fn test_fetch_from_storage() {
    let mut world = World::default();
    world.insert_resource(TestResult::default());
    world.insert_resource(TestAmount { amount: 10.0 });

    fn fetch_from_storage_test_system(
        consolidator_query: Query<&StorageConsolidator>,
        mut storage_query: Query<&mut Storage>,
        mut result: ResMut<TestResult>,
        params: Res<TestAmount>,
    ) {
        for consolidator in consolidator_query.iter() {
            result.result =
                fetch_from_storage(&consolidator, &mut storage_query, COKE, params.amount);
        }
    }

    let mut stage = SystemStage::parallel();
    stage.add_system(fetch_from_storage_test_system.system());

    let coke_storage_id = world
        .spawn()
        .insert(Storage {
            resource: COKE.to_owned(),
            amount: 3.0,
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
    assert!((world.get::<Storage>(coke_storage_id).unwrap().amount - 3.0).abs() < f64::EPSILON);

    world
        .get_entity_mut(coke_storage_id)
        .unwrap()
        .insert(Storage {
            resource: COKE.to_owned(),
            amount: 10.0,
            capacity: 10.0,
        });

    stage.run(&mut world);

    assert!(world.get_resource::<TestResult>().unwrap().result);
    assert!(world.get::<Storage>(coke_storage_id).unwrap().amount < f64::EPSILON);

    world
        .get_entity_mut(coke_storage_id)
        .unwrap()
        .insert(Storage {
            resource: COKE.to_owned(),
            amount: 3.0,
            capacity: 10.0,
        });

    let second_coke_storage_id = world
        .spawn()
        .insert(Storage {
            resource: COKE.to_owned(),
            amount: 8.0,
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
    assert!(
        (world.get::<Storage>(coke_storage_id).unwrap().amount
            + world.get::<Storage>(second_coke_storage_id).unwrap().amount
            - 1.0)
            .abs()
            < f64::EPSILON
    );
}
