use macroquad::prelude::*;

use crate::configs::*;

pub struct MouseCam {
    offset: Vec2,
    scale: f32,

    last_mouse_pos: Vec2,
    initial_offset: Vec2,
}

impl MouseCam {
    /// Mostly copied from
    /// https://github.com/not-fl3/macroquad/blob/master/src/experimental/camera/mouse.rs

    pub fn new(offset: Vec2, scale: f32) -> Self {
        Self {
            offset,
            scale,

            last_mouse_pos: Vec2::ZERO,
            initial_offset: offset,
        }
    }

    pub fn get_cam(&self) -> Camera2D {
        let aspect = screen_width() / screen_height();
        Camera2D {
            zoom: vec2(self.scale, -self.scale * aspect),
            offset: vec2(self.offset.x, -self.offset.y),
            target: (screen_width() / 2.0, screen_height() / 2.0).into(),
            rotation: 0.,

            render_target: None,
            viewport: None,
        }
    }

    // To be called every frame.
    // To be called after wheel and pan update to take effect
    pub fn update(&mut self, mouse_pos: Vec2, should_offset: bool) {
        let is_fast_zoom = is_key_down(KeyCode::LeftControl);
        let scale_factor = match is_fast_zoom {
            true => 1.5,
            false => 1.05,
        };
        match mouse_wheel() {
            (_x, y) if y != 0.0 => {
                self.wheel_update(mouse_position_local(), y, scale_factor);
            }
            _ => (),
        }

        if is_mouse_button_down(MouseButton::Right) {
            self.pan_update();
        }

        if is_mouse_button_down(MouseButton::Middle) {
            self.offset = self.initial_offset;
            self.scale = INITIAL_CAMERA_SCALE;
        }

        set_camera(&self.get_cam());
        if should_offset {
            self.offset += mouse_pos - self.last_mouse_pos;
        }
        self.last_mouse_pos = mouse_pos;
    }

    pub fn wheel_update(&mut self, center: Vec2, wheel_value: f32, scale_factor: f32) {
        if wheel_value > 0. {
            self.scale_mul(center, scale_factor);
        } else if wheel_value < 0. {
            self.scale_mul(center, 1.0 / scale_factor);
        }
    }

    pub fn pan_update(&mut self) {
        let current_mouse_pos: Vec2 = mouse_position_local().into();
        let mouse_delta = current_mouse_pos - self.last_mouse_pos;
        self.last_mouse_pos = current_mouse_pos;
        self.offset += mouse_delta;
    }

    fn scale_mul(&mut self, center: Vec2, mul_to_scale: f32) {
        self.scale_new(center, self.scale * mul_to_scale);
    }

    fn scale_new(&mut self, center: Vec2, new_scale: f32) {
        self.offset = (self.offset - center) * (new_scale / self.scale) + center;
        self.scale = new_scale;
    }
}
