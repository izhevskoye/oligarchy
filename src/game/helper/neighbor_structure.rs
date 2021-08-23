use crate::game::{
    assets::{MapSettings, Position, RequiresUpdate},
    constants::{MapTile, VariantOffsets},
    helper::eval_neighbor::EvalNeighbor,
    setup::MAP_ID,
};

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct LayerIndex(u16);

impl LayerIndex {
    pub fn new(v: u16) -> Self {
        Self(v)
    }
}

#[derive(Default, Clone, Debug)]
pub struct NeighborStructure {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
}

impl From<NeighborStructure> for u16 {
    fn from(ns: NeighborStructure) -> u16 {
        if ns.north && ns.south && ns.west && ns.east {
            return VariantOffsets::NorthEastSouthWest as u16;
        }

        if ns.south && ns.west && ns.east {
            return VariantOffsets::EastSouthWest as u16;
        }

        if ns.north && ns.west && ns.east {
            return VariantOffsets::NorthEastWest as u16;
        }

        if ns.north && ns.south && ns.west {
            return VariantOffsets::NorthSouthWest as u16;
        }

        if ns.north && ns.south && ns.east {
            return VariantOffsets::NorthEastSouth as u16;
        }

        if ns.north && ns.east {
            return VariantOffsets::NorthEast as u16;
        }

        if ns.north && ns.west {
            return VariantOffsets::NorthWest as u16;
        }

        if ns.south && ns.east {
            return VariantOffsets::EastSouth as u16;
        }

        if ns.south && ns.west {
            return VariantOffsets::SouthWest as u16;
        }

        if ns.north && ns.south {
            return VariantOffsets::NorthSouth as u16;
        }

        if ns.west && ns.east {
            return VariantOffsets::EastWest as u16;
        }

        if ns.north {
            return VariantOffsets::North as u16;
        }

        if ns.south {
            return VariantOffsets::South as u16;
        }

        if ns.west {
            return VariantOffsets::West as u16;
        }

        if ns.east {
            return VariantOffsets::East as u16;
        }

        VariantOffsets::None as u16
    }
}

pub fn update_tile<T: 'static + Send + Sync + Default>(
    mut tile_query: Query<(&mut Tile, &Position), (With<T>, With<RequiresUpdate>)>,
    query: Query<(), With<T>>,
    map_query: MapQuery,
    map_settings: Res<MapSettings>,
) where
    MapTile: From<T>,
    LayerIndex: From<T>,
{
    let layer = LayerIndex::from(T::default()).0;

    for (mut tile, position) in tile_query.iter_mut() {
        let mut ns = NeighborStructure::default();
        let pos = UVec2::new(position.position.x, position.position.y);
        let neighbors = map_query.get_tile_neighbors(pos, MAP_ID, layer);
        // N, S, W, E, NW, NE, SW, SE.
        let en = EvalNeighbor {
            map_settings: &map_settings,
            query: &query,
        };

        ns.north = en.eval_neighbor(neighbors[0]);
        ns.south = en.eval_neighbor(neighbors[1]);
        ns.west = en.eval_neighbor(neighbors[2]);
        ns.east = en.eval_neighbor(neighbors[3]);

        let ns_index: u16 = ns.clone().into();
        tile.texture_index = MapTile::from(T::default()) as u16 + ns_index;

        tile.visible = true;
    }
}
