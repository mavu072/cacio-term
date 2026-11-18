/*!
 * Read keyboard events into the input state.
 *
 * The **Keyboard** module captures a keyboard event and stores it in the input state.
 */

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::input::InputState;

/// The **Keyboard** struct/object updates the state and maintains a "held down" state for some keys,
/// since the keyboard events don't manage that for us.
#[derive(Default)]
pub struct Keyboard {
    state: InputState,
}

impl Keyboard {
    /// Updates the state to maintain a "held down" state for some keys.
    pub fn update(&mut self, events: &[KeyEvent]) -> InputState {
        self.state.clear_pressed();

        for key in events {
            let is_down = key.kind != KeyEventKind::Release;
            let is_pressed = key.kind != KeyEventKind::Press;

            match key.code {
                KeyCode::Char('a' | 'A') => {
                    if is_pressed {
                        self.state.reset = true;
                    }
                    self.state.reset = is_down
                }
                KeyCode::Char('b' | 'B') if is_pressed => self.state.light = true,
                KeyCode::Char('c' | 'C') if is_pressed => self.state.mode = true,
                KeyCode::Char('d' | 'D') => {
                    if is_pressed {
                        self.state.start = true;
                    }
                    self.state.start = is_down
                }
                KeyCode::Esc | KeyCode::Char('q' | 'Q') if is_pressed => self.state.quit = true,
                _ => {}
            }
        }

        self.state
    }
}
