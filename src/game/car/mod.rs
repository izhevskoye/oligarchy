mod calculate_destination;
mod drive_to_destination;
mod instructions;

use bevy::{core::FixedTimestep, prelude::*};
use bevy_egui::egui::Ui;
use std::fmt;

use super::assets::{Direction, InfoUI, Resource};

pub struct Destination {
    pub destination: UVec2,
}

pub struct Waypoints {
    pub waypoints: Vec<UVec2>,
}

#[derive(Clone)]
pub enum CarInstructions {
    Nop,
    GoTo(UVec2),
    WaitForLoad(Resource),
    WaitForUnload(Resource),
}

impl fmt::Display for CarInstructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CarInstructions::Nop => write!(f, "Idle"),
            CarInstructions::GoTo(position) => write!(f, "Drive to {}", position),
            CarInstructions::WaitForLoad(resource) => write!(f, "Wait to load {:?}", resource),
            CarInstructions::WaitForUnload(resource) => write!(f, "Wait to unload {:?}", resource),
        }
    }
}

pub struct Car {
    pub position: UVec2,
    pub direction: Direction,
    pub instructions: Vec<CarInstructions>,
    pub current_instruction: usize,
}

impl InfoUI for Car {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Car");
        });
    }
}

pub fn drive_system() -> SystemSet {
    // TODO: refactor time
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.2))
        .with_system(drive_to_destination::drive_to_destination.system())
}

pub fn instruction_system() -> SystemSet {
    // TODO: refactor time
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(instructions::car_instruction.system())
}

pub fn calculate_system() -> SystemSet {
    // TODO: refactor time
    SystemSet::new().with_system(calculate_destination::calculate_destination.system())
}
