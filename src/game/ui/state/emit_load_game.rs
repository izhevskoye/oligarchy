use bevy::prelude::*;
use std::{fs::File, io::prelude::*, path::Path};

use crate::game::state_manager::{GameState, LoadGameEvent};

pub fn emit_load_game(
    commands: &mut Commands,
    load_game: &mut EventWriter<LoadGameEvent>,
    file_name: &str,
) {
    let path = Path::new(file_name);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            log::error!("Could not read file: {}", why);
            return;
        }
    };

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let state: Result<GameState, serde_yaml::Error> = serde_yaml::from_str(&content);

    match state {
        Ok(state) => {
            commands.insert_resource(state.settings.clone());
            load_game.send(LoadGameEvent { state })
        }
        Err(why) => log::error!("Could not load state: {}", why),
    }
}
