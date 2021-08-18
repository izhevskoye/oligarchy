mod eval_neighbor;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::{
    account::PurchaseCost,
    assets::{
        resource_specifications::ResourceSpecifications, MapSettings, Position, RequiresUpdate,
    },
    constants::MapTile,
    construction::UnderConstruction,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

use eval_neighbor::EvalNeighbor;

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Street;

impl PurchaseCost for Street {
    fn price(&self, _resources: &ResourceSpecifications) -> i64 {
        100
    }
}

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

        if ns.north {
            return MapTile::StreetNorthEnd as u16;
        }

        if ns.south {
            return MapTile::StreetSouthEnd as u16;
        }

        if ns.west {
            return MapTile::StreetWestEnd as u16;
        }

        if ns.east {
            return MapTile::StreetEastEnd as u16;
        }

        MapTile::StreetNone as u16
    }
}

#[allow(clippy::type_complexity)]
pub fn update_streets(
    mut tile_query: Query<
        (&mut Tile, &Position),
        (
            With<Street>,
            With<RequiresUpdate>,
            Without<UnderConstruction>,
        ),
    >,
    street_query: Query<(), With<Street>>,
    map_query: MapQuery,
    map_settings: Res<MapSettings>,
) {
    for (mut tile, position) in tile_query.iter_mut() {
        let mut ns = NeighborStructure::default();
        let pos = UVec2::new(position.position.x, position.position.y);
        let neighbors = map_query.get_tile_neighbors(pos, MAP_ID, BUILDING_LAYER_ID);
        // N, S, W, E, NW, NE, SW, SE.
        let en = EvalNeighbor {
            map_settings: &map_settings,
            street_query: &street_query,
        };

        ns.north = en.eval_neighbor(neighbors[0]);
        ns.south = en.eval_neighbor(neighbors[1]);
        ns.west = en.eval_neighbor(neighbors[2]);
        ns.east = en.eval_neighbor(neighbors[3]);

        tile.texture_index = ns.clone().into();
        tile.visible = true;
    }
}
