use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{Building, ExportStation, RequiresUpdate, Resource, Storage},
    building_specifications::BuildingSpecifications,
    constants::MapTile,
};

use super::{
    assets::Occupied,
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

pub fn storage_update(mut query: Query<(&mut Tile, &Storage), With<RequiresUpdate>>) {
    for (mut tile, storage) in query.iter_mut() {
        tile.texture_index = match storage.resource {
            Resource::Coal => MapTile::StorageCoal,
            Resource::Coke => MapTile::StorageCoke,
            Resource::Limestone => MapTile::StorageLimestone,
            Resource::IronOre => MapTile::StorageIronOre,
            Resource::Iron => MapTile::StorageIron,
            Resource::Steel => MapTile::StorageSteel,
            // _ => MapTile::Storage,
        } as u16;
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

pub fn ground_update(
    query: Query<&RequiresUpdate, With<Occupied>>,
    mut tile_query: Query<&mut Tile>,
    mut map_query: MapQuery,
) {
    for update in query.iter() {
        let entity = map_query
            .get_tile_entity(update.position, MAP_ID, GROUND_LAYER_ID)
            .unwrap();

        let mut tile = tile_query.get_mut(entity).unwrap();
        tile.texture_index = MapTile::GroundFactory as u16;

        map_query.notify_chunk_for_tile(update.position, MAP_ID, GROUND_LAYER_ID);

        log::info!("ground updated!!!");
    }
}
