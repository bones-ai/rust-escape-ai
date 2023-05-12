use macroquad::prelude::*;

use crate::editor::Editor;
use crate::population::Population;
use crate::*;

pub struct Simulation {
    population: Population,
    frame_count: usize,
    generation_count: u32,
}

pub struct SimulationStats {
    pub frame_count: usize,
    pub generation_count: u32,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            population: Population::new(),
            frame_count: 0,
            generation_count: 1,
        }
    }

    pub fn update(&mut self, editor: &Editor) -> Option<SimulationStats> {
        if editor.settings.is_pause {
            return None;
        }

        self.population.update(self.frame_count, editor);
        self.frame_count += 1;

        if self.frame_count >= NUM_FRAMES && editor.settings.is_ai_enabled {
            self.start_new_generation(!editor.settings.is_random_ai);
        }

        Some(SimulationStats {
            frame_count: self.frame_count,
            generation_count: self.generation_count,
        })
    }

    fn start_new_generation(&mut self, is_selection: bool) {
        if is_selection {
            self.population.selection();
        } else {
            self.population = Population::new();
        }

        self.frame_count = 0;
        self.generation_count += 1;
    }

    pub fn draw(&self, editor: &Editor) {
        if !editor.settings.is_draw {
            return;
        }

        self.population.draw(editor);
    }
}

impl SimulationStats {
    pub fn new() -> Self {
        Self {
            frame_count: 1,
            generation_count: 1,
        }
    }
}
