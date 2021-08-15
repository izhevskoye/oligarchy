#[cfg(test)]
mod tests;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::constants::CURRENCY;

use super::assets::resource_specifications::ResourceSpecifications;

const START_VALUE: i64 = 250000;

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Account {
    pub value: i64,
}

pub struct AccountTransaction {
    pub amount: i64,
}

pub struct MaintenanceCost {
    pub amount: f64,
}

impl MaintenanceCost {
    pub fn new_from_cost(cost: i64) -> Self {
        Self {
            amount: cost as f64 * 0.00005,
        }
    }
}

pub fn account_transactions(
    mut account: ResMut<Account>,
    mut transactions: EventReader<AccountTransaction>,
) {
    for event in transactions.iter() {
        account.value += event.amount;
    }
}

pub fn maintenance_cost(mut account: ResMut<Account>, query: Query<&MaintenanceCost>) {
    let mut sum = 0.0;
    for cost in query.iter() {
        sum += cost.amount;
    }

    account.value -= sum as i64;
}

pub fn reset_account(mut account: ResMut<Account>) {
    account.value = START_VALUE;
}

pub trait PurchaseCost {
    fn price(&self, resources: &ResourceSpecifications) -> i64;

    fn price_description(&self, resources: &ResourceSpecifications) -> String {
        format!(
            "Labor and material worth {} {}",
            self.price(resources),
            CURRENCY
        )
    }
}
