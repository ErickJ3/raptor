use super::{
    loader::{self, DirectoryContents},
    node::FileNode,
};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, TryRecvError, channel};
use std::thread;
use std::time::Instant;

pub enum LoadState {
    Idle,
    Loading {
        rx: Receiver<std::io::Result<DirectoryContents>>,
        started_at: Instant,
    },
}

pub struct Navigator {
    pub current_path: PathBuf,
    pub entries: Vec<FileNode>,
    pub grid_width: i32,
    pub history: Vec<PathBuf>,
    pub show_hidden: bool,
    pub load_state: LoadState,
}

impl Navigator {
    pub fn new(initial_path: PathBuf) -> Self {
        let mut nav = Self {
            current_path: initial_path.clone(),
            entries: vec![],
            grid_width: 1,
            history: vec![],
            show_hidden: false,
            load_state: LoadState::Idle,
        };
        nav.load(&initial_path);
        nav
    }

    pub fn load(&mut self, path: &PathBuf) {
        self.current_path = path.clone();
        self.entries.clear();
        self.grid_width = 1;

        let (tx, rx) = channel();
        let path_clone = path.clone();
        let show_hidden = self.show_hidden;
        thread::spawn(move || {
            let result = loader::load_directory(&path_clone, show_hidden);
            let _ = tx.send(result);
        });
        self.load_state = LoadState::Loading {
            rx,
            started_at: Instant::now(),
        };
    }

    pub fn poll(&mut self) {
        let result = match &self.load_state {
            LoadState::Loading { rx, .. } => match rx.try_recv() {
                Ok(r) => r,
                Err(TryRecvError::Empty) => return,
                Err(TryRecvError::Disconnected) => {
                    self.load_state = LoadState::Idle;
                    return;
                }
            },
            LoadState::Idle => return,
        };
        self.load_state = LoadState::Idle;
        if let Ok(contents) = result {
            self.entries = contents.nodes;
            self.grid_width = contents.grid_width;
        }
    }

    pub fn is_loading(&self) -> bool {
        matches!(self.load_state, LoadState::Loading { .. })
    }

    pub fn loading_elapsed_secs(&self) -> f32 {
        match &self.load_state {
            LoadState::Loading { started_at, .. } => started_at.elapsed().as_secs_f32(),
            LoadState::Idle => 0.0,
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

    pub fn grid_height(&self) -> i32 {
        if self.grid_width == 0 {
            0
        } else {
            (self.entries.len() as f32 / self.grid_width as f32).ceil() as i32
        }
    }
}
