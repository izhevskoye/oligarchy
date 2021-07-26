mod blast_furnace;
mod coke_furnace;
mod export_station;
mod oxygen_converter;
mod production_building;

use bevy::{core::FixedTimestep, prelude::*};

pub fn production_system() -> SystemSet {
    // TODO: refactor time
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(export_station::export_station.system())
        .with_system(oxygen_converter::oxygen_converter.system())
        .with_system(blast_furnace::blast_furnace.system())
        .with_system(coke_furnace::coke_furnace.system())
        .with_system(production_building::production_building.system())
}
