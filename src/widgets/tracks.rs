use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Row, Table},
    Frame, Terminal,
};

/// Tracks that are available and chosen
/// Differenciate between available and chosen
/// TODO: provide a filter for the above functionality
pub fn tracks() -> impl widgets::Widget {
    List::new([ListItem::new("Tracks available")])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Tracks"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol("█")
}