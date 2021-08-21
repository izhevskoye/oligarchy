use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use rand::{thread_rng, Rng};

use crate::game::{
    assets::{MapSettings, Position, RequiresUpdate},
    constants::{MapTile, CHUNK_SIZE},
    helper::{get_entity::get_entity, neighbor_structure::LayerIndex},
    setup::GROUND_LAYER_ID,
    state_manager::NewGameEvent,
};

pub struct BlockedForBuilding;

#[derive(Default)]
pub struct Forrest;

impl From<Forrest> for MapTile {
    fn from(_: Forrest) -> MapTile {
        MapTile::ForrestTilesOffset
    }
}

impl From<Forrest> for LayerIndex {
    fn from(_: Forrest) -> LayerIndex {
        LayerIndex::new(GROUND_LAYER_ID)
    }
}

#[derive(Default)]
pub struct Water;

impl From<Water> for MapTile {
    fn from(_: Water) -> MapTile {
        MapTile::WaterTilesOffset
    }
}

impl From<Water> for LayerIndex {
    fn from(_: Water) -> LayerIndex {
        LayerIndex::new(GROUND_LAYER_ID)
    }
}

#[allow(clippy::type_complexity)]
pub fn generate_tiles(
    mut commands: Commands,
    mut map_query: MapQuery,
    map_settings: Res<MapSettings>,
    mut events: EventReader<NewGameEvent>,
) {
    for _ in events.iter() {
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

                        commands.entity(entity).insert(Forrest);

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
