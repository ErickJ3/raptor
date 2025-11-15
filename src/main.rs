use macroquad::prelude::*;

mod app;
mod camera;
mod config;
mod filesystem;
mod input;
mod math;
mod preview;
mod render;

use app::AppState;
use input::KeyboardHandler;
use render::render_frame;

fn window_conf() -> Conf {
    Conf {
        window_title: config::WINDOW_TITLE.to_string(),
        window_width: config::WINDOW_WIDTH,
        window_height: config::WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = AppState::new();

    loop {
        let commands = KeyboardHandler::poll_commands();
        for command in commands {
            state.execute_command(command);
        }

        state.update();

        render_frame(&state);

        use crate::preview::render::{PreviewLayout, render_preview};
        render_preview(&state.preview.content, PreviewLayout::default());

        next_frame().await
    }
}
