// src/ui/main_menu.rs
use crate::ui::state::UIScreen;

pub fn handle_main_menu_input(selected: usize, input: InputEvent) -> UIScreen {
    match input {
        InputEvent::Up => UIScreen::MainMenu(selected.saturating_sub(1)),
        InputEvent::Down => UIScreen::MainMenu((selected + 1) % 5),
        InputEvent::Click(x, y) => {
            // Calculate which menu item was clicked based on coordinates
            // This is a simplified example - you'll need to adjust based on your UI layout
            let item_height = 50; // Approximate height of each menu item
            let start_y = 100;    // Approximate Y position where menu starts
            
            if x >= 100 && x <= 700 { // Check if click is within menu width
                let index = (y - start_y) / item_height;
                if index < 5 { // Number of menu items
                    // If valid menu item was clicked, select it
                    if index as usize == selected {
                        // If clicking on already selected item, treat as Select
                        return handle_main_menu_input(selected, InputEvent::Select);
                    } else {
                        // Otherwise just select the item
                        return UIScreen::MainMenu(index as usize);
                    }
                }
            }
            
            UIScreen::MainMenu(selected) // Default: no change
        },
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
    Click(i32, i32), // Added Click variant with x, y coordinates
    None,
}