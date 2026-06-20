mod datetime;
mod globals;

// Locals
use crate::datetime::local_datetime;
use crate::globals::{DEFAULT_HOUR_FORMAT, WTC_BRAND, WTC_MODEL};

// Crates
// use color_eyre::eyre::Ok;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Span, ToSpan},
    widgets::{Block, Paragraph, Widget},
};
use std::{io, time::Duration};

#[derive(Debug, Default)]
pub struct App {
    brand: String,
    model: String,
    day: String,
    year: String,
    date_month: String,
    clock: String,
    hour_format: i8,
    light_on: bool,
    light_timer: i8,
    exit: bool,
}

fn main() -> io::Result<()> {
    // Create app with defaults
    let mut app = App::default();

    // Override specific fields with your global watch constants
    app.brand = String::from(WTC_BRAND);
    app.model = String::from(WTC_MODEL);
    app.hour_format = DEFAULT_HOUR_FORMAT;

    // Run app
    ratatui::run(|terminal| app.run(terminal))
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.exit == false {
            // Run background tasks concurrently
            self.run_background_tasks();

            // Render tui - pass frame and state into render dynamically
            terminal.draw(|frame| self.draw(frame))?;

            // Handle events
            self.handle_events()?;
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
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
            KeyCode::Char('a') => self.adjust_clock(),
            KeyCode::Char('m') | KeyCode::Tab => self.switch_watch_modes(),
            KeyCode::Char('h') => self.toggle_hour_format(),
            KeyCode::Char('l') | KeyCode::Backspace | KeyCode::Char(' ') => self.light_on(),
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            _ => {} // Left empty so random key slips don't instantly close your app,
        }

        Ok(())
    }

    fn switch_watch_modes(&mut self) {
        todo!()
    }

    fn adjust_clock(&mut self) {
        todo!()
    }

    fn update_live_clock(&mut self) {
        // Get current time
        let datetime = local_datetime(self.hour_format);

        // Live clock functions
        self.clock = datetime.0;
        self.day = datetime.1;
        self.year = datetime.2;
        self.date_month = datetime.3;
    }

    fn toggle_hour_format(&mut self) {
        if self.hour_format == 24 {
            self.hour_format = 12;
        } else {
            self.hour_format = 24;
        }
    }

    fn light_on(&mut self) {
        self.light_on = true;
        self.light_timer = 12; // Light stays on for 12 loops ~ 1sec
    }

    fn light_off(&mut self) {
        if self.light_on {
            if self.light_timer > 0 {
                self.light_timer -= 1;
            } else {
                self.light_on = false; // Switch off once timer hits zero
            }
        }
    }

    fn run_background_tasks(&mut self) {
        // Update live clock
        self.update_live_clock();

        // Handle light switch
        self.light_off();
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Alarm Func
        let alarm_functions = "SNZ      ALM     SIG";

        // Get light on/off color
        let (fg_col, bg_col) = if self.light_on {
            (Color::Rgb(0, 0, 0), Color::Rgb(80, 158, 49))
        } else {
            (Color::Rgb(0, 0, 0), Color::Rgb(43, 84, 27))
        };

        // Header
        let header = Line::from(
            [
                Span::raw(" "),
                self.brand.to_span(),
                Span::raw(" "),
                self.model.to_span().gray().bold(),
                Span::raw(" "),
            ]
            .as_ref(),
        );

        // Commands
        let commands = Line::from(
            [
                " Adjust ".into(),
                "<A>".gray().bold(),
                " Mode ".into(),
                "<M>".gray().bold(),
                " Light ".into(),
                "<L/Space>".gray().bold(),
                " 12/24H ".into(),
                "<H>".gray().bold(),
                " Quit ".into(),
                "<Q/Esc> ".gray().bold(),
            ]
            .as_ref(),
        );

        // Header Block
        let header_block = Block::bordered()
            .title(header.centered())
            .border_set(border::ROUNDED);

        // Commands Block
        let commands_block = Block::bordered()
            .title_bottom(commands.centered())
            .border_set(border::PLAIN);

        // Outer
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(area);

        // Inner first
        let inner_first_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(outer_layout[0]);

        // Inner last
        let inner_last_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(outer_layout[2]);

        // Day Display
        Paragraph::new(self.day.to_span())
            .centered()
            .bold()
            .fg(fg_col)
            .bg(bg_col)
            .block(header_block)
            .render(inner_first_layout[0], buf);

        // Alarm Functions Display
        Paragraph::new(alarm_functions)
            .centered()
            .bold()
            .fg(fg_col)
            .bg(bg_col)
            .block(Block::bordered().border_set(border::EMPTY))
            .render(inner_first_layout[1], buf);

        // Datetime Display
        Paragraph::new(self.clock.to_span())
            .centered()
            .bold()
            .fg(fg_col)
            .bg(bg_col)
            .block(commands_block)
            .render(outer_layout[1], buf);

        // Year Display
        Paragraph::new(self.year.to_span())
            .centered()
            .bold()
            .fg(fg_col)
            .bg(bg_col)
            .block(Block::bordered().border_set(border::PLAIN))
            .render(inner_last_layout[0], buf);

        // Date/Month Display
        Paragraph::new(self.date_month.to_span())
            .centered()
            .bold()
            .fg(fg_col)
            .bg(bg_col)
            .block(Block::bordered().border_set(border::PLAIN))
            .render(inner_last_layout[1], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Import Ratatui's virtual test simulator dependencies
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn test_render_layout() {
        // Initialize app using Default trait
        let mut app = App::default();

        // Override defaults with manual values
        app.brand = "cacio".to_string();
        app.model = "term".to_string();
        app.clock = "12:00:00".to_string();
        app.day = "Monday".to_string();
        app.year = "2026".to_string();
        app.date_month = "6-19".to_string();
        app.hour_format = 12;
        app.light_timer = 3;

        // Debug log from Debug trait: run test with "cargo test -- --nocapture"
        println!("{:?}", app);

        // Spawn a virtual terminal in memory with a mock 50x5 grid viewport
        let backend = TestBackend::new(50, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        // Draw using your standard closure execution flow
        terminal
            .draw(|frame| {
                app.draw(frame);
            })
            .unwrap();

        // Extract the text matrix to inspect rendering frames
        let buffer = terminal.backend().buffer();
        let string_representation = format!("{buffer:?}");

        assert!(string_representation.contains("cacio"));
        assert!(string_representation.contains("term"));
        assert!(string_representation.contains("Monday"));
        assert!(string_representation.contains("12:00:00"));
        assert!(string_representation.contains("Mode"));
        assert!(string_representation.contains("Light"));
        assert!(string_representation.contains("Quit"));
    }
}
