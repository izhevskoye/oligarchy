mod export_station;
mod production_building;

use bevy::{core::FixedTimestep, prelude::*};

pub fn production_system() -> SystemSet {
    // TODO: refactor time
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(export_station::export_station.system())
        .with_system(production_building::production_building.system())
}
