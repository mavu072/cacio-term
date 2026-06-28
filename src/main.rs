mod audio;
mod datetime;
mod globals;
mod watchmodes;

// Locals
use crate::datetime::local_datetime;
use crate::globals::{DEFAULT_HOUR_FORMAT, WTC_BRAND, WTC_MODEL};
use crate::watchmodes::alarm::AlarmMode;

// Crates
// use color_eyre::eyre::Ok;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use std::{io, time::Duration};

// Entrypoint
fn main() -> io::Result<()> {
    // Create app with defaults
    let mut app = App::default();

    // Override specific fields with global watch constants
    app.brand = String::from(WTC_BRAND);
    app.model = String::from(WTC_MODEL);
    app.hour_format = DEFAULT_HOUR_FORMAT;

    // Run app
    ratatui::run(|terminal| app.run(terminal))
}

#[derive(Debug, Default)]
enum WatchMode {
    #[default]
    Timekeeping,
    Alarm,
    Stopwatch,
    DualTime,
}

impl WatchMode {
    fn as_str(&self) -> &'static str {
        match self {
            WatchMode::Timekeeping => "TM",
            WatchMode::Alarm => "AL",
            WatchMode::Stopwatch => "ST",
            WatchMode::DualTime => "DT",
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    brand: String,
    model: String,
    active_mode: WatchMode,
    day: String,
    year: String,
    date_month: String,
    clock: String,
    hour_format: i8,
    light_on: bool,
    light_timer: i8,
    exit: bool,
    alarm: AlarmMode,
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

    fn header(&self) -> Line<'static> {
        // Returns the program header using a cheap borrowed array.
        // Uses same principle as `key_commands` function.
        Line::from(
            [
                Span::raw(" "),
                Span::raw(self.brand.to_string()),
                Span::raw(" "),
                Span::raw(self.model.to_string()).gray().bold(),
                Span::raw(" "),
            ]
            .as_ref(),
        )
    }

    fn key_commands(&self) -> Line<'static> {
        // Returns the key commands.
        // 1. Creates a local fixed-size stack array and coerces it into a reference.
        // 2. The array is borrowed and no heap memory allocation is required.
        // 3. Uses 'static for lifetime binding because the strings must live for the entire duration of the program.
        Line::from(
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
        )
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
            _ => {} // Left empty so random key slips don't instantly close the app,
        }

        Ok(())
    }

    fn switch_watch_modes(&mut self) {
        // Deterministic mode switching sequence
        match self.active_mode {
            WatchMode::Timekeeping => self.switch_mode(WatchMode::Alarm),
            WatchMode::Alarm => self.switch_mode(WatchMode::Stopwatch),
            WatchMode::Stopwatch => self.switch_mode(WatchMode::DualTime),
            WatchMode::DualTime => self.switch_mode(WatchMode::Timekeeping),
        }
    }

    fn switch_mode(&mut self, mode: WatchMode) {
        self.active_mode = mode;
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

    fn get_colors(&self) -> (Color, Color) {
        // Get light on/off color

        let (fg_col, bg_col) = if self.light_on {
            (Color::Rgb(0, 0, 0), Color::Rgb(80, 158, 49))
        } else {
            (Color::Rgb(0, 0, 0), Color::Rgb(43, 84, 27))
        };

        (fg_col, bg_col)
    }

    fn run_background_tasks(&mut self) {
        // Update live clock
        self.update_live_clock();

        // Handle light switch
        self.light_off();
    }
}

fn draw_lcd(
    bg: Color,
    fg: Color,
    area: Rect,
    buf: &mut Buffer,
) -> ([Rect; 2], [Rect; 1], [Rect; 2]) {
    // Creates a three layer layout and forces each layer to return a fixed sized array.

    // Watch LCD height and width
    let lcd_height = 9; // cells
    let lcd_width = 30;

    let fractional_height = lcd_height / 3; // total_height/number of rows
    let franctional_width = lcd_width / 2; // total_width/number of columns

    // V-center: Float the entire watch module vertically.
    let vertical_center = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(lcd_height)])
        .flex(Flex::Center)
        .split(area); // Use inner area

    // H-center: Set width size and center it.
    let horizontal_center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(lcd_width)])
        .flex(Flex::Center)
        .split(vertical_center[0]);

    // Fill the LCD with BG and define FG
    Block::new().bg(bg).fg(fg).render(horizontal_center[0], buf);

    // Rows
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(fractional_height),
            Constraint::Length(fractional_height),
            Constraint::Length(fractional_height),
        ])
        .split(horizontal_center[0]);

    // Inner first
    let inner_first_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(franctional_width),
            Constraint::Length(franctional_width),
        ])
        .areas(rows[0]);

    // Inner second
    let inner_second_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(lcd_width)])
        .areas(rows[1]);

    // Inner third
    let inner_third_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(franctional_width),
            Constraint::Length(franctional_width),
        ])
        .areas(rows[2]);

    (inner_first_layout, inner_second_layout, inner_third_layout)
}

fn draw_paragraph(text: &str, optional_block: Option<Block>, area: Rect, buf: &mut Buffer) {
    let block = match optional_block {
        Some(unwrapped) => unwrapped, // Return unwrapped block
        None => Block::bordered().border_set(border::PLAIN),
    };

    Paragraph::new(text)
        .centered()
        .bold()
        .block(block)
        .render(area, buf);
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // === SETUP ===
        // todo implement self.active_mode;

        // Alarm Func: todo implement
        let alarm_functions = "SNZ  ALM  SIG";

        // Foreground and Background colors
        let (fg_col, bg_col) = self.get_colors();

        // === LAYOUT ===
        // Full screen area
        let full_area = Block::bordered()
            .border_set(border::THICK)
            .fg(Color::Gray)
            .title(self.header().centered())
            .title_bottom(self.key_commands().centered());

        // Define the watch LCD area within the full screen area
        let lcd_area = full_area.inner(area);

        full_area.render(area, buf);

        // Draw LCD screen & layout
        let (first_row, second_row, last_row) = draw_lcd(bg_col, fg_col, lcd_area, buf);

        // === DISPLAYS ===
        // Day Display
        draw_paragraph(self.day.as_str(), None, first_row[0], buf);
        // Alarm Functions Display
        draw_paragraph(
            alarm_functions,
            Some(
                Block::new()
                    .borders(Borders::BOTTOM)
                    .padding(Padding::top(1)),
            ),
            first_row[1],
            buf,
        );
        // Clock Display
        draw_paragraph(self.clock.as_str(), None, second_row[0], buf);
        // Year Display
        draw_paragraph(self.year.as_str(), None, last_row[0], buf);
        // Date/Month Display
        draw_paragraph(self.date_month.as_str(), None, last_row[1], buf);
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

        // Draw using standard closure execution
        terminal
            .draw(|frame| {
                app.draw(frame);
            })
            .unwrap();

        // Extract the text matrix to inspect rendering frames
        let buffer = terminal.backend().buffer();
        let string_representation = format!("{buffer:?}");

        assert!(string_representation.contains("Monday"));
        assert!(string_representation.contains("12:00:00"));
        assert!(string_representation.contains("Mode"));
        assert!(string_representation.contains("Light"));
        assert!(string_representation.contains("Quit"));
    }
}
