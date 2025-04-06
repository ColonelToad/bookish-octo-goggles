use crate::ui::state::UIScreen;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct MainMenu<'a> {
    canvas: Canvas<Window>,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: sdl2::ttf::Font<'a, 'static>,
}

impl<'a> MainMenu<'a> {
    pub fn new(canvas: Canvas<Window>, ttf_context: &'a sdl2::ttf::Sdl2TtfContext) -> Self {
        let texture_creator = canvas.texture_creator();

        // Load a font for rendering text - adjust path as needed for your project
        let font = ttf_context
            .load_font("assets/font.ttf", 16)
            .expect("Could not load font");

        MainMenu {
            canvas,
            texture_creator,
            font,
        }
    }

    pub fn render(&mut self, screen: &UIScreen) {
        // Setting the background color to be really dark green
        self.canvas.set_draw_color(Color::RGB(0, 30, 0));
        // Clearing the screen to remove artifacts
        self.canvas.clear();

        // Actually rendering the screen
        match screen {
            UIScreen::Welcome => {
                // Rendering the logo with the buttons on the bottom
                self.render_welcome_screen("assets/sit.png", 0.8, &["APPS", "PROFILE", "SETTINGS"])
            }

            // Rendering what you see after you click enter
            UIScreen::MainMenu(selected) => self.render_main_menu(
                *selected,
                &["Calendar", "Media", "Gallery", "Terminal", "IDE"],
            ),
        }

        // Pushing it to the screen
        self.canvas.present();
    }

    fn render_welcome_screen(&mut self, image_path: &str, scale: f32, buttons: &[&str]) {
        // Load the logo image
        let texture = self
            .texture_creator
            .load_texture(image_path)
            .expect("Failed to load logo image");

        // Get dimensions
        let query = texture.query();
        let width = (query.width as f32 * scale) as u32;
        let height = (query.height as f32 * scale) as u32;

        // Center the image
        let (canvas_width, canvas_height) = self.canvas.output_size().unwrap();
        let dest_rect = Rect::new(
            ((canvas_width - width) / 2) as i32,
            ((canvas_height - height) / 3) as i32, // Position at top third
            width,
            height,
        );

        // Render the logo
        self.canvas
            .copy(&texture, None, Some(dest_rect))
            .expect("Failed to render logo");

        // Render buttons at the bottom
        let button_height = 40;
        let button_padding = 20;
        let total_button_width = buttons.len() as u32 * 120; // Assuming 120px per button
        let start_x = (canvas_width - total_button_width) / 2;

        for (i, &button_text) in buttons.iter().enumerate() {
            let button_rect = Rect::new(
                (start_x + i as u32 * 120) as i32,
                (canvas_height - button_height - button_padding) as i32,
                120,
                button_height,
            );

            // Draw button background
            self.canvas.set_draw_color(Color::RGB(0, 60, 0));
            self.canvas
                .fill_rect(button_rect)
                .expect("Failed to render button background");

            // Render button text
            let surface = self
                .font
                .render(button_text)
                .blended(Color::RGB(0, 255, 0))
                .expect("Could not render text");

            let text_texture = self
                .texture_creator
                .create_texture_from_surface(&surface)
                .expect("Could not create texture from surface");

            let text_query = text_texture.query();
            let text_rect = Rect::new(
                button_rect.x() + ((button_rect.width() - text_query.width) / 2) as i32,
                button_rect.y() + ((button_rect.height() - text_query.height) / 2) as i32,
                text_query.width,
                text_query.height,
            );

            self.canvas
                .copy(&text_texture, None, Some(text_rect))
                .expect("Failed to render button text");
        }
    }

    fn render_main_menu(&mut self, selected: usize, options: &[&str]) {
        // Get canvas dimensions
        let (canvas_width, canvas_height) = self.canvas.output_size().unwrap();

        // Render header
        let header_text = "PIP-BOY MAIN MENU";
        let surface = self
            .font
            .render(header_text)
            .blended(Color::RGB(0, 255, 0))
            .expect("Could not render header text");

        let header_texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .expect("Could not create texture from surface");

        let header_query = header_texture.query();
        let header_rect = Rect::new(
            (canvas_width - header_query.width) as i32 / 2,
            50,
            header_query.width,
            header_query.height,
        );

        self.canvas
            .copy(&header_texture, None, Some(header_rect))
            .expect("Failed to render header");

        // Render menu options
        let item_height = 40;
        let item_spacing = 10;
        let total_height = options.len() as u32 * (item_height + item_spacing);
        let start_y = (canvas_height - total_height) / 2;

        for (i, &option_text) in options.iter().enumerate() {
            let is_selected = i == selected;

            // Set color based on selection
            let text_color = if is_selected {
                Color::RGB(255, 255, 0) // Yellow for selected
            } else {
                Color::RGB(0, 200, 0) // Green for unselected
            };

            // Draw item background if selected
            let item_rect = Rect::new(
                (canvas_width / 4) as i32,
                (start_y + i as u32 * (item_height + item_spacing)) as i32,
                canvas_width / 2,
                item_height,
            );

            if is_selected {
                self.canvas.set_draw_color(Color::RGB(0, 60, 0));
                self.canvas
                    .fill_rect(item_rect)
                    .expect("Failed to render item background");
            }

            // Render option text
            let surface = self
                .font
                .render(option_text)
                .blended(text_color)
                .expect("Could not render option text");

            let text_texture = self
                .texture_creator
                .create_texture_from_surface(&surface)
                .expect("Could not create texture from surface");

            let text_query = text_texture.query();
            let text_rect = Rect::new(
                (canvas_width / 2 - text_query.width / 2) as i32,
                item_rect.y() + ((item_rect.height() - text_query.height) / 2) as i32,
                text_query.width,
                text_query.height,
            );

            self.canvas
                .copy(&text_texture, None, Some(text_rect))
                .expect("Failed to render option text");

            // Draw selected indicator
            if is_selected {
                let indicator_rect =
                    Rect::new((canvas_width / 4 - 20) as i32, text_rect.y(), 10, 10);
                self.canvas.set_draw_color(Color::RGB(255, 255, 0));
                self.canvas
                    .fill_rect(indicator_rect)
                    .expect("Failed to render selection indicator");
            }
        }
    }
}
