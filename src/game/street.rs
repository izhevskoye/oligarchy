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
    helper::{
        eval_neighbor::EvalNeighbor,
        neighbor_structure::{LayerIndex, NeighborStructure},
    },
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Street;

impl From<Street> for MapTile {
    fn from(_: Street) -> MapTile {
        MapTile::ExportStation
    }
}

impl From<Street> for LayerIndex {
    fn from(_: Street) -> LayerIndex {
        LayerIndex::new(BUILDING_LAYER_ID)
    }
}

impl PurchaseCost for Street {
    fn price(&self, _resources: &ResourceSpecifications) -> i64 {
        100
    }
}

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
            query: &street_query,
        };

        ns.north = en.eval_neighbor(neighbors[0]);
        ns.south = en.eval_neighbor(neighbors[1]);
        ns.west = en.eval_neighbor(neighbors[2]);
        ns.east = en.eval_neighbor(neighbors[3]);

        let ns_index: u16 = ns.clone().into();
        tile.texture_index = ns_index + MapTile::from(Street::default()) as u16;
        tile.visible = true;
    }
}
