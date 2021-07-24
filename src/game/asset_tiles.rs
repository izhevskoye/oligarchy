use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::{
        BlastFurnace, CokeFurnace, ExportStation, OxygenConverter, Quarry, RequiresUpdate,
        Resource, Storage,
    },
    constants::MapTile,
};

pub fn quarry_update(mut query: Query<(&mut Tile, &Quarry), With<RequiresUpdate>>) {
    for (mut tile, quarry) in query.iter_mut() {
        tile.texture_index = match quarry.resource {
            Resource::Limestone => MapTile::LimestoneQuarry,
            Resource::Coal => MapTile::CoalQuarry,
            Resource::IronOre => MapTile::IronOreQuarry,
            _ => panic!("Invalid Quarry Type"),
        } as u16;
        tile.visible = true;
    }
}

pub fn storage_update(mut query: Query<&mut Tile, (With<Storage>, With<RequiresUpdate>)>) {
    for mut tile in query.iter_mut() {
        tile.texture_index = MapTile::Storage as u16;
        tile.visible = true;
    }
}

pub fn coke_furnace_update(mut query: Query<&mut Tile, (With<CokeFurnace>, With<RequiresUpdate>)>) {
    for mut tile in query.iter_mut() {
        tile.texture_index = MapTile::CokeFurnace as u16;
        tile.visible = true;
    }
}

pub fn blast_furnace_update(
    mut query: Query<&mut Tile, (With<BlastFurnace>, With<RequiresUpdate>)>,
) {
    for mut tile in query.iter_mut() {
        tile.texture_index = MapTile::BlastFurnace as u16;
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

pub fn oxygen_converter_update(
    mut query: Query<&mut Tile, (With<OxygenConverter>, With<RequiresUpdate>)>,
) {
    for mut tile in query.iter_mut() {
        tile.texture_index = MapTile::OxygenConverter as u16;
        tile.visible = true;
    }
}
