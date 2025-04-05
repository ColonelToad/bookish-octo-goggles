use gif::{ColorOutput, Decoder};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 480;
const TAB_NAMES: [&str; 3] = ["APPS", "SETTINGS", "PROFILE"];
const OPTIONS_PER_TAB: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq)]
enum MenuState {
    TabSelection,
    OptionSelection(usize),
}

pub struct MainMenu<'a> {
    frames: Vec<Texture<'a>>,
    frame_durations: Vec<u16>,
    current_frame: usize,
    start_time: Instant,
    font: Font<'a, 'static>,
    state: MenuState,
    selected_tab: usize,
    selected_option: usize,
}

impl<'a> MainMenu<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext, // Added explicit lifetime
    ) -> Result<Self, String> {
        let (frames, frame_durations) = Self::load_gif(texture_creator, "./assets/walk.gif")?;
        let font = ttf_context.load_font(Path::new("./assets/font.ttf"), 24)?;

        Ok(Self {
            frames,
            frame_durations,
            current_frame: 0,
            start_time: Instant::now(),
            font,
            state: MenuState::TabSelection,
            selected_tab: 0,
            selected_option: 0,
        })
    }

    fn load_gif(
        texture_creator: &'a TextureCreator<WindowContext>,
        path: &str,
    ) -> Result<(Vec<Texture<'a>>, Vec<u16>), String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mut decoder = Decoder::new(BufReader::new(file)).map_err(|e| e.to_string())?;
        decoder.set_output_color(ColorOutput::RGBA);
        let mut reader = decoder.read_info().map_err(|e| e.to_string())?;

        let mut frames = Vec::new();
        let mut frame_durations = Vec::new();

        while let Some(frame) = reader.read_next_frame().map_err(|e| e.to_string())? {
            let surface = sdl2::surface::Surface::from_data(
                &frame.buffer,
                frame.width.into(),
                frame.height.into(),
                frame.width as u32 * 4,
                sdl2::pixels::PixelFormatEnum::RGBA32,
            )
            .map_err(|e| e.to_string())?;

            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            frames.push(texture);
            frame_durations.push(frame.delay);
        }

        Ok((frames, frame_durations))
    }

    pub fn handle_input(&mut self, event: &super::input_handler::InputEvent) {
        match (self.state, event) {
            (MenuState::TabSelection, super::input_handler::InputEvent::EncoderTurn(0, dir)) => {
                self.selected_tab = match dir.as_str() {
                    "RIGHT" => (self.selected_tab + 1) % 3,
                    "LEFT" => (self.selected_tab + 2) % 3,
                    _ => self.selected_tab,
                };
            }
            (
                MenuState::OptionSelection(_),
                super::input_handler::InputEvent::EncoderTurn(1, dir),
            ) => {
                self.selected_option = match dir.as_str() {
                    "RIGHT" => (self.selected_option + 1) % OPTIONS_PER_TAB,
                    "LEFT" => (self.selected_option + OPTIONS_PER_TAB - 1) % OPTIONS_PER_TAB,
                    _ => self.selected_option,
                };
            }
            (MenuState::TabSelection, super::input_handler::InputEvent::ButtonPress(0)) => {
                self.state = MenuState::OptionSelection(self.selected_tab);
            }
            (MenuState::OptionSelection(_), super::input_handler::InputEvent::ButtonPress(0)) => {
                self.state = MenuState::TabSelection;
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        let elapsed = self.start_time.elapsed().as_millis() as u16;
        let total_time: u16 = self.frame_durations.iter().map(|d| d * 10).sum();
        let cycle_time = elapsed % total_time;

        let mut accumulated = 0;
        for (i, &duration) in self.frame_durations.iter().enumerate() {
            accumulated += duration * 10;
            if cycle_time < accumulated {
                self.current_frame = i;
                break;
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Draw animated background
        canvas.copy(
            &self.frames[self.current_frame],
            None,
            Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT),
        )?;

        // Draw tabs
        let tab_width = SCREEN_WIDTH / 3;
        for (i, name) in TAB_NAMES.iter().enumerate() {
            let color = if i == self.selected_tab {
                Color::RGB(255, 0, 0)
            } else {
                Color::RGB(255, 255, 255)
            };

            let surface = self
                .font
                .render(name)
                .solid(color)
                .map_err(|e| e.to_string())?;

            let texture_creator = canvas.texture_creator();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let query = texture.query();
            let x = (i as u32 * tab_width) + (tab_width - query.width) / 2;
            let y = SCREEN_HEIGHT - 50;

            canvas.copy(
                &texture,
                None,
                Rect::new(x as i32, y as i32, query.width, query.height),
            )?;
        }

        // Draw options if in submenu
        if let MenuState::OptionSelection(tab) = self.state {
            let options = match tab {
                0 => ["Music Player", "Calendar", "Camera", "Files", "Back"],
                1 => ["Wi-Fi", "Bluetooth", "Display", "Sound", "Back"],
                2 => ["User Info", "Theme", "Security", "About", "Back"],
                _ => [""; 5],
            };

            for (i, option) in options.iter().enumerate() {
                let color = if i == self.selected_option {
                    Color::RGB(255, 0, 0)
                } else {
                    Color::RGB(255, 255, 255)
                };

                let surface = self
                    .font
                    .render(option)
                    .solid(color)
                    .map_err(|e| e.to_string())?;

                let texture = canvas
                    .texture_creator()
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;

                let query = texture.query();
                let x = (SCREEN_WIDTH - query.width) / 2;
                let y = 100 + (i as u32 * 40);

                canvas.copy(
                    &texture,
                    None,
                    Rect::new(x as i32, y as i32, query.width, query.height),
                )?;
            }
        }

        canvas.present();
        Ok(())
    }
}

