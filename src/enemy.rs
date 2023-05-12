use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::level::GameItem;
use crate::*;

#[derive(Clone)]
enum EnemyKind {
    Spike(f32),
    Crab(bool),
}

#[derive(Clone)]
pub struct EnemyManager {
    enemies: Vec<Enemy>,
    spikes: Vec<Enemy>,
}

#[derive(Clone)]
struct Enemy {
    pos: (usize, usize),
    kind: EnemyKind,
    item: GameItem,
}

impl EnemyManager {
    pub fn new(enemies: Vec<GameItem>, spikes: Vec<GameItem>) -> Self {
        Self {
            enemies: enemies
                .iter()
                .map(|e| Enemy::new(e.pos.0, e.pos.1, EnemyKind::Crab(false), e))
                .collect(),
            spikes: spikes
                .iter()
                .map(|e| Enemy::new(e.pos.0, e.pos.1, EnemyKind::Spike(0.0), e))
                .collect(),
        }
    }

    pub fn update(&mut self, agent_pos: &(usize, usize)) -> bool {
        for e in self.enemies.iter_mut() {
            if e.update(agent_pos) {
                return true;
            }
        }
        for s in self.spikes.iter_mut() {
            if s.update(agent_pos) {
                return true;
            }
        }

        false
    }

    pub fn draw(&self, scale_factor: f32, offset_x: f32, offset_y: f32) {
        self.enemies
            .iter()
            .for_each(|e| e.draw(scale_factor, offset_x, offset_y));
        self.spikes
            .iter()
            .for_each(|e| e.draw(scale_factor, offset_x, offset_y));
    }
}

impl Enemy {
    fn new(x: usize, y: usize, kind: EnemyKind, game_item: &GameItem) -> Self {
        Self {
            pos: (x, y),
            kind,
            item: game_item.clone(),
        }
    }

    fn update(&mut self, agent_pos: &(usize, usize)) -> bool {
        let (x, y) = agent_pos;

        match self.kind {
            EnemyKind::Crab(v) => {
                let resources = RESOURCES.get().unwrap();
                match self.item.value {
                    88 => {
                        let mut new_loc = self.pos.clone();
                        new_loc.1 = match v {
                            true => new_loc.1 + 1,
                            false => new_loc.1 - 1,
                        };

                        let is_wall = resources
                            .lvl_map
                            .get_tile(LAYER_WALLS, new_loc.0 as u32, new_loc.1 as u32)
                            .is_some();
                        if is_wall {
                            self.kind = EnemyKind::Crab(!v);
                            new_loc.1 = match v {
                                true => new_loc.1 - 2,
                                false => new_loc.1 + 2,
                            };
                        }

                        self.pos = new_loc;
                    }
                    _ => {
                        let mut new_loc = self.pos.clone();
                        // new_loc.0 += 1;
                        new_loc.0 = match v {
                            true => new_loc.0 - 1,
                            false => new_loc.0 + 1,
                        };

                        let is_wall = resources
                            .lvl_map
                            .get_tile(LAYER_WALLS, new_loc.0 as u32, new_loc.1 as u32)
                            .is_some();
                        if is_wall {
                            self.kind = EnemyKind::Crab(!v);
                            new_loc.0 = match v {
                                true => new_loc.0 + 2,
                                false => new_loc.0 - 2,
                            };
                        }

                        self.pos = new_loc;
                    }
                }
            }
            EnemyKind::Spike(v) => {
                self.kind = EnemyKind::Spike((v + 10.0) % 360.0);
            }
        }

        if *x == self.pos.0 && *y == self.pos.1 {
            return true;
        }

        false
    }

    pub fn draw(&self, scale_factor: f32, offset_x: f32, offset_y: f32) {
        let textures = TEXTURES.get().unwrap();
        let texture = match self.kind {
            EnemyKind::Crab(_) => textures.crab_texture,
            EnemyKind::Spike(_) => {
                match self.item.value {
                    101 => textures.small_spike_texture,
                    104 => textures.large_spike_texture,
                    // Blank spikes (made just for collision, aren't to be rendered)
                    _ => return,
                }
            }
        };

        let mut render_scale = 1.0;
        if texture == textures.large_spike_texture {
            render_scale = 2.0;
        }

        let rotation = match self.kind {
            EnemyKind::Spike(v) => v,
            EnemyKind::Crab(v) => match self.item.value {
                88 => {
                    if v {
                        PI
                    } else {
                        0.0
                    }
                }
                _ => {
                    if v {
                        PI + PI / 2.0
                    } else {
                        PI / 2.0
                    }
                }
            },
        };

        draw_texture_ex(
            texture,
            self.pos.0 as f32 * scale_factor + offset_x,
            self.pos.1 as f32 * scale_factor + offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::splat(scale_factor * render_scale)),
                rotation,
                ..Default::default()
            },
        );
    }
}
