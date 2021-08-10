mod account;
mod asset_tiles;
mod assets;
mod camera;
mod car;
mod constants;
mod current_selection;
mod current_tool;
mod goals;
mod production;
mod remove_update;
mod setup;
mod state_manager;
mod statistics;
mod storage;
mod street;
mod texture;
mod ui;

use bevy::{core::FixedTimestep, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;

use self::{
    account::{Account, AccountTransaction},
    assets::{ClickedTile, MapSettings, RemovedBuildingEvent, SelectedTool},
    constants::{
        CAR_DRIVE_TICK_SPEED, CAR_INSTRUCTION_TICK_SPEED, GOAL_UPDATE_TICK_SPEED,
        PRODUCTION_TICK_SPEED,
    },
    current_selection::CurrentlySelected,
    goals::GoalManager,
    state_manager::{LoadGameEvent, SaveGameEvent},
};

#[derive(Default)]
pub struct Game {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    Menu,
    Update,
    UpdateEnd,
    CurrentSelection,
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
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
            .insert_resource(assets::building_specifications::load_specifications())
            .insert_resource(assets::resource_specifications::load_specifications())
            .add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(TilemapPlugin)
            .add_state(AppState::MainMenu)
            .add_event::<LoadGameEvent>()
            .add_event::<SaveGameEvent>()
            .add_event::<RemovedBuildingEvent>()
            .add_event::<AccountTransaction>()
            .add_startup_system(assets::integrity::integrity_check.system())
            //
            // MENU
            //
            .add_system_set(
                SystemSet::new()
                    .with_system(ui::state::save_ui.system())
                    .label(Label::Menu),
            )
            //
            // GAME
            //
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(setup::setup_map.system())
                    .with_system(account::reset_account.system())
                    .with_system(goals::generate_goals.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(GOAL_UPDATE_TICK_SPEED))
                    .with_system(goals::update_goals.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame).with_system(setup::teardown.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(camera::movement.system())
                    .with_system(texture::set_texture_filters_to_nearest.system())
                    .with_system(
                        current_selection::spawn_selected
                            .system()
                            .after(Label::CurrentSelection),
                    )
                    .with_system(
                        current_selection::current_selection
                            .system()
                            .after(UILabel::UIEnd)
                            .label(Label::CurrentSelection),
                    )
                    .with_system(
                        state_manager::load_game::load_game
                            .system()
                            .after(Label::Menu),
                    )
                    .with_system(
                        state_manager::save_game::save_game
                            .system()
                            .after(Label::Menu),
                    ),
            )
            // UI Systems
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .before(UILabel::UIEnd)
                    .with_system(ui::account::account_ui.system().after(UILabel::InfoUI))
                    .with_system(ui::info::info_ui.system().label(UILabel::InfoUI))
                    .with_system(ui::goals::goals_ui.system().after(UILabel::InfoUI))
                    .with_system(ui::export_station::edit_ui.system().after(UILabel::InfoUI))
                    .with_system(
                        ui::production_building::edit_ui
                            .system()
                            .after(UILabel::InfoUI),
                    )
                    .with_system(
                        ui::car_instructions::program_ui
                            .system()
                            .after(UILabel::InfoUI),
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
                    .with_system(current_tool::storage::storage_placement.system())
                    .with_system(current_tool::export_station::export_station_placement.system())
                    .with_system(
                        current_tool::delivery_station::delivery_station_placement.system(),
                    )
                    .with_system(current_tool::car::car_placement.system())
                    .with_system(current_tool::building::building_placement.system())
                    .with_system(current_tool::bulldoze::bulldoze.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(
                    car::calculate_destination
                        .system()
                        .before(Label::UpdateEnd)
                        .after(Label::CurrentSelection),
                ),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .before(Label::Update)
                    .with_run_criteria(FixedTimestep::step(PRODUCTION_TICK_SPEED))
                    .with_system(production::export_station::export_station.system())
                    .with_system(
                        production::production_building::production_building
                            .system()
                            .after(IdleLabel::SpawnIdle),
                    )
                    .with_system(production::spawn_idle.system().label(IdleLabel::SpawnIdle))
                    .with_system(account::maintenance_cost.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .before(Label::Update)
                    .with_run_criteria(FixedTimestep::step(CAR_DRIVE_TICK_SPEED))
                    .with_system(car::drive_to_destination::drive_to_destination.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .before(Label::Update)
                    .with_run_criteria(FixedTimestep::step(CAR_INSTRUCTION_TICK_SPEED))
                    .with_system(car::instructions::car_instruction.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label(Label::Update)
                    .before(Label::UpdateEnd)
                    .with_system(asset_tiles::building_update.system())
                    .with_system(asset_tiles::storage_update.system())
                    .with_system(asset_tiles::export_station_update.system())
                    .with_system(asset_tiles::delivery_station_update.system())
                    .with_system(asset_tiles::ground_update.system())
                    .with_system(street::update_streets.system())
                    .with_system(storage::update_consolidators.system())
                    .with_system(car::spawn_car.system())
                    .with_system(car::update_car.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(remove_update::remove_update.system())
                    .with_system(account::account_transactions.system())
                    .label(Label::UpdateEnd),
            )
            .run();
    }
}
