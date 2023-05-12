use std::time::Instant;

use macroquad::prelude::*;

use crate::*;

pub enum AgentCommand {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Clone)]
pub struct Agent {
    pub pos: (usize, usize),
    game_size: (usize, usize),
    birth_ts: Instant,
}

impl Agent {
    pub fn new() -> Self {
        let resources = RESOURCES.get().unwrap();
        let lvl = &resources.lvl_info;
        Self {
            pos: lvl.agent.clone(),
            game_size: lvl.size.clone(),
            birth_ts: Instant::now(),
        }
    }

    pub fn update(&mut self, command: AgentCommand, has_all_keys: bool) {
        let resources = RESOURCES.get().unwrap();

        if self.is_agent_asleep() {
            return;
        }

        // Update player pos
        let mut new_loc = self.pos.clone();
        match command {
            AgentCommand::Left => new_loc = (new_loc.0 - 1, new_loc.1),
            AgentCommand::Right => new_loc = (new_loc.0 + 1, new_loc.1),
            AgentCommand::Bottom => new_loc = (new_loc.0, new_loc.1 + 1),
            AgentCommand::Top => new_loc = (new_loc.0, new_loc.1 - 1),
        }

        // Make sure new player pos isnt a wall
        let is_not_wall = resources
            .lvl_map
            .get_tile(LAYER_WALLS, new_loc.0 as u32, new_loc.1 as u32)
            .is_none();
        let is_not_door = resources
            .lvl_map
            .get_tile(LAYER_DOOR, new_loc.0 as u32, new_loc.1 as u32)
            .is_none();

        if is_not_wall && (is_not_door || has_all_keys) && self.is_in_bounds(new_loc.0, new_loc.1) {
            self.pos = new_loc;
        }
    }

    pub fn draw(&self, scale_factor: f32, offset_x: f32, offset_y: f32) {
        let textures = TEXTURES.get().unwrap();
        let mut texture = match self.birth_ts.elapsed().as_secs() {
            0 => textures.agent_sleep1_texture,
            1 => textures.agent_sleep2_texture,
            2 => textures.agent_sleep3_texture,
            _ => textures.agent_texture,
        };

        if !IS_PLAY_SLEEP_ANIMATION {
            texture = textures.agent_texture;
        }

        draw_texture_ex(
            texture,
            self.pos.0 as f32 * scale_factor + offset_x,
            self.pos.1 as f32 * scale_factor + offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::splat(scale_factor)),
                ..Default::default()
            },
        );
    }

    fn is_agent_asleep(&self) -> bool {
        if !IS_PLAY_SLEEP_ANIMATION {
            return false;
        }

        match self.birth_ts.elapsed().as_secs() {
            0 => true,
            1 => true,
            2 => true,
            3 => true,
            _ => false,
        }
    }

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x <= self.game_size.0 - 1 && y <= self.game_size.1 - 1
    }
}

impl AgentCommand {
    pub fn from_int(value: u8) -> Self {
        match value {
            0 => AgentCommand::Top,
            1 => AgentCommand::Left,
            2 => AgentCommand::Bottom,
            _ => AgentCommand::Right,
        }
    }

    pub fn to_int(&self) -> u8 {
        match self {
            AgentCommand::Top => 0,
            AgentCommand::Left => 1,
            AgentCommand::Bottom => 2,
            AgentCommand::Right => 3,
        }
    }
}
