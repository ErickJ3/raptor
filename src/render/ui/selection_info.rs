use crate::config;
use crate::filesystem::FileNode;
use macroquad::prelude::*;

pub fn draw_selection_info(node: &FileNode) {
    let panel_y = screen_height() - config::FOOTER_HEIGHT;

    let line1 = format!("SELECTED: {} | TYPE: {}", node.name, node.type_display(),);

    let line2 = if node.is_dir {
        format!(
            "CONTENTS: {} items | POS: ({}, {})",
            node.children_count, node.grid_pos.0, node.grid_pos.1
        )
    } else {
        format!(
            "SIZE: {} | POS: ({}, {})",
            node.size_display(),
            node.grid_pos.0,
            node.grid_pos.1
        )
    };

    let line3 = format!("PATH: {}", node.path.to_string_lossy());

    let font_size = config::INFO_FONT_SIZE;
    let line_spacing = config::INFO_LINE_SPACING * config::INFO_FONT_SIZE;

    let right_offset = 20.0;
    let x_pos = screen_width() - right_offset;

    let draw_right_aligned = |text: &str, y_offset: f32| {
        let text_width = measure_text(text, None, font_size as u16, 1.0).width;
        draw_text(
            text,
            x_pos - text_width,
            panel_y + 40.0 + y_offset,
            font_size,
            config::TEXT_HIGHLIGHT,
        );
    };

    draw_right_aligned(&line1, 0.0);
    draw_right_aligned(&line2, line_spacing);
    draw_right_aligned(&line3, line_spacing * 2.0);

    let line4 = format!("BLOCK HEIGHT: {:.2}", node.calculate_height());
    draw_right_aligned(&line4, line_spacing * 3.0);
}
