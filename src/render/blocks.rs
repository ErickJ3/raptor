use crate::config;
use crate::filesystem::FileNode;
use macroquad::prelude::*;
use std::collections::HashSet;

pub fn get_block_color(
    node: &FileNode,
    is_selected: bool,
    is_hovered: bool,
    is_match: bool,
    dim: bool,
) -> Color {
    let base = if is_selected {
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
    };

    let mut color = base;
    if is_match && !is_selected {
        color = config::TEXT_HIGHLIGHT;
    }
    if dim {
        color.a *= 0.18;
    }
    color
}

pub fn draw_block(
    node: &FileNode,
    is_selected: bool,
    is_hovered: bool,
    is_match: bool,
    dim: bool,
) {
    let height = node.calculate_height();
    let color = get_block_color(node, is_selected, is_hovered, is_match, dim);
    let pos = node.world_position();

    draw_cube(
        pos,
        Vec3::new(config::BLOCK_WIDTH, height, config::BLOCK_DEPTH),
        None,
        color,
    );

    let outline_alpha = if dim { 0.25 } else { 1.0 };
    let outline_color = Color::new(color.r, color.g, color.b, outline_alpha);
    draw_cube_wires(
        pos,
        Vec3::new(config::BLOCK_WIDTH, height, config::BLOCK_DEPTH),
        outline_color,
    );

    if is_selected || is_hovered || is_match {
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
    match_set: Option<HashSet<usize>>,
) {
    let has_filter = match_set.is_some();
    for (i, node) in entries.iter().enumerate() {
        let is_selected = selected_index == Some(i);
        let is_hovered = hover_index == Some(i);
        let is_match = match_set.as_ref().is_some_and(|s| s.contains(&i));
        let dim = has_filter && !is_match && !is_selected;
        draw_block(node, is_selected, is_hovered, is_match, dim);
    }
}
