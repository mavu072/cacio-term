use ratatui::style::Color;

/// Get foreground and background for LCD depending on light on/off.
pub fn get_lcd_colors(light_on: bool) -> (Color, Color) {
    let (fg, bg) = if light_on {
        get_lcd_on_colors()
    } else {
        get_lcd_off_colors()
    };

    (fg, bg)
}

/// Gets the foreground and background to emulate an LCD with a backlight ON.
///
/// Returns `(fg, bg)`.
fn get_lcd_on_colors() -> (Color, Color) {
    (Color::Rgb(0, 0, 0), Color::Rgb(80, 158, 49))
}

/// Gets the foreground and background to emulate an LCD with a backlight OFF.
///
/// Returns `(fg, bg)`.
fn get_lcd_off_colors() -> (Color, Color) {
    (Color::Rgb(0, 0, 0), Color::Rgb(43, 84, 27))
}
