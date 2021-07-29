use glob::glob;
use std::{collections::HashMap, fs::File, io::prelude::*, path::Path};

use crate::game::assets::BuildingSpecification;

pub type BuildingSpecifications = HashMap<String, BuildingSpecification>;

pub fn load_file(buildings: &mut BuildingSpecifications, file_name: &str) {
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

    let state: Result<BuildingSpecifications, serde_yaml::Error> = serde_yaml::from_str(&content);

    match state {
        Ok(state) => {
            for (id, building) in state.into_iter() {
                log::info!("load building spec {}", id);
                buildings.insert(id, building);
            }
        }
        Err(why) => log::error!("Could not load state: {}", why),
    }
}

pub fn load_specifications() -> BuildingSpecifications {
    let mut buildings = HashMap::new();
    for file in glob("assets/buildings/**/*.yml").expect("Failed to read files") {
        load_file(&mut buildings, &format!("{}", file.unwrap().display()));
    }
    buildings
}
