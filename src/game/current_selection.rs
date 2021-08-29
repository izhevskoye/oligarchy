use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    assets::{ClickedTile, Position},
    car::Car,
    constants::{
        MapTile, VehicleTile, TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_SIZE, Z_SELECTION_INDICATOR,
    },
    setup::{BUILDING_LAYER_ID, MAP_ID},
    street::Street,
};

#[derive(Default)]
pub struct CurrentlySelected {
    pub entity: Option<Entity>,
    pub locked: bool,
    pub editing: bool,
    pub renaming: bool,
    pub statistics: bool,
}

pub fn current_selection(
    clicked_tile: Res<ClickedTile>,
    map_query: MapQuery,
    car_query: Query<(Entity, &Position), With<Car>>,
    mut currently_selected: ResMut<CurrentlySelected>,
) {
    if currently_selected.locked || currently_selected.editing {
        return;
    }

    if clicked_tile.occupied_vehicle {
        if let Some(pos) = clicked_tile.vehicle_pos {
            for (entity, position) in car_query.iter() {
                if position.position == pos {
                    currently_selected.entity = Some(entity);
                    return;
                }
            }
        }
    }

    if clicked_tile.occupied_building {
        if let Some(pos) = clicked_tile.pos {
            if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, BUILDING_LAYER_ID) {
                currently_selected.entity = Some(entity);
                return;
            }
        }
    }

    if !clicked_tile.dragging {
        currently_selected.entity = None;
    }
}

pub fn spawn_selected(
    mut commands: Commands,
    position_query: Query<(&Position, Option<&Car>, Option<&Street>)>,
    assets: Res<AssetServer>,
    // TODO: add more and more??
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    currently_selected: Res<CurrentlySelected>,
    mut selection_marker: Local<Option<(Entity, UVec2)>>,
) {
    let mut despawn = true;

    if let Some((cache_entity, cache_position)) = selection_marker.as_ref() {
        if let Some(entity) = currently_selected.entity {
            if let Ok((position, _car, street)) = position_query.get(entity) {
                if position.position == *cache_position
                    && entity == *cache_entity
                    && street.is_none()
                {
                    despawn = false;
                }
            }
        }
    } else {
        despawn = false;
    }

    if despawn {
        if let Some((entity, _)) = selection_marker.as_ref() {
            commands.entity(*entity).despawn_recursive();
            *selection_marker = None;
        }
    }

    if let Some(entity) = currently_selected.entity {
        if let Ok((position, car, street)) = position_query.get(entity) {
            if street.is_some() {
                return;
            }

            let texture_handle = assets.load("oligarchy_tiles.png");
            let tile_size = if car.is_none() {
                TILE_SIZE
            } else {
                TILE_SIZE / 2.0
            };
            let texture_atlas = if car.is_none() {
                TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::splat(tile_size),
                    TILE_MAP_WIDTH as usize,
                    TILE_MAP_HEIGHT as usize,
                )
            } else {
                TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::splat(tile_size),
                    TILE_MAP_WIDTH as usize * 2,
                    TILE_MAP_HEIGHT as usize * 2,
                )
            };
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let sprite = if car.is_none() {
                TextureAtlasSprite::new(MapTile::Selection as u32)
            } else {
                TextureAtlasSprite::new(VehicleTile::Selection as u32)
            };
            let mut transform = Transform::default();
            let tile_position = Vec2::new(
                position.position.x as f32 + 0.5,
                position.position.y as f32 + 0.5,
            );
            let translation = (tile_position * tile_size).extend(Z_SELECTION_INDICATOR);
            transform.translation = translation;

            let entity = commands
                .spawn()
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite,
                    transform,
                    ..Default::default()
                })
                .id();

            *selection_marker = Some((entity, position.position));
        }
    }
}
