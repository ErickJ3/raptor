use crate::config;
use macroquad::prelude::*;
use std::path::PathBuf;

pub fn draw_header(current_path: &PathBuf, total_nodes: usize, grid_width: i32, grid_height: i32) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        config::HEADER_HEIGHT,
        config::UI_PANEL_COLOR,
    );

    draw_text(
        "LOCATION: ",
        20.0,
        30.0,
        config::HEADER_FONT_SIZE,
        config::TEXT_SECONDARY,
    );

    let path_str = current_path.display().to_string();
    let truncated_path = if path_str.len() > 60 {
        format!("...{}", &path_str[path_str.len() - 57..])
    } else {
        path_str
    };

    draw_text(
        &truncated_path,
        130.0,
        30.0,
        config::HEADER_FONT_SIZE,
        config::TEXT_PRIMARY,
    );

    let stats = format!(
        "NODES: {}  |  GRID: {}x{}",
        total_nodes, grid_width, grid_height
    );
    let stats_width =
        measure_text(&stats, None, config::HEADER_FONT_SIZE.round() as u16, 1.0).width;
    draw_text(
        &stats,
        screen_width() - stats_width - 20.0,
        30.0,
        config::HEADER_FONT_SIZE,
        config::TEXT_SECONDARY,
    );
}
