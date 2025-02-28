// src/input_handler.rs

//! This module handles input for the application.
//!
//! It distinguishes between keyboard input (using SDL2 events) and hardware input via GPIO.
//! Two rotary encoders (Encoder A and Encoder B) are supported, each needing three GPIO pins,
//! and four separate buttons are used for navigation.
//!
//! The design defaults to keyboard input unless a hardware signal is detected.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, InputPin};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// Represents a rotary encoder with three GPIO pins:
/// - `clk`: Clock pin
/// - `dt`: Data pin
/// - `sw`: Switch pin (for confirming selection)
pub struct Encoder {
    pub clk: InputPin,
    pub dt: InputPin,
    pub sw: InputPin,
}

impl Encoder {
    /// Creates a new encoder given its three GPIO pin numbers.
    pub fn new(clk_pin: u8, dt_pin: u8, sw_pin: u8) -> Self {
        let gpio = Gpio::new().expect("Failed to initialize GPIO");
        Encoder {
            clk: gpio.get(clk_pin).expect("Failed to get clk pin").into_input_pullup(),
            dt: gpio.get(dt_pin).expect("Failed to get dt pin").into_input_pullup(),
            sw: gpio.get(sw_pin).expect("Failed to get sw pin").into_input_pullup(),
        }
    }

    /// Polls the encoder.
    ///
    /// This function is a placeholder to demonstrate how you might detect
    /// a button press (the encoder's click) or rotation.
    pub fn poll(&self) {
        // Check if the encoder's switch is pressed.
        if self.sw.is_low() {
            println!("Encoder button pressed");
        }
        // For rotation, you would normally check for changes in the clk and dt pins.
        // Here, we simply print the current state of the pins.
        println!("Encoder: clk={}, dt={}", self.clk.is_low(), self.dt.is_low());
    }
}

/// Handles all input for the application.
pub struct InputHandler {
    /// Flag indicating whether to use keyboard input.
    pub use_keyboard: Arc<AtomicBool>,
    /// Encoder A (e.g., used for first-level selection)
    pub encoder_a: Encoder,
    /// Encoder B (e.g., used for second-level selection)
    pub encoder_b: Encoder,
    /// Four hardware buttons used for navigation.
    /// Their ordering corresponds to: Home, Apps, Profile, Settings.
    pub button_pins: [InputPin; 4],
}

impl InputHandler {
    /// Creates a new InputHandler, initializing both encoders and button pins.
    ///
    /// The pin configuration is as follows:
    /// - Encoder A: GPIO 17 (clk), GPIO 27 (dt), GPIO 22 (switch)
    /// - Encoder B: GPIO 5 (clk), GPIO 6 (dt), GPIO 13 (switch)
    /// - Buttons: GPIO 23, GPIO 24, GPIO 25, GPIO 12
    pub fn new() -> Self {
        let gpio = Gpio::new().expect("Failed to initialize GPIO");

        // Initialize Encoder A (pins: 17, 27, 22)
        let encoder_a = Encoder::new(17, 27, 22);

        // Initialize Encoder B (pins: 5, 6, 13)
        let encoder_b = Encoder::new(5, 6, 13);

        // Initialize buttons (pins: 23, 24, 25)
        let button_pins = [
            gpio.get(23).expect("Failed to get GPIO 23").into_input_pullup(),
            gpio.get(24).expect("Failed to get GPIO 24").into_input_pullup(),
            gpio.get(25).expect("Failed to get GPIO 25").into_input_pullup(),
        ];

        InputHandler {
            use_keyboard: Arc::new(AtomicBool::new(true)),
            encoder_a,
            encoder_b,
            button_pins,
        }
    }

    /// Checks for hardware activity by polling the button pins.
    ///
    /// If any hardware button is detected as pressed, this function switches
    /// the input mode to use GPIO instead of the keyboard.
    pub fn detect_input_mode(&self) {
        for pin in self.button_pins.iter() {
            if pin.is_low() {
                self.use_keyboard.store(false, Ordering::SeqCst);
                println!("Hardware input detected. Switching to GPIO mode.");
                return;
            }
        }
    }

    /// Processes input events.
    ///
    /// - For keyboard mode: processes SDL2 events.
    /// - For hardware mode: polls the GPIO pins (buttons and encoders).
    pub fn handle_input(&self, event: &Event) {
        if self.use_keyboard.load(Ordering::SeqCst) {
            if let Event::KeyDown { keycode: Some(key), .. } = event {
                match key {
                    Keycode::Num4 => println!("Keyboard: Home button pressed"),
                    Keycode::Num1 => println!("Keyboard: Apps button pressed"),
                    Keycode::Num2 => println!("Keyboard: Profile button pressed"),
                    Keycode::Num3 => println!("Keyboard: Settings button pressed"),
                    _ => {}
                }
            }
        } else {
            // Poll hardware buttons.
            for (i, pin) in self.button_pins.iter().enumerate() {
                if pin.is_low() {
                    println!("Hardware: Button {} pressed", i + 1);
                }
            }
            // Poll the encoders.
            self.encoder_a.poll();
            self.encoder_b.poll();
        }
    }

    /// A simple polling loop to demonstrate hardware input reading.
    ///
    /// In a real implementation, you might replace this with interrupt-driven callbacks.
    pub fn poll_hardware_inputs(&self) {
        loop {
            self.detect_input_mode();
            // Poll buttons.
            for (i, pin) in self.button_pins.iter().enumerate() {
                if pin.is_low() {
                    println!("Polling: Button {} pressed", i + 1);
                }
            }
            // Poll encoders.
            self.encoder_a.poll();
            self.encoder_b.poll();

            // Sleep briefly to reduce CPU usage.
            thread::sleep(Duration::from_millis(100));
        }
    }
}