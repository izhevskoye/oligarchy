mod assets;
mod blast_furnace;
mod camera;
mod car;
mod coke_furnace;
mod constants;
mod current_selection;
mod current_tool;
mod export_station;
mod oxygen_converter;
mod quarry;
mod setup;
mod storage;
mod street;
mod texture;

use bevy::{core::FixedTimestep, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use self::assets::{CurrentlySelected, SelectedTool, Storage, Tool};

#[derive(Default)]
pub struct Game {}

impl Game {
    pub fn run(&self) {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();

        App::build()
            .insert_resource(CurrentlySelected { entity: None })
            .insert_resource(SelectedTool { tool: Tool::None })
            .add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(TilemapPlugin)
            .add_startup_system(setup::setup_map.system())
            .add_system(camera::movement.system())
            .add_system(texture::set_texture_filters_to_nearest.system())
            .add_system(storage::update_consolidators.system())
            .add_system(street::update_streets.system())
            .add_system(car::calculate_destination.system())
            .add_system(current_selection::current_selection.system())
            .add_system(current_tool::current_tool.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(export_station::export_station.system())
                    .with_system(oxygen_converter::oxygen_converter.system())
                    .with_system(blast_furnace::blast_furnace.system())
                    .with_system(quarry::quarry.system())
                    .with_system(coke_furnace::coke_furnace.system())
                    .with_system(car::car_instruction.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.2))
                    .with_system(car::drive_to_destination.system()),
            )
            .add_system(ui_example.system())
            .run();
    }
}

fn ui_example(
    egui_context: ResMut<EguiContext>,
    storage_query: Query<&Storage>,
    currently_selected: Res<CurrentlySelected>,
    mut selected_tool: ResMut<SelectedTool>,
) {
    egui::Window::new("Hello").show(egui_context.ctx(), |ui| {
        ui.label("world");
        if ui.small_button("None").clicked() {
            selected_tool.tool = Tool::None;
        }
        if ui.small_button("Street").clicked() {
            selected_tool.tool = Tool::Street;
        }
    });

    if let Some(entity) = currently_selected.entity {
        if let Ok(storage) = storage_query.get(entity) {
            egui::SidePanel::left("side_panel")
                .default_width(200.0)
                .show(egui_context.ctx(), |ui| {
                    ui.heading("Side Panel");

                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{:?} {} / {}",
                            storage.resource, storage.amount, storage.capacity
                        ));
                    });
                });
        }
    }
}
