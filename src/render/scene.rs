use super::{blocks, effects, grid, labels, search_overlay, ui};
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
        state.search.match_set(),
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

    if state.navigator.is_loading() {
        draw_loading_overlay(state.navigator.loading_elapsed_secs());
    }

    if state.search.active {
        search_overlay::draw(&state.search);
    }

    effects::draw_scanlines();
    effects::draw_vignette();
}

fn draw_loading_overlay(elapsed: f32) {
    let dots = ".".repeat(((elapsed * 3.0) as usize % 4) + 1);
    let text = format!("SCANNING{}", dots);
    let size = measure_text(&text, None, 28, 1.0);
    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;
    draw_rectangle(
        cx - size.width / 2.0 - 16.0,
        cy - size.height - 8.0,
        size.width + 32.0,
        size.height + 16.0,
        Color::new(0.0, 0.05, 0.0, 0.85),
    );
    draw_text(
        &text,
        cx - size.width / 2.0,
        cy,
        28.0,
        config::TEXT_PRIMARY,
    );
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

    if let Some(idx) = state.selected
        && let Some(node) = state.navigator.entries.get(idx)
    {
        ui::draw_selection_info(node);
    }
}
