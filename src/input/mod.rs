/*!
 * An interface for managing input from any source (e.g. keyboard).
 *
 * The **Input** module is solely responsible for abstracting the input into a simple struct/object
 * so that callers don't concern themselves with the input source, and only handle the events.
 */

use crate::input::keyboard::Keyboard;

mod keyboard;

pub struct Input {
    // holding / pressed A
    pub reset: bool,

    // holding / pressed B
    pub light: bool,

    // holding / pressed C
    pub mode: bool,

    // holding / pressed D
    pub start: bool,

    // holding / pressed Esc
    pub quit: bool,

    // link input methods
    keyboard: Keyboard,
}

pub struct InputState {
    pub reset: bool,
    pub light: bool,
    pub mode: bool,
    pub start: bool,
    pub quit: bool,
}
