mod input_handler;
mod main_menu;

use sdl2::pixels::Color;
use input_handler::{InputEvent, InputHandler};
use main_menu::MainMenu;

fn main() -> Result<(), String> {
    // Initialize SDL
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // Create window and canvas
    let window = video.window("Pi Tablet", 800, 480)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Initialize components
    let (input_handler, event_rx) = InputHandler::new();
    let mut main_menu = MainMenu::new(&texture_creator, &ttf_context)?;

    // Main loop
    let mut running = true;
    while running {
        // Handle input events
        while let Ok(event) = event_rx.try_recv() {
            match event {
                InputEvent::ModeChange(mode) => {
                    println!("Input mode changed to {:?}", mode);
                }
                InputEvent::ButtonPress(2) => running = false, // Exit on third button
                _ => main_menu.handle_input(&event),
            }
        }

        // Update and render
        main_menu.update();
        main_menu.draw(&mut canvas)?;
    }

    Ok(())
}