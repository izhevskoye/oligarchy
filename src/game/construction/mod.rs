#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::game::{
    account::{Account, AccountTransaction},
    assets::{
        building_specifications::BuildingSpecification,
        resource_specifications::ResourceSpecifications, InfoUI, RequiresUpdate,
    },
    constants::CURRENCY,
};
use bevy::prelude::*;
use bevy_egui::egui::Ui;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnderConstruction {
    resources_needed: HashMap<String, f64>,
    labor: f64,
}

impl UnderConstruction {
    pub fn from_building_specification(specification: &BuildingSpecification) -> Self {
        Self {
            resources_needed: specification.cost.resources.clone(),
            labor: specification.cost.base,
        }
    }

    pub fn from_fixed_cost(labor: i64) -> Self {
        Self {
            resources_needed: HashMap::new(),
            labor: labor as f64,
        }
    }
}

impl InfoUI for UnderConstruction {
    fn ui(&self, ui: &mut Ui, resources: &ResourceSpecifications) {
        ui.label("Under construction.");

        if self.labor > f64::EPSILON || !self.resources_needed.is_empty() {
            ui.label("Remaining:");
        }

        if self.labor > f64::EPSILON {
            ui.label(format!("Labor worth {} {}", self.labor, CURRENCY));
        }

        for (resource, amount) in self.resources_needed.iter() {
            let resource = resources.get(resource).unwrap();
            ui.label(format!(
                "{} {} worth {} {}",
                amount,
                resource.name,
                resource.cost * amount,
                CURRENCY
            ));
        }
    }
}

const MAX_RESOURCE: f64 = 5.0;
const MAX_LABOR: f64 = 1000.0;

pub fn construction(
    mut commands: Commands,
    mut query: Query<(Entity, &mut UnderConstruction)>,
    resources: Res<ResourceSpecifications>,
    account: Res<Account>,
    mut events: EventWriter<AccountTransaction>,
) {
    let mut sum = 0.0;
    for (entity, mut construction) in query.iter_mut() {
        if sum >= account.value as f64 {
            return;
        }

        let item = construction.resources_needed.clone().into_iter().next();

        if let Some((resource, amount)) = item {
            let amount_buy = amount.min(MAX_RESOURCE);

            if amount - amount_buy <= 0.0 {
                construction.resources_needed.remove(&resource);
            } else {
                construction
                    .resources_needed
                    .insert(resource.to_owned(), amount - amount_buy);
            }

            let resource = resources.get(&resource).unwrap();
            let price = (resource.cost * amount_buy) as i64;
            events.send(AccountTransaction { amount: -price });
            sum += price as f64;
            continue;
        }

        if construction.labor > 0.0 {
            let invest_max = construction.labor.min(MAX_LABOR);
            construction.labor -= invest_max;
            events.send(AccountTransaction {
                amount: -invest_max as i64,
            });
            sum += invest_max;
            continue;
        }

        commands
            .entity(entity)
            .remove::<UnderConstruction>()
            .insert(RequiresUpdate);
    }
}
