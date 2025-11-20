use crate::config;
use macroquad::prelude::*;
use std::path::PathBuf;

pub fn draw_breadcrumb(
    path_components: &[(String, PathBuf)],
    has_parent: bool,
    dir_count: usize,
    file_count: usize,
) {
    let y_pos = config::HEADER_HEIGHT;

    draw_rectangle(
        0.0,
        y_pos,
        screen_width(),
        config::BREADCRUMB_HEIGHT,
        config::UI_BREADCRUMB_COLOR,
    );

    let mut x_pos = 20.0;
    let text_y = y_pos + 17.0;

    if has_parent {
        draw_text(
            "[U/-]",
            x_pos,
            text_y,
            config::INFO_FONT_SIZE,
            config::TEXT_WARNING,
        );
        x_pos += 70.0;
    }

    for (i, (name, _path)) in path_components.iter().enumerate() {
        let separator = if i > 0 { " / " } else { "" };
        let display = format!("{}{}", separator, name);

        let is_current = i == path_components.len() - 1;
        let color = if is_current {
            config::TEXT_HIGHLIGHT
        } else {
            Color::new(0.0, 0.83, 1.0, 0.8)
        };

        draw_text(&display, x_pos, text_y, config::INFO_FONT_SIZE, color);
        x_pos +=
            measure_text(&display, None, config::INFO_FONT_SIZE.round() as u16, 1.0).width + 2.0;

        if x_pos > screen_width() - 200.0 {
            draw_text("...", x_pos, text_y, config::INFO_FONT_SIZE, color);
            break;
        }
    }

    let children_info = format!("{} dirs | {} files", dir_count, file_count);
    let info_width = measure_text(
        &children_info,
        None,
        config::LABEL_FONT_SIZE.round() as u16,
        1.0,
    )
    .width;
    draw_text(
        &children_info,
        screen_width() - info_width - 20.0,
        text_y,
        config::LABEL_FONT_SIZE,
        Color::new(0.0, 1.0, 0.25, 0.7),
    );
}
