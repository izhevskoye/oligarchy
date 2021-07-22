use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::constants::{
    MapTile, CHUNK_SIZE, MAP_HEIGHT, MAP_WIDTH, TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_SIZE,
};

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

    // buildings

    let (layer_builder, layer_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, map_settings, MAP_ID, BUILDING_LAYER_ID);
    map.add_layer(&mut commands, BUILDING_LAYER_ID, layer_entity);

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

    let (layer_builder, layer_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, map_settings, MAP_ID, VEHICLE_LAYER_ID);
    map.add_layer(&mut commands, VEHICLE_LAYER_ID, layer_entity);

    map_query.build_layer(&mut commands, layer_builder, material_handle);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(GlobalTransform::default());
}
