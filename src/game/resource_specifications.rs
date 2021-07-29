use glob::glob;
use std::{collections::HashMap, fs::File, io::prelude::*, path::Path};

use crate::game::assets::ResourceSpecification;

pub type ResourceSpecifications = HashMap<String, ResourceSpecification>;

pub fn load_file(resources: &mut ResourceSpecifications, file_name: &str) {
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

    let state: Result<ResourceSpecifications, serde_yaml::Error> = serde_yaml::from_str(&content);

    match state {
        Ok(state) => {
            for (id, resource) in state.into_iter() {
                log::info!("load resource spec {}", id);
                resources.insert(id, resource);
            }
        }
        Err(why) => log::error!("Could not load state: {}", why),
    }
}

pub fn load_specifications() -> ResourceSpecifications {
    let mut resources = HashMap::new();
    for file in glob("assets/resources/**/*.yml").expect("Failed to read files") {
        load_file(&mut resources, &format!("{}", file.unwrap().display()));
    }
    resources
}
