#[cfg(test)]
mod tests;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct StatisticTracker {
    data: HashMap<String, f64>,
}

impl StatisticTracker {
    pub fn merge(&mut self, other: &StatisticTracker) {
        for (resource, amount) in other.data.iter() {
            let value = amount + self.data.get(resource).unwrap_or(&0.0);

            self.data.insert(resource.to_owned(), value);
        }
    }

    pub fn track(&mut self, resource: &str, amount: f64) {
        self.data.insert(
            resource.to_owned(),
            self.data.get(resource).unwrap_or(&0.0) + amount,
        );
    }

    pub fn get(&self, resource: &str) -> f64 {
        *self.data.get(resource).unwrap_or(&0.0)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Statistics {
    pub production: StatisticTracker,
    pub consumption: StatisticTracker,
    pub export: StatisticTracker,
    pub import: StatisticTracker,
}
