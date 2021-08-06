use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::{resource_specifications::ResourceSpecifications, statistics::Statistics};

pub struct Goal {
    pub amount: f64,
    pub current: f64,
}

#[derive(Default)]
pub struct GoalManager {
    pub goals: HashMap<String, Goal>,
}

// TODO: TEST
pub fn generate_goals(resources: Res<ResourceSpecifications>, mut manager: ResMut<GoalManager>) {
    for resource in resources.keys() {
        manager.goals.insert(
            resource.to_string(),
            Goal {
                amount: 10000.0,
                current: 0.0,
            },
        );
    }
}

// TODO: TEST
pub fn update_goals(query: Query<&Statistics>, mut manager: ResMut<GoalManager>) {
    for (_, goal) in manager.goals.iter_mut() {
        goal.current = 0.0;
    }

    for statistic in query.iter() {
        for (resource, count) in statistic.export.data().iter() {
            if let Some(goal) = manager.goals.get_mut(resource) {
                goal.current += count;
            }
        }
    }
}
