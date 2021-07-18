use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Resource {
    Coal,
    Coke,
    Limestone,
    IronOre,
    Iron,
    Steel,
}

pub struct Quarry {
    pub resource: Resource,
}

pub struct Storage {
    pub resource: Resource,
    pub amount: i64,
    pub capacity: i64,
}

pub struct StorageConsolidator {
    pub connected_storage: Vec<Entity>,
}

pub struct CokeFurnace;

pub struct BlastFurnace;

pub struct OxygenConverter;

pub struct RequiresUpdate {
    pub position: UVec2,
}

pub struct ExportStation {
    pub goods: Vec<Resource>,
}

pub struct Street;
