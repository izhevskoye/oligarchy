use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct StatisticTracker {
    data: HashMap<String, f64>,
}

impl StatisticTracker {
    pub fn track(&mut self, resource: &str, amount: f64) {
        self.data.insert(
            resource.to_owned(),
            self.data.get(resource).unwrap_or(&0.0) + amount,
        );
    }

    #[cfg(test)]
    pub fn get(&self, resource: &str) -> f64 {
        *self.data.get(resource).unwrap_or(&0.0)
    }

    pub fn data(&self) -> &HashMap<String, f64> {
        &self.data
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Statistics {
    pub production: StatisticTracker,
    pub consumption: StatisticTracker,
    pub export: StatisticTracker,
}
