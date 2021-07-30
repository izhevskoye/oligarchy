use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::constants::{
    MapTile, CHUNK_SIZE, MAP_HEIGHT, MAP_WIDTH, TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_SIZE,
};

pub const MAP_ID: u16 = 0;
pub const GROUND_LAYER_ID: u16 = 0;
pub const BUILDING_LAYER_ID: u16 = 1;

pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.far = 1000.0 / 0.1;

    let mut map_settings = LayerSettings::new(
        UVec2::new(MAP_WIDTH, MAP_HEIGHT),
        UVec2::new(CHUNK_SIZE, CHUNK_SIZE),
        Vec2::new(TILE_SIZE, TILE_SIZE),
        Vec2::new(TILE_SIZE * TILE_MAP_WIDTH, TILE_SIZE * TILE_MAP_HEIGHT),
    );
    map_settings.mesh_type = TilemapMeshType::Square;

    let center = map_settings.get_pixel_center();
    camera.transform.translation += Vec3::new(center.x, center.y, 0.0);
    commands.spawn_bundle(camera);

    let texture_handle = asset_server.load("oligarchy_tiles.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(MAP_ID, map_entity);

    let (mut layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        map_settings.clone(),
        MAP_ID,
        GROUND_LAYER_ID,
    );
    map.add_layer(&mut commands, GROUND_LAYER_ID, layer_entity);

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

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(GlobalTransform::default());
}
