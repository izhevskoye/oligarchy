use bevy::prelude::*;

use super::{Car, Destination, Waypoints};

pub fn calculate_destination(
    mut commands: Commands,
    mut car_query: Query<(Entity, &Destination), With<Car>>,
) {
    for (car_entity, destination) in car_query.iter_mut() {
        let waypoints = vec![destination.destination];

        // TODO: better :)

        commands
            .entity(car_entity)
            .insert(Waypoints { waypoints })
            .remove::<Destination>();
    }
}
