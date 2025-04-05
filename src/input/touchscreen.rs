use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use crate::input::InputEvent;

pub struct TouchscreenInput;

impl TouchscreenInput {
    pub fn poll() -> Vec<InputEvent> {
        let mut events = vec![];
        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            if let Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } = event { 
                events.push(InputEvent::Touch(x, y));
            }
        }
        events
    }
}