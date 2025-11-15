pub mod load;
pub mod render;

use macroquad::prelude::*;

pub enum PreviewContent {
    None,
    Image(Texture2D),
    Text(String),
    Info(String),
}

pub struct Preview {
    pub content: PreviewContent,
}

impl Preview {
    pub fn new() -> Self {
        Self {
            content: PreviewContent::None,
        }
    }

    pub fn clear(&mut self) {
        self.content = PreviewContent::None;
    }
}
