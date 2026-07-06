use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    widgets::{Block, Padding, Paragraph, Widget},
};

pub fn draw_lcd(
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
    let fractional_width = lcd_width / 2; // total_width/number of columns

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
            Constraint::Length(fractional_width),
            Constraint::Length(fractional_width),
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
            Constraint::Length(fractional_width),
            Constraint::Length(fractional_width),
        ])
        .areas(rows[2]);

    (inner_first_layout, inner_second_layout, inner_third_layout)
}

pub fn draw_paragraph(text: &str, optional_block: Option<Block>, area: Rect, buf: &mut Buffer) {
    let block = match optional_block {
        Some(unwrapped) => unwrapped, // Return unwrapped block
        None => Block::bordered().border_set(border::PLAIN),
    };

    Paragraph::new(text)
        .centered()
        .block(block)
        .render(area, buf);
}
