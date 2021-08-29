use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use hierarchical_pathfinding::prelude::*;

use crate::game::{
    assets::{
        BlockedForBuilding, CanDriveOver, Occupied, Position, RemovedBuildingEvent, RequiresUpdate,
    },
    setup::{BUILDING_LAYER_ID, GROUND_LAYER_ID, MAP_ID},
    street::Street,
};

const STREET_COST: isize = 1;
const GRASS_COST: isize = 10;
const BUILDING_COST: isize = -1;

pub fn cost_fn<'a, 'b: 'a>(
    map_query: &'b MapQuery,
    street_query: &'a Query<(), With<Street>>,
    occupied_query: &'a Query<(), (With<Occupied>, Without<CanDriveOver>)>,
    blocked_query: &'a Query<(), With<BlockedForBuilding>>,
) -> impl 'a + Fn((usize, usize)) -> isize {
    move |(x, y)| {
        let pos = UVec2::new(x as u32, y as u32);
        if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, GROUND_LAYER_ID) {
            if blocked_query.get(entity).is_ok() {
                return BUILDING_COST;
            }
        }

        match map_query.get_tile_entity(pos, MAP_ID, BUILDING_LAYER_ID) {
            Ok(entity) => {
                if street_query.get(entity).is_ok() {
                    STREET_COST
                } else if occupied_query.get(entity).is_ok() {
                    BUILDING_COST
                } else {
                    GRASS_COST
                }
            }
            Err(_) => GRASS_COST,
        }
    }
}

#[derive(Default)]
pub struct Pathfinding {
    pub cache: Option<PathCache<ManhattanNeighborhood>>,
}

pub fn update(
    street_query: Query<(), With<Street>>,
    occupied_query: Query<(), (With<Occupied>, Without<CanDriveOver>)>,
    blocked_query: Query<(), With<BlockedForBuilding>>,
    update_query: Query<&Position, (With<Tile>, With<RequiresUpdate>)>,
    map_query: MapQuery,
    mut pathfinding: ResMut<Pathfinding>,
    mut removed_events: EventReader<RemovedBuildingEvent>,
) {
    if pathfinding.cache.is_none() {
        log::info!("Building pathfinding cache");
        let (_entity, layer) = map_query.get_layer(MAP_ID, BUILDING_LAYER_ID).unwrap();
        let mut size = layer.get_layer_size_in_tiles();
        size.x -= 1;
        size.y -= 1;

        let cache = PathCache::new(
            (size.x as usize, size.y as usize),
            cost_fn(&map_query, &street_query, &occupied_query, &blocked_query),
            ManhattanNeighborhood::new(size.x as usize, size.y as usize),
            PathCacheConfig {
                chunk_size: 2,
                ..Default::default()
            },
        );

        pathfinding.cache = Some(cache);
    } else {
        let mut changes: Vec<(usize, usize)> = update_query
            .iter()
            .map(|position| (position.position.x as usize, position.position.y as usize))
            .collect();

        for event in removed_events.iter() {
            changes.push((event.position.x as usize, event.position.y as usize));
        }

        if !changes.is_empty() {
            if changes.len() > 10 {
                log::warn!("Too many updates, discarding pathfinding cache!");
                pathfinding.cache = None;
            } else {
                // safe unwrap due because it is always created above
                log::info!("Updating pathfinding cache: {:?}", changes);
                let pathfinding = pathfinding.cache.as_mut().unwrap();
                pathfinding.tiles_changed(
                    &changes,
                    cost_fn(&map_query, &street_query, &occupied_query, &blocked_query),
                );
            }
        }
    }
}
