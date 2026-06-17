mod datetime;
mod globals;
mod interface;

use crate::datetime::local_datetime;
use crate::globals::{DEFAULT_HOUR_FORMAT, WTC_BRAND, WTC_MODEL};
use crate::interface::render;
use color_eyre::eyre::Ok;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use std::time::Duration;

struct AppState {
    brand: String,
    model: String,
    datetime: String,
    hour_format: i8,
    light_on: bool,
    light_timer: i8,
    exit: bool,
}

fn main() -> color_eyre::Result<()> {
    // Overring the default handlers for crashes and runtime errors - prints a highly formatted error
    color_eyre::install()?;

    // Run app
    ratatui::run(app)?;

    Ok(()) // Returns '()' so no semicolon
}

fn app(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    let mut state = AppState {
        brand: String::from(WTC_BRAND),
        model: String::from(WTC_MODEL),
        datetime: String::new(),
        hour_format: DEFAULT_HOUR_FORMAT,
        light_on: false,
        light_timer: 0,
        exit: false,
    };

    while state.exit == false {
        // Calc
        state.datetime = local_datetime(state.hour_format);

        // Run background tasks concurrently
        run_background_tasks(&mut state);

        // Render tui - pass frame and state into render dynamically
        terminal.draw(|frame| render(frame, &state))?;

        // Handle events
        handle_events(&mut state)?;
    }

    Ok(())
}

fn handle_events(state: &mut AppState) -> color_eyre::Result<()> {
    // Poll the terminal input stream for 100 milliseconds.
    // If no keys are pressed within 100ms, it skips this block cleanly and exits early.
    if !event::poll(Duration::from_millis(100))? {
        return Ok(());
    }

    // Flatten the Event matching. If it's not a Key event, exit early
    let Event::Key(key) = event::read()? else {
        return Ok(());
    };

    // Filter release/repeat events -> Allow keypress
    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    // Handlers
    match key.code {
        KeyCode::Char('a') => {} // todo
        KeyCode::Char('m') => {} // todo
        KeyCode::Char('h') => toggle_hour_format(state),
        KeyCode::Char('l') => switch_on_light(state),
        KeyCode::Char('q') => state.exit = true,
        _ => {} // Left empty so random key slips don't instantly close your app,
    }

    Ok(())
}

fn toggle_hour_format(state: &mut AppState) {
    if state.hour_format == 24 {
        state.hour_format = 12;
    } else {
        state.hour_format = 24;
    }
}

fn switch_on_light(state: &mut AppState) {
    state.light_on = true;
    state.light_timer = 12; // Light stays on for 12 loops ~ 1sec
}

fn run_background_tasks(state: &mut AppState) {
    if state.light_on {
        if state.light_timer > 0 {
            state.light_timer -= 1;
        } else {
            state.light_on = false; // Switch off once timer hits zero
        }
    }
}
