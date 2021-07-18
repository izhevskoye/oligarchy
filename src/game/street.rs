use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::{RequiresUpdate, Street},
    constants::MapTile,
    setup::{BUILDING_LAYER_ID, MAP_ID, VEHICLE_LAYER_ID},
};

#[derive(Default, Clone, Debug)]
pub struct NeighborStructure {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
    pub source: Box<Option<NeighborStructure>>,
}

impl From<NeighborStructure> for u16 {
    fn from(ns: NeighborStructure) -> u16 {
        if ns.north && ns.south && ns.west && ns.east {
            return MapTile::StreetNorthEastSouthWest as u16;
        }

        if ns.south && ns.west && ns.east {
            return MapTile::StreetEastSouthWest as u16;
        }

        if ns.north && ns.west && ns.east {
            return MapTile::StreetNorthEastWest as u16;
        }

        if ns.north && ns.south && ns.west {
            return MapTile::StreetNorthSouthWest as u16;
        }

        if ns.north && ns.south && ns.east {
            return MapTile::StreetNorthEastSouth as u16;
        }

        if ns.north && ns.east {
            return MapTile::StreetNorthEast as u16;
        }

        if ns.north && ns.west {
            return MapTile::StreetNorthWest as u16;
        }

        if ns.south && ns.east {
            return MapTile::StreetEastSouth as u16;
        }

        if ns.south && ns.west {
            return MapTile::StreetSouthWest as u16;
        }

        if ns.north && ns.south {
            return MapTile::StreetNorthSouth as u16;
        }

        if ns.west && ns.east {
            return MapTile::StreetEastWest as u16;
        }

        if ns.north || ns.south {
            return MapTile::StreetNorthSouth as u16;
        }

        if ns.west || ns.east {
            return MapTile::StreetEastWest as u16;
        }

        MapTile::StreetNorthEastSouthWest as u16
    }
}

fn eval_neighbor(entity: Option<Entity>, street_query: &Query<&Street>) -> bool {
    if let Some(entity) = entity {
        if street_query.get(entity).is_ok() {
            return true;
        }
    }

    false
}

pub fn update_streets(
    mut commands: Commands,
    mut tile_query: Query<(Entity, &mut Tile, &RequiresUpdate), With<Street>>,
    street_query: Query<&Street>,
    mut map_query: MapQuery,
) {
    for (entity, mut tile, update) in tile_query.iter_mut() {
        let mut ns = NeighborStructure::default();
        let pos = UVec2::new(update.position.x, update.position.y);
        let neighbors = map_query.get_tile_neighbors(pos, MAP_ID, BUILDING_LAYER_ID);
        // N, S, W, E, NW, NE, SW, SE.
        ns.north = eval_neighbor(neighbors[0].1, &street_query);
        ns.south = eval_neighbor(neighbors[1].1, &street_query);
        ns.west = eval_neighbor(neighbors[2].1, &street_query);
        ns.east = eval_neighbor(neighbors[3].1, &street_query);

        tile.texture_index = ns.clone().into();

        map_query.notify_chunk_for_tile(pos, MAP_ID, BUILDING_LAYER_ID);

        {
            let mut pos = pos * 2;
            pos.y += 1;
            let entity = map_query
                .get_tile_entity(pos, MAP_ID, VEHICLE_LAYER_ID)
                .unwrap();

            commands.entity(entity).insert(NeighborStructure {
                north: false,
                east: false,
                south: ns.south || (ns.east && !ns.west) || (!ns.south && ns.north),
                west: ns.west,
                source: Box::new(Some(ns.clone())),
            });
            map_query.notify_chunk_for_tile(pos, MAP_ID, VEHICLE_LAYER_ID);
        }
        {
            let mut pos = pos * 2;
            pos.y += 1;
            pos.x += 1;
            let entity = map_query
                .get_tile_entity(pos, MAP_ID, VEHICLE_LAYER_ID)
                .unwrap();

            commands.entity(entity).insert(NeighborStructure {
                north: ns.north,
                east: false,
                south: false,
                west: ns.west || (ns.south && !ns.north) || ns.east,
                source: Box::new(Some(ns.clone())),
            });
            map_query.notify_chunk_for_tile(pos, MAP_ID, VEHICLE_LAYER_ID);
        }
        {
            let pos = pos * 2;
            let entity = map_query
                .get_tile_entity(pos, MAP_ID, VEHICLE_LAYER_ID)
                .unwrap();

            commands.entity(entity).insert(NeighborStructure {
                north: false,
                east: ns.east || ns.north || ns.west,
                south: ns.south,
                west: false,
                source: Box::new(Some(ns.clone())),
            });
            map_query.notify_chunk_for_tile(pos, MAP_ID, VEHICLE_LAYER_ID);
        }
        {
            let mut pos = pos * 2;
            pos.x += 1;
            let entity = map_query
                .get_tile_entity(pos, MAP_ID, VEHICLE_LAYER_ID)
                .unwrap();

            commands.entity(entity).insert(NeighborStructure {
                north: ns.north || (ns.west && !ns.east),
                east: ns.east,
                south: false,
                west: false,
                source: Box::new(Some(ns.clone())),
            });
            map_query.notify_chunk_for_tile(pos, MAP_ID, VEHICLE_LAYER_ID);
        }

        commands.entity(entity).remove::<RequiresUpdate>();
    }
}
