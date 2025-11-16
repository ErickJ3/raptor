use crate::config;
use macroquad::prelude::*;

pub fn draw_status_bar(labels_enabled: bool, show_hidden: bool) {
    let panel_y = screen_height() - config::FOOTER_HEIGHT;

    draw_rectangle(
        0.0,
        panel_y,
        screen_width(),
        config::FOOTER_HEIGHT,
        config::UI_PANEL_COLOR,
    );

    draw_text(
        "NAV: h j k l  |  o/ENTER: Open  |  .: Hidden  |  u/-: Parent  |  /: Root  |  TAB: Labels",
        20.0,
        panel_y + 20.0,
        14.0,
        config::TEXT_WARNING,
    );

    draw_text(
        "MOUSE: Right-drag rotate | Scroll zoom | Click select | Double-click enter",
        20.0,
        panel_y + 40.0,
        12.0,
        Color::new(0.0, 0.83, 1.0, 0.7),
    );

    let status = format!(
        "FPS: {} | Labels: {} | Hidden: {}",
        get_fps(),
        if labels_enabled { "ON" } else { "OFF" },
        if show_hidden { "ON" } else { "OFF" },
    );

    draw_text(
        &status,
        20.0,
        panel_y + 60.0,
        14.0,
        Color::new(0.0, 1.0, 0.25, 0.5),
    );
}
