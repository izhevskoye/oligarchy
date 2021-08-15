use std::collections::HashMap;

use crate::game::{
    account::PurchaseCost,
    assets::{
        building_specifications::BuildingSpecificationCost,
        resource_specifications::{ResourceSpecification, ResourceSpecifications},
    },
};

use super::BuildingSpecification;

#[test]
fn purchase_cost() {
    let wood_amount = 5.0;
    let wood_price = 13.0;
    let steel_amount = 10.0;
    let steel_price = 25.0;
    let base_price = 500.0;

    let mut resources = HashMap::new();
    resources.insert("steel".to_owned(), steel_amount);
    resources.insert("wood".to_owned(), wood_amount);

    let specification = BuildingSpecification {
        cost: BuildingSpecificationCost {
            base: base_price,
            resources,
        },
        ..Default::default()
    };

    let mut resources = ResourceSpecifications::new();
    resources.insert(
        "steel".to_owned(),
        ResourceSpecification {
            name: "Steel".to_owned(),
            cost: steel_price,
            ..Default::default()
        },
    );
    resources.insert(
        "wood".to_owned(),
        ResourceSpecification {
            name: "Wood".to_owned(),
            cost: wood_price,
            ..Default::default()
        },
    );

    let price = base_price + wood_price * wood_amount + steel_price * steel_amount;
    assert_eq!(specification.price(&resources), price as i64);

    let mut parts = specification
        .price_description(&resources)
        .split("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    parts.sort();

    let correct = vec![
        "10 Steel worth 250 RUB",
        "5 Wood worth 65 RUB",
        "Labor worth 500 RUB",
    ];

    assert_eq!(parts, correct);
}
