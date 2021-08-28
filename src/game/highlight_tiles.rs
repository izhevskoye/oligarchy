use bevy::prelude::*;

use crate::game::constants::MapTile;

use super::constants::{TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_SIZE, Z_SELECTION_INDICATOR};

#[derive(Default, Debug)]
pub struct HighlightTiles {
    tiles: Vec<(UVec2, Entity)>,
}

impl HighlightTiles {
    fn needs_update(&self, event: &HighlightTilesUpdateEvent) -> bool {
        let current: Vec<UVec2> = self.tiles.clone().into_iter().map(|(pos, _)| pos).collect();

        for pos in event.tiles.iter() {
            if !current.contains(&pos) {
                return true;
            }
        }

        for pos in current.iter() {
            if !event.tiles.contains(&pos) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
pub struct HighlightTilesUpdateEvent {
    pub tiles: Vec<UVec2>,
}

impl HighlightTilesUpdateEvent {
    pub fn from_position(position: UVec2) -> Self {
        Self {
            tiles: vec![position],
        }
    }

    pub fn from_positions(positions: Vec<UVec2>) -> Self {
        Self { tiles: positions }
    }
}

pub fn update_highlight(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut events: EventReader<HighlightTilesUpdateEvent>,
    mut tiles: ResMut<HighlightTiles>,
) {
    let mut updated = false;
    for event in events.iter() {
        updated = true;

        if !tiles.needs_update(&event) {
            continue;
        }

        for (_, entity) in tiles.tiles.iter() {
            commands.entity(*entity).despawn_recursive();
        }

        tiles.tiles = vec![];

        for position in event.tiles.iter() {
            let texture_handle = assets.load("oligarchy_tiles.png");
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                Vec2::splat(TILE_SIZE),
                TILE_MAP_WIDTH as usize,
                TILE_MAP_HEIGHT as usize,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let sprite = TextureAtlasSprite::new(MapTile::HoverIndicator as u32);

            let mut transform = Transform::default();
            let tile_position = Vec2::new(position.x as f32 + 0.5, position.y as f32 + 0.5);
            let translation = (tile_position * TILE_SIZE).extend(Z_SELECTION_INDICATOR);
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

            tiles.tiles.push((*position, entity));
        }
    }

    if !updated {
        for (_, entity) in tiles.tiles.iter() {
            commands.entity(*entity).despawn_recursive();
        }

        tiles.tiles = vec![];
    }
}
