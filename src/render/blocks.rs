use crate::config;
use crate::filesystem::FileNode;
use macroquad::prelude::*;

pub fn get_block_color(node: &FileNode, is_selected: bool, is_hovered: bool) -> Color {
    if is_selected {
        config::SELECTED_COLOR
    } else if is_hovered {
        if node.is_dir {
            config::HOVER_DIR_COLOR
        } else {
            config::HOVER_FILE_COLOR
        }
    } else if node.is_dir {
        config::DIR_COLOR
    } else {
        config::FILE_COLOR
    }
}

pub fn draw_block(node: &FileNode, is_selected: bool, is_hovered: bool, dir_height_by_size: bool, max_size: u64) {
    let height = node.calculate_height_normalized(dir_height_by_size, max_size);
    let color = get_block_color(node, is_selected, is_hovered);
    
    // Position block with bottom at ground level (y=0), centered at height/2
    let pos = Vec3::new(
        node.grid_pos.0 as f32 * config::GRID_SPACING,
        height / 2.0,
        node.grid_pos.1 as f32 * config::GRID_SPACING,
    );

    draw_cube(
        pos,
        Vec3::new(config::BLOCK_WIDTH, height, config::BLOCK_DEPTH),
        None,
        color,
    );

    let outline_color = Color::new(color.r, color.g, color.b, 1.0);
    draw_cube_wires(
        pos,
        Vec3::new(config::BLOCK_WIDTH, height, config::BLOCK_DEPTH),
        outline_color,
    );

    if is_selected || is_hovered {
        let glow_color = Color::new(color.r, color.g, color.b, 0.2);
        draw_cube(
            pos,
            Vec3::new(
                config::BLOCK_WIDTH + 0.2,
                height + 0.2,
                config::BLOCK_DEPTH + 0.2,
            ),
            None,
            glow_color,
        );
    }
}

pub fn draw_all_blocks(
    entries: &[FileNode],
    selected_index: Option<usize>,
    hover_index: Option<usize>,
    dir_height_by_size: bool,
) {
    // Calculate max size for normalization
    let max_size = entries
        .iter()
        .map(|node| node.size)
        .max()
        .unwrap_or(0);

    for (i, node) in entries.iter().enumerate() {
        let is_selected = selected_index == Some(i);
        let is_hovered = hover_index == Some(i);
        draw_block(node, is_selected, is_hovered, dir_height_by_size, max_size);
    }
}
