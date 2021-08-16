use std::collections::HashMap;

use crate::game::{
    account::AccountTransaction,
    assets::{
        building_specifications::BuildingSpecification,
        resource_specifications::ResourceSpecifications,
    },
};
use bevy::prelude::*;

use super::assets::RequiresUpdate;

#[derive(Debug)]
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
}

pub fn construction(
    mut commands: Commands,
    mut query: Query<(Entity, &mut UnderConstruction)>,
    resources: Res<ResourceSpecifications>,
    mut events: EventWriter<AccountTransaction>,
) {
    for (entity, mut construction) in query.iter_mut() {
        let item = construction.resources_needed.clone().into_iter().next();

        if let Some((resource, amount)) = item {
            let amount_buy = amount.min(1.0);

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
            return;
        }

        if construction.labor > 0.0 {
            let invest_max = construction.labor.min(100.0);
            construction.labor -= invest_max;
            events.send(AccountTransaction {
                amount: -invest_max as i64,
            });
            return;
        }

        commands
            .entity(entity)
            .remove::<UnderConstruction>()
            .insert(RequiresUpdate);
    }
}
