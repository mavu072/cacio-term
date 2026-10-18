mod audio;
mod datetime;
mod globals;
mod render;
mod structs;

use crate::globals::{DEFAULT_HOUR_FORMAT, WTC_BRAND, WTC_MODEL};
use crate::structs::app::App;

// use color_eyre::eyre::Ok;
use std::io;

// Entrypoint
fn main() -> io::Result<()> {
    // Create app with defaults
    let mut app = App::default();

    // Override specific fields with global watch constants
    app.set_brand(WTC_BRAND);
    app.set_model(WTC_MODEL);
    app.set_hour_format(DEFAULT_HOUR_FORMAT);

    // Run app
    ratatui::run(|terminal| app.run(terminal))
}
