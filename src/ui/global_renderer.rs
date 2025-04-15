use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::ttf::Font;
use sdl2::pixels::Color;
use crate::ui::state::UIScreen;

pub struct GlobalRenderer<'a> {
    pub canvas: Canvas<Window>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub font: Font<'a, 'static>,
    pub show_popup: bool,
    pub popup_selection: Option<String>,
}

impl<'a> GlobalRenderer<'a> {
    pub fn render(&mut self, screen: &UIScreen) {
        self.canvas.set_draw_color(Color::RGB(0, 30, 0));
        self.canvas.clear();
        
        // Render the current screen
        match screen {
            UIScreen::Welcome => self.render_welcome(),
            UIScreen::MainMenu(selected) => self.render_main_menu(*selected),
        }
        
        // Render notification popup if active
        if self.show_popup {
            self.render_notification();
        }
        
        self.canvas.present();
    }
    
    pub fn render_notification(&mut self) {
        if let Some(message) = &self.popup_selection {
            let popup_x = 150;
            let popup_y = 30;
            let popup_width: u32 = 500;
            let popup_height: u32 = 100;
    
            // Shadow background
            self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 128));
            self.canvas
                .fill_rect(Rect::new(
                    popup_x + 4, // shadow offset
                    popup_y + 4, // shadow offset
                    popup_width,
                    popup_height,
                ))
                .unwrap();
    
            // Popup background
            self.canvas.set_draw_color(Color::RGBA(30, 30, 30, 220));
            self.canvas
                .fill_rect(Rect::new(
                    popup_x,
                    popup_y,
                    popup_width,
                    popup_height,
                ))
                .unwrap();
    
            // Text rendering
            let surface = self.font.render(message).blended(Color::WHITE).unwrap();
            let texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();
    
            let text_x = 175; //popup_x + ((popup_width as i32 - surface.width() as i32) / 2);
            let text_y = popup_y + ((popup_height as i32 - surface.height() as i32) / 2);
    
            let target = Rect::new(text_x, text_y, surface.width(), surface.height());
            self.canvas.copy(&texture, None, Some(target)).unwrap();
            
            // Draw Open/Dismiss buttons
            if message.contains("Holotape Detected!") {
                // Open button
                let btn_width: i32 = 100;
                let btn_height: i32 = 40;
                let open_x = 400;
                let dismiss_x = 510;
                let btn_y = 60;
                
                self.canvas.set_draw_color(Color::RGB(0, 100, 0));
                self.canvas.fill_rect(Rect::new(open_x, btn_y, btn_width as u32, btn_height as u32)).unwrap();
                self.canvas.set_draw_color(Color::GREEN);
                self.canvas.draw_rect(Rect::new(open_x, btn_y, btn_width as u32, btn_height as u32)).unwrap();
                
                // Dismiss button
                self.canvas.set_draw_color(Color::RGB(100, 0, 0));
                self.canvas.fill_rect(Rect::new(dismiss_x, btn_y, btn_width as u32, btn_height as u32)).unwrap();
                self.canvas.set_draw_color(Color::RGB(255, 0, 0));
                self.canvas.draw_rect(Rect::new(dismiss_x, btn_y, btn_width as u32, btn_height as u32)).unwrap();
                
                // Button text
                let open_surface = self.font.render("Open").blended(Color::WHITE).unwrap();
                let open_texture = self.texture_creator.create_texture_from_surface(&open_surface).unwrap();
                let open_text_x = open_x + (btn_width - open_surface.width() as i32) / 2;
                let open_text_y = btn_y + (btn_height - open_surface.height() as i32) / 2;
                self.canvas.copy(&open_texture, None, Some(Rect::new(
                    open_text_x, open_text_y, open_surface.width(), open_surface.height()
                ))).unwrap();
                
                let dismiss_surface = self.font.render("Dismiss").blended(Color::WHITE).unwrap();
                let dismiss_texture = self.texture_creator.create_texture_from_surface(&dismiss_surface).unwrap();
                let dismiss_text_x = dismiss_x + (btn_width - dismiss_surface.width() as i32) / 2;
                let dismiss_text_y = btn_y + (btn_height - dismiss_surface.height() as i32) / 2;
                self.canvas.copy(&dismiss_texture, None, Some(Rect::new(
                    dismiss_text_x, dismiss_text_y, dismiss_surface.width(), dismiss_surface.height()
                ))).unwrap();
            }
        }
    }

    fn render_welcome(&mut self) {
        let texture = self.texture_creator.load_texture("assets/sit.png").unwrap();
        let query = texture.query();
        let (original_width, original_height) = (query.width, query.height);
        // Define the resizing percentage (e.g., 0.8 for 80%)
        let scale_factor: f32 = 0.8;
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
        let surface = self.font.render("WELCOME, USER!").blended(Color::GREEN).unwrap();
        let texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();
        let text_width = surface.width() as i32;
        let target = Rect::new((800 - text_width) / 2, 50, surface.width(), surface.height());
        self.canvas.copy(&texture, None, Some(target)).unwrap();
        // Bottom options: APPS, PROFILE, SETTINGS
        let labels = ["APPS", "PROFILE", "SETTINGS"];
        let spacing = 800 / labels.len() as i32;
        let y = 440;
        let box_height = 40;
        let box_width = spacing;
        for (i, label) in labels.iter().enumerate() {
            let x = i as i32 * spacing;
            let surface = self.font.render(label).blended(Color::GREEN).unwrap();
            let texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();
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
    
    fn render_main_menu(&mut self, selected: usize) {
        let options = ["Calendar", "Media", "Gallery", "Terminal", "IDE"];
        for (i, option) in options.iter().enumerate() {
            let y = 100 + (i * 50) as i32;
            let surface = self.font.render(option).blended(
                if i == selected { Color::WHITE } else { Color::GREEN }
            ).unwrap();
            let texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();
            let target = Rect::new(100, y, surface.width(), surface.height());
            self.canvas.copy(&texture, None, Some(target)).unwrap();
        }
    }
}