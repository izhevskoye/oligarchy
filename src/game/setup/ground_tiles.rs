use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use rand::{thread_rng, Rng};

use crate::game::{
    assets::{BlockedForBuilding, Forest, MapSettings, Position, RequiresUpdate, Water},
    constants::CHUNK_SIZE,
    helper::get_entity::get_entity,
    setup::GROUND_LAYER_ID,
    GenerateGroundTilesEvent,
};

pub fn generate_tiles(
    mut commands: Commands,
    mut map_query: MapQuery,
    map_settings: Res<MapSettings>,
    mut events: EventReader<GenerateGroundTilesEvent>,
) {
    for _ in events.iter() {
        log::info!("Generating ground tiles");
        let mut random = thread_rng();
        let perlin = Perlin::new();
        let seed = random.gen();
        let perlin = perlin.set_seed(seed);

        for x in 0..map_settings.width * CHUNK_SIZE - 1 {
            for y in 0..map_settings.height * CHUNK_SIZE - 1 {
                let position = UVec2::new(x, y);
                let val = perlin.get([(x as f64 + 0.2) / 15.0, (y as f64 + 0.3) / 15.0]);

                let entity = if val < -0.5 {
                    let entity =
                        get_entity(&mut commands, &mut map_query, position, GROUND_LAYER_ID);

                    commands.entity(entity).insert(Water);

                    Some(entity)
                } else {
                    let val = perlin.get([(x as f64 + 0.6) / 13.0, (y as f64 + 0.1) / 13.0, 1.0]);

                    if val < -0.5 {
                        let entity =
                            get_entity(&mut commands, &mut map_query, position, GROUND_LAYER_ID);

                        commands.entity(entity).insert(Forest);

                        Some(entity)
                    } else {
                        None
                    }
                };

                if let Some(entity) = entity {
                    commands
                        .entity(entity)
                        .insert(Position { position })
                        .insert(BlockedForBuilding)
                        .insert(RequiresUpdate);
                }
            }
        }
    }
}
