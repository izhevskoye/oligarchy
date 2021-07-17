use crate::game::assets::{BlastFurnace, CokeFurnace, OxygenConverter, RequiresUpdate};

use super::{
    assets::{Quarry, Resource, Storage, StorageConsolidator},
    constants::{
        MapTile, CHUNK_SIZE, MAP_HEIGHT, MAP_WIDTH, TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_SIZE,
    },
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub const MAP_ID: u16 = 0;
pub const LAYER_ID: u16 = 0;
pub const BUILDING_LAYER_ID: u16 = 1;

pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.far = 1000.0 / 0.1;
    commands.spawn_bundle(camera);

    let texture_handle = asset_server.load("oligarchy_tiles.png");
    let (hw, hh) = (TILE_SIZE, TILE_SIZE);
    let (wc, hc) = (TILE_MAP_WIDTH, TILE_MAP_HEIGHT);
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(MAP_ID, map_entity);

    let mut map_settings = LayerSettings::new(
        UVec2::new(MAP_WIDTH, MAP_HEIGHT),
        UVec2::new(CHUNK_SIZE, CHUNK_SIZE),
        Vec2::new(hw, hh),
        Vec2::new(hw * wc, hh * hc),
    );
    map_settings.mesh_type = TilemapMeshType::Square;

    let (mut layer_builder, layer_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, map_settings.clone(), MAP_ID, LAYER_ID);
    map.add_layer(&mut commands, LAYER_ID, layer_entity);

    layer_builder.fill(
        UVec2::new(0, 0),
        UVec2::new(CHUNK_SIZE * MAP_WIDTH - 1, CHUNK_SIZE * MAP_HEIGHT - 1),
        Tile {
            texture_index: MapTile::Ground as u16,
            ..Default::default()
        }
        .into(),
    );

    map_query.build_layer(&mut commands, layer_builder, material_handle.clone());

    let (mut layer_builder, layer_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, map_settings, MAP_ID, BUILDING_LAYER_ID);
    map.add_layer(&mut commands, BUILDING_LAYER_ID, layer_entity);

    {
        let pos = UVec2::new(11, 10);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands.entity(entity).insert(Storage {
            resource: Resource::Coal,
            amount: 0,
            capacity: 20,
        });
        entity
    };

    {
        let pos = UVec2::new(10, 10);
        let tile = Tile {
            texture_index: MapTile::Quarry as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Quarry {
                resource: Resource::Coal,
            })
            .insert(StorageConsolidator {
                connected_storage: vec![],
            })
            .insert(RequiresUpdate { position: pos });
    }

    {
        let pos = UVec2::new(13, 10);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands.entity(entity).insert(Storage {
            resource: Resource::Coke,
            amount: 0,
            capacity: 20,
        });
        entity
    };

    {
        let pos = UVec2::new(12, 10);
        let tile = Tile {
            texture_index: MapTile::CokeFurnace as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(CokeFurnace)
            .insert(StorageConsolidator {
                connected_storage: vec![],
            })
            .insert(RequiresUpdate { position: pos });
    }

    {
        let pos = UVec2::new(14, 11);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands.entity(entity).insert(Storage {
            resource: Resource::Limestone,
            amount: 0,
            capacity: 20,
        });
        entity
    };

    {
        let pos = UVec2::new(13, 11);
        let tile = Tile {
            texture_index: MapTile::Quarry as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Quarry {
                resource: Resource::Limestone,
            })
            .insert(StorageConsolidator {
                connected_storage: vec![],
            })
            .insert(RequiresUpdate { position: pos });
    }

    {
        let pos = UVec2::new(15, 10);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands.entity(entity).insert(Storage {
            resource: Resource::IronOre,
            amount: 0,
            capacity: 20,
        });

        entity
    };

    {
        let pos = UVec2::new(16, 10);
        let tile = Tile {
            texture_index: MapTile::Quarry as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Quarry {
                resource: Resource::IronOre,
            })
            .insert(StorageConsolidator {
                connected_storage: vec![],
            })
            .insert(RequiresUpdate { position: pos });
    }

    {
        let pos = UVec2::new(14, 9);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands.entity(entity).insert(Storage {
            resource: Resource::Iron,
            amount: 0,
            capacity: 20,
        });

        entity
    };

    {
        let pos = UVec2::new(14, 10);
        let tile = Tile {
            texture_index: MapTile::BlastFurnace as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(BlastFurnace)
            .insert(StorageConsolidator {
                connected_storage: vec![],
            })
            .insert(RequiresUpdate { position: pos });
    }

    {
        let pos = UVec2::new(14, 7);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands.entity(entity).insert(Storage {
            resource: Resource::Steel,
            amount: 0,
            capacity: 20,
        });

        entity
    };

    {
        let pos = UVec2::new(14, 8);
        let tile = Tile {
            texture_index: MapTile::OxygenConverter as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(OxygenConverter)
            .insert(StorageConsolidator {
                connected_storage: vec![],
            })
            .insert(RequiresUpdate { position: pos });
    }

    map_query.build_layer(&mut commands, layer_builder, material_handle);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(GlobalTransform::default());
}
