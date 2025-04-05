// src/apps/mainmenu_renderer.rs

// This is the file that actually calls the render pipeline to render the mainmenu you see upon
// launching the pip-boy, or go to the main menu proper after exiting an app.

// use GlobalRenderer so that we can call the main_menu render function
use crate::global_renderer::GlobalRenderer;
// not sure what this does lmao
use crate::ui::state::UIScreen;

// to actually render the screen, we probably need this to be here
pub fn render(&mut self, screen: &UIScreen) {
    // setting the background color to be really dark green
    self.canvas.set_draw_color(Color::RGB(0, 30, 0));
    // after that clearing the rest of it for artifacts
    self.canvas.clear();

    // actually rendering the screen
    match screen {
        UIScreen::Welcome => {
            // rendering the logo with the buttons on the bottom
            self.render_welcome_screen("assets/sit.png", 0.8, &["APPS", "PROFILE", "SETTINGS"])
        }

        // rendering what you see after you click enter
        UIScreen::MainMenu(selected) => self.render_main_menu(
            *selected,
            &["Calendar", "Media", "Gallery", "Terminal", "IDE"],
        ),
    }

    // pushing it to the screen
    self.canvas.present();
}
