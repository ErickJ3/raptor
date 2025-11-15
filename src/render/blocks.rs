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

pub fn draw_block(node: &FileNode, is_selected: bool, is_hovered: bool) {
    let height = node.calculate_height();
    let color = get_block_color(node, is_selected, is_hovered);
    let pos = node.world_position();

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
) {
    for (i, node) in entries.iter().enumerate() {
        let is_selected = selected_index == Some(i);
        let is_hovered = hover_index == Some(i);
        draw_block(node, is_selected, is_hovered);
    }
}
