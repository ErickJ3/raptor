use macroquad::prelude::*;
use std::path::PathBuf;

use crate::camera::CameraController;
use crate::config;
use crate::filesystem::Navigator;
use crate::input::{Command, MouseState};
use crate::render::ScanEffect;

pub struct AppState {
    pub navigator: Navigator,
    pub camera: CameraController,
    pub mouse: MouseState,
    pub scan_effect: ScanEffect,
    pub selected: Option<usize>,
    pub show_labels: bool,
    pub show_hidden: bool,
}

impl AppState {
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        Self {
            navigator: Navigator::new(home),
            camera: CameraController::new(),
            mouse: MouseState::new(),
            scan_effect: ScanEffect::new(),
            selected: None,
            show_labels: true,
            show_hidden: false,
        }
    }

    pub fn update(&mut self) {
        self.camera.update();

        let camera3d = self.camera.to_camera3d();
        self.mouse.update(&self.navigator.entries, &camera3d);
        self.scan_effect.update(get_frame_time());

        if self.mouse.is_dragging {
            self.camera
                .rotate(self.mouse.drag_delta.x, self.mouse.drag_delta.y);
        }
        if self.mouse.scroll_delta != 0.0 {
            self.camera.zoom(self.mouse.scroll_delta);
        }

        if let Some(clicked_idx) = self.mouse.clicked_index {
            if self.selected == Some(clicked_idx) {
                self.execute_command(Command::EnterDirectory);
            } else {
                self.selected = Some(clicked_idx);
            }
        }
    }

    pub fn execute_command(&mut self, command: Command) {
        match command {
            Command::MoveLeft => self.move_selection(-1, 0),
            Command::MoveRight => self.move_selection(1, 0),
            Command::MoveUp => self.move_selection(0, -1),
            Command::MoveDown => self.move_selection(0, 1),

            Command::GoToFirst => {
                if !self.navigator.entries.is_empty() {
                    self.selected = Some(0);
                    self.focus_camera_on_selection();
                }
            }

            Command::GoToLast => {
                if !self.navigator.entries.is_empty() {
                    self.selected = Some(self.navigator.entries.len() - 1);
                    self.focus_camera_on_selection();
                }
            }

            Command::EnterDirectory => {
                if let Some(idx) = self.selected {
                    if self.navigator.enter_directory(idx) {
                        self.on_directory_changed();
                    }
                }
            }

            Command::GoBack => {
                if self.navigator.go_back() {
                    self.on_directory_changed();
                }
            }

            Command::GoToParent => {
                if self.navigator.go_to_parent() {
                    self.on_directory_changed();
                }
            }

            Command::GoToRoot => {
                self.navigator.go_to_root();
                self.on_directory_changed();
            }

            Command::GoHome => {
                self.navigator.go_home();
                self.on_directory_changed();
            }

            Command::ToggleHidden => {
                self.navigator.show_hidden = !self.navigator.show_hidden;
                self.navigator.load(&self.navigator.current_path.clone());
                self.on_directory_changed();
            }

            Command::ToggleLabels => {
                self.show_labels = !self.show_labels;
            }

            Command::Select(idx) => {
                if idx < self.navigator.entries.len() {
                    self.selected = Some(idx);
                    self.focus_camera_on_selection();
                }
            }

            Command::ClearSelection => {
                self.selected = None;
            }
        }
    }

    fn move_selection(&mut self, dx: i32, dz: i32) {
        if self.navigator.entries.is_empty() {
            return;
        }

        if self.selected.is_none() {
            self.selected = Some(0);
            self.focus_camera_on_selection();
            return;
        }

        let current_idx = self.selected.unwrap();
        let current_node = &self.navigator.entries[current_idx];
        let (cx, cz) = current_node.grid_pos;

        let target_pos = (cx + dx, cz + dz);

        if let Some(new_idx) = self.navigator.find_node_at_grid_pos(target_pos) {
            self.selected = Some(new_idx);
            self.focus_camera_on_selection();
        }
    }

    fn focus_camera_on_selection(&mut self) {
        if let Some(idx) = self.selected {
            if let Some(node) = self.navigator.entries.get(idx) {
                self.camera.set_target(Vec3::new(
                    node.grid_pos.0 as f32 * config::GRID_SPACING,
                    0.0,
                    node.grid_pos.1 as f32 * config::GRID_SPACING,
                ));
            }
        }
    }

    fn on_directory_changed(&mut self) {
        self.selected = None;
        self.scan_effect.reset();
        self.camera.reset_target();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
