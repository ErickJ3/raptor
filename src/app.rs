use macroquad::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::camera::CameraController;
use crate::config;
use crate::filesystem::Navigator;
use crate::input::{Command, MouseState};
use crate::render::ScanEffect;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
pub enum AppMode {
    Normal,
}

pub struct ScanningProgress {
    pub directories: VecDeque<(PathBuf, usize)>, // (path, depth)
    pub progress_text: String,
    pub max_depth: Option<usize>,
    pub scan_root: PathBuf, // Track which directory we're scanning
}

pub struct AppState {
    pub navigator: Navigator,
    pub camera: CameraController,
    pub mouse: MouseState,
    pub scan_effect: ScanEffect,
    pub selected: Option<usize>,
    pub show_labels: bool,
    pub show_hidden: bool,
    pub mode: AppMode,
    pub deep_scan: bool,
    pub scanning_path: Option<String>,
    pub scanning: Option<ScanningProgress>,
    pub dir_height_by_size: bool,
    pub paused_scanning: Option<ScanningProgress>,
    pub paused_scan_root: Option<PathBuf>,
    // Cache of scanned directory sizes: path -> list of (child_path, size)
    pub scan_cache: HashMap<PathBuf, Vec<(PathBuf, u64)>>,
}

impl AppState {
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        let mut navigator = Navigator::new(home);
        navigator.deep_scan = false; // temporary, will be set after prompt
        Self {
            navigator,
            camera: CameraController::new(),
            mouse: MouseState::new(),
            scan_effect: ScanEffect::new(),
            selected: None,
            show_labels: true,
            show_hidden: false,
            mode: AppMode::Normal,
            deep_scan: false,
            scanning_path: None,
            scanning: None,
            dir_height_by_size: false,
            paused_scanning: None,
            paused_scan_root: None,
            scan_cache: HashMap::new(),
        }
    }

    fn apply_cached_sizes(&mut self) {
        let current_path = self.navigator.current_path.clone();
        if let Some(cached) = self.scan_cache.get(&current_path) {
            // Restore cached sizes to the current entries
            for (path, size) in cached {
                if let Some(node) = self.navigator.entries.iter_mut().find(|n| &n.path == path) {
                    node.size = *size;
                }
            }
        }
    }

    fn cache_scanned_sizes(&mut self) {
        let current_path = self.navigator.current_path.clone();
        let sizes: Vec<(PathBuf, u64)> = self.navigator.entries
            .iter()
            .map(|node| (node.path.clone(), node.size))
            .collect();
        self.scan_cache.insert(current_path, sizes);
    }

    fn update_size_animations(&mut self) {
        // Smoothly animate node sizes towards their target (actual) sizes
        for node in self.navigator.entries.iter_mut() {
            let target = node.size as f32;
            if (node.animated_size - target).abs() > 0.1 {
                // Lerp towards target size
                node.animated_size = node.animated_size.lerp(target, config::SIZE_ANIMATION_SPEED);
            } else {
                // Snap to target when close enough
                node.animated_size = target;
            }
        }
    }

    pub fn update(&mut self) {
        match self.mode {
            AppMode::Normal => {
                self.camera.update();

                let camera3d = self.camera.to_camera3d();
                self.mouse.update(&self.navigator.entries, &camera3d, self.dir_height_by_size);
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

                // Handle incremental scanning after input processing
                // so navigation clears the scanning state before we try to process
                if let Some(mut scanning) = self.scanning.take() {
                    self.process_scan_step(&mut scanning);
                    if !scanning.directories.is_empty() {
                        self.scanning = Some(scanning);
                    }
                }

                // Update size animations for smooth transitions
                self.update_size_animations();
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
                if let Some(idx) = self.selected
                    && self.navigator.enter_directory(idx)
                {
                    self.on_directory_changed();
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
                self.navigator.load(&self.navigator.current_path.clone(), None);
                self.on_directory_changed();
            }

            Command::ToggleLabels => {
                self.show_labels = !self.show_labels;
            }

            Command::ToggleDeepScan => {
                self.deep_scan = !self.deep_scan;
                if self.deep_scan {
                    self.start_incremental_scan(None);
                } else {
                    // Clear both active and paused scans when turning off deep scan
                    self.scanning = None;
                    self.paused_scanning = None;
                    self.paused_scan_root = None;
                    self.navigator.load(&self.navigator.current_path.clone(), None);
                    self.on_directory_changed();
                }
            }

            Command::ToggleDirHeightMode => {
                self.dir_height_by_size = !self.dir_height_by_size;
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

        // Make movement relative to camera yaw
        // Camera forward direction: (-cos(yaw), -sin(yaw))
        // Camera right direction: (-sin(yaw), cos(yaw))
        // world_movement = -dx * right + (-dz) * forward
        let camera_yaw = self.camera.yaw;
        let dx_f = dx as f32;
        let dz_f = dz as f32;
        let sin_yaw = camera_yaw.sin();
        let cos_yaw = camera_yaw.cos();
        
        let rel_dx = dx_f * sin_yaw + dz_f * cos_yaw;
        let rel_dz = -dx_f * cos_yaw + dz_f * sin_yaw;
        let rel_dx = rel_dx.round() as i32;
        let rel_dz = rel_dz.round() as i32;

        let target_pos = (cx + rel_dx, cz + rel_dz);

        if let Some(new_idx) = self.navigator.find_node_at_grid_pos(target_pos) {
            self.selected = Some(new_idx);
            self.focus_camera_on_selection();
        }
    }

    fn focus_camera_on_selection(&mut self) {
        if let Some(idx) = self.selected
            && let Some(node) = self.navigator.entries.get(idx)
        {
            self.camera.set_target(Vec3::new(
                node.grid_pos.0 as f32 * config::GRID_SPACING,
                0.0,
                node.grid_pos.1 as f32 * config::GRID_SPACING,
            ));
        }
    }

    fn start_incremental_scan(&mut self, max_depth: Option<usize>) {
        let scan_root = self.navigator.current_path.clone();
        
        // Check if we have a paused scan for this directory
        if let Some(paused) = self.paused_scanning.take() {
            if paused.scan_root == scan_root {
                // Resume the paused scan
                self.scanning = Some(paused);
                self.paused_scan_root = None;
                return;
            } else {
                // Paused scan is for a different directory, discard it
                self.paused_scan_root = None;
            }
        }
        
        // Start a new scan
        let mut directories = VecDeque::new();
        directories.push_back((scan_root.clone(), 0));
        self.scanning = Some(ScanningProgress {
            directories,
            progress_text: "Scanning...".to_string(),
            max_depth,
            scan_root,
        });
        // Load without deep scan - we'll calculate sizes incrementally
        self.navigator.load(&self.navigator.current_path.clone(), None);
        // Apply any cached sizes from a previous scan of this directory
        self.apply_cached_sizes();
        // Reset camera and scan effect, but preserve selection so user can watch sizes update
        self.scan_effect.reset();
        self.camera.reset_target();
    }

    fn process_scan_step(&mut self, scanning: &mut ScanningProgress) {
        // If we've navigated away from the directory being scanned, stop
        if !self.navigator.current_path.starts_with(&scanning.scan_root) && self.navigator.current_path != scanning.scan_root {
            return;
        }

        if let Some((dir_path, depth)) = scanning.directories.pop_front() {
            let max_depth = scanning.max_depth;
            
            // Only calculate size and update nodes for depth >= 1
            // (depth 0 is the scan root itself, which is not in navigator.entries)
            if depth >= 1 {
                let size = self.calculate_directory_size_sync(&dir_path);
                
                // Update the node for this directory if it's in current entries
                if let Some(node) = self.navigator.entries.iter_mut().find(|n| n.path == dir_path) {
                    node.size = size;
                }
                
                // Accumulate size to all ancestor directories up to the scan root
                let mut current_path = dir_path.clone();
                while let Some(parent) = current_path.parent() {
                    if parent == scanning.scan_root {
                        // Update the scan root (parent directory) if it's in entries
                        if let Some(parent_node) = self.navigator.entries.iter_mut().find(|n| n.path == scanning.scan_root) {
                            parent_node.size += size;
                        }
                        break;
                    } else if parent.starts_with(&scanning.scan_root) {
                        // Update intermediate parent directories
                        if let Some(parent_node) = self.navigator.entries.iter_mut().find(|n| n.path == parent) {
                            parent_node.size += size;
                        }
                        current_path = parent.to_path_buf();
                    } else {
                        break;
                    }
                }
            }
            
            // Add subdirectories to the queue if we haven't hit max depth
            if max_depth.is_none() || depth < max_depth.unwrap() {
                if let Ok(entries) = std::fs::read_dir(&dir_path) {
                    for entry in entries.flatten() {
                        if let Ok(metadata) = entry.metadata() {
                            if metadata.is_dir() {
                                scanning.directories.push_back((entry.path(), depth + 1));
                            }
                        }
                    }
                }
            }
            
            scanning.progress_text = format!("Scanning: {} ({} remaining)", dir_path.display(), scanning.directories.len());
        } else {
            // Scanning complete - cache the results
            self.cache_scanned_sizes();
            self.scanning = None;
            self.scanning_path = None;
        }
    }

    fn calculate_directory_size_sync(&self, path: &PathBuf) -> u64 {
        let mut total_size = 0u64;
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        total_size += metadata.len();
                    }
                    // Don't recurse - subdirectories will be processed in their own queue steps
                }
            }
        }
        total_size
    }

    fn on_directory_changed(&mut self) {
        self.selected = None;
        self.scan_effect.reset();
        self.camera.reset_target();
        // Apply cached sizes from previous scan if available
        self.apply_cached_sizes();
        // Pause any ongoing scan when navigating
        if let Some(scanning) = self.scanning.take() {
            self.paused_scan_root = Some(scanning.scan_root.clone());
            self.paused_scanning = Some(scanning);
        }
        self.deep_scan = false;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
