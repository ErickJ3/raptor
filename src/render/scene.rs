use super::{blocks, effects, grid, labels, ui};
use crate::app::AppState;
use crate::config;
use macroquad::prelude::*;

pub fn render_frame(state: &AppState) {
    clear_background(config::BACKGROUND_COLOR);
    render_normal(state);
}

fn render_normal(state: &AppState) {
    let camera = state.camera.to_camera3d();

    set_camera(&camera);

    grid::draw_grid_floor();
    blocks::draw_all_blocks(
        &state.navigator.entries,
        state.selected,
        state.mouse.hover_index,
        state.dir_height_by_size,
    );
    state.scan_effect.draw();

    set_default_camera();

    if state.show_labels {
        labels::draw_labels(
            &state.navigator.entries,
            state.selected,
            state.mouse.hover_index,
            &camera,
            state.dir_height_by_size,
        );
    }

    render_ui(state);

    if let Some(ref scanning) = state.scanning {
        let panel_y = screen_height() - config::FOOTER_HEIGHT;
        draw_text(&scanning.progress_text, 20.0, panel_y + 80.0, config::LABEL_FONT_SIZE, config::TEXT_HIGHLIGHT);
    }

    effects::draw_scanlines();
    effects::draw_vignette();
}


fn render_ui(state: &AppState) {
    ui::draw_header(
        &state.navigator.current_path,
        state.navigator.entries.len(),
        state.navigator.grid_width,
        state.navigator.grid_height(),
    );

    let (dir_count, file_count) = state.navigator.count_by_type();
    ui::draw_breadcrumb(
        &state.navigator.get_path_components(),
        state.navigator.has_parent(),
        dir_count,
        file_count,
    );

    ui::draw_status_bar(state.show_labels, state.show_hidden, state.deep_scan, state.dir_height_by_size);

    if let Some(idx) = state.selected
        && let Some(node) = state.navigator.entries.get(idx)
    {
        ui::draw_selection_info(node, state.dir_height_by_size);
    }
}
