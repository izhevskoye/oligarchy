use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{Direction, RequiresUpdate},
    setup::{MAP_ID, VEHICLE_LAYER_ID},
};

use super::{Car, Waypoints};

pub fn drive_to_destination(
    mut commands: Commands,
    mut car_query: Query<(Entity, &mut Car, &mut Waypoints)>,
    tile_query: Query<&Tile>,
    mut map_query: MapQuery,
) {
    for (car_entity, mut car, mut waypoint) in car_query.iter_mut() {
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

        // TODO: better :)
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

        let can_drive_to_new_pos = if let Ok(entity) =
            map_query.get_tile_entity(new_car_position, MAP_ID, VEHICLE_LAYER_ID)
        {
            tile_query.get(entity).is_err()
        } else {
            true
        };

        if !can_drive_to_new_pos {
            log::warn!("Car is blocked");
        }

        if can_drive_to_new_pos {
            if let Ok(entity) = map_query.get_tile_entity(car.position, MAP_ID, VEHICLE_LAYER_ID) {
                map_query.notify_chunk_for_tile(car.position, MAP_ID, VEHICLE_LAYER_ID);
                commands.entity(entity).despawn_recursive();
            }

            car.position = new_car_position;
            if direction != Direction::None {
                car.direction = direction;
            }

            commands.entity(car_entity).insert(RequiresUpdate {
                position: car.position,
            });
        }
    }
}
