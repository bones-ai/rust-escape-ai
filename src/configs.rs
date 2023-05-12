/// Window
pub const WINDOW_BACKGROUND_COLOR: (u8, u8, u8, u8) = (36, 36, 36, 1);
pub const INITIAL_CAMERA_SCALE: f32 = 0.00106;
pub const IS_FULL_SCREEN: bool = true;
pub const WINDOW_WIDTH: u32 = 1920;
pub const WINDOW_HEIGHT: u32 = 1080;

/// Agent & Game
pub const IS_PLAY_SLEEP_ANIMATION: bool = false;
pub const MUTATION_PROBABILITY: f32 = 10.0;
pub const FF_WEIGHT_THRESHOLD: f32 = 100000.0;
pub const POP_RETENTION_RATE: f32 = 5.0;
pub const POP_EXPO_PERCENTAGE: f32 = 10.0;

/// Simulation
pub const NUM_FRAMES: usize = 200;
pub const NUM_GAMES_IN_ROW: u32 = 35;
pub const NUM_GAMES: u32 = 1020;
pub const UNIT_FRAME_SIZE: f32 = 8.0;
pub const FRAME_SCALE: f32 = 10.0;

/// Resources
pub const TILESET_PATH: &str = "tiled/tileset.png";
pub const TILE_SET_NAME: &str = "tileset.png";

/// Levels
pub const LVL_BACKGROUND_SPRITE: &str = "tiled/lvl2.png";
pub const LVL_MAP_PATH: &str = "tiled/lvl2.json";
pub const LAYER_WALLS: &str = "walls";
pub const LAYER_DOOR: &str = "door";
pub const LAYER_KEYS: &str = "keys";
pub const LAYER_PLAYER: &str = "player";
pub const LAYER_SPIKES: &str = "spikes";
pub const LAYER_ENEMIES: &str = "enemies";
