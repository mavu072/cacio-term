use actually_beep::beep_with_hz_and_millis;

pub fn system_beep() {
    beep_with_hz_and_millis(2000, 100).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_beep() {
        system_beep();
        assert!(true);
    }
}
