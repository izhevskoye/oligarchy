use bevy::prelude::*;

// TODO: load and reset

#[derive(Default)]
pub struct Account {
    // TODO: not f64!
    pub value: f64,
}

pub struct AccountTransaction {
    pub amount: f64,
}

pub fn account_transactions(
    mut account: ResMut<Account>,
    mut transactions: EventReader<AccountTransaction>,
) {
    for event in transactions.iter() {
        account.value += event.amount;
    }
}
