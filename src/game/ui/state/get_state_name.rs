use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*, path::Path};

use crate::game::assets::StateName;

#[derive(Default, Serialize, Deserialize)]
pub struct SaveGameState {
    pub state_name: StateName,
}

pub fn get_state_name(file_name: &str) -> Option<String> {
    let path = Path::new(file_name);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            log::error!("Could not read file: {}", why);
            return None;
        }
    };

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let state: Result<SaveGameState, serde_yaml::Error> = serde_yaml::from_str(&content);

    match state {
        Ok(state) => Some(state.state_name.name),
        Err(why) => {
            log::error!("Could not load state: {}", why);
            None
        }
    }
}
