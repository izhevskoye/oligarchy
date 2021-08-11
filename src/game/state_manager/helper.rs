use directories_next::ProjectDirs;
use std::fs::create_dir_all;

const QUALIFIER: &str = "game";
const ORGANIZATION: &str = "xqyz";
const APPLICATION: &str = "oligarchy";

pub fn generate_save_game_path() -> String {
    let proj_dirs = ProjectDirs::from(&QUALIFIER, &ORGANIZATION, &APPLICATION)
        .expect("Could not find project directories");

    let save_game_path = proj_dirs.config_dir().join("saves");

    log::info!(
        "initialize save game path as '{}'",
        save_game_path.display()
    );

    create_dir_all(&save_game_path).expect("Could not create save game path.");

    save_game_path.to_str().unwrap().to_string()
}
