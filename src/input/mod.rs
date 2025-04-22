pub mod touchscreen;
pub mod encoder;
pub mod button;
pub mod keyboard;

use sdl2::Sdl;
use sdl2::EventPump;

use crate::input::touchscreen::TouchscreenInput;
use crate::input::encoder::RotaryEncoder;
use crate::input::button::ButtonInput;
use crate::input::keyboard::KeyboardInput;


#[derive(Debug, Clone)]
pub enum InputEvent {
    Touch(i32, i32),
    ButtonPressed(&'static str),
    EncoderTurned { id: u8, delta: i8 },
    EncoderPressed(u8),
    KeyPress(char),
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
    Activate,
    Select,
}

pub struct InputManager<'a> {
    pub encoder1: RotaryEncoder,
    pub encoder2: RotaryEncoder,
    pub button: ButtonInput,
    pub keyboard: KeyboardInput,
    pub touchscreen: TouchscreenInput<'a>,
    pub dev_mode: bool,
}

impl<'a> InputManager<'a> {
    pub fn new(_sdl_context: &Sdl, event_pump: &'a mut EventPump, dev_mode: bool) -> Self {
        InputManager {
            touchscreen: TouchscreenInput::new(event_pump),
            encoder1: if dev_mode {
                RotaryEncoder::fake()
            } else {
                RotaryEncoder::new(0, 17, 27, 22)
            },
            encoder2: if dev_mode {
                RotaryEncoder::fake()
            } else {
                RotaryEncoder::new(0, 23, 24, 25)
            },
            button: if dev_mode {
                ButtonInput::fake()
            } else {
                ButtonInput::new()
            },
            keyboard: if dev_mode {
                KeyboardInput::Fake
            } else {
                KeyboardInput::new()
            },
            dev_mode,
        }
    }

    pub fn poll_inputs(&mut self) -> Vec<InputEvent> {
        let mut events = vec![];

        events.extend(self.touchscreen.poll());
        events.extend(self.encoder1.poll());
        events.extend(self.encoder2.poll());
        events.extend(self.button.poll());

        // only include keyboard input in non-dev mode
        if !self.dev_mode {
            events.extend(self.keyboard.poll());
        }

        events
    }
}
