mod input_handler;
use input_handler::InputHandler;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureQuery};
use sdl2::ttf::Font;
use std::time::{Duration, Instant};
use std::path::Path;
use sdl2::image::LoadTexture;


const SCREEN_WIDTH: u32 = 300;
const SCREEN_HEIGHT: u32 = 350;

// Animation settings
const FRAME_COUNT: i32 = 7; // number of frames in the animation
const FRAME_WIDTH: u32 = 300;
const FRAME_HEIGHT: u32 = 350;

// The tab names corresponding to keys 0-3 and the scrolling text strings.
const TAB_NAMES: [&str; 4] = ["MAIN", "APPS", "SETTINGS", "PROFILE"];
const SCROLL_TEXTS: [&str; 4] = ["Main Menu", "Applications", "Settings", "Profile"];

#[derive(Clone, Copy)]
enum Tab {
    Main,
    Apps,
    Stg, // Settings
    Prof,
}

impl Tab {
    fn from_index(index: i32) -> Self {
        match index {
            0 => Tab::Main,
            1 => Tab::Apps,
            2 => Tab::Stg,
            3 => Tab::Prof,
            _ => Tab::Main,
        }
    }

    fn as_index(self) -> i32 {
        match self {
            Tab::Main => 0,
            Tab::Apps => 1,
            Tab::Stg => 2,
            Tab::Prof => 3,
        }
    }

    fn scroll_text(self) -> &'static str {
        SCROLL_TEXTS[self.as_index() as usize]
    }
}

fn create_text_texture<'a>(
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: &Font,
    text: &str,
    color: Color,
) -> Result<(Texture<'a>, TextureQuery), String> {
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let query = texture.query();
    Ok((texture, query))
}

fn main() -> Result<(), String> {
    // Initialize SDL2 and TTF
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // Create the window
    let window = video_subsystem
        .window("Pip-Boy Menu", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // Set draw color for clearing the screen
    canvas.set_draw_color(Color::BLACK);

    let texture_creator = canvas.texture_creator();

    // Load font
    let font_path = Path::new("./assets/fixedsys-excelsior-301.ttf");
    let font = ttf_context.load_font(font_path, 20)?;

    // Pre-render the top banner tab labels.
    let mut tab_textures: Vec<(Texture, Rect)> = Vec::new();
    for (i, &name) in TAB_NAMES.iter().enumerate() {
        let (texture, query) =
            create_text_texture(&texture_creator, &font, name, Color::RGB(0, 255, 0))?;
        // Position: x = i * (SCREEN_WIDTH/5) + 10, y = 10
        let x = (i as u32 * (SCREEN_WIDTH / 5)) + 10;
        let rect = Rect::new(x as i32, 10, query.width, query.height);
        tab_textures.push((texture, rect));
    }

    // Scrolling text at bottom
    let mut current_tab = Tab::Main;
    let mut scroll_text = current_tab.scroll_text().to_string();
    let mut scroll_x = SCREEN_WIDTH as i32; // start off-screen to the right
    let scroll_y = (SCREEN_HEIGHT as i32) - 40;
    let (mut scroll_texture, mut scroll_query) =
        create_text_texture(&texture_creator, &font, &scroll_text, Color::RGB(0, 255, 0))?;

    // Load the spritesheet for the walking animation.
    let sprite_path = Path::new("./assets/spritesheet.png");
    let sprite_sheet = texture_creator.load_texture(sprite_path)?;
    let TextureQuery { width, height, .. } = sprite_sheet.query();
    println!("Loaded spritesheet: {} x {}", width, height);

    // Scale the sprite to fit between the top labels and scrolling text.
    let available_width = (SCREEN_WIDTH as f32) - 20.0;
    let available_height = (SCREEN_HEIGHT as f32) - 100.0;
    let scale_factor_width = available_width / (FRAME_WIDTH as f32);
    let scale_factor_height = available_height / (FRAME_HEIGHT as f32);
    let scale_factor = scale_factor_width.min(scale_factor_height);

    // Animation timing setup.
    let mut last_frame_time = Instant::now();
    let frame_duration = Duration::from_millis(200); // change frame every 0.2 seconds
    let mut current_frame: i32 = 0;

    // Initialize our input handler to process both keyboard and hardware input.
    let input_handler = InputHandler::new();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // First, check if any hardware input is active.
        input_handler.detect_input_mode();

        // Process SDL2 events.
        for event in event_pump.poll_iter() {
            // Let the input handler process the event.
            input_handler.handle_input(&event);

            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    // For keyboard input, update the current tab.
                    current_tab = match key {
                        Keycode::Num0 => Tab::Main,
                        Keycode::Num1 => Tab::Apps,
                        Keycode::Num2 => Tab::Stg,
                        Keycode::Num3 => Tab::Prof,
                        _ => current_tab,
                    };
                    // Update the scrolling text to reflect the new tab.
                    scroll_text = current_tab.scroll_text().to_string();
                    match create_text_texture(&texture_creator, &font, &scroll_text, Color::RGB(0, 255, 0)) {
                        Ok((texture, query)) => {
                            scroll_texture = texture;
                            scroll_query = query;
                        }
                        Err(e) => return Err(e),
                    }
                    // Reset scroll position.
                    scroll_x = SCREEN_WIDTH as i32;
                }
                _ => {}
            }
        }

        // Update scrolling text: move left by 2 pixels each frame.
        scroll_x -= 2;
        if scroll_x < -(scroll_query.width as i32) {
            scroll_x = SCREEN_WIDTH as i32;
        }

        // Update animation only if we're on the MAIN tab.
        if let Tab::Main = current_tab {
            if last_frame_time.elapsed() >= frame_duration {
                current_frame = (current_frame + 1) % FRAME_COUNT;
                last_frame_time = Instant::now();
            }
        }

        // Clear the canvas.
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Draw the top banner.
        for (texture, rect) in &tab_textures {
            canvas.copy(texture, None, *rect)?;
        }

        // If on the MAIN tab, draw the walking animation.
        if let Tab::Main = current_tab {
            // Define the source rectangle for the current frame.
            let src_rect = Rect::new(
                current_frame * (width as i32 / FRAME_COUNT as i32),
                0,
                width / FRAME_COUNT as u32,
                height,
            );
            // Compute the destination rectangle: center horizontally and position at y = 50.
            let dest_width = (FRAME_WIDTH as f32 * scale_factor) as u32;
            let dest_height = (FRAME_HEIGHT as f32 * scale_factor) as u32;
            let dest_x = ((SCREEN_WIDTH - dest_width) / 2) as i32;
            let dest_y = 50;
            let dest_rect = Rect::new(dest_x, dest_y, dest_width, dest_height);
            canvas.copy(&sprite_sheet, src_rect, dest_rect)?;
        }

        // Draw the scrolling text at the bottom.
        let scroll_rect = Rect::new(scroll_x, scroll_y, scroll_query.width, scroll_query.height);
        canvas.copy(&scroll_texture, None, scroll_rect)?;

        // Present the canvas.
        canvas.present();

        // Delay to roughly cap the frame rate.
        ::std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}