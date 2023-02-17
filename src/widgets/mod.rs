pub mod bottom_bar;
pub mod dashboard;
pub mod exercises;
pub mod help;
pub mod listblock;
pub mod tracks;

use crate::layout::divider;

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

pub fn draw_blocks<B: Backend>(frame: &mut Frame<B>, layout: divider::Layouts) {
    let top_main = Paragraph::new("Welcome to the exercism cli")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    let tracks = tracks::tracks();
    let exercises = exercises::exercises();
    let description = dashboard::dashboard();
    let bottom_bar = bottom_bar::bottom_bar();

    frame.render_widget(top_main, layout.top_main);
    frame.render_widget(tracks, layout.tracks);
    frame.render_widget(exercises, layout.exercises);
    frame.render_widget(description, layout.description);
    frame.render_widget(bottom_bar, layout.logs);
}
