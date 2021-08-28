use bevy::prelude::*;
use bevy_ecs_tilemap::MapQuery;
use bevy_egui::{egui, EguiContext};

use crate::game::{
    assets::{ClickedTile, Position},
    car::{Car, CarController, DepotController},
    current_selection::CurrentlySelected,
    highlight_tiles::HighlightTilesUpdateEvent,
    production::{DeliveryStation, Depot},
    setup::{BUILDING_LAYER_ID, MAP_ID},
};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum EditMode {
    AddPickup,
    AddDelivery,
    AddCar,
    None,
}

impl Default for EditMode {
    fn default() -> Self {
        Self::None
    }
}

pub fn edit_ui(
    egui_context: ResMut<EguiContext>,
    mut depot_query: Query<(Entity, &mut Depot)>,
    delivery_query: Query<(), With<DeliveryStation>>,
    mut car_query: Query<(&mut Car, &Position)>,
    mut currently_selected: ResMut<CurrentlySelected>,
    clicked_tile: Res<ClickedTile>,
    mut edit_mode: Local<EditMode>,
    map_query: MapQuery,
    mut highlight: EventWriter<HighlightTilesUpdateEvent>,
) {
    if !currently_selected.editing {
        return;
    }

    if EditMode::None == *edit_mode && currently_selected.locked {
        return;
    }

    if let Some(entity) = currently_selected.entity {
        if let Ok((entity, mut depot)) = depot_query.get_mut(entity) {
            if clicked_tile.occupied_building {
                if let Some(pos) = clicked_tile.pos {
                    if let Ok(entity) = map_query.get_tile_entity(pos, MAP_ID, BUILDING_LAYER_ID) {
                        if delivery_query.get(entity).is_ok() {
                            if EditMode::AddDelivery == *edit_mode {
                                currently_selected.locked = false;
                                *edit_mode = EditMode::None;
                                depot.deliveries.insert(pos);
                            }

                            if EditMode::AddPickup == *edit_mode {
                                currently_selected.locked = false;
                                *edit_mode = EditMode::None;
                                depot.pickups.insert(pos);
                            }
                        }
                    }
                }
            }

            if clicked_tile.occupied_vehicle {
                if let Some(pos) = clicked_tile.vehicle_pos {
                    if EditMode::AddCar == *edit_mode {
                        currently_selected.locked = false;
                        *edit_mode = EditMode::None;

                        for (mut car, position) in car_query.iter_mut() {
                            if position.position != pos {
                                continue;
                            }

                            car.controller =
                                CarController::DepotControlled(DepotController { depot: entity });
                        }
                    }
                }
            }

            egui::Window::new("Depot").show(egui_context.ctx(), |ui| {
                if EditMode::None != *edit_mode && ui.button("Abort selection").clicked() {
                    *edit_mode = EditMode::None;
                    currently_selected.locked = false;
                }

                if ui.button("Add car").clicked() {
                    *edit_mode = EditMode::AddCar;
                    currently_selected.locked = true;
                }

                egui::CollapsingHeader::new("Deliveries").show(ui, |ui| {
                    let button = ui.button("Add");

                    if button.clicked() {
                        *edit_mode = EditMode::AddDelivery;
                        currently_selected.locked = true;
                    }

                    if button.hovered() {
                        let positions: Vec<UVec2> = depot.deliveries.iter().cloned().collect();
                        highlight.send(HighlightTilesUpdateEvent::from_positions(positions));
                    }

                    for point in depot.deliveries.clone().iter() {
                        ui.horizontal(|ui| {
                            if ui.label(format!("{}", point)).hovered() {
                                highlight.send(HighlightTilesUpdateEvent::from_position(*point));
                            }

                            let button = ui.button("Delete");

                            if button.clicked() {
                                depot.deliveries.remove(point);
                            }

                            if button.hovered() {
                                highlight.send(HighlightTilesUpdateEvent::from_position(*point));
                            }
                        });
                    }
                });

                egui::CollapsingHeader::new("Pickups").show(ui, |ui| {
                    let button = ui.button("Add");

                    if button.clicked() {
                        *edit_mode = EditMode::AddPickup;
                        currently_selected.locked = true;
                    }

                    if button.hovered() {
                        let positions: Vec<UVec2> = depot.pickups.iter().cloned().collect();
                        highlight.send(HighlightTilesUpdateEvent::from_positions(positions));
                    }

                    for point in depot.pickups.clone().iter() {
                        ui.horizontal(|ui| {
                            if ui.label(format!("{}", point)).hovered() {
                                highlight
                                    .send(HighlightTilesUpdateEvent::from_position(point.clone()));
                            }

                            let button = ui.button("Delete");

                            if button.clicked() {
                                depot.pickups.remove(point);
                            }

                            if button.hovered() {
                                highlight.send(HighlightTilesUpdateEvent::from_position(*point));
                            }
                        });
                    }
                });
            });
        }
    }
}
