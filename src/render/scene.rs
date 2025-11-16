use super::{blocks, effects, grid, labels, ui};
use crate::app::AppState;
use crate::config;
use macroquad::prelude::*;

pub fn render_frame(state: &AppState) {
    clear_background(config::BACKGROUND_COLOR);

    let camera = state.camera.to_camera3d();

    set_camera(&camera);

    grid::draw_grid_floor();
    blocks::draw_all_blocks(
        &state.navigator.entries,
        state.selected,
        state.mouse.hover_index,
    );
    state.scan_effect.draw();

    set_default_camera();

    if state.show_labels {
        labels::draw_labels(
            &state.navigator.entries,
            state.selected,
            state.mouse.hover_index,
            &camera,
        );
    }

    render_ui(state);

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

    ui::draw_status_bar(state.show_labels, state.show_hidden);

    if let Some(idx) = state.selected {
        if let Some(node) = state.navigator.entries.get(idx) {
            ui::draw_selection_info(node);
        }
    }
}
