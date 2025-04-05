use crate::ui::state::UIScreen;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::video::WindowContext;

pub struct GlobalRenderer<'a> {
    pub canvas: Canvas<Window>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub font: Font<'a, 'static>,
}

impl<'a> GlobalRenderer<'a> {
    fn render_welcome_screen(&mut self, logo_path: &str, scale_factor: f32, labels: &[&str]) {
        let texture = self.texture_creator.load_texture(logo_path).unwrap();
        let query = texture.query();
        let (original_width, original_height) = (query.width, query.height);

        // Calculate the new width and height
        let width = (original_width as f32 * scale_factor) as u32;
        let height = (original_height as f32 * scale_factor) as u32;

        let dst = Rect::new(
            400 - (width as i32 / 2),
            240 - (height as i32 / 2),
            width,
            height,
        );
        self.canvas.copy(&texture, None, Some(dst)).unwrap();

        let surface = self
            .font
            .render("WELCOME, USER!")
            .blended(Color::GREEN)
            .unwrap();
        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let text_width = surface.width() as i32;
        let target = Rect::new(
            (800 - text_width) / 2,
            50,
            surface.width(),
            surface.height(),
        );
        self.canvas.copy(&texture, None, Some(target)).unwrap();

        // Bottom options: APPS, PROFILE, SETTINGS
        let spacing = 800 / labels.len() as i32;
        let y = 440;
        let box_height = 40;
        let box_width = spacing;

        for (i, label) in labels.iter().enumerate() {
            let x = i as i32 * spacing;
            let surface = self.font.render(label).blended(Color::GREEN).unwrap();
            let texture = self
                .texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();

            let text_width = surface.width();
            let text_height = surface.height();
            let text_x = x + (box_width - text_width as i32) / 2;
            let text_y = y + (box_height - text_height as i32) / 2;

            // Draw bounding box
            let rect = Rect::new(x, y, box_width as u32, box_height as u32);
            self.canvas.set_draw_color(Color::RGB(0, 100, 0));
            self.canvas.fill_rect(rect).unwrap();
            self.canvas.set_draw_color(Color::GREEN);
            self.canvas.draw_rect(rect).unwrap();

            // Draw label centered in the box
            let target = Rect::new(text_x, text_y, surface.width(), surface.height());
            self.canvas.copy(&texture, None, Some(target)).unwrap();
        }
    }

    fn render_main_menu(&mut self, selected: usize, options: &[&str]) {
        for (i, option) in options.iter().enumerate() {
            let y = 100 + (i * 50) as i32;
            let surface = self
                .font
                .render(option)
                .blended(if i == selected {
                    Color::WHITE
                } else {
                    Color::GREEN
                })
                .unwrap();
            let texture = self
                .texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            let target = Rect::new(100, y, surface.width(), surface.height());
            self.canvas.copy(&texture, None, Some(target)).unwrap();
        }
    }
}
