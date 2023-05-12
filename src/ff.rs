use std::collections::HashMap;

use crate::*;

pub struct FF {
    q: Vec<(usize, usize)>,
    mat: HashMap<(usize, usize), usize>,
    weight: usize,
    start_pos: (usize, usize),
    grid_bounds: (usize, usize),
}

impl FF {
    pub fn new(start_pos: &(usize, usize), size: &(usize, usize)) -> Self {
        FF {
            q: Vec::new(),
            mat: HashMap::new(),
            weight: 0,
            start_pos: start_pos.clone(),
            grid_bounds: size.clone(),
        }
    }

    fn update_q(&mut self, (x, y): (usize, usize)) {
        if self.q.contains(&(x, y)) {
            return;
        }

        if x > self.grid_bounds.0 || y > self.grid_bounds.1 {
            return;
        }

        let tile = RESOURCES
            .get()
            .unwrap()
            .lvl_map
            .get_tile(LAYER_WALLS, x as u32, y as u32);
        if tile.is_some() {
            return;
        }

        if self.mat.contains_key(&(x, y)) {
            return;
        }

        self.q.push((x, y))
    }

    fn process(&mut self) {
        loop {
            if self.q.is_empty() {
                break;
            }

            let (x, y) = self.q.remove(0);

            self.weight += 1;
            self.mat.insert((x, y), self.weight);

            self.update_q((x + 1, y));
            self.update_q((x - 1, y));
            self.update_q((x, y + 1));
            self.update_q((x, y - 1));
        }
    }

    pub fn solve(&mut self) -> HashMap<(usize, usize), usize> {
        let (x, y) = self.start_pos;
        self.q.push((x, y));
        self.process();

        self.mat.clone()
    }
}
