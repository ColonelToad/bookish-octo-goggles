use rppal::gpio::{Gpio, InputPin};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Hardware,
    Keyboard,
}

#[derive(Debug)]
pub enum InputEvent {
    EncoderTurn(usize, String),
    ButtonPress(usize),
    ModeChange(InputMode),
}

#[derive(Clone)]
struct EncoderState {
    last_a: bool,
    last_b: bool,
    last_change: Instant,
}

impl Default for EncoderState {
    fn default() -> Self {
        Self {
            last_a: false,
            last_b: false,
            last_change: Instant::now(),
        }
    }
}

pub struct InputHandler {
    encoders: Vec<(InputPin, InputPin)>,
    buttons: Vec<InputPin>,
    encoder_states: Vec<EncoderState>,
    button_states: Vec<bool>,
    event_tx: mpsc::SyncSender<InputEvent>,
    mode: Arc<Mutex<InputMode>>,
}

impl InputHandler {
    pub fn new() -> (Arc<Mutex<Self>>, mpsc::Receiver<InputEvent>) {
        let gpio = Gpio::new().expect("GPIO init failed");
        let (event_tx, event_rx) = mpsc::sync_channel(10);

        let encoders = vec![
            (
                gpio.get(17).unwrap().into_input_pullup(),
                gpio.get(27).unwrap().into_input_pullup(),
            ),
            (
                gpio.get(22).unwrap().into_input_pullup(),
                gpio.get(5).unwrap().into_input_pullup(),
            ),
        ];

        let buttons = vec![
            gpio.get(23).unwrap().into_input_pullup(),
            gpio.get(24).unwrap().into_input_pullup(),
            gpio.get(25).unwrap().into_input_pullup(),
        ];

        let handler = Arc::new(Mutex::new(Self {
            encoders,
            buttons,
            encoder_states: vec![EncoderState::default(); 2],
            button_states: vec![false; 3],
            event_tx,
            mode: Arc::new(Mutex::new(InputMode::Hardware)),
        }));

        let thread_handler = Arc::clone(&handler);
        thread::spawn(move || Self::polling_loop(thread_handler));

        (handler, event_rx)
    }

    fn polling_loop(handler: Arc<Mutex<Self>>) {
        loop {
            let mut locked = handler.lock().unwrap();
            locked.check_mode_change();
            locked.poll_encoders();
            locked.poll_buttons();
            drop(locked);
            thread::sleep(std::time::Duration::from_millis(5));
        }
    }

    fn check_mode_change(&mut self) {
        if self.buttons[0].is_low() && !self.button_states[0] {
            let mut mode = self.mode.lock().unwrap();
            *mode = match *mode {
                InputMode::Hardware => InputMode::Keyboard,
                InputMode::Keyboard => InputMode::Hardware,
            };
            let _ = self.event_tx.send(InputEvent::ModeChange(*mode));
            self.button_states[0] = true;
        } else if self.buttons[0].is_high() {
            self.button_states[0] = false;
        }
    }

    fn poll_encoders(&mut self) {
        if *self.mode.lock().unwrap() != InputMode::Hardware {
            return;
        }

        for (i, (pin_a, pin_b)) in self.encoders.iter().enumerate() {
            let current_a = pin_a.is_low();
            let current_b = pin_b.is_low();
            let state = &mut self.encoder_states[i];

            if state.last_change.elapsed() < std::time::Duration::from_millis(5) {
                continue;
            }

            let direction = match (state.last_a, state.last_b, current_a, current_b) {
                (false, true, true, true) => Some("RIGHT"),
                (true, false, true, true) => Some("LEFT"),
                (true, true, false, true) => Some("RIGHT"),
                (true, true, true, false) => Some("LEFT"),
                _ => None,
            };

            if let Some(dir) = direction {
                let _ = self.event_tx.send(InputEvent::EncoderTurn(i, dir.to_string()));
                state.last_change = Instant::now();
            }

            state.last_a = current_a;
            state.last_b = current_b;
        }
    }

    fn poll_buttons(&mut self) {
        if *self.mode.lock().unwrap() != InputMode::Hardware {
            return;
        }

        for (i, button) in self.buttons.iter().enumerate().skip(1) {
            let current_state = button.is_low();
            if current_state != self.button_states[i] {
                thread::sleep(std::time::Duration::from_millis(5));
                if button.is_low() == current_state {
                    let _ = self.event_tx.send(InputEvent::ButtonPress(i));
                    self.button_states[i] = current_state;
                }
            }
        }
    }
}