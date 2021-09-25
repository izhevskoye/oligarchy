use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct PlayTime {
    pub seconds: i64,
}

pub fn track_time(mut time: ResMut<PlayTime>) {
    time.seconds += 1;
}
