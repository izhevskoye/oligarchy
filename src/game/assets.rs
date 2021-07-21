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

#[derive(Default)]
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

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    North,
    South,
    West,
    East,
    None,
}

pub struct Car {
    pub position: UVec2,
    pub direction: Direction,
    pub instructions: Vec<CarInstructions>,
    pub current_instruction: usize,
}

pub enum CarInstructions {
    GoTo(UVec2),
    WaitForLoad(Resource),
    WaitForUnload(Resource),
}

pub struct Destination {
    pub destination: UVec2,
}

pub struct Waypoints {
    pub waypoints: Vec<UVec2>,
}

pub struct CurrentlySelected {
    pub entity: Option<Entity>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Tool {
    None,
    Street,
    Storage(Resource),
    Quarry(Resource),
    CokeFurnace,
    BlastFurnace,
    OxygenConverter,
    ExportStation,
}

pub struct SelectedTool {
    pub tool: Tool,
}

#[derive(Default)]
pub struct ClickedTile {
    pub pos: Option<UVec2>,
    pub occupied_building: bool,
}

pub struct Occupied;
