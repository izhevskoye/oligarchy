use crate::game::{
    assets::Position,
    constants::{TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_SIZE, Z_IDLE_INDICATOR},
    production::Idle,
};
use bevy::prelude::*;

pub fn spawn_idle(
    mut commands: Commands,
    mut idle_query: Query<(&mut Idle, &Position)>,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (mut idle, position) in idle_query.iter_mut() {
        if idle.entity.is_some() {
            continue;
        }

        let texture_handle = assets.load("oligarchy_tiles.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::splat(TILE_SIZE),
            TILE_MAP_WIDTH as usize,
            TILE_MAP_HEIGHT as usize,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let sprite = TextureAtlasSprite::new(63);
        let mut transform = Transform::default();
        let position = Vec2::new(
            position.position.x as f32 + 0.5,
            position.position.y as f32 + 0.5,
        );
        let translation = (position * TILE_SIZE).extend(Z_IDLE_INDICATOR);
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

        idle.entity = Some(entity);
    }
}
