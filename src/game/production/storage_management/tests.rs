use super::*;
use crate::game::{
    assets::resource_specifications::{ResourceSpecification, ResourceSpecifications},
    production::StorageManagement,
    statistics::Statistics,
    storage::Storage,
};
use bevy::prelude::*;

const COKE: &str = "coke";
const COAL: &str = "coal";

struct TestSetup {
    world: World,
    stage: SystemStage,
}

impl TestSetup {
    fn add_storage(&mut self, resource: &str, amount: f64) -> Entity {
        self.world
            .spawn()
            .insert(Storage {
                resource: resource.to_owned(),
                amount,
                capacity: 10.0,
            })
            .id()
    }

    fn get_storage_amount(&self, entity: Entity) -> f64 {
        self.world.get::<Storage>(entity).unwrap().amount
    }

    fn assert_storage_amount(&self, entity: Entity, amount: f64) {
        let actual = self.get_storage_amount(entity);
        assert!(
            (actual - amount).abs() < f64::EPSILON,
            "expected storage to be {} but it was {}",
            amount,
            actual
        );
    }

    fn new() -> Self {
        let mut world = World::default();

        let mut resource_specifications = ResourceSpecifications::new();
        resource_specifications.insert(COKE.to_owned(), ResourceSpecification::default());
        resource_specifications.insert(COAL.to_owned(), ResourceSpecification::default());

        world.insert_resource(resource_specifications);

        let mut stage = SystemStage::parallel();
        stage.add_system(storage_management.system());

        Self { world, stage }
    }
}

#[test]
fn moves_some_resources() {
    let mut setup = TestSetup::new();

    let coal_storage_id = setup.add_storage(COAL, 0.0);
    let second_coal_storage_id = setup.add_storage(COAL, 100.0);

    setup
        .world
        .spawn()
        .insert(Statistics::default())
        .insert(StorageManagement)
        .insert(StorageConsolidator {
            connected_storage: vec![coal_storage_id, second_coal_storage_id],
        });

    setup.stage.run(&mut setup.world);

    setup.assert_storage_amount(coal_storage_id, 5.0);
    setup.assert_storage_amount(second_coal_storage_id, 95.0);
}

#[test]
fn moves_resource_with_highest_distance() {
    let mut setup = TestSetup::new();

    let coal_storage_id = setup.add_storage(COAL, 0.0);
    let second_coal_storage_id = setup.add_storage(COAL, 100.0);

    let coke_storage_id = setup.add_storage(COKE, 0.0);
    let second_coke_storage_id = setup.add_storage(COKE, 200.0);

    setup
        .world
        .spawn()
        .insert(Statistics::default())
        .insert(StorageManagement)
        .insert(StorageConsolidator {
            connected_storage: vec![
                coal_storage_id,
                second_coal_storage_id,
                coke_storage_id,
                second_coke_storage_id,
            ],
        });

    setup.stage.run(&mut setup.world);

    setup.assert_storage_amount(coal_storage_id, 0.0);
    setup.assert_storage_amount(second_coal_storage_id, 100.0);

    setup.assert_storage_amount(coke_storage_id, 5.0);
    setup.assert_storage_amount(second_coke_storage_id, 195.0);
}

#[test]
fn move_with_three() {
    let mut setup = TestSetup::new();

    let coal_storage_id = setup.add_storage(COAL, 0.0);
    let second_coal_storage_id = setup.add_storage(COAL, 100.0);
    let third_coal_storage_id = setup.add_storage(COAL, 75.0);

    setup
        .world
        .spawn()
        .insert(Statistics::default())
        .insert(StorageManagement)
        .insert(StorageConsolidator {
            connected_storage: vec![
                coal_storage_id,
                second_coal_storage_id,
                third_coal_storage_id,
            ],
        });

    setup.stage.run(&mut setup.world);

    setup.assert_storage_amount(coal_storage_id, 5.0);
    setup.assert_storage_amount(second_coal_storage_id, 95.0);
    setup.assert_storage_amount(third_coal_storage_id, 75.0);
}
