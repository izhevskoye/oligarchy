use super::*;

#[test]
fn goal_generation() {
    let mut world = World::default();

    let mut stage = SystemStage::parallel();
    stage.add_system(generate_goals.system());

    let goal_manager = GoalManager::default();
    world.insert_resource(goal_manager);
    world.insert_resource(MapSettings::default());

    stage.run(&mut world);

    let goal_manager = world.get_resource::<GoalManager>().unwrap();

    assert!(!goal_manager.goals.is_empty());
}

#[test]
fn remove_goals_when_completed() {
    let mut world = World::default();

    let mut stage = SystemStage::parallel();
    stage.add_system(update_goals.system());

    let mut goal_manager = GoalManager::default();
    goal_manager.goals.insert(
        "yolo".to_owned(),
        Goal {
            amount: 10.0,
            current: 0.0,
        },
    );

    world.insert_resource(goal_manager);
    world.insert_resource(StatisticTracker::default());
    world.insert_resource(MapSettings::default());

    stage.run(&mut world);

    let goal_manager = world.get_resource::<GoalManager>().unwrap();

    assert!(!goal_manager.goals.is_empty());

    // we set all statistics to complete all goals
    let mut statistics = Statistics::default();

    for (resource, goal) in goal_manager.goals.iter() {
        statistics.export.track(resource, goal.amount);
    }

    world.spawn().insert(statistics);

    stage.run(&mut world);

    let goal_manager = world.get_resource::<GoalManager>().unwrap();

    assert!(goal_manager.goals.is_empty());
}
