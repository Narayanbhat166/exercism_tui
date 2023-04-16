pub mod bottom_bar;
pub mod description;
pub mod exercise_information;
pub mod exercises;
pub mod help;
pub mod listblock;
pub mod track_information;
pub mod tracks;

use std::sync::{Arc, Mutex};

use crate::layout::divider;
use crate::App;

use tui::{
    backend::Backend,
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn draw_blocks<B: Backend>(
    frame: &mut Frame<B>,
    layout: divider::Layouts,
    app: Arc<Mutex<App>>,
) {
    let top_main = Paragraph::new("Welcome to the exercism cli")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    // If possible try to avoid this
    {
        let mut app = app.lock().unwrap();
        app.description.max_height = layout.description.height;
    }

    let app = app.as_ref();

    let tracks = tracks::tracks(app);
    let exercises = exercises::exercises(app);

    let track_information = track_information::track_information(app);
    let exercise_information = exercise_information::exercise_information(app);
    let bottom_bar = bottom_bar::bottom_bar(app);
    let help_table = help::help_table();

    {
        let mut app = app.lock().unwrap();
        frame.render_stateful_widget(tracks, layout.tracks, &mut app.tracks.state);
        frame.render_stateful_widget(exercises, layout.exercises, &mut app.exercises.state);
        frame.render_widget(description::description(&mut app), layout.description);
    }

    frame.render_widget(top_main, layout.top_main);
    frame.render_widget(track_information, layout.tracks_information);
    frame.render_widget(exercise_information, layout.exercise_information);
    frame.render_widget(help_table, layout.help);
    // Draw the throbber onyl if there are some logs to be printed
    // This can happen if there is some network activity going on
    if let Some(bottom_widget) = bottom_bar {
        frame.render_widget(bottom_widget, layout.logs);
    }
}
