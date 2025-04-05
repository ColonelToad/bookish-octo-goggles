// src/launcher.rs

use std::process::Command;

pub struct Launcher;

impl Launcher {
    pub fn open_file_explorer() {
        if let Err(e) = Command::new("pcmanfm").spawn() {
            eprintln!("Failed to launch File Explorer: {}", e);
        }
    }

    pub fn open_terminal() {
        if let Err(e) = Command::new("lxterminal").spawn() {
            eprintln!("Failed to launch Terminal: {}", e);
        }
    }

    pub fn open_ide() {
        if let Err(e) = Command::new("thonny").spawn() {
            eprintln!("Failed to launch Thonny IDE: {}", e);
        }
    }
}

// Integration with Input Manager
pub enum LaunchableApp {
    FileExplorer,
    Terminal,
    IDE,
}

pub fn launch_app(app: LaunchableApp) {
    match app {
        LaunchableApp::FileExplorer => Launcher::open_file_explorer(),
        LaunchableApp::Terminal => Launcher::open_terminal(),
        LaunchableApp::IDE => Launcher::open_ide(),
    }
}
