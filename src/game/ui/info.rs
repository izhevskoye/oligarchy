use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use egui::Ui;

use crate::game::assets::{
    BlastFurnace, CokeFurnace, CurrentlySelected, ExportStation, OxygenConverter, Quarry, Storage,
};

trait InfoUI {
    fn ui(&self, _ui: &mut Ui) {}
}

impl InfoUI for Storage {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(format!(
                "{:?} {} / {}",
                self.resource, self.amount, self.capacity
            ));
        });
    }
}

impl InfoUI for ExportStation {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("Export Station for {:?}", self.goods));
        });
    }
}

impl InfoUI for Quarry {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("{:?} Quarry", self.resource));
        });
    }
}

impl InfoUI for CokeFurnace {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Coke Furnace");
        });
    }
}

impl InfoUI for BlastFurnace {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Blast Furnace");
        });
    }
}

impl InfoUI for OxygenConverter {
    fn ui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Oxygen Converter");
        });
    }
}

pub fn info_ui(
    egui_context: ResMut<EguiContext>,
    queries: (
        Query<&Storage>,
        Query<&ExportStation>,
        Query<&Quarry>,
        Query<&CokeFurnace>,
        Query<&BlastFurnace>,
        Query<&OxygenConverter>,
    ),
    currently_selected: Res<CurrentlySelected>,
) {
    if let Some(entity) = currently_selected.entity {
        let item: Option<&dyn InfoUI> = if let Ok(item) = queries.0.get(entity) {
            Some(item)
        } else if let Ok(item) = queries.1.get(entity) {
            Some(item)
        } else if let Ok(item) = queries.2.get(entity) {
            Some(item)
        } else if let Ok(item) = queries.3.get(entity) {
            Some(item)
        } else if let Ok(item) = queries.4.get(entity) {
            Some(item)
        } else if let Ok(item) = queries.5.get(entity) {
            Some(item)
        } else {
            None
        };

        if let Some(item) = item {
            egui::SidePanel::left("side_panel")
                .default_width(200.0)
                .show(egui_context.ctx(), |ui| {
                    ui.heading("Info");

                    item.ui(ui);
                });
        }
    }
}
