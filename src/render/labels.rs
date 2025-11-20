use crate::config;
use crate::filesystem::FileNode;
use crate::math::world_to_screen;
use macroquad::prelude::*;

pub fn draw_labels(
    entries: &[FileNode],
    selected_index: Option<usize>,
    hover_index: Option<usize>,
    camera: &Camera3D,
) {
    for (i, node) in entries.iter().enumerate() {
        let height = node.calculate_height();
        let world_pos = Vec3::new(
            node.grid_pos.0 as f32 * config::GRID_SPACING,
            height + 0.5,
            node.grid_pos.1 as f32 * config::GRID_SPACING,
        );

        if let Some(screen_pos) = world_to_screen(world_pos, camera)
            && screen_pos.x > 0.0
            && screen_pos.x < screen_width()
            && screen_pos.y > 0.0
            && screen_pos.y < screen_height()
        {
            let is_selected = selected_index == Some(i);
            let is_hovered = hover_index == Some(i);

            draw_single_label(node, screen_pos, is_selected, is_hovered);
        }
    }
}

fn draw_single_label(node: &FileNode, screen_pos: Vec2, is_selected: bool, is_hovered: bool) {
    let display_name = node.display_name(config::LABEL_MAX_LENGTH);
    let font_size = if is_selected || is_hovered {
        config::LABEL_FOCUSED_FONT_SIZE
    } else {
        config::LABEL_FONT_SIZE
    };

    let text_size = measure_text(&display_name, None, font_size as u16, 1.0);

    let bg_color = if is_selected {
        Color::new(1.0, 0.0, 1.0, 0.7)
    } else if is_hovered {
        Color::new(0.0, 1.0, 0.25, 0.7)
    } else {
        Color::new(0.0, 0.0, 0.0, 0.6)
    };

    draw_rectangle(
        screen_pos.x - text_size.width / 2.0 - 4.0,
        screen_pos.y - text_size.height - 2.0,
        text_size.width + 8.0,
        text_size.height + 4.0,
        bg_color,
    );

    let text_color = if is_selected {
        Color::new(1.0, 1.0, 1.0, 1.0)
    } else if node.is_dir {
        config::TEXT_PRIMARY
    } else {
        config::TEXT_SECONDARY
    };

    draw_text(
        &display_name,
        screen_pos.x - text_size.width / 2.0,
        screen_pos.y,
        font_size,
        text_color,
    );

    if node.is_dir {
        draw_text(
            "[]",
            screen_pos.x - text_size.width / 2.0 - config::LABEL_FONT_SIZE,
            screen_pos.y,
            font_size,
            config::TEXT_WARNING,
        );
    }
}
