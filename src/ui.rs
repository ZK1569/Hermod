use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

use crate::app::{App, InputMode};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // ========== Header ==========
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Hermod v0.1",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);
    // ========== End Header ==========
    // ========== Messages ==========
    match app.input_mode {
        InputMode::Normal => {}

        #[allow(clippy::cast_possible_truncation)]
        InputMode::Editing => frame.set_cursor_position(Position::new(
            chunks[2].x + app.character_index as u16 + 1,
            chunks[2].y + 1,
        )),
    }

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let lines = vec![
                Line::from(format!("{i}:")),
                Line::from(format!("{}", m.1)),
                Line::from(format!("")),
            ];

            let aligned_lines: Vec<Line> = if m.0 {
                lines.into_iter().map(|line| line.right_aligned()).collect()
            } else {
                lines
            };

            ListItem::new(Text::from(aligned_lines))
        })
        .collect();
    let messages = List::new(messages).block(Block::bordered().title("Messages"));
    frame.render_widget(messages, chunks[1]);

    // ========== END Messages ==========
    // ========== Input ==========
    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Input"));
    frame.render_widget(input, chunks[2]);
    // ========== END Input ==========
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
