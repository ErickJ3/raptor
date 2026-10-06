use super::commands::Command;
use macroquad::prelude::*;

pub struct KeyboardHandler;

impl KeyboardHandler {
    pub fn poll_commands(search_active: bool, pending_g: &mut bool) -> Vec<Command> {
        let mut commands = vec![];

        if search_active {
            while let Some(c) = get_char_pressed() {
                if c == '\u{1b}' || c == '\u{8}' || c == '\r' || c == '\n' || c == '\t' {
                    continue;
                }
                if !c.is_control() {
                    commands.push(Command::SearchAppend(c));
                }
            }
            if is_key_pressed(KeyCode::Backspace) {
                commands.push(Command::SearchBackspace);
            }
            if is_key_pressed(KeyCode::Enter) {
                commands.push(Command::SearchCommit);
            }
            if is_key_pressed(KeyCode::Escape) {
                commands.push(Command::SearchCancel);
            }
            return commands;
        }

        if *pending_g {
            if is_key_pressed(KeyCode::R) {
                commands.push(Command::GoToRoot);
                *pending_g = false;
                return commands;
            }
            if is_key_pressed(KeyCode::G) {
                commands.push(Command::GoToFirst);
                *pending_g = false;
                return commands;
            }
            if is_key_pressed(KeyCode::Escape) {
                *pending_g = false;
                return commands;
            }
            if any_navigation_key_pressed() {
                *pending_g = false;
            } else {
                return commands;
            }
        }

        if is_key_pressed(KeyCode::G)
            && !is_key_down(KeyCode::LeftShift)
            && !is_key_down(KeyCode::RightShift)
        {
            *pending_g = true;
            return commands;
        }

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
            commands.push(Command::StartSearch);
            let _ = get_char_pressed();
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

        if is_key_pressed(KeyCode::F) {
            commands.push(Command::RevealInFileManager);
        }

        commands
    }
}

fn any_navigation_key_pressed() -> bool {
    [
        KeyCode::H,
        KeyCode::J,
        KeyCode::K,
        KeyCode::L,
        KeyCode::Left,
        KeyCode::Right,
        KeyCode::Up,
        KeyCode::Down,
        KeyCode::Enter,
        KeyCode::O,
        KeyCode::U,
        KeyCode::Minus,
        KeyCode::Slash,
        KeyCode::Home,
        KeyCode::Backspace,
        KeyCode::Period,
        KeyCode::Tab,
        KeyCode::F,
    ]
    .iter()
    .any(|k| is_key_pressed(*k))
}
