use std::collections::HashMap;

use crate::game::assets::resource_specifications::ResourceSpecification;

use super::*;
use bevy::{app::Events, prelude::*};

struct Setup {
    amount_in_storage: f64,
    connected_storage: bool,
    good_set_to_export: bool,
}

const RESOURCE: &str = "coke";
const RESOURCE_COST: f64 = 10.0;

struct TestSetup {
    world: World,
    station_id: Entity,
    storage_id: Entity,
}

impl TestSetup {
    fn assert_storage_amount(&self, amount: f64) {
        assert!(
            (self.world.get::<Storage>(self.storage_id).unwrap().amount - amount).abs()
                < f64::EPSILON
        );
    }

    fn assert_exported_statistic(&self, amount: f64) {
        let current = self
            .world
            .get::<Statistics>(self.station_id)
            .unwrap()
            .export
            .get(RESOURCE);

        assert!((current - amount).abs() < f64::EPSILON);
    }

    fn assert_event_sum(&self, amount: i64) {
        let events = self
            .world
            .get_resource::<Events<AccountTransaction>>()
            .unwrap();
        let mut reader = events.get_reader();

        let mut sum = 0;
        for event in reader.iter(&events) {
            sum += event.amount;
        }

        assert!(
            sum == amount,
            "expected event sum {} but was {}",
            amount,
            sum
        );
    }

    fn new(setup: Setup) -> Self {
        let mut world = World::default();
        let mut resources = HashMap::new();
        resources.insert(
            RESOURCE.to_owned(),
            ResourceSpecification {
                cost: RESOURCE_COST,
                ..Default::default()
            },
        );
        world.insert_resource(resources);
        world.insert_resource(Events::<AccountTransaction>::default());

        let mut stage = SystemStage::parallel();
        stage.add_system(export_station.system());

        let storage_id = world
            .spawn()
            .insert(Storage {
                resource: RESOURCE.to_owned(),
                amount: setup.amount_in_storage,
                capacity: 10.0,
            })
            .id();

        let connected_storage = if setup.connected_storage {
            vec![storage_id]
        } else {
            vec![]
        };

        let goods = if setup.good_set_to_export {
            vec![RESOURCE.to_owned()]
        } else {
            vec![]
        };

        let station_id = world
            .spawn()
            .insert(ExportStation { goods })
            .insert(Statistics::default())
            .insert(StorageConsolidator { connected_storage })
            .id();

        stage.run(&mut world);

        Self {
            world,
            station_id,
            storage_id,
        }
    }
}

#[test]
fn no_connection_and_not_configured() {
    let setup = TestSetup::new(Setup {
        amount_in_storage: 10.0,
        connected_storage: false,
        good_set_to_export: false,
    });

    setup.assert_storage_amount(10.0);
    setup.assert_exported_statistic(0.0);
    setup.assert_event_sum(0);
}

#[test]
fn connection_but_not_configured() {
    let setup = TestSetup::new(Setup {
        amount_in_storage: 10.0,
        connected_storage: true,
        good_set_to_export: false,
    });

    setup.assert_storage_amount(10.0);
    setup.assert_exported_statistic(0.0);
    setup.assert_event_sum(0);
}

#[test]
fn connection_and_configured() {
    let setup = TestSetup::new(Setup {
        amount_in_storage: 1.0,
        connected_storage: true,
        good_set_to_export: true,
    });

    setup.assert_storage_amount(0.0);
    setup.assert_exported_statistic(1.0);
    setup.assert_event_sum(RESOURCE_COST as i64);
}

#[test]
fn maximum_export_amount() {
    let setup = TestSetup::new(Setup {
        amount_in_storage: 100.0,
        connected_storage: true,
        good_set_to_export: true,
    });

    setup.assert_storage_amount(90.0);
    setup.assert_exported_statistic(10.0);
    setup.assert_event_sum(10 * RESOURCE_COST as i64);
}

#[test]
fn connection_and_configured_but_empty() {
    let setup = TestSetup::new(Setup {
        amount_in_storage: 0.0,
        connected_storage: true,
        good_set_to_export: true,
    });

    setup.assert_storage_amount(0.0);
    setup.assert_exported_statistic(0.0);
    setup.assert_event_sum(0);
}
