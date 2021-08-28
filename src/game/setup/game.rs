use bevy::{prelude::*, render::camera::Camera};
use bevy_ecs_tilemap::prelude::*;

use crate::game::{
    assets::{ClickedTile, MapSettings},
    car::Car,
    constants::{MapTile, CHUNK_SIZE, TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_SIZE},
    current_tool::SelectedTool,
    ui::state::MainMenuState,
};

use super::{BUILDING_LAYER_ID, GROUND_LAYER_ID, MAP_ID};

pub fn teardown(
    mut commands: Commands,
    mut map_query: MapQuery,
    car_query: Query<Entity, With<Car>>,
    sprite_query: Query<Entity, With<TextureAtlasSprite>>,
    mut menu_state: ResMut<State<MainMenuState>>,
) {
    map_query.depsawn_map(&mut commands, MAP_ID);

    for entity in sprite_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in car_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let _ = menu_state.set(MainMenuState::Main);
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
    map_settings: Res<MapSettings>,
    camera_query: Query<Entity, With<Camera>>,
    mut menu_state: ResMut<State<MainMenuState>>,
) {
    let _ = menu_state.set(MainMenuState::None);

    commands.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)));

    // reset tools
    commands.insert_resource(SelectedTool::default());
    commands.insert_resource(ClickedTile::default());

    for entity in camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.far = 1000.0 / 0.1;

    let mut layer_settings = LayerSettings::new(
        UVec2::new(map_settings.width, map_settings.height),
        UVec2::new(CHUNK_SIZE, CHUNK_SIZE),
        Vec2::new(TILE_SIZE, TILE_SIZE),
        Vec2::new(TILE_SIZE * TILE_MAP_WIDTH, TILE_SIZE * TILE_MAP_HEIGHT),
    );
    layer_settings.mesh_type = TilemapMeshType::Square;

    let center = layer_settings.get_pixel_center();
    camera.transform.translation += Vec3::new(center.x, center.y, 0.0);
    commands.spawn_bundle(camera);

    let texture_handle = asset_server.load("oligarchy_tiles.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(MAP_ID, map_entity);

    let (mut layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        layer_settings.clone(),
        MAP_ID,
        GROUND_LAYER_ID,
    );
    map.add_layer(&mut commands, GROUND_LAYER_ID, layer_entity);

    layer_builder.fill(
        UVec2::new(0, 0),
        UVec2::new(
            CHUNK_SIZE * map_settings.width - 1,
            CHUNK_SIZE * map_settings.height - 1,
        ),
        Tile {
            texture_index: MapTile::Ground as u16,
            ..Default::default()
        }
        .into(),
    );

    map_query.build_layer(&mut commands, layer_builder, material_handle.clone());

    // buildings

    let (layer_builder, layer_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, MAP_ID, BUILDING_LAYER_ID);
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
