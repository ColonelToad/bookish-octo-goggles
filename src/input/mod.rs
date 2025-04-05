pub mod touchscreen;
pub mod encoder;
pub mod button;
pub mod keyboard;

use touchscreen::TouchscreenInput;
use encoder::EncoderInput;
use button::ButtonInput;
use keyboard::KeyboardInput;

pub fn poll_inputs() -> Vec<InputEvent> {
    let mut events = vec![];
    events.extend(TouchscreenInput::poll());
    events.extend(EncoderInput::poll());
    events.extend(ButtonInput::poll());
    events.extend(KeyboardInput::poll());
    events
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    Touch(i32, i32),
    ButtonPressed(u8),
    EncoderTurned { id: u8, delta: i8 },
    KeyPress(char),
}