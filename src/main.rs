use macroquad::prelude::*;

mod app;
mod camera;
mod config;
mod filesystem;
mod input;
mod math;
mod render;
mod search;

use app::AppState;
use input::KeyboardHandler;
use render::render_frame;

fn window_conf() -> Conf {
    Conf {
        window_title: config::WINDOW_TITLE.to_string(),
        window_width: config::WINDOW_WIDTH,
        window_height: config::WINDOW_HEIGHT,
        fullscreen: false,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = AppState::new();

    loop {
        let commands =
            KeyboardHandler::poll_commands(state.search.active, &mut state.pending_g);
        for command in commands {
            state.execute_command(command);
        }

        state.update();

        render_frame(&state);
        next_frame().await
    }
}
