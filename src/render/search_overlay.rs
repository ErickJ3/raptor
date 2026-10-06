use crate::config;
use crate::search::SearchState;
use macroquad::prelude::*;

pub fn draw(search: &SearchState) {
    let prompt = format!("> {}_", search.query);
    let count_str = if search.query.is_empty() {
        String::from("type to search")
    } else {
        format!("{} match(es)", search.matches.len())
    };

    let prompt_size = measure_text(&prompt, None, 22, 1.0);
    let count_size = measure_text(&count_str, None, 14, 1.0);

    let pad = 16.0;
    let box_w = prompt_size.width.max(count_size.width) + pad * 2.0;
    let box_h = prompt_size.height + count_size.height + pad * 2.0 + 6.0;
    let x = (screen_width() - box_w) / 2.0;
    let y = 70.0;

    draw_rectangle(x, y, box_w, box_h, Color::new(0.0, 0.05, 0.0, 0.92));
    draw_rectangle_lines(x, y, box_w, box_h, 2.0, config::TEXT_PRIMARY);

    draw_text(
        &prompt,
        x + pad,
        y + pad + prompt_size.height,
        22.0,
        config::TEXT_PRIMARY,
    );
    draw_text(
        &count_str,
        x + pad,
        y + pad + prompt_size.height + 6.0 + count_size.height,
        14.0,
        config::TEXT_SECONDARY,
    );
}
