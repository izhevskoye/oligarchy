mod account;
mod asset_tiles;
mod assets;
mod camera;
mod car;
mod constants;
mod construction;
mod current_selection;
mod current_tool;
mod goals;
mod helper;
mod highlight_tiles;
mod pathfinder;
mod production;
mod remove_update;
mod setup;
mod state_manager;
mod statistics;
mod storage;
mod street;
mod texture;
mod time;
mod ui;

use bevy::{
    core::FixedTimestep, diagnostic::FrameTimeDiagnosticsPlugin, ecs::schedule::ShouldRun,
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;

use self::{
    account::{Account, AccountTransaction},
    assets::{ClickedTile, Forest, MapSettings, RemovedBuildingEvent, StateName, Water},
    car::instructions::{
        CarGoToInstructionEvent, CarLoadInstructionEvent, CarUnloadInstructionEvent,
    },
    constants::{
        CAR_DRIVE_TICK_SPEED, CAR_INSTRUCTION_TICK_SPEED, GOAL_UPDATE_TICK_SPEED,
        PRODUCTION_TICK_SPEED,
    },
    current_selection::CurrentlySelected,
    current_tool::SelectedTool,
    goals::GoalManager,
    highlight_tiles::{HighlightTiles, HighlightTilesUpdateEvent},
    pathfinder::Pathfinding,
    state_manager::{LoadGameEvent, NewGameEvent, SaveGameEvent},
    statistics::StatisticTracker,
    street::Street,
    time::PlayTime,
    ui::state::{ConfirmDialogState, MainMenuState, SaveGameList},
};

#[derive(Default, Debug)]
pub struct NewGameSetup {
    ground_tiles: bool,
    street: bool,
}

pub struct GenerateStreetEvent;
pub struct GenerateGroundTilesEvent;

#[derive(Default)]
pub struct Game {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    NewGameMenu,
    ProcessLoad,
    ProcessSave,
    Update,
    UpdateEnd,
    CurrentSelection,
    HighlightTiles,
    Pathfinding,
    NewGameHandling,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum IdleLabel {
    SpawnIdle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum UILabel {
    InfoUI,
    UIEnd,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum CarLabel {
    Instruction,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
}

fn and_is_in_game(In(input): In<ShouldRun>, state: Res<State<AppState>>) -> ShouldRun {
    if state.current() == &AppState::InGame {
        input
    } else {
        ShouldRun::No
    }
}

impl Game {
    pub fn run(&self) {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();

        App::build()
            .init_resource::<CurrentlySelected>()
            .init_resource::<SelectedTool>()
            .init_resource::<ClickedTile>()
            .init_resource::<MapSettings>()
            .init_resource::<GoalManager>()
            .init_resource::<Account>()
            .init_resource::<StateName>()
            .init_resource::<PlayTime>()
            .init_resource::<StatisticTracker>()
            .init_resource::<ConfirmDialogState>()
            .init_resource::<SaveGameList>()
            .init_resource::<HighlightTiles>()
            .init_resource::<Pathfinding>()
            .init_resource::<Option<NewGameSetup>>()
            .insert_resource(assets::building_specifications::load_specifications())
            .insert_resource(assets::resource_specifications::load_specifications())
            .insert_resource(WindowDescriptor {
                title: "Oligarchy".to_owned(),
                ..Default::default()
            })
            .add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(TilemapPlugin)
            .add_state(AppState::MainMenu)
            .add_state(MainMenuState::Main)
            .add_event::<NewGameEvent>()
            .add_event::<LoadGameEvent>()
            .add_event::<SaveGameEvent>()
            .add_event::<RemovedBuildingEvent>()
            .add_event::<AccountTransaction>()
            .add_event::<CarLoadInstructionEvent>()
            .add_event::<CarUnloadInstructionEvent>()
            .add_event::<CarGoToInstructionEvent>()
            .add_event::<HighlightTilesUpdateEvent>()
            .add_event::<GenerateStreetEvent>()
            .add_event::<GenerateGroundTilesEvent>()
            .add_startup_system(assets::integrity::integrity_check.system())
            .add_startup_system(setup::setup.system())
            //
            // MENU
            //
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu).with_system(setup::title::setup.system()),
            )
            .add_system_set(
                SystemSet::on_resume(AppState::MainMenu).with_system(setup::title::setup.system()),
            )
            .add_system_set(
                SystemSet::on_pause(AppState::MainMenu)
                    .with_system(setup::title::teardown.system()),
            )
            .add_system_set(
                SystemSet::on_update(MainMenuState::ConfirmDialog)
                    .with_system(ui::state::confirm_dialog.system())
                    .before(Label::ProcessSave),
            )
            .add_system_set(
                SystemSet::on_update(MainMenuState::Main)
                    .with_system(ui::state::main_menu.system()),
            )
            .add_system_set(
                SystemSet::on_update(MainMenuState::New)
                    .with_system(ui::state::new_game_menu.system())
                    .label(Label::NewGameMenu),
            )
            .add_system_set(
                SystemSet::on_update(MainMenuState::Load)
                    .with_system(ui::state::load_save_game_menu.system())
                    .before(Label::ProcessLoad),
            )
            .add_system_set(
                SystemSet::on_update(MainMenuState::Save)
                    .with_system(ui::state::load_save_game_menu.system())
                    .before(Label::ProcessSave),
            )
            //
            // GAME
            //
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(setup::game::setup.system())
                    .with_system(account::reset_account.system())
                    .with_system(goals::generate_goals.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(GOAL_UPDATE_TICK_SPEED as f64)
                            .chain(and_is_in_game.system()),
                    )
                    .with_system(goals::update_goals.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame).with_system(setup::game::teardown.system()),
            )
            // save / load
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(
                        state_manager::load_game::load_game
                            .system()
                            .label(Label::ProcessLoad)
                            .before(Label::Update),
                    )
                    .with_system(
                        state_manager::save_game::save_game
                            .system()
                            .label(Label::ProcessSave),
                    ),
            )
            // bevy has a dumb bug or else we could make the above on_in_stack_update
            // https://github.com/bevyengine/bevy/pull/2211
            .add_system_set(
                SystemSet::on_update(AppState::Paused).with_system(
                    state_manager::save_game::save_game
                        .system()
                        .label(Label::ProcessSave),
                ),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        camera::movement_allowed
                            .system()
                            .chain(and_is_in_game.system()),
                    )
                    .with_system(camera::movement.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(texture::set_texture_filters_to_nearest.system())
                    .with_system(
                        current_selection::spawn_selected
                            .system()
                            .after(Label::CurrentSelection),
                    )
                    .with_system(
                        highlight_tiles::update_highlight
                            .system()
                            .label(Label::HighlightTiles),
                    )
                    .with_system(
                        current_selection::current_selection
                            .system()
                            .after(UILabel::UIEnd)
                            .label(Label::CurrentSelection),
                    )
                    .with_system(setup::new_game_setup.system().label(Label::NewGameHandling)),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .after(Label::NewGameHandling)
                    .before(Label::Update)
                    .with_system(setup::ground_tiles::generate_tiles.system())
                    .with_system(
                        setup::street::generate_street
                            .system()
                            .after(Label::Pathfinding),
                    ),
            )
            // UI Systems
            .add_system_set(
                SystemSet::on_update(AppState::Paused)
                    .before(UILabel::UIEnd)
                    .with_system(ui::pause::pause_menu.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .before(UILabel::UIEnd)
                    .with_system(ui::pause::pause_menu.system())
                    .with_system(ui::info::info_ui.system().label(UILabel::InfoUI))
                    .with_system(ui::goals::goals_ui.system().after(UILabel::InfoUI))
                    .with_system(
                        ui::import_export_station::edit_ui
                            .system()
                            .after(UILabel::InfoUI),
                    )
                    .with_system(
                        ui::depot::edit_ui
                            .system()
                            .after(UILabel::InfoUI)
                            .before(Label::HighlightTiles),
                    )
                    .with_system(
                        ui::statistics::statistics_ui
                            .system()
                            .after(UILabel::InfoUI),
                    )
                    .with_system(
                        ui::production_building::edit_ui
                            .system()
                            .after(UILabel::InfoUI),
                    )
                    .with_system(
                        ui::car_instructions::program_ui
                            .system()
                            .after(UILabel::InfoUI)
                            .before(Label::HighlightTiles),
                    )
                    .with_system(ui::construction::construction_ui.system())
                    .with_system(ui::name::name_ui.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label(UILabel::UIEnd)
                    .with_system(ui::mouse_pos_to_tile::mouse_pos_to_tile.system()),
            )
            // Current Tool
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .after(Label::CurrentSelection)
                    .before(Label::Update)
                    .with_system(current_tool::street::street_placement.system())
                    .with_system(current_tool::street::path_placement.system())
                    .with_system(current_tool::depot::depot_placement.system())
                    .with_system(current_tool::storage::storage_placement.system())
                    .with_system(
                        current_tool::import_export_station::import_export_station_placement
                            .system(),
                    )
                    .with_system(
                        current_tool::storage_management::storage_management_placement.system(),
                    )
                    .with_system(
                        current_tool::delivery_station::delivery_station_placement.system(),
                    )
                    .with_system(current_tool::car::car_placement.system())
                    .with_system(current_tool::building::building_placement.system())
                    .with_system(current_tool::bulldoze::bulldoze.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(
                        pathfinder::update
                            .system()
                            .before(Label::UpdateEnd)
                            .after(Label::CurrentSelection)
                            .label(Label::Pathfinding),
                    )
                    .with_system(
                        car::calculate_destination
                            .system()
                            .after(Label::Pathfinding)
                            .before(Label::UpdateEnd),
                    ),
            )
            .add_system_set(
                SystemSet::new()
                    .before(Label::Update)
                    .with_run_criteria(
                        FixedTimestep::step(PRODUCTION_TICK_SPEED as f64)
                            .chain(and_is_in_game.system()),
                    )
                    .with_system(production::import_export_station::import_export_station.system())
                    .with_system(production::storage_management::storage_management.system())
                    .with_system(
                        production::production_building::production_building
                            .system()
                            .after(IdleLabel::SpawnIdle),
                    )
                    .with_system(
                        production::idle::spawn_idle
                            .system()
                            .label(IdleLabel::SpawnIdle),
                    )
                    .with_system(account::maintenance_cost.system())
                    .with_system(construction::construction.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .before(Label::Update)
                    .with_run_criteria(
                        FixedTimestep::step(CAR_DRIVE_TICK_SPEED as f64)
                            .chain(and_is_in_game.system()),
                    )
                    .with_system(car::drive_to_destination::drive_to_destination.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .before(Label::Update)
                    .with_run_criteria(
                        FixedTimestep::step(CAR_INSTRUCTION_TICK_SPEED as f64)
                            .chain(and_is_in_game.system()),
                    )
                    .with_system(
                        car::instructions::car_instruction
                            .system()
                            .label(CarLabel::Instruction),
                    )
                    .with_system(
                        car::instructions::load
                            .system()
                            .after(CarLabel::Instruction),
                    )
                    .with_system(
                        car::instructions::unload
                            .system()
                            .after(CarLabel::Instruction),
                    )
                    .with_system(
                        car::instructions::goto
                            .system()
                            .after(CarLabel::Instruction),
                    ),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .before(Label::UpdateEnd)
                    .with_system(storage::update_consolidators.system())
                    .with_system(car::update_car.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label(Label::Update)
                    .before(Label::UpdateEnd)
                    .with_system(asset_tiles::construction_update.system())
                    .with_system(asset_tiles::building_update.system())
                    .with_system(asset_tiles::depot_update.system())
                    .with_system(asset_tiles::storage_update.system())
                    .with_system(asset_tiles::import_export_station_update.system())
                    .with_system(asset_tiles::delivery_station_update.system())
                    .with_system(asset_tiles::storage_management_update.system())
                    .with_system(asset_tiles::ground_update.system())
                    .with_system(street::update_streets.system())
                    .with_system(helper::neighbor_structure::update_tile::<Water>.system())
                    .with_system(helper::neighbor_structure::update_tile::<Forest>.system())
                    .with_system(helper::neighbor_structure::update_tile::<Street>.system())
                    .with_system(car::spawn_car.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(remove_update::remove_update.system())
                    .with_system(account::account_transactions.system())
                    .label(Label::UpdateEnd),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0).chain(and_is_in_game.system()))
                    .with_system(time::track_time.system()),
            )
            .run();
    }
}
