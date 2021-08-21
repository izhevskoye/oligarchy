pub const TILE_SIZE: f32 = 16.0;
pub const TILE_MAP_WIDTH: f32 = 16.0;
pub const TILE_MAP_HEIGHT: f32 = 16.0;
pub const CHUNK_SIZE: u32 = 16;

pub const STORAGE_SIZE: f64 = 250.0;
pub const CAR_STORAGE_SIZE: f64 = 20.0;

pub const PRODUCTION_TICK_SPEED: f64 = 2.5;
pub const CAR_DRIVE_TICK_SPEED: f64 = 0.2;
pub const CAR_INSTRUCTION_TICK_SPEED: f64 = 0.25;
pub const GOAL_UPDATE_TICK_SPEED: f64 = PRODUCTION_TICK_SPEED;

pub const Z_CAR: f32 = 1.0;
pub const Z_IDLE_INDICATOR: f32 = 1.5;
pub const Z_SELECTION_INDICATOR: f32 = 2.0;
pub const CURRENCY: &str = "RUB";
pub const UNIT: &str = "t";

#[derive(FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum MapTile {
    StorageManagement = 1,
    Ground,
    Storage,
    ExportStation = 10,
    GroundFactory = 32,
    StreetNone = 42,
    StreetNorthEast,
    StreetNorthEastWest,
    StreetNorthWest,
    StreetEastEnd,
    StreetWestEnd,
    DeliveryStation = 77,
    Construction = 80,
    ConstructionStreet,
    Depot,
    WaterTilesOffset = 96,
    ForrestTilesOffset = 144,
}

#[derive(FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum VariantOffsets {
    EastSouth = 1,
    EastWest,
    SouthWest,
    EastSouthWest,
    NorthSouthWest,
    NorthSouth = 17,
    NorthEastSouthWest,
    NorthEastSouth,
    South,
    North,
    None = 32,
    NorthEast,
    NorthEastWest,
    NorthWest,
    East,
    West,
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum VehicleTile {
    BlueVertical = 84,
    BlueHorizontal = 114,
}
