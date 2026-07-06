// Block will only compile if the beep feature is on
#[cfg(feature = "beep_with_hz_and_millis")]
pub fn system_beep() {
    actually_beep::beep_with_hz_and_millis(2000, 100).unwrap();
}

// Fallback when running in CI without the feature
#[cfg(not(feature = "beep_with_hz_and_millis"))]
pub fn system_beep() {
    println!("Beep skipped.");
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
