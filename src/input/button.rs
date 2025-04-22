use crate::input::InputEvent;
use rppal::gpio::{Gpio, InputPin, Level};
use std::time::{Duration, Instant};

const BUTTON_GPIO: u8 = 26;
const DEBOUNCE_MS: u64 = 50;
const STATES: [&str; 3] = ["APPS", "PROFILE", "SETTINGS"];

pub enum ButtonInput {
    Real {
        pin: InputPin,
        last_state: bool,
        last_time: Instant,
        current_index: usize,
    },
    Fake,
}

impl ButtonInput {
    pub fn new() -> Self {
        let gpio = Gpio::new().expect("Failed to access GPIO");
        let pin = gpio.get(BUTTON_GPIO).expect("Invalid pin").into_input_pullup();

        ButtonInput::Real {
            pin,
            last_state: false,
            last_time: Instant::now(),
            current_index: 0,
        }
    }

    pub fn fake() -> Self {
        ButtonInput::Fake
    }

    pub fn poll(&mut self) -> Vec<InputEvent> {
        match self {
            ButtonInput::Real {
                pin,
                last_state,
                last_time,
                current_index,
            } => {
                let mut events = Vec::new();
                let now = Instant::now();

                let is_pressed = pin.read() == Level::Low;

                if is_pressed != *last_state
                    && now.duration_since(*last_time) > Duration::from_millis(DEBOUNCE_MS)
                {
                    *last_state = is_pressed;
                    *last_time = now;

                    if is_pressed {
                        *current_index = (*current_index + 1) % STATES.len();
                        events.push(InputEvent::ButtonPressed(STATES[*current_index]));
                    }
                }

                events
            }
            ButtonInput::Fake => vec![], // Dev mode: no button input
        }
    }
}