use super::commands::Command;
use macroquad::prelude::*;

pub struct KeyboardHandler;

impl KeyboardHandler {
    pub fn poll_commands() -> Vec<Command> {
        let mut commands = vec![];

        // Vim-style navigation
        if is_key_pressed(KeyCode::H) {
            commands.push(Command::MoveLeft);
        }
        if is_key_pressed(KeyCode::J) {
            commands.push(Command::MoveDown);
        }
        if is_key_pressed(KeyCode::K) {
            commands.push(Command::MoveUp);
        }
        if is_key_pressed(KeyCode::L) {
            commands.push(Command::MoveRight);
        }

        // Arrow keys
        if is_key_pressed(KeyCode::Left) {
            commands.push(Command::MoveLeft);
        }
        if is_key_pressed(KeyCode::Right) {
            commands.push(Command::MoveRight);
        }
        if is_key_pressed(KeyCode::Up) {
            commands.push(Command::MoveUp);
        }
        if is_key_pressed(KeyCode::Down) {
            commands.push(Command::MoveDown);
        }

        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::O) {
            commands.push(Command::EnterDirectory);
        }
        if is_key_pressed(KeyCode::Backspace) || is_key_pressed(KeyCode::Escape) {
            commands.push(Command::GoBack);
        }
        if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::U) {
            commands.push(Command::GoToParent);
        }
        if is_key_pressed(KeyCode::Slash) {
            commands.push(Command::GoToRoot);
        }
        if is_key_pressed(KeyCode::Home) {
            commands.push(Command::GoHome);
        }

        if is_key_pressed(KeyCode::Key0) {
            commands.push(Command::GoToFirst);
        }
        if is_key_pressed(KeyCode::G)
            && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
        {
            commands.push(Command::GoToLast);
        }

        if is_key_pressed(KeyCode::Period) {
            commands.push(Command::ToggleHidden);
        }

        if is_key_pressed(KeyCode::Tab) {
            commands.push(Command::ToggleLabels);
        }

        commands
    }
}
