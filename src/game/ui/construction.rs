use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};

use crate::game::{
    assets::{Resource, SelectedTool, Tool},
    building_specifications::BuildingSpecifications,
};

pub fn construction_ui(
    egui_context: ResMut<EguiContext>,
    mut selected_tool: ResMut<SelectedTool>,
    buildings: Res<BuildingSpecifications>,
) {
    egui::Window::new("Construction")
        .anchor(Align2::RIGHT_TOP, [-10.0, 10.0])
        .show(egui_context.ctx(), |ui| {
            egui::Grid::new("items").show(ui, |ui| {
                if ui.small_button("None").clicked() {
                    selected_tool.tool = Tool::None;
                }
                if ui.small_button("Bulldoze").clicked() {
                    selected_tool.tool = Tool::Bulldoze;
                }
                ui.end_row();

                if ui.small_button("Street").clicked() {
                    selected_tool.tool = Tool::Street;
                }
                if ui.small_button("Export Station").clicked() {
                    selected_tool.tool = Tool::ExportStation;
                }
                ui.end_row();
            });

            ui.heading("Cars");

            egui::Grid::new("cars").show(ui, |ui| {
                if ui.small_button("Steel Truck").clicked() {
                    selected_tool.tool = Tool::Car(Resource::Steel);
                }
                if ui.small_button("Iron Truck").clicked() {
                    selected_tool.tool = Tool::Car(Resource::Iron);
                }
                ui.end_row();

                if ui.small_button("Iron Ore Truck").clicked() {
                    selected_tool.tool = Tool::Car(Resource::IronOre);
                }
                if ui.small_button("Coal Truck").clicked() {
                    selected_tool.tool = Tool::Car(Resource::Coal);
                }
                ui.end_row();

                if ui.small_button("Coke Truck").clicked() {
                    selected_tool.tool = Tool::Car(Resource::Coke);
                }
                if ui.small_button("Limestone Truck").clicked() {
                    selected_tool.tool = Tool::Car(Resource::Limestone);
                }
                ui.end_row();
            });

            ui.heading("Storage");

            egui::Grid::new("storage").show(ui, |ui| {
                if ui.small_button("Coal Storage").clicked() {
                    selected_tool.tool = Tool::Storage(Resource::Coal);
                }
                if ui.small_button("Coke Storage").clicked() {
                    selected_tool.tool = Tool::Storage(Resource::Coke);
                }
                ui.end_row();

                if ui.small_button("Limestone Storage").clicked() {
                    selected_tool.tool = Tool::Storage(Resource::Limestone);
                }
                if ui.small_button("Iron Ore Storage").clicked() {
                    selected_tool.tool = Tool::Storage(Resource::IronOre);
                }
                ui.end_row();

                if ui.small_button("Iron Storage").clicked() {
                    selected_tool.tool = Tool::Storage(Resource::Iron);
                }
                if ui.small_button("Steel Storage").clicked() {
                    selected_tool.tool = Tool::Storage(Resource::Steel);
                }
                ui.end_row();
            });

            ui.heading("Furnace");

            egui::Grid::new("furnace").show(ui, |ui| {
                if ui.small_button("Coke Furnace").clicked() {
                    selected_tool.tool = Tool::CokeFurnace;
                }
                if ui.small_button("Blast Furnace").clicked() {
                    selected_tool.tool = Tool::BlastFurnace;
                }
                ui.end_row();
                if ui.small_button("Oxygen Converter").clicked() {
                    selected_tool.tool = Tool::OxygenConverter;
                }
                ui.end_row();
            });

            ui.heading("Buildings");

            egui::Grid::new("buildings").show(ui, |ui| {
                for (index, (id, building)) in buildings.iter().enumerate() {
                    if ui.small_button(&building.name).clicked() {
                        selected_tool.tool = Tool::Building(id.clone());
                    }

                    if index != 0 && index % 1 == 0 {
                        ui.end_row();
                    }
                }
            });
        });
}
