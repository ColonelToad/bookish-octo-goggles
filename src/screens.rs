use sd12 :: render :: {Canvas, Texture, TextureCreator};
use sd12 :: tff :: Font;
use sd12 :: video :: Window;
use sd12 :: rect :: Rect;
use sd12 :: pixels :: Color;
use sd12 :: render :: TextureQuery;

//Import main
use crate :: main :: {SCREEN_WIDTH, SCREEN_HEIGHT, Tab, create_text_texture};

const TabNames: [&str; 4] = ["MAIN", "APPS", "SETTINGS", "PROFILES"]; 

pub struct Screen <'a> {

    //Currently active tab
    pub active_tab: Tab,

    // Vector of tab textures and screen bounding recttangles
    pub tab_textures: Vec<(Texture<'a>, Rect)>,

    // The scrolling text to display related to rge current tab.
    pub display_scroll_text: String,
}

impl <'a> Screen <'a> {

    //Create a new screen with default tab and empty textures
    pub fn new() -> Self {
        Self {
            active_tab: Tab::Main,
            tab_textures: Vec![],
            display_scroll_text: Tab ::Main.scroll_text().to_string(),
            scroll_active: false,
        }
    }

    //Checks if the touch at (x,y) falls under one of the tab rectangles
    pub fn touch_handler (&mut self, x: i32, y: i32) -> bool {
        for (i, &(_, ref rect)) in self.tab_textures.iter().enumerate() {
            if rect.contains_point((x, y)) {
                self.active_tab = match i {
                    0 => Tab::Main,
                    1 => Tab::Apps,
                    2 => Tab::Settings,
                    3 => Tab::Profiles,
                    _ => self.current_tab,
                };

                self.display_scroll_text = self.active_tab.scroll_text().to_string();

                //Disable scroll if a new tab is selected
                self.scroll_active = false;
                return true;
            }
        }
        false
    }

    pub fn scroll_touch_handler(&mut self, start_x: i32,  end_x: i32) -> bool {
        let swipe_min = 20;
        if(end_x - start_x).abs() >= swipe_min {
            self.scroll_active = true;
            return true;
        }
        false
    }
}
   
        