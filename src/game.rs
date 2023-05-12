use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::agent::{Agent, AgentCommand};
use crate::enemy::EnemyManager;
use crate::level::LevelInfo;
use crate::*;

#[derive(Clone)]
pub struct Game {
    pub lvl: LevelInfo,
    pub fitness: f32,
    pub is_key_collected: bool,
    pub is_complete: bool,
    pub is_dead: bool,

    agent: Agent,
    moves: Vec<u8>,
    enemy_manager: EnemyManager,

    // Steps it took to complete key and door step
    num_key_steps: u32,
    num_door_steps: u32,
}

impl Game {
    pub fn new() -> Self {
        let resources = RESOURCES.get().unwrap();
        let lvl = resources.lvl_info.clone();
        Self {
            enemy_manager: EnemyManager::new(lvl.enemies.clone(), lvl.spikes.clone()),
            agent: Agent::new(),
            moves: (0..NUM_FRAMES).map(|_| gen_range(0, 4)).collect(),

            // At last to avoid borrow error
            lvl,

            is_key_collected: false,
            is_complete: false,
            is_dead: false,
            num_key_steps: 0,
            num_door_steps: 0,
            fitness: 0.0
        }
    }

    pub fn with_moves(moves: &Vec<u8>) -> Self {
        let mut g = Game::new();
        g.moves = moves.clone();

        g
    }

    pub fn clone_with_moves(parent: &Game) -> Self {
        Game::with_moves(&parent.moves)
    }

    pub fn get_current_agent_pos(&self) -> &(usize, usize) {
        &self.agent.pos
    }

    pub fn update(&mut self, frame_count: usize) {
        if self.is_complete {
            return;
        }
        if self.is_dead {
            self.num_door_steps = NUM_FRAMES as u32;
            self.num_key_steps = NUM_FRAMES as u32;
            return;
        }

        self.num_door_steps += 1;
        if !self.is_key_collected {
            self.num_key_steps += 1;
        }

        let command = AgentCommand::from_int(*self.moves.get(frame_count).unwrap_or(&0));
        self.agent.update(command, self.is_key_collected);

        self.is_dead = self.enemy_manager.update(&self.agent.pos);
        self.handle_key_collision();
        self.is_complete = self.check_player_at_door();
    }

    pub fn fitness(&mut self, ff_key: &usize, ff_door: &usize) -> f32 {
        if self.is_complete {
            let key_val = NUM_FRAMES as f32 - self.num_key_steps as f32 + 1.0;
            let door_val = NUM_FRAMES as f32 - self.num_door_steps as f32 + 1.0;
            self.fitness = key_val * 20.0 + door_val * 20.0 + FF_WEIGHT_THRESHOLD * 2.0;
            return self.fitness;
        }

        if !self.is_key_collected {
            self.fitness = 1.0 / *ff_key as f32 * 1000.0;
            return self.fitness;
        }

        let f_key = 10.0 + (NUM_FRAMES as f32 / self.num_key_steps as f32);
        let f_door = (1.0 / *ff_door as f32) * 1000.0;
        self.fitness = f_door + f_key;

        if self.is_key_collected {
            self.fitness += 1000.0;
        }

        self.fitness
    }

    pub fn update_manual(&mut self, command: AgentCommand) {
        self.moves[0] = command.to_int();
        self.update(0);
    }

    pub fn crossover(first: &Self, second: &Self) -> Self {
        let split_point = gen_range(0, first.moves.len());
        let mut new_moves = Vec::from_iter(first.moves[0..split_point].iter().cloned());
        new_moves.extend_from_slice(&second.moves[split_point..]);

        for m in new_moves.iter_mut() {
            if gen_range(0.0, 1.0) > (MUTATION_PROBABILITY as f32) * 0.001 {
                continue;
            }

            *m = gen_range(0, 4);
        }

        Game::with_moves(&new_moves)
    }

    fn check_player_at_door(&self) -> bool {
        let (x, y) = self.agent.pos;
        let (dx, dy) = self.lvl.door;

        x == dx && y == dy
    }

    fn handle_key_collision(&mut self) {
        if self.is_key_collected {
            return;
        }

        let (x, y) = self.agent.pos;
        let (a, b) = self.lvl.key;
        if x == a && y == b {
            self.is_key_collected = true;
        }
    }

    pub fn draw(&self, offset_x: f32, offset_y: f32) {
        let resources = RESOURCES.get().unwrap();
        let textures = TEXTURES.get().unwrap();
        let scale_factor = UNIT_FRAME_SIZE * FRAME_SCALE;
        let w = self.lvl.size.0 as f32 * scale_factor;
        let h = self.lvl.size.1 as f32 * scale_factor;

        // Draw level background image
        let background_tint = match self.is_complete {
            true => Color::from_rgba(116, 242, 145, 255),
            false => WHITE,
        };
        draw_texture_ex(
            resources.lvl_background_sprite,
            offset_x,
            offset_y,
            background_tint,
            DrawTextureParams {
                dest_size: Some((w, h).into()),
                ..Default::default()
            },
        );

        // Draw door
        if !self.is_key_collected {
            resources
                .lvl_map
                .draw_tiles(LAYER_DOOR, Rect::new(offset_x, offset_y, w, h), None);
        }

        // Draw keys
        if !self.is_key_collected {
            draw_texture_ex(
                textures.key_texture,
                self.lvl.key.0 as f32 * scale_factor + offset_x,
                self.lvl.key.1 as f32 * scale_factor + offset_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(scale_factor)),
                    ..Default::default()
                },
            );
        }

        // Debug print
        // let f = format!("{}", f);
        // draw_text(f.as_str(), 50.0, 50.0, 40.0, RED);

        if !self.is_dead {
            // Draw agent, on top of other sprites
            self.agent.draw(scale_factor, offset_x, offset_y);
        }
        self.enemy_manager.draw(scale_factor, offset_x, offset_y);
    }
}
