// src/ui/main_menu.rs

use crate::ui::state::UIScreen;

pub fn handle_main_menu_input(selected: usize, input: InputEvent) -> UIScreen {
    match input {
        InputEvent::Up => UIScreen::MainMenu(selected.saturating_sub(1)),
        InputEvent::Down => UIScreen::MainMenu((selected + 1) % 5),
        InputEvent::Select => {
            match selected {
                0 => UIScreen::Welcome, // Calendar placeholder
                1 => UIScreen::Welcome, // Media placeholder
                2 => UIScreen::Welcome, // Gallery placeholder
                3 => UIScreen::Welcome, // Terminal placeholder
                4 => UIScreen::Welcome, // IDE placeholder
                _ => UIScreen::MainMenu(0),
            }
        },
        _ => UIScreen::MainMenu(selected),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Up,
    Down,
    Select,
    None,
}