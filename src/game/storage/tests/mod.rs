mod distribute_to_storage;
mod fetch_from_storage;
mod has_in_storage;
mod has_space_in_storage;

use super::*;

const COKE: &str = "coke";

#[derive(Default)]
struct TestAmount {
    pub amount: f64,
}

#[derive(Default)]
struct TestResult {
    pub result: bool,
}
