use std::collections::HashMap;

use super::*;
use crate::game::{
    account::AccountTransaction,
    assets::resource_specifications::ResourceSpecification,
    production::{Product, ProductDependency, ProductEnhancer},
    statistics::Statistics,
    storage::Storage,
};
use bevy::{app::Events, prelude::*};

const COKE: &str = "coke";
const SLUG: &str = "slug";
const ENHANCER: &str = "enhancer";
const ENHANCER_REPLACEMENT: &str = "enhancer_replacement";
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
        assert!((self.get_storage_amount(entity) - amount).abs() < f64::EPSILON);
    }

    fn set_storage_amount(&mut self, entity: Entity, amount: f64) {
        self.world.get_mut::<Storage>(entity).unwrap().amount = amount;
    }

    fn assert_production_statistic(&self, resource: &str, building_id: Entity, amount: f64) {
        let current_amount = self
            .world
            .get::<Statistics>(building_id)
            .unwrap()
            .production
            .get(resource);

        assert!(
            (current_amount - amount).abs() < f64::EPSILON,
            "current was {}, but expected {}",
            current_amount,
            amount
        );
    }

    fn assert_consumption_statistic(&self, resource: &str, building_id: Entity, amount: f64) {
        let current_amount = self
            .world
            .get::<Statistics>(building_id)
            .unwrap()
            .consumption
            .get(resource);

        assert!(
            (current_amount - amount).abs() < f64::EPSILON,
            "current was {}, but expected {}",
            current_amount,
            amount
        );
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

    fn new() -> Self {
        let mut world = World::default();

        let mut resource_specifications = ResourceSpecifications::new();
        resource_specifications.insert(COKE.to_owned(), ResourceSpecification::default());
        resource_specifications.insert(COAL.to_owned(), ResourceSpecification::default());
        resource_specifications.insert(SLUG.to_owned(), ResourceSpecification::default());
        resource_specifications.insert(
            ENHANCER_REPLACEMENT.to_owned(),
            ResourceSpecification::default(),
        );
        let mut substitute = HashMap::new();
        substitute.insert(ENHANCER_REPLACEMENT.to_owned(), 1.0);
        resource_specifications.insert(
            ENHANCER.to_owned(),
            ResourceSpecification {
                substitute,
                ..Default::default()
            },
        );

        world.insert_resource(resource_specifications);
        world.insert_resource(Events::<AccountTransaction>::default());

        let mut stage = SystemStage::parallel();
        stage.add_system(production_building.system());

        Self { world, stage }
    }
}

#[test]
fn produces_resource() {
    let mut setup = TestSetup::new();

    let coke_storage_id = setup.add_storage(COKE, 0.0);
    let coal_storage_id = setup.add_storage(COAL, 10.0);

    let building_id = setup
        .world
        .spawn()
        .insert(Statistics::default())
        .insert(ProductionBuilding {
            products: vec![
                (
                    Product {
                        resource: COKE.to_owned(),
                        rate: 1.0,
                        requisites: vec![ProductDependency {
                            resource: COAL.to_owned(),
                            rate: 2.0,
                        }],
                        ..Default::default()
                    },
                    true,
                ),
                (
                    Product {
                        resource: COKE.to_owned(),
                        rate: 1.0,
                        requisites: vec![ProductDependency {
                            resource: COAL.to_owned(),
                            rate: 2.0,
                        }],
                        ..Default::default()
                    },
                    false,
                ),
            ],
        })
        .insert(StorageConsolidator {
            connected_storage: vec![coal_storage_id, coke_storage_id],
        })
        .id();

    setup.stage.run(&mut setup.world);

    setup.assert_storage_amount(coke_storage_id, 1.0);
    setup.assert_storage_amount(coal_storage_id, 8.0);
    setup.assert_production_statistic(COKE, building_id, 1.0);
    setup.assert_consumption_statistic(COAL, building_id, 2.0);

    // if already full
    setup.set_storage_amount(coke_storage_id, 10.0);

    setup.stage.run(&mut setup.world);

    // no overflow
    setup.assert_storage_amount(coke_storage_id, 10.0);
    setup.assert_storage_amount(coal_storage_id, 8.0);
    setup.assert_production_statistic(COKE, building_id, 1.0);
    setup.assert_consumption_statistic(COAL, building_id, 2.0);

    // no requisites left
    setup.set_storage_amount(coal_storage_id, 0.0);
    setup.set_storage_amount(coke_storage_id, 0.0);
    setup.assert_production_statistic(COKE, building_id, 1.0);
    setup.assert_consumption_statistic(COAL, building_id, 2.0);

    setup.stage.run(&mut setup.world);

    // no production
    setup.assert_storage_amount(coke_storage_id, 0.0);
    setup.assert_storage_amount(coal_storage_id, 0.0);
    setup.assert_production_statistic(COKE, building_id, 1.0);
    setup.assert_consumption_statistic(COAL, building_id, 2.0);
}

#[test]
fn produces_byproducts() {
    let mut setup = TestSetup::new();

    let coke_storage_id = setup.add_storage(COKE, 0.0);
    let coal_storage_id = setup.add_storage(COAL, 10.0);
    let slug_storage_id = setup.add_storage(SLUG, 0.0);

    let building_id = setup
        .world
        .spawn()
        .insert(Statistics::default())
        .insert(ProductionBuilding {
            products: vec![(
                Product {
                    resource: COKE.to_owned(),
                    rate: 1.0,
                    requisites: vec![ProductDependency {
                        resource: COAL.to_owned(),
                        rate: 2.0,
                    }],
                    byproducts: vec![ProductDependency {
                        resource: SLUG.to_owned(),
                        rate: 1.0,
                    }],
                    ..Default::default()
                },
                true,
            )],
        })
        .insert(StorageConsolidator {
            connected_storage: vec![coal_storage_id, coke_storage_id],
        })
        .id();

    setup.stage.run(&mut setup.world);

    setup.assert_storage_amount(coke_storage_id, 1.0);
    setup.assert_storage_amount(coal_storage_id, 8.0);
    setup.assert_storage_amount(slug_storage_id, 0.0);
    setup.assert_production_statistic(COKE, building_id, 1.0);
    setup.assert_production_statistic(SLUG, building_id, 0.0);
    setup.assert_consumption_statistic(COAL, building_id, 2.0);

    // slug storage connected
    setup
        .world
        .get_entity_mut(building_id)
        .unwrap()
        .insert(StorageConsolidator {
            connected_storage: vec![coal_storage_id, coke_storage_id, slug_storage_id],
        });

    setup.stage.run(&mut setup.world);

    setup.assert_storage_amount(coke_storage_id, 2.0);
    setup.assert_storage_amount(coal_storage_id, 6.0);
    setup.assert_storage_amount(slug_storage_id, 1.0);
    setup.assert_production_statistic(COKE, building_id, 2.0);
    setup.assert_production_statistic(SLUG, building_id, 1.0);
    setup.assert_consumption_statistic(COAL, building_id, 4.0);
}

#[test]
fn increases_production_with_enhancers() {
    let mut setup = TestSetup::new();

    let coke_storage_id = setup.add_storage(COKE, 0.0);
    let coal_storage_id = setup.add_storage(COAL, 10.0);
    let enhancer_storage_id = setup.add_storage(ENHANCER, 10.0);
    let slug_storage_id = setup.add_storage(SLUG, 0.0);

    let building_id = setup
        .world
        .spawn()
        .insert(Statistics::default())
        .insert(ProductionBuilding {
            products: vec![(
                Product {
                    resource: COKE.to_owned(),
                    rate: 1.0,
                    cost: 0.0,
                    requisites: vec![ProductDependency {
                        resource: COAL.to_owned(),
                        rate: 2.0,
                    }],
                    enhancers: vec![ProductEnhancer {
                        resource: ENHANCER.to_owned(),
                        rate: 1.0,
                        modifier: 2.0,
                    }],
                    byproducts: vec![ProductDependency {
                        resource: SLUG.to_owned(),
                        rate: 1.0,
                    }],
                },
                true,
            )],
        })
        .insert(StorageConsolidator {
            connected_storage: vec![coal_storage_id, coke_storage_id, slug_storage_id],
        })
        .id();

    setup.stage.run(&mut setup.world);

    setup.assert_storage_amount(coke_storage_id, 1.0);
    setup.assert_storage_amount(coal_storage_id, 8.0);
    setup.assert_storage_amount(slug_storage_id, 1.0);
    setup.assert_storage_amount(enhancer_storage_id, 10.0);
    setup.assert_production_statistic(COKE, building_id, 1.0);
    setup.assert_production_statistic(SLUG, building_id, 1.0);
    setup.assert_consumption_statistic(ENHANCER, building_id, 0.0);
    setup.assert_consumption_statistic(COAL, building_id, 2.0);

    // enhancer storage connected
    setup
        .world
        .get_entity_mut(building_id)
        .unwrap()
        .insert(StorageConsolidator {
            connected_storage: vec![
                coal_storage_id,
                coke_storage_id,
                slug_storage_id,
                enhancer_storage_id,
            ],
        });

    setup.stage.run(&mut setup.world);

    setup.assert_storage_amount(coke_storage_id, 3.0);
    setup.assert_storage_amount(coal_storage_id, 6.0);
    setup.assert_storage_amount(slug_storage_id, 3.0);
    setup.assert_storage_amount(enhancer_storage_id, 9.0);
    setup.assert_production_statistic(COKE, building_id, 3.0);
    setup.assert_production_statistic(SLUG, building_id, 3.0);
    setup.assert_consumption_statistic(ENHANCER, building_id, 1.0);
    setup.assert_consumption_statistic(COAL, building_id, 4.0);
}

#[test]
fn increases_production_with_enhancer_substitute() {
    let mut setup = TestSetup::new();

    let coke_storage_id = setup.add_storage(COKE, 0.0);
    let coal_storage_id = setup.add_storage(COAL, 10.0);
    let enhancer_storage_id = setup.add_storage(ENHANCER_REPLACEMENT, 10.0);
    let slug_storage_id = setup.add_storage(SLUG, 0.0);

    let building_id = setup
        .world
        .spawn()
        .insert(Statistics::default())
        .insert(ProductionBuilding {
            products: vec![(
                Product {
                    resource: COKE.to_owned(),
                    rate: 1.0,
                    cost: 0.0,
                    requisites: vec![ProductDependency {
                        resource: COAL.to_owned(),
                        rate: 2.0,
                    }],
                    enhancers: vec![ProductEnhancer {
                        resource: ENHANCER.to_owned(),
                        rate: 1.0,
                        modifier: 2.0,
                    }],
                    byproducts: vec![ProductDependency {
                        resource: SLUG.to_owned(),
                        rate: 1.0,
                    }],
                },
                true,
            )],
        })
        .insert(StorageConsolidator {
            connected_storage: vec![
                coal_storage_id,
                coke_storage_id,
                slug_storage_id,
                enhancer_storage_id,
            ],
        })
        .id();

    setup.stage.run(&mut setup.world);

    setup.assert_storage_amount(coke_storage_id, 2.0);
    setup.assert_storage_amount(coal_storage_id, 8.0);
    setup.assert_storage_amount(slug_storage_id, 2.0);
    setup.assert_storage_amount(enhancer_storage_id, 9.0);
    setup.assert_production_statistic(COKE, building_id, 2.0);
    setup.assert_production_statistic(SLUG, building_id, 2.0);
    setup.assert_consumption_statistic(ENHANCER_REPLACEMENT, building_id, 1.0);
    setup.assert_consumption_statistic(COAL, building_id, 2.0);
}

#[test]
fn no_production_is_marked_idle() {
    let mut setup = TestSetup::new();

    let coke_storage_id = setup.add_storage(COKE, 0.0);
    let coal_storage_id = setup.add_storage(COAL, 10.0);

    let building_id = setup
        .world
        .spawn()
        .insert(Statistics::default())
        .insert(ProductionBuilding {
            products: vec![(
                Product {
                    resource: COKE.to_owned(),
                    rate: 1.0,
                    requisites: vec![ProductDependency {
                        resource: COAL.to_owned(),
                        rate: 2.0,
                    }],
                    ..Default::default()
                },
                true,
            )],
        })
        .insert(StorageConsolidator {
            connected_storage: vec![],
        })
        .id();

    setup.stage.run(&mut setup.world);

    assert!(setup.world.get::<Idle>(building_id).is_some());

    setup
        .world
        .get_entity_mut(building_id)
        .unwrap()
        .insert(StorageConsolidator {
            connected_storage: vec![coal_storage_id, coke_storage_id],
        });

    setup.stage.run(&mut setup.world);

    assert!(setup.world.get::<Idle>(building_id).is_none());
}

#[test]
fn random_production() {
    let mut setup = TestSetup::new();

    let coke_storage_id = setup.add_storage(COKE, 0.0);
    let coal_storage_id = setup.add_storage(COAL, 0.0);

    setup
        .world
        .spawn()
        .insert(Statistics::default())
        .insert(ProductionBuilding {
            products: vec![
                (
                    Product {
                        resource: COAL.to_owned(),
                        rate: 1.0,
                        requisites: vec![],
                        ..Default::default()
                    },
                    true,
                ),
                (
                    Product {
                        resource: COKE.to_owned(),
                        rate: 1.0,
                        requisites: vec![],
                        ..Default::default()
                    },
                    true,
                ),
            ],
        })
        .insert(StorageConsolidator {
            connected_storage: vec![coal_storage_id, coke_storage_id],
        });

    setup.stage.run(&mut setup.world);

    assert!(
        ((setup.get_storage_amount(coke_storage_id) - 1.0).abs() < f64::EPSILON)
            ^ ((setup.get_storage_amount(coal_storage_id) - 1.0).abs() < f64::EPSILON)
    );
}

#[test]
fn ignores_under_construction() {
    let mut setup = TestSetup::new();

    let coal_storage_id = setup.add_storage(COAL, 0.0);

    let building_id = setup
        .world
        .spawn()
        .insert(Statistics::default())
        .insert(ProductionBuilding {
            products: vec![(
                Product {
                    resource: COAL.to_owned(),
                    rate: 1.0,
                    ..Default::default()
                },
                true,
            )],
        })
        .insert(StorageConsolidator {
            connected_storage: vec![coal_storage_id],
        })
        .insert(UnderConstruction::from_fixed_cost(0))
        .id();

    setup.stage.run(&mut setup.world);

    setup.assert_storage_amount(coal_storage_id, 0.0);
    setup.assert_production_statistic(COAL, building_id, 0.0);
}

#[test]
fn with_cost() {
    let mut setup = TestSetup::new();

    let coke_storage_id = setup.add_storage(COKE, 0.0);

    setup
        .world
        .spawn()
        .insert(Statistics::default())
        .insert(ProductionBuilding {
            products: vec![(
                Product {
                    resource: COKE.to_owned(),
                    cost: 10.0,
                    rate: 1.0,
                    requisites: vec![],
                    ..Default::default()
                },
                true,
            )],
        })
        .insert(StorageConsolidator {
            connected_storage: vec![coke_storage_id],
        });

    setup.stage.run(&mut setup.world);
    setup.assert_storage_amount(coke_storage_id, 1.0);
    setup.assert_event_sum(-10);
}
