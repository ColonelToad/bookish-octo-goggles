// src/main.rs
mod ui;

// Import these instead of creating them as mods
// Later you can create these files as needed
mod apps;
mod input;
mod launcher;

use rusb::{Context, UsbContext};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sysinfo::{System, SystemExt};
use ui::global_renderer::GlobalRenderer;
use ui::main_menu::handle_main_menu_input;
use ui::main_menu::InputEvent as MenuInput;
use ui::state::UIScreen;
use ui::welcome::handle_welcome_input;
use ui::welcome::InputEvent as WelcomeInput;

// Function to check if a USB flash drive (holotape) is connected
fn flash_drive_connected() -> bool {
    match Context::new() {
        Ok(context) => {
            if let Ok(devices) = context.devices() {
                for device in devices.iter() {
                    if let Ok(config_desc) = device.active_config_descriptor() {
                        for interface in config_desc.interfaces() {
                            for interface_desc in interface.descriptors() {
                                if interface_desc.class_code() == 0x08 {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            false
        }
        Err(_) => false,
    }
}

fn get_usb_drive_path() -> Option<String> {
    let system = System::new_all();
    for disk in system.disks() {
        //if disk.name().to_string_lossy().contains("SanDisk Corp. Cruzer Glide") {
        // Convert Windows path (e.g., "E:\") to WSL path (e.g., "/mnt/e/")
        //let mount_point = disk.mount_point().to_string_lossy();
        //if mount_point.starts_with("D:\\") {
        //eprintln!("USB drive found: {}", mount_point);
        return Some("/dev/bus/usb/001".to_string());

        // }
        // else {
        //     eprintln!("USB drive found: {}", mount_point);
        //     return Some(mount_point.to_string())
        // }
        //}
    }
    None
}


fn open_holotape() {
    if let Some(usb_drive_path) = get_usb_drive_path() {
        eprintln!("Opening USB drive at: {}", usb_drive_path);
        if let Err(e) = std::process::Command::new("explorer.exe")
            .arg(usb_drive_path)
            .spawn()
        {
            eprintln!("Failed to open file explorer: {}", e);
        }
    } else {
        eprintln!("No USB drive detected. Please insert a USB drive.");
    }
}

fn main() {
    // Initialize SDL2 contexts
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Create window and canvas
    let window = video_subsystem
        .window("PipBoy", 800, 480)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let font = ttf_context.load_font("assets/font.ttf", 24).unwrap();

    // Initialize UI state
    let mut screen = UIScreen::Welcome;

    // Create a Global Renderer for all screens
    let mut renderer = GlobalRenderer {
        canvas,
        texture_creator: &texture_creator,
        font,
        show_popup: false,
        popup_selection: None,
    };

    // USB notification settings
    let mut notification_timer = 0.0;
    let notification_duration = 3.0;

    // Track if we've already shown the notification for this session
    let mut shown_usb_notification = false;

    // Last checked USB state
    let mut last_usb_state = false;

    'running: loop {
        // Process events first
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    if renderer.show_popup {
                        // Close popup with Enter key
                        renderer.popup_selection = Some("Yes".to_string());
                        renderer.show_popup = false;
                        notification_timer = 0.0; // Reset timer
                    } else {
                        screen = match screen {
                            UIScreen::MainMenu(sel) => {
                                handle_main_menu_input(sel, MenuInput::Select)
                            }
                            UIScreen::Welcome => handle_welcome_input(WelcomeInput::Button(0)),
                        };
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if !renderer.show_popup {
                        if let UIScreen::MainMenu(sel) = screen {
                            screen = handle_main_menu_input(sel, MenuInput::Up);
                        }
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if !renderer.show_popup {
                        if let UIScreen::MainMenu(sel) = screen {
                            screen = handle_main_menu_input(sel, MenuInput::Down);
                        }
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    if renderer.show_popup {
                        // Check if clicking on yes/no buttons - adjusted to more realistic coordinates
                        if y >= 30 && y <= 280 {
                            // Button height area
                            if x >= 400 && x <= 500 {
                                // Yes button area
                                renderer.popup_selection = Some("Yes".to_string());
                                renderer.show_popup = false;
                                notification_timer = 0.0; // Reset timer
                                                          // Handle yes action here
                                open_holotape(); // Placeholder for actual function
                                                 //this function should open file explorer or similar
                            } else if x >= 510 && x <= 600 {
                                // No button area
                                renderer.popup_selection = Some("No".to_string());
                                renderer.show_popup = false;
                                notification_timer = 0.0; // Reset timer
                            }
                        }
                    } else {
                        screen = match screen {
                            UIScreen::Welcome => handle_welcome_input(WelcomeInput::Touch(x, y)),
                            UIScreen::MainMenu(sel) => {
                                handle_main_menu_input(sel, MenuInput::Click(x, y))
                            }
                            // Handle other screens...
                        };
                    }
                }

                _ => {}
            }
        }

        // Check for USB drive (holotape) connection
        let current_usb_state = flash_drive_connected();

        // Only show notification when USB state changes from false to true
        if current_usb_state && !last_usb_state && !shown_usb_notification {
            renderer.show_popup = true;
            renderer.popup_selection = Some("Holotape Detected!".to_string());
            notification_timer = notification_duration;
            shown_usb_notification = true; // Mark that we've shown the notification
        } else if !current_usb_state {
            // Reset notification state when USB is removed
            shown_usb_notification = false;
        }

        last_usb_state = current_usb_state;

        // Update notification timer
        if notification_timer > 0.0 {
            notification_timer -= 0.016;
            if notification_timer <= 0.0 {
                renderer.show_popup = false;
            }
        }

        // Render current screen
        renderer.render(&screen);

        // Frame timing
        std::thread::sleep(Duration::from_millis(16));
    }
}
