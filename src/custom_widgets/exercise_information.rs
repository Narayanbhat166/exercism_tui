

use tui::{
    text::Text,
    widgets::{self, Paragraph},
    widgets::{Block, Borders, Wrap},
};

use crate::App;

pub fn exercise_information(app: &App) -> impl widgets::Widget {
    let exercise_title;
    if let Some(current_exercise) = app.exercises.get_current_item() {
        exercise_title = current_exercise.title;
        Paragraph::new(Text::from(current_exercise.blurb))
    } else {
        exercise_title = String::new();
        Paragraph::new(Text::from(
            "Select an exercise to view it's information".to_string(),
        ))
    }
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .title(format!("Exercise [{}]", exercise_title))
            .border_type(widgets::BorderType::Rounded)
            .borders(Borders::ALL),
    )
}
