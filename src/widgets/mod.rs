pub mod bottom_bar;
pub mod dashboard;
pub mod exercises;
pub mod help;
pub mod listblock;
pub mod tracks;

use crate::layout::divider;
use crate::App;

use tui::{
    backend::Backend,
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn draw_blocks<B: Backend>(frame: &mut Frame<B>, layout: divider::Layouts, app: &mut App) {
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
    let help_table = help::help_table();

    frame.render_widget(top_main, layout.top_main);
    frame.render_stateful_widget(tracks, layout.tracks, &mut app.tracks.state);
    frame.render_widget(exercises, layout.exercises);
    frame.render_widget(description, layout.description);
    frame.render_widget(help_table, layout.help);
    frame.render_widget(bottom_bar, layout.logs);
}
