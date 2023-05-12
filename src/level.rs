use macroquad_tiled::Map;

use crate::*;

#[derive(Clone, Debug)]
pub struct GameItem {
    pub pos: (usize, usize),
    pub value: u32,
}

#[derive(Clone)]
pub struct LevelInfo {
    pub size: (usize, usize),
    pub key: (usize, usize),
    pub door: (usize, usize),
    pub agent: (usize, usize),
    pub spikes: Vec<GameItem>,
    pub enemies: Vec<GameItem>,
}

impl LevelInfo {
    pub fn new(map: &Map) -> Self {
        LevelInfo::parse_level(map)
    }

    fn get_one_item(w: u32, h: u32, layer: &str, map: &Map) -> Option<(usize, usize)> {
        if !map.contains_layer(layer) {
            return None;
        }

        for i in 0..w {
            for j in 0..h {
                match map.get_tile(layer, i, j) {
                    Some(_) => return Some((i as usize, j as usize)),
                    None => continue,
                }
            }
        }

        None
    }

    fn get_all_items(w: u32, h: u32, layer: &str, map: &Map) -> Vec<GameItem> {
        let mut items = Vec::new();
        if !map.contains_layer(layer) {
            return items;
        }

        for i in 0..w {
            for j in 0..h {
                match map.get_tile(layer, i, j) {
                    Some(v) => items.push(GameItem::new((i as usize, j as usize), v.id)),
                    None => continue,
                }
            }
        }

        items
    }

    fn parse_level(map: &Map) -> Self {
        // Size of the map
        let w = map.layers[LAYER_WALLS].width;
        let h = map.layers[LAYER_WALLS].height;

        // Search for a door
        let door = LevelInfo::get_one_item(w, h, LAYER_DOOR, map);
        if door.is_none() {
            panic!("No doors found");
        }

        // Search for keys
        let key = LevelInfo::get_one_item(w, h, LAYER_KEYS, map);
        if key.is_none() {
            panic!("No keys found");
        }

        // Search for player spawn pos
        let player = LevelInfo::get_one_item(w, h, LAYER_PLAYER, map);
        if player.is_none() {
            panic!("No player spawn loc");
        }

        // Other details
        let spikes = LevelInfo::get_all_items(w, h, LAYER_SPIKES, map);
        let enemies = LevelInfo::get_all_items(w, h, LAYER_ENEMIES, map);

        Self {
            size: (w as usize, h as usize),
            key: key.unwrap(),
            door: door.unwrap(),
            agent: player.unwrap(),
            spikes,
            enemies,
        }
    }
}

impl GameItem {
    fn new(pos: (usize, usize), value: u32) -> Self {
        Self { pos, value }
    }
}
