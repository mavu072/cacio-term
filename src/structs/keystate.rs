/// A single key captured state.
#[derive(Debug, Default)]
pub struct KeyState {
    /// Character value of the key.
    char_value: char,
    /// Number of clicks.
    click_count: u16,
    /// Keypress duration in milliseconds.
    press_duration: u32,
}

impl KeyState {
    /// An empty constructor.
    fn new() -> KeyState {
        KeyState::default()
    }

    /// A constructor that accepts arguments.
    #[allow(dead_code)]
    fn from(char_value: char, press_duration_ms: u32, click_count: u16) -> KeyState {
        KeyState {
            char_value,
            click_count,
            press_duration: press_duration_ms,
        }
    }

    pub fn char_value(&self) -> char {
        self.char_value
    }

    pub fn set_char_value(&mut self, char_value: char) {
        self.char_value = char_value;
    }

    pub fn click_count(&self) -> u16 {
        self.click_count
    }

    pub fn set_click_count(&mut self, click_count: u16) {
        self.click_count = click_count;
    }

    pub fn press_duration(&self) -> u32 {
        self.press_duration
    }

    pub fn set_press_duration(&mut self, duration_ms: u32) {
        self.press_duration = duration_ms;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_create_new() {
        let state = KeyState::new();
        assert_eq!(state.char_value(), '\0');
        assert_eq!(state.click_count(), 0);
        assert_eq!(state.press_duration(), 0);
    }

    #[test]
    fn test_create_from() {
        let state = KeyState::from('a', 1000, 11);
        assert_eq!(state.char_value(), 'a');
        assert_eq!(state.click_count(), 11);
        assert_eq!(state.press_duration(), 1000);
    }

    #[test]
    fn test_set_char_value() {
        let mut state = KeyState::new();
        state.set_char_value('a');
        assert_eq!(state.char_value(), 'a');
    }

    #[test]
    fn test_set_press_duration() {
        let mut state = KeyState::new();
        state.set_press_duration(1000);
        assert_eq!(state.press_duration(), 1000);
    }

    #[test]
    fn test_set_click_count() {
        let mut state = KeyState::new();
        state.set_click_count(3);
        assert_eq!(state.click_count(), 3);
    }

    #[test]
    fn test_store_state() {
        let mut key_state_map: HashMap<char, KeyState> = HashMap::new();
        key_state_map.insert('a', KeyState::from('a', 1000, 11));
        key_state_map.insert('b', KeyState::from('b', 100, 3));

        for (key, state) in key_state_map {
            match key {
                'a' => {
                    assert_eq!(state.click_count(), 11);
                    assert_eq!(state.press_duration(), 1000);
                }
                'b' => {
                    assert_eq!(state.click_count(), 3);
                    assert_eq!(state.press_duration(), 100);
                }
                _ => assert!(false),
            }
        }
    }
}
