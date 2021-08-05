use bevy::prelude::*;

use crate::game::resource_specifications::ResourceSpecifications;

pub struct Goal {
    pub resource: String,
    pub amount: f64,
    pub current: f64,
}

#[derive(Default)]
pub struct GoalManager {
    pub goals: Vec<Goal>,
}

pub fn generate_goals(resources: Res<ResourceSpecifications>, mut manager: ResMut<GoalManager>) {
    for resource in resources.keys() {
        manager.goals.push(Goal {
            resource: resource.to_string(),
            amount: 10000.0,
            current: 0.0,
        });
    }
}
