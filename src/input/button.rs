use crate::input::InputEvent;
use std::cell::RefCell;

thread_local! {
    static BUTTON_INDEX: RefCell<u8> = RefCell::new(0);
}

pub struct ButtonInput;

impl ButtonInput {
    pub fn poll() -> Vec<InputEvent> {
        BUTTON_INDEX.with(|idx| {
            let mut i = idx.borrow_mut();
            let button_id = *i % 3;
            *i += 1;
            vec![InputEvent::ButtonPressed(button_id)]
        })
    }
}