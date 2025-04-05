// src/ui/welcome.rs

use crate::ui::state::UIScreen;

pub fn handle_welcome_input(input: InputEvent) -> UIScreen {
    match input {
        InputEvent::Touch(x, y) => {
            // Simple bounding box check for bottom option tap
            if y > 390 && y < 450 {
                if x < 200 {
                    return UIScreen::MainMenu(0); // APPS
                } else if x < 400 {
                    return UIScreen::MainMenu(1); // PROFILE
                } else {
                    return UIScreen::MainMenu(2); // SETTINGS
                }
            }
            UIScreen::Welcome
        }
        InputEvent::Button(index) => match index {
            0 => UIScreen::MainMenu(0), // APPS
            1 => UIScreen::MainMenu(1), // PROFILE
            2 => UIScreen::MainMenu(2), // SETTINGS
            _ => UIScreen::Welcome,
        },
        _ => UIScreen::Welcome,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Touch(i32, i32),
    Button(u8),
    None,
}
