use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{Building, ExportStation, RequiresUpdate},
    building_specifications::BuildingSpecifications,
    constants::MapTile,
    storage::Storage,
};

use super::{
    assets::{DeliveryStation, Occupied, Position},
    resource_specifications::ResourceSpecifications,
    setup::{GROUND_LAYER_ID, MAP_ID},
};

pub fn building_update(
    mut query: Query<(&mut Tile, &Building), With<RequiresUpdate>>,
    buildings: Res<BuildingSpecifications>,
) {
    for (mut tile, building) in query.iter_mut() {
        let building = buildings.get(&building.id).unwrap();

        tile.texture_index = building.tile;
        tile.visible = true;
    }
}

pub fn storage_update(
    mut query: Query<(&mut Tile, &Storage), With<RequiresUpdate>>,
    resources: Res<ResourceSpecifications>,
) {
    for (mut tile, storage) in query.iter_mut() {
        let resource = resources
            .get(&storage.resource)
            .unwrap_or_else(|| panic!("Expected {} resource to exist", storage.resource));

        tile.texture_index = resource.storage_tile.unwrap_or(MapTile::Storage as u16);
        tile.visible = true;
    }
}

pub fn export_station_update(
    mut query: Query<&mut Tile, (With<ExportStation>, With<RequiresUpdate>)>,
) {
    for mut tile in query.iter_mut() {
        tile.texture_index = MapTile::ExportStation as u16;
        tile.visible = true;
    }
}

pub fn delivery_station_update(
    mut query: Query<&mut Tile, (With<DeliveryStation>, With<RequiresUpdate>)>,
) {
    for mut tile in query.iter_mut() {
        tile.texture_index = MapTile::DeliveryStation as u16;
        tile.visible = true;
    }
}

pub fn ground_update(
    query: Query<&Position, (With<Occupied>, With<RequiresUpdate>)>,
    mut tile_query: Query<&mut Tile>,
    mut map_query: MapQuery,
) {
    for position in query.iter() {
        let entity = map_query
            .get_tile_entity(position.position, MAP_ID, GROUND_LAYER_ID)
            .unwrap();

        let mut tile = tile_query.get_mut(entity).unwrap();
        tile.texture_index = MapTile::GroundFactory as u16;

        map_query.notify_chunk_for_tile(position.position, MAP_ID, GROUND_LAYER_ID);
    }
}
