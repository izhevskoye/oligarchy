pub const TILE_SIZE: f32 = 16.0;
pub const TILE_MAP_WIDTH: f32 = 16.0;
pub const TILE_MAP_HEIGHT: f32 = 16.0;
pub const CHUNK_SIZE: u32 = 16;

pub const STORAGE_SIZE: f64 = 250.0;
pub const CAR_STORAGE_SIZE: f64 = 20.0;

pub const PRODUCTION_TICK_SPEED: f64 = 2.5;
pub const CAR_DRIVE_TICK_SPEED: f64 = 0.2;
pub const CAR_INSTRUCTION_TICK_SPEED: f64 = 0.25;

pub const Z_CAR: f32 = 1.0;
pub const Z_IDLE_INDICATOR: f32 = 1.5;
pub const Z_SELECTION_INDICATOR: f32 = 2.0;

#[derive(FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum MapTile {
    WaterDeep,
    Water,
    Ground,
    Storage,
    BlastFurnace,
    CoalQuarry,
    IronOreQuarry,
    LimestoneQuarry,
    OxygenConverter,
    CokeFurnace,
    ExportStation,
    StreetEastSouth,
    StreetEastWest,
    StreetSouthWest,
    StreetEastSouthWest,
    StreetNorthSouthWest,
    StorageCoal,
    StorageCoke,
    StorageIron,
    StorageSteel,
    StorageLimestone,
    StorageIronOre,
    StreetNorthSouth = 27,
    StreetNorthEastSouthWest,
    StreetNorthEastSouth,
    StreetSouthEnd,
    StreetNorthEnd,
    GroundFactory = 32,
    StreetNone = 42,
    StreetNorthEast,
    StreetNorthEastWest,
    StreetNorthWest,
    StreetEastEnd,
    StreetWestEnd,
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum VehicleTile {
    Empty = 64,
    GreenHorizontal = 82,
    YellowHorizontal,
    BlueVertical,
    GreenVertical,
    BlueHorizontal = 114,
    RedHorizontal,
    RedVertical,
    YellowVertical,
}
