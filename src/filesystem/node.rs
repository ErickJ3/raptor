use crate::config;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub animated_size: f32,  // For smooth height animations
    pub children_count: usize,
    pub grid_pos: (i32, i32),
}

impl FileNode {
    pub fn new(
        name: String,
        path: PathBuf,
        is_dir: bool,
        size: u64,
        children_count: usize,
    ) -> Self {
        Self {
            name,
            path,
            is_dir,
            size,
            animated_size: size as f32,
            children_count,
            grid_pos: (0, 0),
        }
    }

    pub fn calculate_height(&self, by_size: bool) -> f32 {
        if by_size {
            // Size-based height (linear scale for both files and directories)
            // Use animated_size for smooth transitions
            if self.animated_size > 0.0 {
                (self.animated_size * config::SIZE_HEIGHT_SCALE)
                    .clamp(config::MIN_BLOCK_HEIGHT, config::MAX_BLOCK_HEIGHT)
            } else {
                // Size not yet scanned, use children count as fallback
                (self.children_count as f32).sqrt() * config::DIR_HEIGHT_MULTIPLIER
                    + config::DIR_HEIGHT_BASE
            }
        } else {
            // Default: directories by children count, files by size
            if self.is_dir {
                (self.children_count as f32).sqrt() * config::DIR_HEIGHT_MULTIPLIER
                    + config::DIR_HEIGHT_BASE
            } else {
                // Files always show by size using animated value
                (self.animated_size * config::SIZE_HEIGHT_SCALE)
                    .clamp(config::MIN_BLOCK_HEIGHT, config::MAX_BLOCK_HEIGHT)
            }
        }
    }

    pub fn calculate_height_normalized(&self, by_size: bool, max_size: u64) -> f32 {
        if !by_size || max_size == 0 {
            // Fall back to regular calculation
            return self.calculate_height(by_size);
        }

        // Size-based height with normalization: max_size gets MAX_BLOCK_HEIGHT
        if self.animated_size > 0.0 {
            let scale_factor = config::MAX_BLOCK_HEIGHT / (max_size as f32);
            (self.animated_size * scale_factor)
                .clamp(config::MIN_BLOCK_HEIGHT, config::MAX_BLOCK_HEIGHT)
        } else {
            // Size not yet scanned, use children count as fallback
            (self.children_count as f32).sqrt() * config::DIR_HEIGHT_MULTIPLIER
                + config::DIR_HEIGHT_BASE
        }
    }


    pub fn display_name(&self, max_length: usize) -> String {
        if self.name.len() > max_length {
            format!("{}...", &self.name[..max_length.saturating_sub(3)])
        } else {
            self.name.clone()
        }
    }

    pub fn size_display(&self) -> String {
        if self.is_dir {
            format!("{} items", self.children_count)
        } else {
            bytesize::ByteSize(self.size).to_string()
        }
    }

    pub fn type_display(&self) -> &'static str {
        if self.is_dir { "DIR" } else { "FILE" }
    }
}
