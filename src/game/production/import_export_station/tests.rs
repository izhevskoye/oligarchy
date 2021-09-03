use std::collections::HashMap;

use crate::game::{
    assets::resource_specifications::ResourceSpecification, production::ImportExportDirection,
};

use super::*;
use bevy::{app::Events, prelude::*};

struct Setup {
    station_direction: ImportExportDirection,
    amount_in_storage: f64,
    connected_storage: bool,
    second_connected_storage: bool,
    good_set_to_export: bool,
}

const RESOURCE: &str = "coke";
const SECOND_RESOURCE: &str = "coal";
const RESOURCE_COST: f64 = 10.0;

struct TestSetup {
    world: World,
    station_id: Entity,
    storage_id: Entity,
    second_storage_id: Option<Entity>,
}

impl TestSetup {
    fn storage_amount(&self) -> f64 {
        self.world.get::<Storage>(self.storage_id).unwrap().amount
    }

    fn assert_storage_amount(&self, amount: f64) {
        let current = self.storage_amount();

        assert!(
            (current - amount).abs() < f64::EPSILON,
            "Expected storage to be {}, but was {}",
            amount,
            current
        );
    }

    fn second_storage_amount(&self) -> f64 {
        let second_storage_id = self
            .second_storage_id
            .expect("Second storage to be configured");

        self.world.get::<Storage>(second_storage_id).unwrap().amount
    }

    fn assert_imported_statistic(&self, amount: f64) {
        let current = self
            .world
            .get::<Statistics>(self.station_id)
            .unwrap()
            .import
            .get(RESOURCE);

        assert!((current - amount).abs() < f64::EPSILON);
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
        for event in reader.iter(events) {
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
        resources.insert(
            SECOND_RESOURCE.to_owned(),
            ResourceSpecification {
                cost: RESOURCE_COST,
                ..Default::default()
            },
        );
        world.insert_resource(resources);
        world.insert_resource(Events::<AccountTransaction>::default());

        let mut stage = SystemStage::parallel();
        stage.add_system(import_export_station.system());

        let storage_id = world
            .spawn()
            .insert(Storage {
                resource: RESOURCE.to_owned(),
                amount: setup.amount_in_storage,
                capacity: 10.0,
            })
            .id();

        let mut connected_storage = if setup.connected_storage {
            vec![storage_id]
        } else {
            vec![]
        };

        let mut goods = if setup.good_set_to_export {
            vec![RESOURCE.to_owned()]
        } else {
            vec![]
        };

        let second_storage_id = if setup.second_connected_storage {
            let second_storage_id = world
                .spawn()
                .insert(Storage {
                    resource: RESOURCE.to_owned(),
                    amount: setup.amount_in_storage,
                    capacity: 10.0,
                })
                .id();

            connected_storage.push(second_storage_id);
            goods.push(SECOND_RESOURCE.to_owned());

            Some(second_storage_id)
        } else {
            None
        };

        let station_id = world
            .spawn()
            .insert(ImportExportStation {
                goods,
                direction: setup.station_direction,
            })
            .insert(Statistics::default())
            .insert(StorageConsolidator { connected_storage })
            .id();

        stage.run(&mut world);

        Self {
            world,
            station_id,
            storage_id,
            second_storage_id,
        }
    }
}

mod export {
    use super::*;

    #[test]
    fn no_connection_and_not_configured() {
        let setup = TestSetup::new(Setup {
            amount_in_storage: 10.0,
            connected_storage: false,
            good_set_to_export: false,
            second_connected_storage: false,
            station_direction: ImportExportDirection::Export,
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
            second_connected_storage: false,
            station_direction: ImportExportDirection::Export,
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
            second_connected_storage: false,
            station_direction: ImportExportDirection::Export,
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
            second_connected_storage: false,
            station_direction: ImportExportDirection::Export,
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
            second_connected_storage: false,
            station_direction: ImportExportDirection::Export,
        });

        setup.assert_storage_amount(0.0);
        setup.assert_exported_statistic(0.0);
        setup.assert_event_sum(0);
    }

    #[test]
    fn only_exports_one() {
        let setup = TestSetup::new(Setup {
            amount_in_storage: 10.0,
            connected_storage: true,
            good_set_to_export: true,
            second_connected_storage: true,
            station_direction: ImportExportDirection::Export,
        });

        assert!(
            (setup.storage_amount() + setup.second_storage_amount() - 10.0).abs() < f64::EPSILON
        );
        setup.assert_exported_statistic(10.0);
    }
}

mod import {
    use super::*;

    #[test]
    fn no_connection_and_not_configured() {
        let setup = TestSetup::new(Setup {
            amount_in_storage: 0.0,
            connected_storage: false,
            good_set_to_export: false,
            second_connected_storage: false,
            station_direction: ImportExportDirection::Import,
        });

        setup.assert_storage_amount(0.0);
        setup.assert_imported_statistic(0.0);
        setup.assert_event_sum(0);
    }

    #[test]
    fn connection_but_not_configured() {
        let setup = TestSetup::new(Setup {
            amount_in_storage: 0.0,
            connected_storage: true,
            good_set_to_export: false,
            second_connected_storage: false,
            station_direction: ImportExportDirection::Import,
        });

        setup.assert_storage_amount(0.0);
        setup.assert_imported_statistic(0.0);
        setup.assert_event_sum(0);
    }

    #[test]
    fn connection_and_configured() {
        let setup = TestSetup::new(Setup {
            amount_in_storage: 9.0,
            connected_storage: true,
            good_set_to_export: true,
            second_connected_storage: false,
            station_direction: ImportExportDirection::Import,
        });

        setup.assert_storage_amount(10.0);
        setup.assert_imported_statistic(1.0);
        setup.assert_event_sum((IMPORT_SURCHARGE * -RESOURCE_COST) as i64);
    }

    #[test]
    fn maximum_export_amount() {
        let setup = TestSetup::new(Setup {
            amount_in_storage: 0.0,
            connected_storage: true,
            good_set_to_export: true,
            second_connected_storage: false,
            station_direction: ImportExportDirection::Import,
        });

        setup.assert_storage_amount(10.0);
        setup.assert_imported_statistic(10.0);
        setup.assert_event_sum((IMPORT_SURCHARGE * 10.0 * -RESOURCE_COST) as i64);
    }

    #[test]
    fn connection_and_configured_but_full() {
        let setup = TestSetup::new(Setup {
            amount_in_storage: 10.0,
            connected_storage: true,
            good_set_to_export: true,
            second_connected_storage: false,
            station_direction: ImportExportDirection::Import,
        });

        setup.assert_storage_amount(10.0);
        setup.assert_imported_statistic(0.0);
        setup.assert_event_sum(0);
    }

    #[test]
    fn only_imports_one() {
        let setup = TestSetup::new(Setup {
            amount_in_storage: 0.0,
            connected_storage: true,
            good_set_to_export: true,
            second_connected_storage: true,
            station_direction: ImportExportDirection::Import,
        });

        assert!(
            (setup.storage_amount() + setup.second_storage_amount() - 10.0).abs() < f64::EPSILON
        );
        setup.assert_imported_statistic(10.0);
    }
}
