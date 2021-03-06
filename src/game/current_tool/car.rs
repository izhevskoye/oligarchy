use bevy::prelude::*;

use crate::game::{
    account::{Account, AccountTransaction, MaintenanceCost, PurchaseCost},
    assets::{resource_specifications::ResourceSpecifications, ClickedTile, Editable, Position},
    car::Car,
    constants::CAR_STORAGE_SIZE,
    storage::Storage,
};

use super::{SelectedTool, Tool};

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
                    .insert(MaintenanceCost::new_from_cost(price))
                    .insert(Editable);

                selected_tool.tool = Tool::None;
            }
        }
    }
}
