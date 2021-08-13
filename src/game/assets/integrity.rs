use bevy::prelude::*;

use crate::game::{
    assets::{
        building_specifications::{BuildingSpecificationCost, BuildingSpecifications},
        resource_specifications::ResourceSpecifications,
    },
    production::Product,
};

pub fn integrity_check(
    resources: Res<ResourceSpecifications>,
    buildings: Res<BuildingSpecifications>,
) {
    for building in buildings.values() {
        for product in &building.products {
            check_product(product, &resources);
        }

        check_cost(&building.cost, &resources);
    }
}

fn check_product(product: &Product, resources: &ResourceSpecifications) {
    asset_resource(&product.resource, resources);

    for requisite in &product.requisites {
        asset_resource(&requisite.resource, resources);
    }

    for byproduct in &product.byproducts {
        asset_resource(&byproduct.resource, resources);
    }

    for enhancer in &product.enhancers {
        asset_resource(&enhancer.resource, resources);
    }
}

fn check_cost(cost: &BuildingSpecificationCost, resources: &ResourceSpecifications) {
    for resource in cost.resources.keys() {
        asset_resource(resource, resources);
    }
}

fn asset_resource(resource: &str, resources: &ResourceSpecifications) {
    if !resources.contains_key(resource) {
        panic!("expected '{}' to be a valid resource", resource);
    }
}
