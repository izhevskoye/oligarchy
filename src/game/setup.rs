use crate::game::car::{Car, CarInstructions};

use super::{
    assets::{
        BlastFurnace, CokeFurnace, Direction, ExportStation, Occupied, OxygenConverter, Quarry,
        RequiresUpdate, Resource, Storage, StorageConsolidator, Street,
    },
    constants::{
        MapTile, VehicleTile, CHUNK_SIZE, MAP_HEIGHT, MAP_WIDTH, TILE_MAP_HEIGHT, TILE_MAP_WIDTH,
        TILE_SIZE,
    },
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub const MAP_ID: u16 = 0;
pub const LAYER_ID: u16 = 0;
pub const BUILDING_LAYER_ID: u16 = 1;
pub const VEHICLE_LAYER_ID: u16 = 2;

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

        commands
            .entity(entity)
            .insert(Storage {
                resource: Resource::Coal,
                amount: 0,
                capacity: 20,
            })
            .insert(Occupied);
    }

    {
        let pos = UVec2::new(10, 10);
        let tile = Tile {
            texture_index: MapTile::CoalQuarry as u16,
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
            .insert(RequiresUpdate { position: pos })
            .insert(Occupied);
    }

    {
        let pos = UVec2::new(13, 10);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Storage {
                resource: Resource::Coke,
                amount: 0,
                capacity: 20,
            })
            .insert(Occupied);
    }

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
            .insert(RequiresUpdate { position: pos })
            .insert(Occupied);
    }

    {
        let pos = UVec2::new(14, 11);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Storage {
                resource: Resource::Limestone,
                amount: 0,
                capacity: 20,
            })
            .insert(Occupied);
    }

    {
        let pos = UVec2::new(14, 12);
        let tile = Tile {
            texture_index: MapTile::LimestoneQuarry as u16,
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
            .insert(RequiresUpdate { position: pos })
            .insert(Occupied);
    }

    {
        let pos = UVec2::new(15, 10);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Storage {
                resource: Resource::IronOre,
                amount: 0,
                capacity: 20,
            })
            .insert(Occupied);
    }

    {
        let pos = UVec2::new(25, 10);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Storage {
                resource: Resource::IronOre,
                amount: 0,
                capacity: 20,
            })
            .insert(Occupied);
    }

    {
        let pos = UVec2::new(26, 10);
        let tile = Tile {
            texture_index: MapTile::IronOreQuarry as u16,
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
            .insert(RequiresUpdate { position: pos })
            .insert(Occupied);
    }

    {
        let pos = UVec2::new(14, 9);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Storage {
                resource: Resource::Iron,
                amount: 0,
                capacity: 20,
            })
            .insert(Occupied);
    }

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
            .insert(RequiresUpdate { position: pos })
            .insert(Occupied);
    }

    {
        let pos = UVec2::new(14, 7);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Storage {
                resource: Resource::Steel,
                amount: 0,
                capacity: 20,
            })
            .insert(Occupied);
    }

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
            .insert(RequiresUpdate { position: pos })
            .insert(Occupied);
    }

    {
        let pos = UVec2::new(10, 1);
        let tile = Tile {
            texture_index: MapTile::Storage as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Storage {
                resource: Resource::Steel,
                amount: 0,
                capacity: 20,
            })
            .insert(Occupied);
    }

    {
        let pos = UVec2::new(10, 0);
        let tile = Tile {
            texture_index: MapTile::ExportStation as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(ExportStation {
                goods: vec![Resource::Steel],
            })
            .insert(StorageConsolidator {
                connected_storage: vec![],
            })
            .insert(RequiresUpdate { position: pos })
            .insert(Occupied);
    }

    for x in 16..25 {
        let pos = UVec2::new(x, 10);
        let tile = Tile {
            texture_index: MapTile::StreetNorthEastSouthWest as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Street)
            .insert(RequiresUpdate { position: pos })
            .insert(Occupied);
    }

    for y in 5..10 {
        let pos = UVec2::new(17, y);
        let tile = Tile {
            texture_index: MapTile::StreetNorthEastSouthWest as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Street)
            .insert(RequiresUpdate { position: pos })
            .insert(Occupied);
    }

    for x in 0..17 {
        let pos = UVec2::new(x, 5);
        let tile = Tile {
            texture_index: MapTile::StreetNorthEastSouthWest as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Street)
            .insert(RequiresUpdate { position: pos })
            .insert(Occupied);
    }

    for y in 2..5 {
        let pos = UVec2::new(10, y);
        let tile = Tile {
            texture_index: MapTile::StreetNorthEastSouthWest as u16,
            ..Default::default()
        };
        let _ = layer_builder.set_tile(pos, tile.into());

        let entity = layer_builder.get_tile_entity(pos).unwrap();

        commands
            .entity(entity)
            .insert(Street)
            .insert(RequiresUpdate { position: pos })
            .insert(Occupied);
    }

    map_query.build_layer(&mut commands, layer_builder, material_handle);

    // vehicles

    let texture_handle = asset_server.load("oligarchy_tiles.png");
    let (hw, hh) = (TILE_SIZE / 2.0, TILE_SIZE / 2.0);
    let (wc, hc) = (TILE_MAP_WIDTH * 2.0, TILE_MAP_HEIGHT * 2.0);
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    let mut map_settings = LayerSettings::new(
        UVec2::new(MAP_WIDTH * 2, MAP_HEIGHT * 2),
        UVec2::new(CHUNK_SIZE, CHUNK_SIZE),
        Vec2::new(hw, hh),
        Vec2::new(hw * wc, hh * hc),
    );
    map_settings.mesh_type = TilemapMeshType::Square;

    let (mut layer_builder, layer_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, map_settings, MAP_ID, VEHICLE_LAYER_ID);
    map.add_layer(&mut commands, VEHICLE_LAYER_ID, layer_entity);

    let pos = UVec2::new(1, 10);
    let _ = layer_builder.set_tile(
        pos,
        Tile {
            texture_index: VehicleTile::BlueVertical as u16,
            ..Default::default()
        }
        .into(),
    );

    commands
        .spawn()
        .insert(Car {
            position: pos,
            direction: Direction::North,
            instructions: vec![
                CarInstructions::GoTo(UVec2::new(25, 10)),
                CarInstructions::WaitForLoad(Resource::IronOre),
                CarInstructions::GoTo(UVec2::new(15, 10)),
                CarInstructions::WaitForUnload(Resource::IronOre),
            ],
            current_instruction: 0,
        })
        .insert(Storage {
            resource: Resource::IronOre,
            amount: 0,
            capacity: 4,
        });

    let pos = UVec2::new(0, 10);
    let _ = layer_builder.set_tile(
        pos,
        Tile {
            texture_index: VehicleTile::BlueVertical as u16,
            ..Default::default()
        }
        .into(),
    );

    commands
        .spawn()
        .insert(Car {
            position: pos,
            direction: crate::game::assets::Direction::North,
            instructions: vec![
                CarInstructions::GoTo(UVec2::new(14, 7)),
                CarInstructions::WaitForLoad(Resource::Steel),
                CarInstructions::GoTo(UVec2::new(10, 1)),
                CarInstructions::WaitForUnload(Resource::Steel),
            ],
            current_instruction: 0,
        })
        .insert(Storage {
            resource: Resource::Steel,
            amount: 0,
            capacity: 4,
        });

    map_query.build_layer(&mut commands, layer_builder, material_handle);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(GlobalTransform::default());
}
