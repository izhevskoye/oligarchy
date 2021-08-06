use glob::glob;
use std::{fs::File, io::prelude::*, path::Path};

use super::{GoalManager, GoalSet};

impl GoalManager {
    pub fn load_file(&mut self, file_name: &str) {
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

        let state: Result<Vec<GoalSet>, serde_yaml::Error> = serde_yaml::from_str(&content);

        match state {
            Ok(mut state) => {
                for goal_set in state.iter() {
                    log::info!("load goal spec {}", goal_set.name);
                }
                self.goal_sets.append(&mut state);
            }
            Err(why) => log::error!("Could not load state: {}", why),
        }
    }

    pub fn load_specifications(&mut self) {
        self.goal_sets = vec![];
        for file in glob("assets/goals/**/*.yml").expect("Failed to read files") {
            self.load_file(&format!("{}", file.unwrap().display()));
        }
    }
}
