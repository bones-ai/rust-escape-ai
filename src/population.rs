use std::collections::HashMap;

use ::rand::distributions::WeightedIndex;
use ::rand::prelude::Distribution;
use ::rand::thread_rng;
use macroquad::prelude::*;

use crate::agent::AgentCommand;
use crate::editor::Editor;
use crate::ff::FF;
use crate::game::Game;
use crate::*;

struct FFInfo {
    pub key: HashMap<(usize, usize), usize>,
    pub door: HashMap<(usize, usize), usize>,
}

pub struct Population {
    ff_info: FFInfo,
    games: Vec<Game>,
}

impl Population {
    pub fn new() -> Self {
        Self {
            ff_info: FFInfo::new(),
            games: (0..NUM_GAMES).map(|_| Game::new()).collect(),
        }
    }

    pub fn update(&mut self, frame_count: usize, editor: &Editor) {
        // User input applies only to 1st game
        self.handle_user_input();

        if !editor.settings.is_ai_enabled {
            return;
        }
        self.games.iter_mut().for_each(|g| g.update(frame_count));
    }

    pub fn selection(&mut self) {
        let mut rng = thread_rng();
        let gene_pool = self.calc_fitness();
        let mut new_games = Vec::new();

        let num_retained = NUM_GAMES as f32 * (POP_RETENTION_RATE / 100.0);
        let num_expo = NUM_GAMES as f32 * (POP_EXPO_PERCENTAGE / 100.0);
        let num_children = NUM_GAMES as f32 - num_retained - num_expo;
        
        for _ in 0..num_children as usize {
            let first = self.games[gene_pool.sample(&mut rng)].clone();
            let second = self.games[gene_pool.sample(&mut rng)].clone();
            let new_game = Game::crossover(&first, &second);
            new_games.push(new_game);
        }

        // Retain the best games from the current gen
        self.games
            .sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        let retained_agents: Vec<Game> = (0..num_retained as usize)
            .map(|i| Game::clone_with_moves(&self.games[i]))
            .collect();

        // Exploration agents
        let mut exploration_agents: Vec<Game> = (0..num_expo as usize)
            .map(|_| Game::new())
            .collect();

        self.games.clear();
        self.games = retained_agents;
        self.games.append(&mut exploration_agents);
        self.games.append(&mut new_games);
    }

    fn calc_fitness(&mut self) -> WeightedIndex<f32> {
        let mut max_fitness = 0.0;
        let mut weights = Vec::new();

        for g in self.games.iter_mut() {
            let agent_pos = g.get_current_agent_pos();
            let ff_key = self.ff_info.key.get(&agent_pos).unwrap();
            let ff_door = self.ff_info.door.get(&agent_pos).unwrap();
            let fitness = g.fitness(ff_key, ff_door);
            if fitness > max_fitness {
                max_fitness = fitness;
            }
            weights.push(fitness);
        }
        weights
            .iter_mut()
            .for_each(|i| *i = (*i / max_fitness) * 100.0);

        WeightedIndex::new(&weights).expect("Failed to generate gene pool")
    }

    fn handle_user_input(&mut self) {
        if is_key_pressed(KeyCode::W) {
            self.games[0].update_manual(AgentCommand::Top);
        } else if is_key_pressed(KeyCode::A) {
            self.games[0].update_manual(AgentCommand::Left);
        } else if is_key_pressed(KeyCode::S) {
            self.games[0].update_manual(AgentCommand::Bottom);
        } else if is_key_pressed(KeyCode::D) {
            self.games[0].update_manual(AgentCommand::Right);
        }
    }

    pub fn draw(&self, editor: &Editor) {
        if !editor.settings.is_show_multiple {
            self.games[0].draw(0.0, 0.0);
            return;
        }

        // Draw all games
        let mut offset_x = 0.0;
        let mut offset_y = 0.0;
        let grid_padding = 40.0;
        let (w, h) = self.games[0].lvl.size;
        let (w, h) = (w as f32, h as f32);
        let (w, h) = (
            w * UNIT_FRAME_SIZE * FRAME_SCALE,
            h * UNIT_FRAME_SIZE * FRAME_SCALE,
        );
        for g in self.games.iter() {
            g.draw(offset_x, offset_y);

            offset_x += w + grid_padding;
            if offset_x >= w * NUM_GAMES_IN_ROW as f32 {
                offset_y += h + grid_padding;
                offset_x = 0.0;
            }
        }
    }
}

impl FFInfo {
    pub fn new() -> Self {
        let resources = RESOURCES.get().unwrap();
        Self {
            key: FF::new(&resources.lvl_info.key, &resources.lvl_info.size).solve(),
            door: FF::new(&resources.lvl_info.door, &resources.lvl_info.size).solve(),
        }
    }
}
