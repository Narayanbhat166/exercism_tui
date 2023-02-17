use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};
use tui::widgets::Widget;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Row, Table},
    Frame, Terminal,
};

pub fn dashboard() -> impl widgets::Widget {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Description")
}
