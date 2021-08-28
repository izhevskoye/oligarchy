mod emit_load_game;
mod get_state_name;
mod save_game_list;

mod confirm_dialog;
mod load_save_game_menu;
mod main_menu;
mod new_game_menu;

pub use confirm_dialog::confirm_dialog;
pub use load_save_game_menu::load_save_game_menu;
pub use main_menu::main_menu;
pub use new_game_menu::new_game_menu;
pub use save_game_list::SaveGameList;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ConfirmDialogState {
    DeleteFile(String),
    SaveFile(String),
    ExitGame,
    ExitProgram,
}

impl Default for ConfirmDialogState {
    fn default() -> Self {
        Self::ExitProgram
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MainMenuState {
    None,
    Main,
    New,
    Load,
    Save,
    ConfirmDialog,
}
