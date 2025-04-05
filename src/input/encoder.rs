use crate::input::InputEvent;
use std::cell::RefCell;

thread_local! {
    static ENCODER_POSITION: RefCell<i32> = RefCell::new(0);
}

pub struct EncoderInput;

impl EncoderInput {
    pub fn poll() -> Vec<InputEvent> {
        // Simulate test: alternate delta direction
        ENCODER_POSITION.with(|pos| {
            let mut p = pos.borrow_mut();
            *p += 1;
            let delta = if *p % 2 == 0 { 1 } else { -1 };
            vec![InputEvent::EncoderTurned { id: 0, delta }]
        })
    }
}