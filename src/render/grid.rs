use crate::config;
use macroquad::prelude::*;

pub fn draw_grid_floor() {
    for i in -config::GRID_SIZE..=config::GRID_SIZE {
        let pos = i as f32 * config::GRID_SPACING;
        let extent = config::GRID_SIZE as f32 * config::GRID_SPACING;

        draw_line_3d(
            Vec3::new(-extent, 0.0, pos),
            Vec3::new(extent, 0.0, pos),
            config::GRID_COLOR,
        );

        draw_line_3d(
            Vec3::new(pos, 0.0, -extent),
            Vec3::new(pos, 0.0, extent),
            config::GRID_COLOR,
        );
    }
}
