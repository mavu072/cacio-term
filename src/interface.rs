use crate::AppState; // Import from main.rs

use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Stylize};
use ratatui::symbols::border;
use ratatui::text::{Line, Span, ToSpan};
use ratatui::widgets::{Block, Paragraph};

// Before: fn render(frame: &mut Frame, state: &mut AppState)
// Now: Changed to a shared, read-only reference
pub fn render(frame: &mut Frame, state: &AppState) {
    // Get datetime
    let datetime = state.datetime.to_span();
    let bg_col = if state.light_on {
        Color::Green
    } else {
        Color::Black
    };

    // Title
    // Before: `vec![...]` allocated temporary chunks of heap memory dozens of times a second.
    // Now: `[...]` stays flat on the stack, and `.as_ref()` borrows it as a slice with zero heap overhead.
    let title = Line::from(
        [
            Span::raw(" "),
            state.brand.to_span(),
            Span::raw(" "),
            state.model.to_span().red().bold(),
            Span::raw(" "),
        ]
        .as_ref(),
    );

    // Bottom
    let bottom = Line::from(
        [
            " Adjust ".into(),
            "<A>".blue().bold(),
            " Mode ".into(),
            "<M>".blue().bold(),
            " Light ".into(),
            "<L>".blue().bold(),
            " 12/24H ".into(),
            "<H>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]
        .as_ref(),
    );

    // Block
    let display_block = Block::bordered()
        .title(title)
        .title_alignment(Alignment::Center)
        .title_bottom(bottom.centered())
        .border_set(border::THICK);

    // Datetime Display
    let datetime_display = Paragraph::new(datetime)
        .centered()
        .yellow()
        .bg(bg_col)
        .block(display_block);

    // Render
    frame.render_widget(datetime_display, frame.area());
}

#[cfg(test)]
mod tests {
    use super::*;
    // 1. Import Ratatui's virtual test simulator dependencies
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    #[test]
    fn test_render_layout() {
        // 2. Initialize your manual state
        let app = AppState {
            brand: "cacio".to_string(),
            model: "term".to_string(),
            datetime: "Monday, 12:00:00".to_string(),
            hour_format: 12,
            light_timer: 3,
            exit: false,
            light_on: false,
        };

        // 3. Spawn a virtual terminal in memory with a mock 50x5 grid viewport
        let backend = TestBackend::new(50, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        // 4. Draw using your standard closure execution flow
        terminal
            .draw(|frame| {
                render(frame, &app);
            })
            .unwrap();

        // 5. Extract the text matrix to inspect rendering frames
        let buffer = terminal.backend().buffer();
        let string_representation = format!("{buffer:?}");

        // 6. Assertions pass cleanly without layout errors
        assert!(string_representation.contains("cacio"));
        assert!(string_representation.contains("term"));
        assert!(string_representation.contains("Monday, 12:00:00"));
        assert!(string_representation.contains("Mode"));
        assert!(string_representation.contains("Light"));
        assert!(string_representation.contains("Quit"));
    }
}
