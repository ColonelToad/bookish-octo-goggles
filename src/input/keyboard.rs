use crate::input::InputEvent;

pub struct KeyboardInput;

impl KeyboardInput {
    pub fn poll() -> Vec<InputEvent> {
        // Simulate test: 'a' key pressed
        vec![InputEvent::KeyPress('a')]
    }
}
