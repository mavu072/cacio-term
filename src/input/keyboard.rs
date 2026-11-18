/*!
 * Read keyboard events into the input state.
 *
 * The **Keyboard** module captures a keyboard event and stores it in the input state.
 */

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::input::InputState;

pub struct Keyboard {
    // this object will be used to update the state for maintaining a "held down" state
    // - since crossterm does not do that for us
    state: InputState,
}
