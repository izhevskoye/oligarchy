use crate::game::{
    state_manager::helper::generate_save_game_path, ui::state::get_state_name::get_state_name,
};
use glob::glob;

pub struct SaveGameList {
    pub save_game_path: String,
    pub files: Vec<(String, String)>,
}

impl Default for SaveGameList {
    fn default() -> Self {
        let save_game_path = generate_save_game_path();

        Self {
            files: load_file_list(&save_game_path),
            save_game_path,
        }
    }
}

impl SaveGameList {
    pub fn update_list(&mut self) {
        self.files = load_file_list(&self.save_game_path);
    }
}

fn load_file_list(save_game_path: &str) -> Vec<(String, String)> {
    let mut list = vec![];

    for file in glob(&format!("{}/*.yml", save_game_path)).expect("Failed to read files") {
        let file = file
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let file_name = format!("{}/{}", save_game_path, file);

        if let Some(name) = get_state_name(&file_name) {
            list.push((name, file_name));
        }
    }
    list.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    list
}
