use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::{Building, ExportStation, RequiresUpdate, Storage},
    building_specifications::BuildingSpecifications,
    constants::MapTile,
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

pub fn storage_update(mut query: Query<&mut Tile, (With<Storage>, With<RequiresUpdate>)>) {
    for mut tile in query.iter_mut() {
        tile.texture_index = MapTile::Storage as u16;
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
