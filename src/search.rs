use crate::filesystem::FileNode;
use std::collections::HashSet;

pub struct SearchState {
    pub query: String,
    pub active: bool,
    pub matches: Vec<usize>,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            active: false,
            matches: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.active = true;
        self.query.clear();
        self.matches.clear();
    }

    pub fn cancel(&mut self) {
        self.active = false;
        self.query.clear();
        self.matches.clear();
    }

    pub fn append(&mut self, c: char) {
        self.query.push(c);
    }

    pub fn backspace(&mut self) {
        self.query.pop();
    }

    pub fn recompute(&mut self, entries: &[FileNode]) {
        self.matches.clear();
        if self.query.is_empty() {
            return;
        }
        let needle = self.query.to_lowercase();
        for (i, node) in entries.iter().enumerate() {
            if node.name.to_lowercase().contains(&needle) {
                self.matches.push(i);
            }
        }
    }

    pub fn match_set(&self) -> Option<HashSet<usize>> {
        if self.active && !self.query.is_empty() {
            Some(self.matches.iter().copied().collect())
        } else {
            None
        }
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}
