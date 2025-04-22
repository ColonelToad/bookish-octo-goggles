use sdl2::{event::Event, mouse::MouseButton};
use sdl2::EventPump;
use crate::input::InputEvent;

pub struct TouchscreenInput<'a> {
    event_pump: &'a mut EventPump,
}

impl<'a> TouchscreenInput<'a> {
    pub fn new(event_pump: &'a mut EventPump) -> Self {
        Self { event_pump }
    }

    pub fn poll(&mut self) -> Vec<InputEvent> {
        let mut events = Vec::new();
        for event in self.event_pump.poll_iter() {
            if let Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } = event {
                events.push(InputEvent::Touch(x, y));
            }
        }

        events
    }
}