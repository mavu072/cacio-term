/*!
 * An interface for managing input from any source (e.g. keyboard).
 *
 * The **Input** module is solely responsible for abstracting the input into a simple struct/object
 * so that callers don't concern themselves with the input source, and only handle the events.
 */

use crossterm::event::KeyEvent;

use crate::input::keyboard::Keyboard;

mod keyboard;

#[derive(Default)]
pub struct Input {
    // holding / pressed A
    pub reset: bool,

    // holding / pressed B
    pub light: bool,

    // holding / pressed C
    pub mode: bool,

    // holding / pressed D
    pub start: bool,

    // holding / pressed Esc or Q
    pub quit: bool,

    // link input methods
    keyboard: Keyboard,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, key_events: &[KeyEvent]) {
        // update the keyboard
        let keyboard = self.keyboard.update(key_events);

        // held down keys should remain active while the input source is held down
        self.reset = keyboard.reset;
        self.start = keyboard.start;

        // one-time keys are always reset
        self.light = keyboard.light;
        self.mode = keyboard.mode;
        self.quit = keyboard.quit;
    }
}

#[derive(Default, Clone, Copy)]
pub struct InputState {
    pub reset: bool,
    pub light: bool,
    pub mode: bool,
    pub start: bool,
    pub quit: bool,
}

impl InputState {
    // clear one-shot key presses (those not required for held down behaviour)
    fn clear_pressed(&mut self) {
        self.light = false;
        self.mode = false;
    }
}
