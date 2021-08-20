use bevy::prelude::*;

use crate::game::{assets::MapSettings, constants::CHUNK_SIZE};

pub struct EvalNeighbor<'a, T: 'static>
where
    T: Sync + Send + 'a,
{
    pub map_settings: &'a MapSettings,
    pub query: &'a Query<'a, (), With<T>>,
}

impl<'a, T> EvalNeighbor<'a, T>
where
    T: Sync + Send + 'a,
{
    pub fn eval_neighbor(&self, neighbor: (IVec2, Option<Entity>)) -> bool {
        let (pos, entity) = neighbor;

        if let Some(entity) = entity {
            if self.query.get(entity).is_ok() {
                return true;
            }
        } else if pos.x == -1
            || pos.y == -1
            || pos.x == (self.map_settings.width * CHUNK_SIZE - 1) as i32
            || pos.y == (self.map_settings.height * CHUNK_SIZE - 1) as i32
        {
            return true;
        }

        false
    }
}
