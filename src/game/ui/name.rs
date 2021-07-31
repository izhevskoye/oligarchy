use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::{assets::Name, current_selection::CurrentlySelected};

#[derive(Default)]
pub struct State {
    pub entity: Option<Entity>,
    pub input: String,
}

pub fn name_ui(
    mut commands: Commands,
    egui_context: ResMut<EguiContext>,
    mut name_query: Query<&mut Name>,
    mut currently_selected: ResMut<CurrentlySelected>,
    mut state: Local<State>,
) {
    if !currently_selected.renaming {
        return;
    }

    if let Some(entity) = currently_selected.entity {
        if Some(entity) != state.entity {
            state.entity = Some(entity);

            state.input = if let Ok(name) = name_query.get_mut(entity) {
                name.name.clone()
            } else {
                "".to_string()
            }
        };

        egui::Window::new("Rename").show(egui_context.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Name: ");
                ui.text_edit_singleline(&mut state.input);
            });

            ui.horizontal(|ui| {
                if ui.button("Ok").clicked() {
                    if state.input.is_empty() {
                        commands.entity(entity).remove::<Name>();
                    } else {
                        commands.entity(entity).insert(Name {
                            name: state.input.to_owned(),
                        });
                    }
                    currently_selected.renaming = false;
                    currently_selected.locked = false;
                }
                if ui.button("Abort").clicked() {
                    currently_selected.renaming = false;
                    currently_selected.locked = false;
                }
            });
        });
    }
}
