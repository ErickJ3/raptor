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
        config::LABEL_FONT_SIZE,
        config::TEXT_WARNING,
    );

    draw_text(
        "MOUSE: Right-drag rotate | Scroll zoom | Click select | Double-click enter",
        20.0,
        panel_y + 40.0,
        config::LABEL_FONT_SIZE,
        Color::new(0.0, 0.83, 1.0, 0.7),
    );

    let mut status = format!(
        "Labels: {} | Hidden: {}",
        if labels_enabled { "ON" } else { "OFF" },
        if show_hidden { "ON" } else { "OFF" },
    );

    if config::STATUS_FPS {
        let fps = format!("FPS: {} | ", get_fps());
        status.insert_str(0, &fps);
    }

    draw_text(
        &status,
        20.0,
        panel_y + 60.0,
        config::INFO_FONT_SIZE,
        Color::new(0.0, 1.0, 0.25, 0.5),
    );
}
