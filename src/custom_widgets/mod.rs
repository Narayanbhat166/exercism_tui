pub mod bottom_bar;
pub mod description;
pub mod exercise_information;
pub mod exercises;
pub mod help;
pub mod listblock;
pub mod track_information;
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

    let tracks = tracks::tracks(app);
    let exercises = exercises::exercises(app);

    let track_information = track_information::track_information(app);
    let exercise_information = exercise_information::exercise_information(app);
    let bottom_bar = bottom_bar::bottom_bar();
    let help_table = help::help_table();

    frame.render_stateful_widget(tracks, layout.tracks, &mut app.tracks.state);
    frame.render_stateful_widget(exercises, layout.exercises, &mut app.exercises.state);

    frame.render_widget(top_main, layout.top_main);
    frame.render_widget(track_information, layout.tracks_information);
    frame.render_widget(exercise_information, layout.exercise_information);
    frame.render_widget(description::description(app), layout.description);
    frame.render_widget(help_table, layout.help);
    frame.render_widget(bottom_bar, layout.logs);
}
