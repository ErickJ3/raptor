use crate::config;
use macroquad::prelude::*;

pub struct ScanEffect {
    pub y_position: f32,
    pub active: bool,
}

impl ScanEffect {
    pub fn new() -> Self {
        Self {
            y_position: 0.0,
            active: true,
        }
    }

    pub fn reset(&mut self) {
        self.y_position = 0.0;
        self.active = true;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.active {
            self.y_position += delta_time * config::SCAN_SPEED;
            if self.y_position > config::SCAN_MAX_HEIGHT {
                self.active = false;
            }
        }
    }

    pub fn draw(&self) {
        if self.active {
            draw_cube(
                Vec3::new(0.0, self.y_position, 0.0),
                Vec3::new(100.0, 0.1, 100.0),
                None,
                config::SCAN_COLOR,
            );
        }
    }
}

impl Default for ScanEffect {
    fn default() -> Self {
        Self::new()
    }
}

pub fn draw_scanlines() {
    for y in (0..screen_height() as i32).step_by(config::SCANLINE_STEP) {
        draw_rectangle(
            0.0,
            y as f32,
            screen_width(),
            config::SCANLINE_HEIGHT,
            Color::new(0.0, 0.0, 0.0, config::SCANLINE_ALPHA),
        );
    }
}

pub fn draw_vignette() {
    let edge_alpha = config::VIGNETTE_ALPHA;
    let edge_size = config::VIGNETTE_SIZE;

    // Top edge
    for i in 0..edge_size as i32 {
        let alpha = edge_alpha * (1.0 - i as f32 / edge_size);
        draw_rectangle(
            0.0,
            i as f32,
            screen_width(),
            1.0,
            Color::new(0.0, 0.0, 0.0, alpha),
        );
    }

    // Bottom edge
    for i in 0..edge_size as i32 {
        let alpha = edge_alpha * (i as f32 / edge_size);
        let y = screen_height() - edge_size + i as f32;
        draw_rectangle(
            0.0,
            y,
            screen_width(),
            1.0,
            Color::new(0.0, 0.0, 0.0, alpha),
        );
    }

    // Left edge
    for i in 0..edge_size as i32 {
        let alpha = edge_alpha * (1.0 - i as f32 / edge_size);
        draw_rectangle(
            i as f32,
            0.0,
            1.0,
            screen_height(),
            Color::new(0.0, 0.0, 0.0, alpha),
        );
    }

    // Right edge
    for i in 0..edge_size as i32 {
        let alpha = edge_alpha * (i as f32 / edge_size);
        let x = screen_width() - edge_size + i as f32;
        draw_rectangle(
            x,
            0.0,
            1.0,
            screen_height(),
            Color::new(0.0, 0.0, 0.0, alpha),
        );
    }
}
