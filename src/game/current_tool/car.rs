use bevy::prelude::*;

use crate::game::{
    account::{Account, AccountTransaction, PurchaseCost},
    assets::{ClickedTile, Editable, Position, SelectedTool, Tool},
    car::Car,
    constants::CAR_STORAGE_SIZE,
    resource_specifications::ResourceSpecifications,
    storage::Storage,
};

pub fn car_placement(
    mut commands: Commands,
    mut selected_tool: ResMut<SelectedTool>,
    clicked_tile: Res<ClickedTile>,
    resources: Res<ResourceSpecifications>,
    mut events: EventWriter<AccountTransaction>,
    account: Res<Account>,
) {
    if clicked_tile.dragging {
        return;
    }

    if let Tool::Car(resource) = &selected_tool.tool {
        if !clicked_tile.occupied_vehicle {
            if let Some(pos) = clicked_tile.vehicle_pos {
                let car = Car::default();

                let storage = Storage {
                    resource: resource.clone(),
                    capacity: CAR_STORAGE_SIZE,
                    ..Default::default()
                };

                let price = (car.clone(), storage.clone()).price(&resources);
                if account.value < price {
                    return;
                }

                events.send(AccountTransaction { amount: -price });

                commands
                    .spawn()
                    .insert(Position { position: pos })
                    .insert(car)
                    .insert(storage)
                    .insert(Editable);

                selected_tool.tool = Tool::None;
            }
        }
    }
}
