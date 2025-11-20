use crate::config;
use crate::filesystem::FileNode;
use crate::math::{Ray, ray_box_intersection};
use macroquad::prelude::*;

pub struct MouseState {
    pub hover_index: Option<usize>,
    pub clicked_index: Option<usize>,
    pub is_dragging: bool,
    pub drag_delta: Vec2,
    pub scroll_delta: f32,
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            hover_index: None,
            clicked_index: None,
            is_dragging: false,
            drag_delta: Vec2::ZERO,
            scroll_delta: 0.0,
        }
    }

    pub fn update(&mut self, entries: &[FileNode], camera: &Camera3D) {
        let mouse_pos = mouse_position();

        self.hover_index = Self::raycast_blocks(entries, mouse_pos, camera);
        self.is_dragging = is_mouse_button_down(MouseButton::Right);
        self.drag_delta = if self.is_dragging {
            mouse_delta_position() * 100.0
        } else {
            Vec2::ZERO
        };

        self.scroll_delta = mouse_wheel().1;
        self.clicked_index = if is_mouse_button_pressed(MouseButton::Left) {
            self.hover_index
        } else {
            None
        };
    }

    fn raycast_blocks(
        entries: &[FileNode],
        screen_pos: (f32, f32),
        camera: &Camera3D,
    ) -> Option<usize> {
        let ray = Ray::from_camera(Vec2::new(screen_pos.0, screen_pos.1), camera);

        let mut closest: Option<(usize, f32)> = None;

        for (i, node) in entries.iter().enumerate() {
            let height = node.calculate_height();
            let pos = Vec3::new(
                node.grid_pos.0 as f32 * config::GRID_SPACING,
                height / 2.0,
                node.grid_pos.1 as f32 * config::GRID_SPACING,
            );

            let half_size = Vec3::new(
                config::BLOCK_WIDTH / 2.0,
                height / 2.0,
                config::BLOCK_DEPTH / 2.0,
            );

            if let Some(dist) = ray_box_intersection(&ray, pos, half_size)
                && (closest.is_none() || dist < closest.unwrap().1)
            {
                closest = Some((i, dist));
            }
        }

        closest.map(|(idx, _)| idx)
    }
}

impl Default for MouseState {
    fn default() -> Self {
        Self::new()
    }
}
