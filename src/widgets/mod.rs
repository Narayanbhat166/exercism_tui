pub mod bottom_bar;
pub mod dashboard;
pub mod exercises;
pub mod help;
pub mod listblock;
pub mod tracks;

use crate::layout::divider;


use std::{error::Error};

use tui::{
    backend::{Backend},
    layout::{Alignment},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
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
