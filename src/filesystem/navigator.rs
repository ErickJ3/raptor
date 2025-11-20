use super::{loader, node::FileNode};
use std::path::PathBuf;

pub struct Navigator {
    pub current_path: PathBuf,
    pub entries: Vec<FileNode>,
    pub grid_width: i32,
    pub history: Vec<PathBuf>,
    pub show_hidden: bool,
}

impl Navigator {
    pub fn new(initial_path: PathBuf) -> Self {
        let mut nav = Self {
            current_path: initial_path.clone(),
            entries: vec![],
            grid_width: 1,
            history: vec![],
            show_hidden: false,
        };
        nav.load(&initial_path);
        nav
    }

    pub fn load(&mut self, path: &PathBuf) {
        self.current_path = path.clone();
        self.entries.clear();

        if let Ok(contents) = loader::load_directory(path, self.show_hidden) {
            self.entries = contents.nodes;
            self.grid_width = contents.grid_width;
        }
    }

    pub fn navigate_to(&mut self, path: &PathBuf) {
        self.history.push(self.current_path.clone());
        self.load(path);
    }

    pub fn go_back(&mut self) -> bool {
        if let Some(prev_path) = self.history.pop() {
            self.load(&prev_path);
            true
        } else if let Some(parent) = self.current_path.parent() {
            self.load(&parent.to_path_buf());
            true
        } else {
            false
        }
    }

    pub fn go_to_parent(&mut self) -> bool {
        if let Some(parent) = self.current_path.parent() {
            self.navigate_to(&parent.to_path_buf());
            true
        } else {
            false
        }
    }

    pub fn go_to_root(&mut self) {
        self.navigate_to(&PathBuf::from("/"));
    }

    pub fn go_home(&mut self) {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        self.load(&home);
        self.history.clear();
    }

    pub fn enter_directory(&mut self, index: usize) -> bool {
        if let Some(node) = self.entries.get(index)
            && node.is_dir
        {
            let path = node.path.clone();
            self.navigate_to(&path);
            return true;
        }
        false
    }

    pub fn get_path_components(&self) -> Vec<(String, PathBuf)> {
        loader::get_path_components(&self.current_path)
    }

    pub fn count_by_type(&self) -> (usize, usize) {
        loader::count_by_type(&self.entries)
    }

    pub fn has_parent(&self) -> bool {
        self.current_path.parent().is_some()
    }

    pub fn find_node_at_grid_pos(&self, pos: (i32, i32)) -> Option<usize> {
        self.entries.iter().position(|n| n.grid_pos == pos)
    }

    pub fn grid_height(&self) -> i32 {
        if self.grid_width == 0 {
            0
        } else {
            (self.entries.len() as f32 / self.grid_width as f32).ceil() as i32
        }
    }
}
