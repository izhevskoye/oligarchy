pub mod building;
pub mod bulldoze;
pub mod car;
pub mod delivery_station;
pub mod depot;
pub mod import_export_station;
pub mod storage;
pub mod storage_management;
pub mod street;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::RequiresUpdate,
    production::ImportExportDirection,
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

#[derive(PartialEq, Eq, Debug)]
pub enum Tool {
    None,
    Bulldoze,
    Street,
    Path,
    Storage(String),
    ImportExportStation(ImportExportDirection),
    DeliveryStation,
    StorageManagement,
    Depot,
    Car(String),
    Building(String),
}

pub struct SelectedTool {
    pub tool: Tool,
}

impl Default for SelectedTool {
    fn default() -> Self {
        Self { tool: Tool::None }
    }
}

fn update_neighbor_streets<T: 'static + Send + Sync>(
    commands: &mut Commands,
    map_query: &mut MapQuery,
    pos: UVec2,
    street_query: Query<&T>,
) {
    let neighbors = map_query.get_tile_neighbors(pos, MAP_ID, BUILDING_LAYER_ID);
    for (_pos, neighbor) in neighbors[0..4].iter() {
        if let Some(neighbor) = neighbor {
            if street_query.get(*neighbor).is_ok() {
                commands.entity(*neighbor).insert(RequiresUpdate);
            }
        }
    }
}
