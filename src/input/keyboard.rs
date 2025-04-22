use crate::input::InputEvent;
use evdev::{Device, InputEventKind, Key};
use std::fs::read_dir;

pub enum KeyboardInput {
    Real(Device),
    Fake,
}

impl KeyboardInput {
    pub fn new() -> Self {
        let Ok(entries) = read_dir("/dev/input") else {
            eprintln!("Could not read /dev/input");
            return KeyboardInput::Fake;
        };

        for entry in entries.flatten() {
            let path = entry.path();

            let is_event = path.file_name()
                .and_then(|n| n.to_str())
                .map_or(false, |s| s.starts_with("event"));

            if !is_event {
                continue;
            }

            if let Ok(file) = std::fs::File::open(&path) {
                if let Ok(device) = Device::open(&path) {
                    if device.supported_keys().map_or(false, |keys| {
                        keys.contains(Key::KEY_ENTER) && keys.contains(Key::KEY_A)
                    }) {
                        return KeyboardInput::Real(device);
                    }
                }
                drop(file);
            }
        }

        eprintln!("⚠️  No suitable keyboard input device found.");
        KeyboardInput::Fake
    }

    pub fn fake() -> Self {
        KeyboardInput::Fake
    }

    pub fn poll(&mut self) -> Vec<InputEvent> {
        match self {
            KeyboardInput::Real(device) => {
                let mut events = Vec::new();

                if let Ok(ev_list) = device.fetch_events() {
                    for ev in ev_list {
                        if let InputEventKind::Key(key) = ev.kind() {
                            if ev.value() == 1 {
                                match key {
                                    Key::KEY_UP => events.push(InputEvent::NavigateUp),
                                    Key::KEY_DOWN => events.push(InputEvent::NavigateDown),
                                    Key::KEY_LEFT => events.push(InputEvent::NavigateLeft),
                                    Key::KEY_RIGHT => events.push(InputEvent::NavigateRight),
                                    Key::KEY_SPACE => events.push(InputEvent::Activate),
                                    Key::KEY_ENTER => events.push(InputEvent::Select),
                                    Key::KEY_A => events.push(InputEvent::KeyPress('a')),
                                    Key::KEY_B => events.push(InputEvent::KeyPress('b')),
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                events
            }

            KeyboardInput::Fake => {
                use std::io::{self, Read};
                let mut buf = [0; 1];
                let stdin = io::stdin();
                if let Ok(n) = stdin.lock().read(&mut buf) {
                    if n > 0 {
                        match buf[0] as char {
                            '1' => vec![InputEvent::ButtonPressed("APPS")],
                            '2' => vec![InputEvent::ButtonPressed("PROFILE")],
                            '3' => vec![InputEvent::ButtonPressed("SETTINGS")],
                            '\n' | '\r' => vec![InputEvent::Select],
                            _ => vec![],
                        }
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
        }
    }
}