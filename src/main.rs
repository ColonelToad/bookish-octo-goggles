mod input;
mod launcher;
mod apps;
mod ui;

use input::InputManager;
use std::thread::sleep;
use std::time::Duration;
use ui::state::UIScreen;
use launcher::LaunchableApp;

fn main() {
    let dev_mode = false;
    let sdl_context = sdl2::init().expect("Failed to init SDL");
    let ttf_context = sdl2::ttf::init().expect("Failed to init SDL_ttf");
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    let mut input_manager = InputManager::new(&sdl_context, &mut event_pump, dev_mode);
    let mut screen = UIScreen::Welcome;
    let mut selected_index: i32 = 0;

    'running: loop {
        for input in input_manager.poll_inputs() {
            match screen {
                UIScreen::Welcome => match input {
                    input::InputEvent::Touch(x, _) => {
                        screen = if x < 266 {
                            UIScreen::MainMenu(0)
                        } else if x < 533 {
                            UIScreen::MainMenu(1)
                        } else {
                            UIScreen::MainMenu(2)
                        };
                    }
                    input::InputEvent::ButtonPressed(label) => {
                        screen = match label {
                            "APPS" => UIScreen::MainMenu(0),
                            "PROFILE" => UIScreen::MainMenu(1),
                            "SETTINGS" => UIScreen::MainMenu(2),
                            _ => UIScreen::Welcome,
                        };
                    }
                    input::InputEvent::Select => {
                        screen = UIScreen::MainMenu(0);
                    }
                    _ => {}
                },

                UIScreen::MainMenu(section) => match input {
                    input::InputEvent::NavigateUp => {
                        selected_index = (selected_index - 1).max(0);
                    }
                    input::InputEvent::NavigateDown => {
                        selected_index = (selected_index + 1).min(4);
                    }
                    input::InputEvent::Select => {
                        // Handle app launch based on section and selection
                        match (section, selected_index) {
                            (0, 0) => launcher::launch_app(LaunchableApp::AudioPlayer),
                            (0, 1) => launcher::launch_app(LaunchableApp::Calendar),
                            (0, 2) => launcher::launch_app(LaunchableApp::Terminal),
                            (0, 3) => launcher::launch_app(LaunchableApp::FileExplorer),
                            (0, _) => screen = UIScreen::Welcome,

                            (1, 0) => launcher::launch_app(LaunchableApp::Maps),
                            (1, 1) => launcher::launch_app(LaunchableApp::Notes),
                            (1, 2) => launcher::launch_app(LaunchableApp::TodoList),
                            (1, 3) => launcher::launch_app(LaunchableApp::TextEditor),
                            (1, _) => screen = UIScreen::Welcome,

                            (2, 0) => launcher::launch_app(LaunchableApp::Writer(None)),
                            (2, 1) => launcher::launch_app(LaunchableApp::Calc(None)),
                            (2, 2) => launcher::launch_app(LaunchableApp::Impress(None)),
                            (2, 3) => launcher::launch_app(LaunchableApp::IDE),
                            (2, _) => screen = UIScreen::Welcome,

                            _ => {}
                        }
                    }
                    input::InputEvent::ButtonPressed(_) => {
                        screen = UIScreen::Welcome;
                    }
                    _ => {}
                },
            }
        }

        // C++ UI rendering bridge
        // REMOVE C++ UI rendering bridge
        match screen {
            _ => {}
        }        

        sleep(Duration::from_millis(16));
    }
}