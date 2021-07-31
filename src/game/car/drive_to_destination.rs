use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

use crate::game::{
    assets::{Direction, RequiresUpdate},
    car::{Car, Waypoints},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

pub fn drive_to_destination(
    mut commands: Commands,
    mut car_query: Query<(Entity, &mut Car)>,
    mut waypoint_query: Query<&mut Waypoints>,
    map_query: MapQuery,
) {
    let mut car_positions: HashSet<UVec2> =
        car_query.iter_mut().map(|(_, car)| car.position).collect();

    for (car_entity, mut car) in car_query.iter_mut() {
        if !car.active {
            continue;
        }

        let mut waypoint = match waypoint_query.get_mut(car_entity) {
            Ok(waypoint) => waypoint,
            Err(_) => continue,
        };

        let direction = waypoint.waypoints[0];
        let c_pos = car.position / 2;

        let mut direction = if direction.x < c_pos.x {
            Direction::West
        } else if direction.x > c_pos.x {
            Direction::East
        } else if direction.y < c_pos.y {
            Direction::South
        } else if direction.y > c_pos.y {
            Direction::North
        } else {
            Direction::None
        };

        if direction == Direction::North && car.position.x % 2 == 0 {
            direction = Direction::East;
        } else if direction == Direction::South && car.position.x % 2 == 1 {
            direction = Direction::West;
        } else if direction == Direction::East && car.position.y % 2 == 1 {
            direction = Direction::South;
        } else if direction == Direction::West && car.position.y % 2 == 0 {
            direction = Direction::North;
        }

        let mut new_car_position = car.position;

        if direction == Direction::West {
            new_car_position.x -= 1;
        } else if direction == Direction::East {
            new_car_position.x += 1;
        } else if direction == Direction::South {
            new_car_position.y -= 1;
        } else if direction == Direction::North {
            new_car_position.y += 1;
        } else {
            // we are on correct tile
            waypoint.waypoints = waypoint.waypoints[1..].iter().copied().collect();

            if waypoint.waypoints.is_empty() {
                commands.entity(car_entity).remove::<Waypoints>();
            }
        }

        let can_drive_to_new_pos = !car_positions.contains(&new_car_position);

        if !can_drive_to_new_pos && direction != Direction::None {
            log::warn!("Car is blocked");
            waypoint.mark_blocked();
        } else {
            waypoint.mark_unblocked();
        }

        if can_drive_to_new_pos {
            car_positions.remove(&car.position);
            car_positions.insert(new_car_position);

            car.position = new_car_position;
            if direction != Direction::None {
                car.direction = direction;
            }

            commands.entity(car_entity).insert(RequiresUpdate {
                position: car.position,
            });
        }

        if waypoint.considered_deadlocked() {
            log::error!("Car considered deadlocked. Moving away.");

            let (_entity, layer) = map_query.get_layer(MAP_ID, BUILDING_LAYER_ID).unwrap();
            let size = layer.get_layer_size_in_tiles().as_i32() * 2;

            // move into opposite
            let c_pos = c_pos.as_i32();
            let mut move_away_position = match direction {
                Direction::West => c_pos + IVec2::new(0, 1),
                Direction::East => c_pos + IVec2::new(0, -1),
                Direction::North => c_pos + IVec2::new(-1, 0),
                Direction::South => c_pos + IVec2::new(1, 0),
                Direction::None => c_pos + IVec2::new(1, 0),
            };

            let mut random = thread_rng();
            // randomize sometimes to prevent some deadlock situations
            if random.gen_range(0..3) == 0 {
                move_away_position.x += random.gen_range(-3..3);
                move_away_position.y += random.gen_range(-3..3);
            }

            if move_away_position.x < 0 {
                move_away_position.x = 0;
            }
            if move_away_position.x >= size.x {
                move_away_position.x = size.x;
            }
            if move_away_position.y < 0 {
                move_away_position.y = 0;
            }
            if move_away_position.y >= size.y {
                move_away_position.y = size.y;
            }

            waypoint.waypoints = vec![move_away_position.as_u32()];
            waypoint.mark_unblocked();
        }
    }
}
