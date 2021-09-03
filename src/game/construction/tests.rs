use bevy::app::Events;

use crate::game::assets::resource_specifications::ResourceSpecification;

use super::*;

const STEEL: &str = "steel";
const STEEL_PRICE: f64 = 10.0;

struct TestSetupParams {
    amount: i64,
}

struct TestSetup {
    world: World,
    stage: SystemStage,
}

impl TestSetup {
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

        assert_eq!(sum, amount);
    }

    fn new(params: TestSetupParams) -> Self {
        let mut world = World::default();

        let mut stage = SystemStage::parallel();
        stage.add_system(construction.system());

        world.insert_resource(Account {
            value: params.amount,
        });
        world.insert_resource(Events::<AccountTransaction>::default());
        let mut resources = HashMap::new();
        resources.insert(
            STEEL.to_owned(),
            ResourceSpecification {
                cost: STEEL_PRICE,
                ..Default::default()
            },
        );
        world.insert_resource(resources);

        Self { world, stage }
    }
}

#[test]
fn drains_money() {
    let mut setup = TestSetup::new(TestSetupParams { amount: 1000 });
    let entity = setup
        .world
        .spawn()
        .insert(UnderConstruction::from_fixed_cost(2000))
        .id();

    setup.stage.run(&mut setup.world);

    setup.assert_event_sum(-1000);

    let construction = setup.world.get::<UnderConstruction>(entity).unwrap();

    assert_eq!(construction.labor as i64, 1000);
}

#[test]
fn insufficient_money_for_all() {
    let mut setup = TestSetup::new(TestSetupParams { amount: 1000 });

    let entity = setup
        .world
        .spawn()
        .insert(UnderConstruction::from_fixed_cost(2000))
        .id();

    let entity_two = setup
        .world
        .spawn()
        .insert(UnderConstruction::from_fixed_cost(2000))
        .id();

    setup.stage.run(&mut setup.world);

    setup.assert_event_sum(-1000);

    let mut total_costs = 0.0;

    let construction = setup.world.get::<UnderConstruction>(entity).unwrap();
    total_costs += construction.labor;

    let construction = setup.world.get::<UnderConstruction>(entity_two).unwrap();
    total_costs += construction.labor;

    assert_eq!(total_costs as i64, 3000);
}

#[test]
fn purchases_resources() {
    let mut setup = TestSetup::new(TestSetupParams { amount: 1000 });

    let mut resources_needed = HashMap::new();
    resources_needed.insert(STEEL.to_owned(), 10.0);

    let entity = setup
        .world
        .spawn()
        .insert(UnderConstruction {
            labor: 0.0,
            resources_needed,
        })
        .id();

    setup.stage.run(&mut setup.world);

    setup.assert_event_sum(-(5.0 * STEEL_PRICE) as i64);

    let construction = setup.world.get::<UnderConstruction>(entity).unwrap();
    assert!((construction.resources_needed.get(STEEL).unwrap() - 5.0) < f64::EPSILON);
}
