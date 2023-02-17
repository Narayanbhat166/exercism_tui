use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};
use tui::layout::Rect;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Row, Table},
    Frame, Terminal,
};

pub struct Layouts {
    pub top_main: Rect,
    pub tracks: Rect,
    pub exercises: Rect,
    pub description: Rect,
    pub help: Rect,
    pub logs: Rect,
}

pub fn layout_divider(main_terminal_size: Rect) -> Layouts {
    let main_inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(4),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(main_terminal_size);

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_inner_layout[1]);

    let middle_left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(middle_chunks[0]);

    let middle_left_tracks_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(middle_left_layout[0]);

    Layouts {
        top_main: main_inner_layout[0],
        tracks: middle_left_tracks_layout[0],
        exercises: middle_left_tracks_layout[1],
        description: middle_chunks[1],
        help: middle_left_layout[1],
        logs: main_inner_layout[2],
    }
}
