use macroquad::prelude::*;
use macroquad_tiled::Map;
use once_cell::sync::OnceCell;

use crate::level::LevelInfo;
use crate::*;

// This is a OnceCell static Global var
pub static RESOURCES: OnceCell<Resources> = OnceCell::new();
pub static TEXTURES: OnceCell<Textures> = OnceCell::new();

pub struct Resources {
    pub lvl_map: Map,
    pub lvl_background_sprite: Texture2D,
    pub lvl_info: LevelInfo,
}

pub struct Textures {
    pub agent_texture: Texture2D,
    pub agent_sleep1_texture: Texture2D,
    pub agent_sleep2_texture: Texture2D,
    pub agent_sleep3_texture: Texture2D,
    pub key_texture: Texture2D,
    pub crab_texture: Texture2D,
    pub small_spike_texture: Texture2D,
    pub large_spike_texture: Texture2D,
}

pub async fn init_resources() {
    let resources = Resources::new().await;
    let textures = Textures::new().await;
    match RESOURCES.set(resources) {
        Ok(_) => println!("Resources init successfull"),
        Err(_) => panic!("Failed to load Resources"),
    };
    match TEXTURES.set(textures) {
        Ok(_) => println!("Textures init successfull"),
        Err(_) => panic!("Failed to load Textures"),
    };
}

impl Resources {
    async fn new() -> Self {
        // Load level components
        let lvl_background_sprite = load_texture(LVL_BACKGROUND_SPRITE).await.unwrap();
        lvl_background_sprite.set_filter(FilterMode::Nearest);
        let tiled_map_json = load_string(LVL_MAP_PATH).await.unwrap();
        let tileset = Textures::get_texture(TILESET_PATH).await;
        let lvl_map =
            macroquad_tiled::load_map(&tiled_map_json, &[(TILE_SET_NAME, tileset)], &[]).unwrap();
        let lvl_info = LevelInfo::new(&lvl_map);

        Self {
            lvl_map,
            lvl_background_sprite,
            lvl_info,
        }
    }
}

impl Textures {
    async fn new() -> Self {
        Self {
            agent_texture: Textures::get_texture("assets/agent.png").await,
            agent_sleep1_texture: Textures::get_texture("assets/agent_sleeping.png").await,
            agent_sleep2_texture: Textures::get_texture("assets/agent_sleeping_2.png").await,
            agent_sleep3_texture: Textures::get_texture("assets/agent_sleeping_3.png").await,

            key_texture: Textures::get_texture("assets/key.png").await,
            crab_texture: Textures::get_texture("assets/crab.png").await,
            small_spike_texture: Textures::get_texture("assets/small_spike.png").await,
            large_spike_texture: Textures::get_texture("assets/large_spike.png").await,
        }
    }

    async fn get_texture(path: &str) -> Texture2D {
        let texture = load_texture(path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        texture
    }
}
