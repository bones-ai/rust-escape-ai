use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::{camera::MouseCam, simulation::SimulationStats, INITIAL_CAMERA_SCALE};

pub struct Settings {
    pub is_pause: bool,
    pub is_draw: bool,
    pub is_restart: bool,
    pub is_frame_skip: bool,
    pub is_random_ai: bool,
    pub is_show_egui: bool,
    pub is_ai_enabled: bool,
    pub is_show_multiple: bool,
    pub slow_mode: bool,
}

pub struct Editor {
    pub settings: Settings,
    mouse_cam: MouseCam,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            is_pause: false,
            is_draw: true,
            is_restart: false,
            is_frame_skip: false,
            is_ai_enabled: true,
            is_show_egui: false,
            is_show_multiple: false,
            is_random_ai: false,
            slow_mode: false,
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            settings: Settings::new(),
            mouse_cam: MouseCam::new(vec2(0.25, 0.04), INITIAL_CAMERA_SCALE),
        }
    }

    pub fn update(&mut self) {
        // Camera update
        self.mouse_cam.update(mouse_position_local(), false);

        // Handle keyboard input
        if is_key_pressed(KeyCode::Space) {
            self.settings.is_pause = !self.settings.is_pause;
        }
        if is_key_pressed(KeyCode::Tab) {
            self.settings.is_show_egui = !self.settings.is_show_egui;
        }
        if is_key_pressed(KeyCode::R) {
            self.settings.is_restart = true;
        }
        if is_key_pressed(KeyCode::Backspace) {
            self.settings.slow_mode = !self.settings.slow_mode;
        }
        if is_key_pressed(KeyCode::Backslash) {
            self.settings.is_ai_enabled = !self.settings.is_ai_enabled;
        }
        if is_key_pressed(KeyCode::RightShift) {
            self.settings.is_frame_skip = !self.settings.is_frame_skip;
        }
    }

    pub fn draw(&mut self, stats: &SimulationStats) {
        if !self.settings.is_show_egui {
            return;
        }

        egui_macroquad::ui(|ctx| {
            egui::Window::new("No Title")
                .title_bar(false)
                .min_width(200.0)
                .default_pos(egui::pos2(20.0, screen_height() - 280.0))
                .show(ctx, |ui| {
                    egui::CollapsingHeader::new("Stats")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label(format!("FPS: {}", get_fps()));
                            ui.label(format!("Frame: {}", stats.frame_count));
                            ui.label(format!("Gen: {}", stats.generation_count));
                        });

                    egui::CollapsingHeader::new("Options")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.checkbox(&mut self.settings.is_draw, "Draw");
                            ui.checkbox(&mut self.settings.slow_mode, "Slow Mode");
                            ui.checkbox(&mut self.settings.is_show_multiple, "Show Multi");
                            ui.checkbox(&mut self.settings.is_ai_enabled, "Enable AI");
                            ui.checkbox(&mut self.settings.is_frame_skip, "Frame skip");
                        });

                    egui::CollapsingHeader::new("Controls")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.checkbox(&mut self.settings.is_pause, "Pause");
                            if ui.add(egui::Button::new("Restart")).clicked() {
                                self.settings.is_restart = true;
                            }
                        });
                });
        });
        egui_macroquad::draw();
    }
}
