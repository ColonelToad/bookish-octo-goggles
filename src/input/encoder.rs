use rppal::gpio::{Gpio, InputPin, Level};
use crate::input::InputEvent;
use std::time::{Duration, Instant};

const DEBOUNCE_MS: u64 = 50;

pub enum RotaryEncoder {
    Real(RealEncoder),
    Fake,
}

pub struct RealEncoder {
    id: u8,
    pin_clk: InputPin,
    pin_dt: InputPin,
    pin_btn: InputPin,
    last_clk: bool,
    last_btn: bool,
    last_btn_time: Instant,
}

impl RotaryEncoder {
    pub fn new(id: u8, clk: u8, dt: u8, btn: u8) -> Self {
        let gpio = Gpio::new().unwrap();
        Self::Real(RealEncoder {
            id,
            pin_clk: gpio.get(clk).unwrap().into_input_pullup(),
            pin_dt: gpio.get(dt).unwrap().into_input_pullup(),
            pin_btn: gpio.get(btn).unwrap().into_input_pullup(),
            last_clk: false,
            last_btn: false,
            last_btn_time: Instant::now(),
        })
    }

    pub fn fake() -> Self {
        Self::Fake
    }

    pub fn poll(&mut self) -> Vec<InputEvent> {
        match self {
            RotaryEncoder::Real(encoder) => encoder.poll(),
            RotaryEncoder::Fake => vec![],
        }
    }
}

impl RealEncoder {
    fn poll(&mut self) -> Vec<InputEvent> {
        let mut events = Vec::new();

        let clk_now = self.pin_clk.read() == Level::Low;
        let dt_now = self.pin_dt.read() == Level::Low;

        if clk_now != self.last_clk {
            self.last_clk = clk_now;
            if clk_now {
                let delta = if dt_now { -1 } else { 1 };
                events.push(InputEvent::EncoderTurned {
                    id: self.id,
                    delta,
                });
            }
        }

        let btn_now = self.pin_btn.read() == Level::Low;
        let now = Instant::now();
        if btn_now != self.last_btn && now.duration_since(self.last_btn_time) > Duration::from_millis(DEBOUNCE_MS) {
            self.last_btn = btn_now;
            self.last_btn_time = now;

            if btn_now {
                events.push(InputEvent::EncoderPressed(self.id));
            }
        }

        events
    }
}
