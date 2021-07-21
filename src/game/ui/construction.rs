use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::assets::{Resource, SelectedTool, Tool};

pub fn construction_ui(egui_context: ResMut<EguiContext>, mut selected_tool: ResMut<SelectedTool>) {
    egui::Window::new("Construction").show(egui_context.ctx(), |ui| {
        egui::Grid::new("items").show(ui, |ui| {
            if ui.small_button("None").clicked() {
                selected_tool.tool = Tool::None;
            }
            if ui.small_button("Street").clicked() {
                selected_tool.tool = Tool::Street;
            }
            if ui.small_button("Export Station").clicked() {
                selected_tool.tool = Tool::ExportStation;
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
            if ui.small_button("Limestone Storage").clicked() {
                selected_tool.tool = Tool::Storage(Resource::Limestone);
            }
            ui.end_row();

            if ui.small_button("Iron Ore Storage").clicked() {
                selected_tool.tool = Tool::Storage(Resource::IronOre);
            }
            if ui.small_button("Iron Storage").clicked() {
                selected_tool.tool = Tool::Storage(Resource::Iron);
            }
            if ui.small_button("Steel Storage").clicked() {
                selected_tool.tool = Tool::Storage(Resource::Steel);
            }
            ui.end_row();
        });

        ui.heading("Quarry");

        egui::Grid::new("quarry").show(ui, |ui| {
            if ui.small_button("Coal Quarry").clicked() {
                selected_tool.tool = Tool::Quarry(Resource::Coal);
            }
            if ui.small_button("Iron Ore Quarry").clicked() {
                selected_tool.tool = Tool::Quarry(Resource::IronOre);
            }
            if ui.small_button("Limestone Quarry").clicked() {
                selected_tool.tool = Tool::Quarry(Resource::Limestone);
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
            if ui.small_button("Oxygen Converter").clicked() {
                selected_tool.tool = Tool::OxygenConverter;
            }
            ui.end_row();
        });
    });
}
