mod calculate_destination;
mod drive_to_destination;
mod instructions;

use bevy::{core::FixedTimestep, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::egui::Ui;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{
    assets::{Direction, InfoUI, RequiresUpdate, Resource},
    constants::VehicleTile,
    setup::{MAP_ID, VEHICLE_LAYER_ID},
};

pub use calculate_destination::calculate_destination;

pub struct Destination {
    pub destination: UVec2,
}

pub struct Waypoints {
    pub waypoints: Vec<UVec2>,
    pub blocked_ticks: i64,
}

impl Waypoints {
    pub fn new(waypoints: Vec<UVec2>) -> Self {
        Self {
            waypoints,
            blocked_ticks: 0,
        }
    }

    pub fn mark_unblocked(&mut self) {
        self.blocked_ticks = 0;
    }

    pub fn mark_blocked(&mut self) {
        self.blocked_ticks += 1;
    }

    pub fn considered_deadlocked(&self) -> bool {
        let mut random = thread_rng();

        random.gen_range(0..20) <= self.blocked_ticks - 25
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CarInstructions {
    Nop,
    GoTo(UVec2),
    WaitForLoad(Resource),
    WaitForUnload(Resource),
    Load(Resource),
    Unload(Resource),
}

impl fmt::Display for CarInstructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CarInstructions::Nop => write!(f, "Idle"),
            CarInstructions::GoTo(position) => write!(f, "Drive to {}", position),
            CarInstructions::WaitForLoad(resource) => write!(f, "Wait to load {:?}", resource),
            CarInstructions::WaitForUnload(resource) => write!(f, "Wait to unload {:?}", resource),
            CarInstructions::Load(resource) => write!(f, "Load {:?}", resource),
            CarInstructions::Unload(resource) => write!(f, "Unload {:?}", resource),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
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

pub fn update_car(
    mut commands: Commands,
    car_query: Query<(Entity, &Car), With<RequiresUpdate>>,
    mut map_query: MapQuery,
) {
    for (entity, car) in car_query.iter() {
        commands.entity(entity).remove::<RequiresUpdate>();

        let tile = Tile {
            texture_index: if car.direction == Direction::North || car.direction == Direction::South
            {
                VehicleTile::BlueVertical
            } else {
                VehicleTile::BlueHorizontal
            } as u16,
            flip_y: car.direction == Direction::South,
            flip_x: car.direction == Direction::East,
            ..Default::default()
        };

        let _ = map_query.set_tile(&mut commands, car.position, tile, MAP_ID, VEHICLE_LAYER_ID);
        map_query.notify_chunk_for_tile(car.position, MAP_ID, VEHICLE_LAYER_ID);
    }
}
