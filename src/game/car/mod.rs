pub mod calculate_destination;
pub mod drive_to_destination;
pub mod instructions;

use bevy::prelude::*;
use bevy_egui::egui::Ui;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::game::{
    account::PurchaseCost,
    assets::{
        resource_specifications::{CarTileDefinition, ResourceSpecifications},
        Direction, InfoUI, Position, RequiresUpdate,
    },
    constants::{VehicleTile, TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_SIZE, Z_CAR},
    storage::Storage,
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
    pub direction: Direction,
    pub instructions: Vec<CarInstructions>,
    pub current_instruction: usize,
    pub active: bool,
}

impl Default for Car {
    fn default() -> Self {
        Self {
            direction: Direction::North,
            instructions: vec![CarInstructions::Nop],
            current_instruction: 0,
            active: false,
        }
    }
}

impl PurchaseCost for (Car, Storage) {
    fn price(&self, resources: &ResourceSpecifications) -> i64 {
        let resource = resources
            .get(&self.1.resource)
            .unwrap_or_else(|| panic!("expected to find resource {} in spec", self.1.resource));

        ((resource.cost * self.1.capacity) / 100.0) as i64 + 250
    }
}

impl InfoUI for Car {
    fn ui(&self, ui: &mut Ui, _resources: &ResourceSpecifications) {
        ui.horizontal(|ui| {
            ui.label("Car");
        });
    }
}

fn update_car_sprite(
    sprite: &mut TextureAtlasSprite,
    transform: &mut Transform,
    car: &Car,
    position: &Position,
    storage: &Storage,
    resources: &Res<ResourceSpecifications>,
) {
    let resource = resources.get(&storage.resource).unwrap();
    let car_tiles = if let Some(tile_spec) = &resource.car_tile {
        tile_spec.clone()
    } else {
        CarTileDefinition {
            vertical: VehicleTile::BlueVertical as u16,
            horizontal: VehicleTile::BlueHorizontal as u16,
        }
    };

    let tile_size = TILE_SIZE / 2.0;
    let position = Vec2::new(
        position.position.x as f32 + 0.5,
        position.position.y as f32 + 0.5,
    );
    let translation = (position * tile_size).extend(Z_CAR);
    transform.translation = translation;

    sprite.index = if car.direction == Direction::North || car.direction == Direction::South {
        car_tiles.vertical
    } else {
        car_tiles.horizontal
    } as u32;
    sprite.flip_x = car.direction == Direction::East;
    sprite.flip_y = car.direction == Direction::South;
}

pub fn spawn_car(
    mut commands: Commands,
    mut car_query: Query<(Entity, &Car, &Storage, &Position), Without<TextureAtlasSprite>>,
    resources: Res<ResourceSpecifications>,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (entity, car, storage, position) in car_query.iter_mut() {
        let texture_handle = assets.load("oligarchy_tiles.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::splat(TILE_SIZE / 2.0),
            TILE_MAP_WIDTH as usize * 2,
            TILE_MAP_HEIGHT as usize * 2,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut sprite = TextureAtlasSprite::new(0);
        let mut transform = Transform::default();
        update_car_sprite(
            &mut sprite,
            &mut transform,
            car,
            position,
            storage,
            &resources,
        );

        commands.entity(entity).insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            sprite,
            ..Default::default()
        });
    }
}

#[allow(clippy::type_complexity)]
pub fn update_car(
    mut commands: Commands,
    mut car_query: Query<
        (
            Entity,
            &Car,
            &Position,
            &Storage,
            &mut Transform,
            &mut TextureAtlasSprite,
        ),
        With<RequiresUpdate>,
    >,
    resources: Res<ResourceSpecifications>,
) {
    for (entity, car, position, storage, mut transform, mut sprite) in car_query.iter_mut() {
        commands.entity(entity).remove::<RequiresUpdate>();

        update_car_sprite(
            &mut sprite,
            &mut transform,
            car,
            position,
            storage,
            &resources,
        );
    }
}
