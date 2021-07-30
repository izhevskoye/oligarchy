mod calculate_destination;
mod drive_to_destination;
mod instructions;

use bevy::{core::FixedTimestep, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::game::{
    assets::{Direction, RequiresUpdate},
    constants::VehicleTile,
    resource_specifications::ResourceSpecifications,
    setup::{MAP_ID, VEHICLE_LAYER_ID},
};

pub use calculate_destination::calculate_destination;

use super::assets::{CarTileDefinition, Storage};

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
    WaitForLoad(String),
    WaitForUnload(String),
    Load(String),
    Unload(String),
}

impl CarInstructions {
    pub fn format(&self, resources: &ResourceSpecifications) -> String {
        match self {
            CarInstructions::Nop => "Idle".to_string(),
            CarInstructions::GoTo(position) => format!("Drive to {}", position),
            CarInstructions::WaitForLoad(resource) => {
                format!("Wait to load {:?}", resources.get(resource).unwrap().name)
            }
            CarInstructions::WaitForUnload(resource) => {
                format!("Wait to unload {:?}", resources.get(resource).unwrap().name)
            }
            CarInstructions::Load(resource) => {
                format!("Load {:?}", resources.get(resource).unwrap().name)
            }
            CarInstructions::Unload(resource) => {
                format!("Unload {:?}", resources.get(resource).unwrap().name)
            }
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
    car_query: Query<(Entity, &Car, &Storage), With<RequiresUpdate>>,
    mut map_query: MapQuery,
    resources: Res<ResourceSpecifications>,
) {
    for (entity, car, storage) in car_query.iter() {
        commands.entity(entity).remove::<RequiresUpdate>();

        let resource = resources.get(&storage.resource).unwrap();
        let car_tiles = if let Some(tile_spec) = &resource.car_tile {
            tile_spec.clone()
        } else {
            CarTileDefinition {
                vertical: VehicleTile::BlueVertical as u16,
                horizontal: VehicleTile::BlueHorizontal as u16,
            }
        };

        let tile = Tile {
            texture_index: if car.direction == Direction::North || car.direction == Direction::South
            {
                car_tiles.vertical
            } else {
                car_tiles.horizontal
            } as u16,
            flip_y: car.direction == Direction::South,
            flip_x: car.direction == Direction::East,
            ..Default::default()
        };

        let _ = map_query.set_tile(&mut commands, car.position, tile, MAP_ID, VEHICLE_LAYER_ID);
        map_query.notify_chunk_for_tile(car.position, MAP_ID, VEHICLE_LAYER_ID);
    }
}
