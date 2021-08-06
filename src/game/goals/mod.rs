mod loader;

use rand::{prelude::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::statistics::Statistics;

use super::assets::{MapSettings, MapSize};

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct GoalSetItemAmounts {
    pub small: f64,
    pub medium: f64,
    pub large: f64,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct GoalSetItem {
    pub resource: String,
    pub amount: GoalSetItemAmounts,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct GoalSet {
    pub name: String,
    pub goals: Vec<GoalSetItem>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Goal {
    pub amount: f64,
    pub current: f64,
}

pub struct GoalManager {
    pub goal_sets: Vec<GoalSet>,
    pub goals: HashMap<String, Goal>,
}

impl Default for GoalManager {
    fn default() -> Self {
        let mut new = Self {
            goal_sets: vec![],
            goals: HashMap::new(),
        };
        new.load_specifications();
        new
    }
}

// TODO: TEST
pub fn generate_goals(mut manager: ResMut<GoalManager>, map_settings: Res<MapSettings>) {
    let mut goal_sets = manager.goal_sets.clone();
    let mut random = thread_rng();
    goal_sets.shuffle(&mut random);

    manager.goals = HashMap::new();
    if let Some(goal_set) = goal_sets.get(0) {
        for goal in &goal_set.goals {
            let amount = match map_settings.size {
                MapSize::Small => goal.amount.small,
                MapSize::Medium => goal.amount.medium,
                MapSize::Large => goal.amount.large,
            };

            manager.goals.insert(
                goal.resource.to_string(),
                Goal {
                    amount,
                    current: 0.0,
                },
            );
        }
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
