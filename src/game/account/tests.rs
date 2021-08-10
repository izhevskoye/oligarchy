use bevy::app::Events;

use super::*;

#[test]
fn transaction_processing() {
    let mut world = World::default();

    world.insert_resource(Account::default());
    world.insert_resource(Events::<AccountTransaction>::default());

    let mut stage = SystemStage::parallel();
    stage.add_system(account_transactions.system());

    let mut events = world
        .get_resource_mut::<Events<AccountTransaction>>()
        .unwrap();
    events.send(AccountTransaction { amount: -100 });
    events.send(AccountTransaction { amount: -100 });
    events.send(AccountTransaction { amount: 50 });

    stage.run(&mut world);

    let account = world.get_resource::<Account>().unwrap();

    assert_eq!(account.value, -150);
}

#[test]
fn reset() {
    let mut world = World::default();

    world.insert_resource(Account { value: 1234 });

    let mut stage = SystemStage::parallel();
    stage.add_system(reset_account.system());

    stage.run(&mut world);

    let account = world.get_resource::<Account>().unwrap();

    assert_eq!(account.value, START_VALUE);
}

#[test]
fn maintenance_cost_emitting() {
    let mut world = World::default();

    world.insert_resource(Account { value: 10000 });

    world.spawn().insert(MaintenanceCost { amount: 0.3 });
    world.spawn().insert(MaintenanceCost { amount: 0.3 });
    world.spawn().insert(MaintenanceCost { amount: 0.3 });
    world.spawn().insert(MaintenanceCost { amount: 100.0 });

    let mut stage = SystemStage::parallel();
    stage.add_system(maintenance_cost.system());

    stage.run(&mut world);

    let account = world.get_resource::<Account>().unwrap();

    // rounds by removing fractions
    assert_eq!(account.value, 10000 - 100);
}
