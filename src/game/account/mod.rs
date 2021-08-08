use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::resource_specifications::ResourceSpecifications;

const START_VALUE: i64 = 250000;

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Account {
    pub value: i64,
}

pub struct AccountTransaction {
    pub amount: i64,
}

pub fn account_transactions(
    mut account: ResMut<Account>,
    mut transactions: EventReader<AccountTransaction>,
) {
    for event in transactions.iter() {
        account.value += event.amount;
    }
}

pub fn reset_account(mut account: ResMut<Account>) {
    account.value = START_VALUE;
}

pub trait PurchaseCost {
    fn price(&self, resources: &ResourceSpecifications) -> i64;
}
